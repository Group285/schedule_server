use mongodb::Database;
use warp::{path, Filter};

use super::modules::ScheduleListOptions;

#[allow(opaque_hidden_inferred_bound)]
pub fn get_schedule_request(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule")
        .and(warp::get())
        .and(warp::query::<ScheduleListOptions>())
        .and(with_db(db.clone()))
        .and_then(super::handlers::list_schedule)
}

#[allow(opaque_hidden_inferred_bound)]
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

#[allow(opaque_hidden_inferred_bound)]
pub(crate) fn update_mark(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule" / i64)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(super::handlers::update_mark)
}

#[allow(opaque_hidden_inferred_bound)]
pub(crate) fn delete_mark(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("schedule" / i64)
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(super::handlers::delete_mark)
}

#[allow(opaque_hidden_inferred_bound)]
pub(crate) fn register(
    db: &Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!("auth" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(super::handlers::auth_validation)
}

#[allow(opaque_hidden_inferred_bound)]
fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
