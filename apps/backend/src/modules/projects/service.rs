use diesel::QueryResult;
// Business logic untuk projects.
use diesel::pg::PgConnection;

use diesel::pg::Pg;
use crate::db::{load_list, new_rec};
use crate::modules::projects::repo::NewProject;

use super::dto::{ProjectDto, ProjectFilter, ProjectListDto};
use super::repo::Project;
use crate::schema::projects;
use diesel::QueryDsl;

const PROJECT_LIST_LIMIT: i64 = 40;

fn map_project(project: Project) -> ProjectDto {
    ProjectDto {
        id: project.id,
        title: project.title,
        description: project.description,
        icon: project.icon,
        badge: project.badge,
        tags: project.tags.into_iter().flatten().collect(),
        created_at: project.created_at.to_rfc3339(),
        updated_at: project.updated_at.to_rfc3339(),
    }
}

fn map_projects(projects: Vec<Project>) -> ProjectListDto {
    let items = projects.into_iter().map(map_project).collect();
    ProjectListDto { items }
}

pub fn list_projects(
    conn: &mut PgConnection,
    filter: ProjectFilter,
) -> Result<ProjectListDto, diesel::result::Error> {
    let query = apply_project_filter(projects::dsl::projects.into_boxed(), filter);
    let projects = load_list(conn, query, PROJECT_LIST_LIMIT)?;

    Ok(map_projects(projects))
}

fn apply_project_filter<'a>(
    mut query: projects::BoxedQuery<'a, Pg>,
    filter: ProjectFilter,
) -> projects::BoxedQuery<'a, Pg> {
    use diesel::prelude::*;
    use crate::schema::projects::dsl;

    match filter {
        ProjectFilter::Query(f) => {
            if let Some(tag) = f.tag {
                query = query.filter(dsl::tags.contains(vec![Some(tag)]));
            }
            if let Some(q) = f.q {
                let pattern = format!("%{}%", q);
                query = query.filter(
                    dsl::title
                        .ilike(pattern.clone())
                        .or(dsl::description.ilike(pattern)),
                );
            }
            if !f.tags.is_empty() {
                let tags = f.tags.into_iter().map(Some).collect::<Vec<_>>();
                query = query.filter(dsl::tags.contains(tags));
            }
        }
        ProjectFilter::Json(f) => {
            if let Some(title) = f.title {
                query = query.filter(dsl::title.ilike(format!("%{}%", title)));
            }
        }
    }

    query
}

pub fn create_project(conn: &mut PgConnection, values: NewProject) -> QueryResult<Project> {
    new_rec(conn, projects::table, values)
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::map_project;
    use crate::modules::projects::repo::Project;

    #[test]
    fn map_project_skips_empty_tags() {
        let project = Project {
            id: 1,
            title: "Portfolio".to_string(),
            description: Some("desc".to_string()),
            icon: Some("icon".to_string()),
            badge: None,
            tags: vec![Some("rust".to_string()), None, Some("web".to_string())],
            created_at: Utc.with_ymd_and_hms(2026, 1, 1, 10, 0, 0).unwrap(),
            updated_at: Utc.with_ymd_and_hms(2026, 1, 2, 10, 0, 0).unwrap(),
        };

        let dto = map_project(project);
        assert_eq!(dto.tags, vec!["rust".to_string(), "web".to_string()]);
    }
}
