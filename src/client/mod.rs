pub mod rawdata;

use std::future;

use crate::database::{add_lessons_to_database, add_subjects_to_database, Lesson};
use futures::StreamExt;
use log::{debug, error};
use mongodb::bson::doc;
use mongodb::Database;

use self::rawdata::RawData;

/// returns None if some error occurs
/// returns Some(...) otherwise
pub async fn get_lessons(from: i64, to: i64, db: Database) -> Option<Vec<Lesson>> {
    debug!("get_lessons call");
    let mut lessons: Vec<Lesson> = db
        .collection::<Lesson>("lessons")
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
        .ok()?
        .filter(|lesson| {
            if let Err(msg) = lesson {
                error!("get_lessons error {:#?}", msg);
                return future::ready(false);
            }
            future::ready(true)
        })
        .map(|lesson| lesson.unwrap())
        .collect()
        .await;

    // 86400 - number of seconds in day
    for x in (from..=to).step_by(86400) {
        if lessons.iter().filter(|lesson| lesson.date == x).count() == 0 {
            let raw_data = RawData::get(from, to).await?;
            lessons = add_lessons_to_database(&db, raw_data.clone()).await?;
            add_subjects_to_database(&db, raw_data.clone()).await?;
            break;
        }
    }
    Some(lessons)
}
