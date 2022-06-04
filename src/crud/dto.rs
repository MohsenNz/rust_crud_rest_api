use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub contacts: Vec<String>,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "users")]
pub struct Birthday {
    pub phone_number: String,
    pub birthday: NaiveDate,
}

#[derive(Deserialize)]
pub struct Contacts {
    pub members: Vec<String>,
}
