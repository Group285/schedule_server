use warp::Filter;

pub fn get_filters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::get_schedule_request()
        .or(filters::post_new_day())
        .or(filters::update_day())
        .or(filters::update_schedule())
        .or(filters::delete_day())
}

mod filters {
    use warp::{path, Filter};

    pub(crate) fn get_schedule_request(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("schedule")
            .and(warp::get())
            // .and(warp::query::<ListOptions>())
            // .and(with_db(db))
            .and_then(super::handlers::list_schedule)
    }

    pub(crate) fn post_new_day(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("schedule")
            .and(warp::post())
            // WARNING: can use too much memory
            .and(warp::body::json())
            .and_then(super::handlers::add_day)
    }

    pub(crate) fn update_day(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("schedule" / u64)
            .and(warp::put())
            .and(warp::body::json())
            .and_then(super::handlers::update_day)
    }

    pub(crate) fn delete_day(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("schedule" / u64)
            .and(warp::delete())
            .and_then(super::handlers::delete_day)
    }

    pub(crate) fn update_schedule(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("dataupdate")
            .and(warp::put())
            .and_then(super::handlers::update_schedule)
    }
}

mod handlers {
    use std::convert::Infallible;

    use reqwest::StatusCode;

    use crate::database::Mark;

    pub async fn list_schedule() -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }

    pub async fn add_day(day: Vec<Mark>) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }

    pub async fn update_day(id: u64, day: Vec<Mark>) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }

    pub async fn delete_day(id: u64) -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }

    pub async fn update_schedule() -> Result<impl warp::Reply, Infallible> {
        Ok(StatusCode::OK)
    }
}
