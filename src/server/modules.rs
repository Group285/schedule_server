use serde::Deserialize;
use warp::reject;

#[derive(Debug, Deserialize)]
pub(crate) struct ScheduleListOptions {
    pub from: Option<i64>,
    pub to: Option<i64>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct RegisterOptions {
    pub uid: String,
}


#[derive(Debug)]
pub(crate) struct Unauthtorized;
impl reject::Reject for Unauthtorized {}
