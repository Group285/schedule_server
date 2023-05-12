use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ScheduleListOptions {
    pub from: i64,
    pub to: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserMarksOptions {
    pub from: i64,
    pub to: i64,
    pub uid: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserListOptions {
    pub name: Option<String>,
    pub uid: Option<String>
}

#[derive(Debug, Deserialize)]
pub(crate) struct RegisterOptions {
    pub uid: String,
}
