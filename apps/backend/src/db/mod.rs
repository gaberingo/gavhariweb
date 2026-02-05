// Inisialisasi koneksi database/pool dan extractor DB untuk handler Actix.
mod query_helper;

use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorInternalServerError;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use dotenvy::dotenv;
use std::env;
use std::future::{ready, Ready};
use std::ops::{Deref, DerefMut};

pub use query_helper::{load_list, new_rec};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Extractor Actix untuk mengambil koneksi DB dari pool secara otomatis di handler.
/// Pemakaian di handler: `async fn handler(conn: DbConn) -> impl Responder { ... }`
pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Ambil `DbPool` dari app data dan kembalikan `DbConn`.
/// Jika pool tidak ada atau gagal mengambil koneksi, return 500.
impl FromRequest for DbConn {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool,
            None => return ready(Err(ErrorInternalServerError("DB pool missing"))),
        };

        match pool.get() {
            Ok(conn) => ready(Ok(DbConn(conn))),
            Err(_) => ready(Err(ErrorInternalServerError("Failed to get DB connection"))),
        }
    }
}

pub fn init_pool() -> DbPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
