use mongodb::Database;
use warp::Filter;

mod filters;
mod handlers;


mod modules {
    use serde::Deserialize;
    use warp::reject;

    #[derive(Debug, Deserialize)]
    pub(crate) struct ScheduleListOptions {
        pub from: Option<i64>,
        pub to: Option<i64>,
    }

    #[derive(Debug)]
    pub(crate) struct Unauthtorized;
    impl reject::Reject for Unauthtorized {}
}

pub fn get_filters(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::get_schedule_request(&db)
        .or(filters::post_new_mark(&db))
        .or(filters::update_mark(&db))
        .or(filters::delete_mark(&db))
}

