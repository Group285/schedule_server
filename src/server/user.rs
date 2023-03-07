// TODO: add a check for valid username
// TODO: add a check if same user._id exists

use std::convert::Infallible;

use mongodb::{bson::doc, Database};
use reqwest::StatusCode;
use warp::{path, Filter};

use crate::database::User;

use super::{filters::with_db, ServerControl, register_validation, is_admin_uid};

impl ServerControl for User {
    fn new_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("user")
            .and(warp::post())
            .and(warp::cookie("uid_schedule_token"))
            // WARNING: can use too much RAM
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(add_user)
    }

    fn delete_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("user" / String)
            .and(warp::delete())
            .and(warp::cookie("uid_schedule_token"))
            .and(with_db(db.clone()))
            .and_then(delete_user)
    }

    fn update_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("user")
            .and(warp::put())
            .and(warp::cookie("uid_schedule_token"))
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(update_user)
    }

    fn combined_filter(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        Self::new_request(db)
            .or(Self::update_request(db))
            .or(Self::delete_request(db))
    }
}

async fn add_user(uid: String, user: User, db: Database) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()) {
        return Ok(StatusCode::UNAUTHORIZED);
    }
    db.collection("users").insert_one(user, None).await.unwrap();
    Ok(StatusCode::OK)
}

async fn update_user(uid: String, user: User, db: Database) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()) {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    let user_updated = db
        .collection("users")
        .update_one(
            doc! {
                "_id": user._id.clone()
            },
            doc! {
                "$set": {
                    "username": user.username,
                    "admin": user.admin
                }
            },
            None,
        )
        .await
        .unwrap();
    if user_updated.matched_count == 0 {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}

async fn delete_user(id: String, uid: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()) {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    let user_deleted = db
        .collection("users")
        .delete_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await
        .unwrap();

    if user_deleted.deleted_count == 0 {
        Ok(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}
