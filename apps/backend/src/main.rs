mod db;
mod modules;
mod routes;
mod schema;
use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inisialisasi logger (ambil level dari RUST_LOG)
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default()) // log request/response
        // Batasi ukuran payload JSON (contoh: 16 KB)
        .app_data(web::JsonConfig::default().limit(16 * 1024))
        .app_data(web::Data::new(pool.clone()))
        .route("/", web::get().to(HttpResponse::Ok))
        .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
