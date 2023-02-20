use log::{debug, info, warn};
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};

use crate::client::rawdata::RawData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub _id: i64,
    pub sort: i64,
    pub date: i64,
    pub start: i64,
    pub end: i64,
    pub subject_id: i64,
    pub classroom: Classroom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Classroom {
    pub _id: i64,
    pub title: String,
    pub has_computers: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub _id: i64,
    pub subject: String,
    pub teacher_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mark {
    pub _id: i64,
    pub lesson_id: i64,
    pub user_id: i64,
    pub mark: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthMark {
    pub _id: i64,
    pub subject_id: i64,
    pub user_id: i64,
    pub mark: String,
    pub month: String,
}

pub async fn add_lessons_to_database(db: &Database, raw_data: Vec<RawData>) -> Option<Vec<Lesson>> {
    info!("adding lessons to database");

    info!("creating collection objects");
    let lessons_coll = db.collection::<Lesson>("lessons");

    info!("parse lessons");
    let lessons: Vec<Lesson> = raw_data
        .iter()
        .map(|data| {
            let lesson = data.get_lesson();
            debug!("lesson parsed: {:#?}", lesson);
            lesson
        })
        .collect();

    let mut insertions = Vec::new();

    for lesson in lessons.clone() {
        insertions.push(lessons_coll.insert_one(lesson, None))
    }

    for result in insertions {
        if let Err(msg) = result.await {
            warn!("insertion error: {:#?}", msg);
        }
    }

    info!("lessons sended successfully");

    Some(lessons)
}

pub async fn add_subjects_to_database(
    db: &Database,
    raw_data: Vec<RawData>,
) -> Option<Vec<Subject>> {
    info!("adding subjects to database");

    debug!("creating collection objects");
    let subjects_coll = db.collection::<Subject>("subjects");

    debug!("parse subjects");
    let subjects: Vec<Subject> = raw_data
        .iter()
        .map(|data| {
            let subject = data.get_subject();
            debug!("lesson parsed: {:#?}", subject);
            subject
        })
        .collect();

    let mut insertions = Vec::new();

    for subject in subjects.clone() {
        insertions.push(subjects_coll.insert_one(subject, None))
    }

    for result in insertions {
        if let Err(msg) = result.await {
            warn!("insertion error: {:#?}", msg);
        }
    }

    info!("subjects sended successfully");

    Some(subjects)
}

pub async fn update_lessons(db: &Database, from: i64, to: i64) -> Option<Vec<Lesson>> {
    info!("updating database");

    db.collection::<Lesson>("lessons")
        .delete_many(
            doc! {
                    "date": doc! {
                        "$gt": from - 1,
                        "$lt": to + 1
                    }
            },
            None,
        )
        .await
        .ok()?;

    let lessons = add_lessons_to_database(db, RawData::get(from, to).await?).await?;

    Some(lessons)
}
