use super::schema::{keys, names, players};
use chrono::{DateTime, Utc};
use diesel::r2d2;
use diesel::PgConnection;
use diesel::{self, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Player {
    pub id: i32,
    pub key_id: i32,
    pub name: String,
    pub score: i16,
    pub updated_on: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
pub struct Key {
    pub id: i32,
    pub key: Uuid,
    pub updated_on: DateTime<Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Deserialize, Serialize)]
pub struct Name {
    pub id: i32,
    pub key_id: i32,
    pub name: String,
    pub updated_on: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub key_id: i32,
    pub name: String,
}

// type alias to use in multiple places
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
