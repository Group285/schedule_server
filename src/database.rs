use chrono::prelude::*;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

use crate::client::RawSubject;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: i64,
    pub sort: i64,
    pub date: DateTime<Utc>,
    pub schedule_id: i64,
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
    pub id: i64,
    pub sort: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub saturday: bool,
}

pub async fn update_database(db: Database, arr: Vec<RawSubject>) -> Option<i8> {
    let subjects_coll = db.collection::<Subject>("subjects");
    let lessons_coll = db.collection::<Lesson>("lessons");
    let schedule_coll = db.collection::<Schedule>("schedule");

    println!("parse subjects");
    let subjects: Vec<Subject> = arr
        .iter()
        .map(|raw_subject| Subject {
            id: raw_subject.subjectId,
            subject: raw_subject.subject.to_string(),
        })
        .collect();

    subjects_coll.insert_many(subjects, None).await.unwrap();

    let mut lessons: Vec<Lesson> = Vec::new();
    for raw_subject in arr {
        println!("parse lessons from\n{:#?}", raw_subject);
        let date = DateTime::from_utc(
            NaiveDateTime::from_timestamp_opt(raw_subject.date + 25200, 0)?,
            Utc,
        );

        println!("date from raw subject {}", date);
        println!(
            "getting schedule with\nsort: {}\nweekday: {}",
            raw_subject.sort + 1,
            date.weekday().number_from_monday() == 6
        );
        let schedule = schedule_coll
            .find_one(
                doc! {
                    "sort": raw_subject.sort + 1,
                    "weekday": date.weekday().number_from_monday() == 6
                },
                None,
            )
            .await
            .ok()?
            .unwrap();
        lessons.push(Lesson {
            id: raw_subject.id,
            sort: raw_subject.sort,
            date,
            schedule_id: schedule.id,
            subject_id: raw_subject.subjectId,
            teacher: raw_subject.teacher,
            classroom: raw_subject.classroom,
        });
    }

    lessons_coll.insert_many(lessons, None).await.unwrap();

    println!("subject and lessons sended successfully");

    Some(4)
}
