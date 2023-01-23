use chrono::prelude::*;
use rorm::Model;
use serde::Deserialize;

#[derive(Model)]
pub struct Lesson {
    #[rorm(primary_key)]
    id: i64,
    day: NaiveDate,
    schedule_id: i64,
    subject_id: i64,
    teacher_id: i64,
    #[rorm(max_length = 16)]
    classroom: String,
}

#[derive(Model)]
pub struct Subject {
    #[rorm(id)]
    id: i64,
    #[rorm(max_length = 255)]
    subject: String,
}

#[derive(Model)]
pub struct User {
    #[rorm(primary_key)]
    id: i64,
    #[rorm(max_length = 255)]
    username: String,
    admin: bool,
}

#[derive(Model, Deserialize)]
pub struct Mark {
    #[rorm(id)]
    id: i64,
    lesson_id: i64,
    user_id: i64,
    #[rorm(max_length = 16)]
    mark: String,
}

#[derive(Model)]
pub struct Teacher {
    #[rorm(id)]
    id: i64,
    #[rorm(max_length = 255)]
    name: String,
    #[rorm(max_length = 255)]
    shortname: String,
}

#[derive(Model)]
pub struct Day {
    #[rorm(id)]
    id: i64,
}

#[derive(Model)]
pub struct Schedule {
    #[rorm(id)]
    id: i64,
    start_time: NaiveTime,
    end_time: NaiveTime,
    saturday: bool,
}

#[derive(Model)]
pub struct MonthMark {
    #[rorm(id)]
    id: i64,
    subject_id: i64,
    #[rorm(max_length = 3)]
    mark: String,
}
