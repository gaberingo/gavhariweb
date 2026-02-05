use actix_web::web::{self,ServiceConfig};
use super::handler;

pub fn project_route(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(handler::list_projects));
    cfg.route("/search", web::post().to(handler::search_projects));
    cfg.route("/create", web::post().to(handler::new_projects));
}
