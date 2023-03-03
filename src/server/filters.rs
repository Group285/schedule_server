use mongodb::Database;
use warp::{path, Filter};

use super::modules::{RegisterOptions, ScheduleListOptions};

pub fn get_schedule_request(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule")
        .and(warp::get())
        .and(warp::query::<ScheduleListOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::list_schedule)
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
