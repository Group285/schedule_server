use chrono::prelude::*;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

use crate::client::RawSubject;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: i64,
    pub date: DateTime<Utc>,
    pub subject_id: i64,
    pub teacher: String,
    pub classroom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub subject: String,
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
    pub lesson_id: i64,
    pub user_id: i64,
    pub mark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthMark {
    pub id: i64,
    pub subject_id: i64,
    pub user_id: i64,
    pub mark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub sort: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub saturday: bool,
}

pub async fn update_database(db: Database, arr: Vec<RawSubject>) -> Option<()> {
    let subjects_coll = db.collection::<Subject>("subjects");
    let lessons_coll = db.collection::<Lesson>("lessons");
    let schedule_coll = db.collection::<Schedule>("schedule");

    let subjects: Vec<Subject> = arr
        .iter()
        .map(|raw_subject| Subject {
            id: raw_subject.subjectId,
            subject: raw_subject.subject.to_string(),
        })
        .collect();

    let lessons = arr.iter().map(|raw_subject| {
        let sort = raw_subject.sort as i64;

        // FIXME: fix find_one being non blocking
        tokio::task::spawn_blocking(move || {
            schedule_coll.find_one(
                doc! {
                    "sort": sort
                },
                None,
            );
        });
    });
    Some(())
}
