use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Error, Formatter};
use std::time::SystemTime;

use chrono::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RawSubject {
    id: usize,
    groupId: usize,
    group: String,
    date: usize,
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

#[derive(Serialize, Debug)]
struct Subject {
    id: usize,
    subject: String,
    teacher: String,
}

#[derive(Debug)]
struct WrongGroupError {}

impl Display for WrongGroupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Raw subject list got a wrong group schedule, check for errors")
    }
}

impl error::Error for WrongGroupError {}

#[derive(Serialize, Debug)]
struct Day {
    date: DateTime<Utc>,
    subjects: Vec<Subject>,
}

impl Day {
    pub fn from_raw_subjects<T>(raw: T) -> Result<Vec<Day>, Box<dyn error::Error>>
        where T: IntoIterator<Item=RawSubject>,
    {
        let result: Vec<(usize, Vec<RawSubject>)> = raw.into_iter()
            .sorted_by(|x, y| Ord::cmp(&x.date, &y.date))
            .group_by(|r| r.date)
            .into_iter()
            .map(|x| (x.0, x.1.collect::<Vec<RawSubject>>()))
            .collect();
        Ok(result.into_iter().map(|x| Day::from_raw(x.0, x.1)).collect())
    }

    fn from_raw(date: usize, raw_subjects: Vec<RawSubject>) -> Self {
        let date = Utc.timestamp_opt((date as i64) + 25200, 0).unwrap();
        Day {
            date,
            subjects: raw_subjects.into_iter().map(|x| Subject::from_raw(&x).unwrap()).collect(),
        }
    }
}

impl Subject {
    pub fn from_raw(raw: &RawSubject) -> Result<Self, Box<dyn error::Error>> {
        if raw.group != "9ИС-285" {
            return Err(Box::try_from(WrongGroupError {}).unwrap());
        }

        Ok(Subject {
            id: raw.id,
            subject: raw.subject.clone(),
            teacher: raw.teacher.clone(),
        })
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let (from, to) = get_current_week();
    println!("Mon: {}, Sun: {}", from, to);
    let subjects = serde_json::from_str::<Vec<RawSubject>>(get_response(from, to).await.as_str())?
        .into_iter();
    let days = Day::from_raw_subjects(subjects)?;


    println!("days count: {}", days.len());
    for day in days {
        println!("{:#?}", day);
    }
    Ok(())
}

/// # get_response() -> String
/// ## Url: https://production.collegeschedule.ru:2096/schedule
/// ### Parameters:
/// * from: time in epoch where we start
/// * to: time in epoch where we end
async fn get_response(from: i64, to: i64) -> String {
    reqwest::get(format!("https://production.collegeschedule.ru:2096/schedule?from={}&to={}&groupId=34&titles=true", from, to))
        .await.unwrap()
        .text()
        .await.unwrap()
}

fn get_current_week() -> (i64, i64) {
    let now = Utc::now();
    let current_week = now.iso_week().week();
    let current_year = now.year();
    let mon = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon)
        .unwrap()
        .and_time(NaiveTime::default())
        .timestamp();
    let sun = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Sun)
        .unwrap()
        .and_time(NaiveTime::default())
        .timestamp();
    (mon, sun)
}