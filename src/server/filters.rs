use mongodb::Database;
use warp::{path, Filter};

use super::modules::{RegisterOptions, ScheduleListOptions, UserMarksOptions};

pub fn get_schedule_request(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule")
        .and(warp::get())
        .and(warp::cookie("uid_schedule_token"))
        .and(warp::query::<ScheduleListOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::list_schedule_with_marks)
}

pub fn get_schedule_request_admin(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule/admin")
        .and(warp::get())
        .and(warp::cookie("uid_schedule_token"))
        .and(warp::query::<ScheduleListOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::list_schedule)
}

pub fn get_user_marks(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("marks")
        .and(warp::get())
        .and(warp::cookie("uid_schedule_token"))
        .and(warp::query::<UserMarksOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::get_user_marks)
}

pub(crate) fn register(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("auth")
        .and(warp::get())
        .and(warp::query::<RegisterOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::auth_validation)
}

pub fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
