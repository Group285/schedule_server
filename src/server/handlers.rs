use log::debug;
use mongodb::{
    bson::{doc, Document},
    Database,
};
use std::convert::Infallible;
use warp::{
    http::{header, Response, StatusCode},
    hyper, Rejection,
};

use crate::{
    client,
    database::{Lesson, Mark, User},
};

use super::{
    modules::{RegisterOptions, ScheduleListOptions},
    register_validation,
};

pub(crate) async fn list_schedule(
    data: ScheduleListOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    let lessons = client::get_lessons(data.from, data.to, db)
        .await
        .unwrap_or(vec![]);
    Ok(warp::reply::json(&lessons))
}

// TODO: add admin check
// TODO: get login cookie
pub(crate) async fn list_schedule_with_marks(
    data: ScheduleListOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    let lessons = client::get_lessons(data.from, data.to, db)
        .await
        .unwrap_or(vec![]);
    Ok(warp::reply::json(&lessons))
}

// TODO: add admin check
pub(crate) async fn list_users(db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

// TODO: add admin check
pub(crate) async fn get_user_marks(db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn auth_validation(
    uid: RegisterOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    let mut response = Response::builder();

    if let Some(user) = register_validation(uid.uid.clone(), db).await {
        response = response
            .header(
                header::SET_COOKIE,
                format!("uid_schedule_token={}", user._id),
            )
            .status(StatusCode::OK);
    } else {
        response = response.status(StatusCode::BAD_REQUEST);
    }

    Ok(response.body(hyper::Body::empty()).unwrap())
}
