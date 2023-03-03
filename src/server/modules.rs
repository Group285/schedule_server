use serde::Deserialize;
use warp::reject;

#[derive(Debug, Deserialize)]
pub(crate) struct ScheduleListOptions {
    pub from: i64,
    pub to: i64,
}
#[derive(Debug, Deserialize)]
pub(crate) struct RegisterOptions {
    pub uid: String,
}
