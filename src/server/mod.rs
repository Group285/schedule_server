use log::debug;
use mongodb::{Database, bson::doc};
use warp::Filter;

use crate::database::{Mark, User};

mod filters;
mod handlers;
mod mark;
mod modules;
mod user;

trait ServerControl {
    fn new_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone;

    fn delete_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone;

    fn update_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone;

    fn combined_filter(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone;
}

#[allow(opaque_hidden_inferred_bound)]
pub fn get_filters(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::get_schedule_request(&db)
        .or(filters::register(&db))
        .or(User::combined_filter(&db))
        .or(Mark::combined_filter(&db))
}

/// returns Some(user) if uid valid
pub async fn register_validation(uid: String, db: Database) -> Option<User> {
    let users = db.collection::<User>("users");

    if let Some(user) = users
        .find_one(
            doc! {
                "_id": &uid,
            },
            None,
        )
        .await
        .unwrap()
    {
        debug!("found user:\n{:#?}", user);
        Some(user)
    } else {
        None
    }
}
