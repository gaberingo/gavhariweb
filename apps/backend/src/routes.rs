// Registrasi route HTTP aplikasi.
use actix_web::web;

use crate::modules::projects;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/projects").configure(projects::route::project_route));
}
