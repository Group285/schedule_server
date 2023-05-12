use log::debug;
use mongodb::{bson::doc, Database};
use warp::Filter;

use crate::database::{Mark, User, MonthMark};

mod filters;
mod handlers;
mod mark;
mod modules;
mod user;
mod monthmark;

#[allow(opaque_hidden_inferred_bound)]
pub fn get_filters(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    filters::get_schedule_request(&db)
        .or(filters::register(&db))
        .or(User::combined_filter(&db))
        .or(Mark::combined_filter(&db))
        .or(MonthMark::combined_filter(&db))
}

/// return true if user is valid and admin
pub async fn is_admin_uid(uid: String, db: Database) -> bool {
    if let Some(user) = register_validation(uid, db).await {
        user.admin
    } else {
        false
    }
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
