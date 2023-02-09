use log::info;
use mongodb::{Collection, Database};
use serde::Serialize;
use warp::Rejection;
use warp::{path, Filter};

use crate::database::User;

use super::modules::ScheduleListOptions;

pub fn get_schedule_request(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule")
        .and(warp::get())
        .and(warp::query::<ScheduleListOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::list_schedule)
}

pub(crate) fn post_new_mark(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule")
        .and(warp::post())
        // WARNING: can use too much RAM
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(super::handlers::add_mark)
}

pub(crate) fn update_mark(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule" / i64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(super::handlers::update_mark)
}

pub(crate) fn delete_mark(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule" / i64)
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(super::handlers::delete_mark)
}

pub(crate) fn register(db: &Database) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("auth" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(super::handlers::auth_validation)
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
