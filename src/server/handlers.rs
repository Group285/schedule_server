use futures::StreamExt;
use mongodb::{bson::doc, options::FindOptions, Database};
use serde_json::json;
use std::convert::Infallible;
use warp::{
    http::{header, Response, StatusCode},
    hyper, Rejection,
};

use crate::database::{Lesson, Mark, User};

use super::modules::{ScheduleListOptions, Unauthtorized};

pub(crate) async fn list_schedule(
    data: ScheduleListOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    let lessons = db.collection::<Lesson>("lessons");
    // FIXME: unwrap raise a error
    let mut lessons_cursor = lessons
        .find(
            None,
            FindOptions::builder()
                .min(doc! {"date": data.from})
                .max(doc! {"date": data.to})
                .build(),
        )
        .await
        .unwrap();

    let mut list = Vec::new();
    while let Some(res) = lessons_cursor.next().await {
        list.push(res.unwrap());
    }

    Ok(warp::reply::json(&list))
}

pub(crate) async fn list_users(db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn get_user_marks(db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn add_mark(mark: Mark, db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn update_mark(
    id: i64,
    mark: Mark,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn delete_mark(id: i64, db: Database) -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn auth_validation(
    uid: String,
    db: Database,
) -> Result<impl warp::Reply, Rejection> {
    let users = db.collection::<User>("users");

    if let Some(user) = users
        .find_one(
            doc! {
                "uid": &uid
            },
            None,
        )
        .await
        .unwrap()
    {
        let cookie = format!("uid_schedule_token={}", &uid);

        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::SET_COOKIE, cookie)
            .body(hyper::Body::empty())
            .unwrap();

        Ok(response)
    } else {
        Err(warp::reject::custom(Unauthtorized))
    }
}
