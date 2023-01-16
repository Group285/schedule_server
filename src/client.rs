use chrono::prelude::*;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RawSubject {
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

/// # get_response() -> String
/// ## Url: https://production.collegeschedule.ru:2096/schedule
/// ### Parameters:
/// - from: time in epoch where we start
/// - to: time in epoch where we end
async fn get_response(from: i64, to: i64) -> String {
    reqwest::get(format!(
        "https://production.collegeschedule.ru:2096/schedule?from={}&to={}&groupId=34&titles=true",
        from, to
    ))
        .await?
        .text()
        .await?
}

fn get_current_week(time: DateTime<Utc>) -> (i64, i64) {
    let current_week = now.iso_week().week();
    let current_year = now.year();
    let mon = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Mon)?
        .and_time(NaiveTime::default())
        .timestamp();
    let sun = NaiveDate::from_isoywd_opt(current_year, current_week, Weekday::Sun)?
        .and_time(NaiveTime::default())
        .timestamp();
    (mon, sun)
}