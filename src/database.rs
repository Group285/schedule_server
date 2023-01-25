use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    id: usize,
    date: DateTime<Utc>,
    // TODO: add subject id
    teacher: String,
    classroom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mark {
    pub id: i64,
    // TODO: add lesson id
    // TODO: add user id
    pub mark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthMark {
    pub id: i64,
    // TODO: add subject id
    // TODO: add user id
    pub mark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub id: i64,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub saturday: bool,
}


