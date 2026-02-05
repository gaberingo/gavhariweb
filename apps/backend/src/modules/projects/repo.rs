// Akses data/persistensi projects.
// Contoh pemakaian di service:
// let projects = repo::list(conn)?;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tags: Vec<Option<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject {
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tags: Vec<Option<String>>,
}

