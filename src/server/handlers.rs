use mongodb::{bson::doc, Database};
use std::{collections::HashMap, convert::Infallible};
use warp::{
    http::{header, Response, StatusCode},
    hyper, Reply,
};

use crate::{
    client,
    database::{Lesson, Mark},
};

use super::{
    is_admin_uid,
    modules::{RegisterOptions, ScheduleListOptions, UserMarksOptions},
    register_validation,
};

pub(crate) async fn list_schedule(
    uid: String,
    data: ScheduleListOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()).await {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let lessons = client::get_lessons(data.from, data.to, db)
        .await
        .unwrap_or(vec![]);
    Ok(warp::reply::json(&lessons).into_response())
}

// TODO: add admin check
// TODO: get login cookie
pub(crate) async fn list_schedule_with_marks(
    uid: String,
    data: ScheduleListOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    let lessons = client::get_lessons(data.from, data.to, db.clone())
        .await
        .unwrap_or(vec![]);

    let mut result: Vec<(Lesson, Option<Mark>)> = Vec::new();

    // iterate over lessons to get all marks for each lesson provided
    for lesson in lessons {
        let mark = db
            .collection::<Mark>("marks")
            .find_one(
                doc! {
                    "lesson_id": lesson._id,
                    "user_id": &uid
                },
                None,
            )
            .await
            .unwrap();

        result.push((lesson, mark));
    }

    Ok(warp::reply::json(&result))
}

// TODO: add admin check
pub(crate) async fn get_user_marks(
    uid: String,
    data: UserMarksOptions,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()).await {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let lessons = client::get_lessons(data.from, data.to, db.clone())
        .await
        .unwrap_or(vec![]);

    let mut result: Vec<Mark> = Vec::new();

    // iterate over lessons to get all marks for each lesson provided
    for lesson in lessons {
        let mark = db
            .collection::<Mark>("marks")
            .find_one(
                doc! {
                    "lesson_id": lesson._id,
                    "user_id": &data.uid
                },
                None,
            )
            .await
            .unwrap();

        if let Some(mark) = mark {
            result.push(mark);
        }
    }

    Ok(warp::reply::json(&result).into_response())
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
