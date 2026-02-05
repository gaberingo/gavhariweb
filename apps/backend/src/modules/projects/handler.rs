use actix_web::{HttpResponse, Responder, web};
use diesel::result::Error as DieselError;

use crate::db::DbConn;

use super::{dto, repo, service};

fn bad_request(message: String) -> HttpResponse {
    HttpResponse::BadRequest().body(message)
}

fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

fn to_list_response(result: Result<dto::ProjectListDto, DieselError>) -> HttpResponse {
    match result {
        Ok(payload) => HttpResponse::Ok().json(payload),
        Err(_) => internal_server_error(),
    }
}

fn handle_list_filter<F, T>(value: T, run: F) -> HttpResponse
where
    T: dto::Validatable + Into<dto::ProjectFilter>,
    F: FnOnce(dto::ProjectFilter) -> Result<dto::ProjectListDto, DieselError>,
{
    if let Err(message) = value.validate() {
        return bad_request(message);
    }

    to_list_response(run(value.into()))
}

fn to_create_response(result: Result<repo::Project, DieselError>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => internal_server_error(),
    }
}

pub async fn list_projects(mut conn: DbConn, q_filter: web::Query<dto::ProjectFilterQuery>) -> impl Responder {
    handle_list_filter(q_filter.into_inner(), |filter| {
        service::list_projects(&mut conn, filter)
    })
}

pub async fn search_projects(mut conn: DbConn, filter: web::Json<dto::ProjectFilterJson>) -> impl Responder {
    handle_list_filter(filter.into_inner(), |parsed_filter| {
        service::list_projects(&mut conn, parsed_filter)
    })
}

pub async fn new_projects(mut conn: DbConn, value: web::Json<repo::NewProject>) -> impl Responder {
    to_create_response(service::create_project(&mut conn, value.into_inner()))
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use diesel::result::Error as DieselError;

    use crate::modules::projects::dto::{ProjectFilterJson, ProjectFilterQuery};
    use crate::modules::projects::repo::Project;

    use super::{handle_list_filter, to_create_response};

    #[actix_web::test]
    async fn list_query_invalid_returns_400_with_message() {
        let invalid = ProjectFilterQuery {
            tag: Some("a".repeat(33)),
            ..Default::default()
        };

        let response = handle_list_filter(invalid, |_| panic!("service should not be called"));
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
        let body = actix_web::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"tag terlalu panjang (max 32)");
    }

    #[actix_web::test]
    async fn list_json_invalid_returns_400_with_message() {
        let invalid = ProjectFilterJson {
            title: Some("a".repeat(101)),
        };

        let response = handle_list_filter(invalid, |_| panic!("service should not be called"));
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
        let body = actix_web::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"title terlalu panjang (max 100)");
    }

    #[test]
    fn list_error_maps_to_500() {
        let valid = ProjectFilterQuery::default();

        let response = handle_list_filter(valid, |_| Err(DieselError::NotFound));
        assert_eq!(
            response.status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn create_success_maps_to_201() {
        let project = Project {
            id: 1,
            title: "Portfolio".to_string(),
            description: None,
            icon: None,
            badge: None,
            tags: vec![Some("rust".to_string())],
            created_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        };

        let response = to_create_response(Ok(project));
        assert_eq!(response.status(), actix_web::http::StatusCode::CREATED);
    }

    #[test]
    fn create_error_maps_to_500() {
        let response = to_create_response(Err(DieselError::NotFound));
        assert_eq!(
            response.status(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
