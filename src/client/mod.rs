pub mod rawdata;

use std::time::Duration;

use rawdata::RawData;
use crate::database::{Classroom, Lesson, Subject};
use chrono::prelude::*;
use futures::StreamExt;
use log::{debug, error, info};
use mongodb::bson::doc;
use mongodb::{Collection, Database, Cursor};
use serde::Deserialize;
use serde_json::Value;
use tokio::{sync::mpsc, time::interval};

/// returns None if some error occurs
/// returns Some(...) otherwise
pub async fn get_lessons(from: i64, to: i64, db: Database) -> Option<Vec<Lesson>> {
    let mut lessons = db.collection::<Lesson>("lessons")
        .find(
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

    let mut result: Vec<Lesson> = Vec::new();

    while let Some(lesson) = lessons.next().await {
        result.push(lesson.ok()?);
    }

    // TODO: check if all days are exist in database
    if result.is_empty() {
        let raw_data = RawData::get(from, to).await?;
        let lessons: Vec<Lesson> = raw_data.iter().map(|data| data.get_lesson()).collect();
        let subjects: Vec<Subject> = raw_data.iter().map(|data| data.get_subject()).collect();
    }
    Some(result)
}
