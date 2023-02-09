use chrono::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};
use log::{debug, error, info, warn};
use mongodb::bson::Document;
use mongodb::{bson::doc, Cursor, Database};
use pretty_env_logger::env_logger::Logger;
use serde::{Deserialize, Serialize};

use crate::client::RawData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: i64,
    pub sort: i64,
    pub date: i64,
    pub start: i64,
    pub end: i64,
    pub subject_id: i64,
    pub classroom: Classroom,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Classroom {
    pub id: i64,
    pub title: String,
    pub has_computers: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub subject: String,
    pub teacher_id: i64,
    pub teacher_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uid: String,
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

pub async fn update_database(db: Database, arr: Vec<RawData>) -> Option<()> {
    // TODO: rework function for new api(we have access to start/end time of lesson)
    let subjects_coll = db.collection::<Subject>("subjects"); let lessons_coll = db.collection::<Lesson>("lessons");

    info!("parse rawdata");

    let mut subjects: Vec<Subject> = Vec::new();
    let mut lessons: Vec<Lesson> = Vec::new();
    for raw_subject in arr {
        // getting subject, skip if exist
        // debug!(
        //     "getting subject with {:#?}",
        //     doc! {
        //         "id": raw_subject.subjectId
        //     },
        // );
        // let finded_subj = subjects_coll
        //     .find_one(
        //         doc! {
        //             "id": raw_subject.subjectId,
        //         },
        //         None,
        //     )
        //     .await
        //     .ok()?;
        // if let Some(subject) = finded_subj {
        //     debug!("subject found, skipping\n{:#?}", subject);
        // } else {
        //     info!("adding subject");
        //     subjects.push(Subject {
        //         id: raw_subject.subjectId,
        //         subject: raw_subject.subject,
        //     });
        // }
        // // getting lesson, skip if exist
        // debug!(
        //     "getting lesson with {:#?}",
        //     doc! {
        //         "id": raw_subject.id
        //     }
        // );
        // let finded_lesson = lessons_coll.find_one(
        //     doc! {
        //         "id": raw_subject.id
        //     },
        //     None,
        // );
        // if let Some(lesson) = finded_lesson {
        //     debug!("lesson found, skipping\n{:#?}", subject);
        // } else {
        //     info!("adding lesson");
        //     lessons.push(Lesson{
        //         id: raw_subject.id,
        //         sort: raw_subject.sort,
        //         date: raw_subject.date,
        //         start_time: raw_subject
        //     });
        // }
    }

    lessons_coll.insert_many(lessons, None).await.ok()?;
    subjects_coll.insert_many(subjects, None).await.ok()?;

    info!("subject and lessons sended successfully");

    Some(())
}
