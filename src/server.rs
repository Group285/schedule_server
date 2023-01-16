use std::convert::Infallible;
use warp::{Filter};
use warp::path::Exact;

pub fn get_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_schedule_request()
        .or(post_new_day())
        .or(update_day())
        .or(update_schedule())
}

fn get_schedule_request() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("schedule")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        // .and(with_db(db))
        .and_then(handlers::list_schedule)
}

fn post_new_day() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("schedule")
        .and(warp::post())
        // WARNING: can use too much memory
        .and(warp::body::json())
        .and_then(handlers::add_day)
}

fn update_day() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("schedule")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(handlers::update_day)
}

fn update_schedule() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dataupdate")
        .and(warp::put())
        .and_then(handlers::update_schedule)
}

mod handlers {
    use std::convert::Infallible;

    pub async fn list_schedule() -> Result<impl warp::Reply, Infallible> {
        todo!()
    }

    pub async fn add_day() -> Result<impl warp::Reply, Infallible> {
        todo!()
    }

    pub async fn update_day() -> Result<impl warp::Reply, Infallible> {
        todo!()
    }

    pub async fn update_schedule() -> Result<impl warp::Reply, Infallible> {
        todo!()
    }
}