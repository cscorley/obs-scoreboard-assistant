use super::schema::players;
use chrono::NaiveDate;
use diesel::{self, Associations, Identifiable, Insertable, Queryable};

#[derive(Identifiable, Queryable, Insertable, Associations, PartialEq, Debug)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub score: i16,
    pub updated_on: NaiveDate,
}
