use std::time::{Duration, Instant};

use chrono::prelude::*;
use serde::Deserialize;
use tokio::{sync::watch, time::interval};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawSubject {
    id: usize,
    groupId: usize,
    group: String,
    date: DateTime<Utc>,
    sort: usize,
    subjectId: usize,
    subject: String,
    teacherId: usize,
    teacher: String,
    classroomId: usize,
    classroom: String,
    startTitle: String,
    endTitle: String,
}

/// # get_response() -> watch::Receiver
/// ## Url: https://production.collegeschedule.ru:2096/schedule
/// ### Parameters:
/// - from: time in epoch where we start
/// - to: time in epoch where we end
pub(crate) async fn get_connection() -> watch::Receiver<Option<Vec<RawSubject>>> {
    let (tx, rx) = watch::channel::<Option<Vec<RawSubject>>>(None);
    let mut interval = interval(Duration::from_secs_f64(21600.0));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            if let Some((from, to)) = get_current_week(Utc::now()) {
                if let Ok(response) = reqwest::get(format!(
                    "https://production.collegeschedule.ru:2096/schedule?from={}&to={}&titles=true",
                    from, to
                ))
                .await
                {
                    if let Ok(text) = response.text().await {
                        let json: Result<Vec<RawSubject>, _> = serde_json::from_str(text.as_str());
                        tx.send(json.ok()).unwrap();
                    } else {
                        tx.send(None).unwrap();
                    }
                }
            }
        }
    });
    rx
}

pub(crate) fn get_current_week(time: DateTime<Utc>) -> Option<(i64, i64)> {
    let current_week = time.iso_week().week();
    let current_year = time.year();
    let mon = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon)?
        .and_time(NaiveTime::default())
        .timestamp();
    let sun = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Sun)?
        .and_time(NaiveTime::default())
        .timestamp();
    Some((mon, sun))
}
