// TODO: add a check for valid lesson_id & user_id
// TODO: add a check if same mark._id exists

use std::convert::Infallible;

use mongodb::{bson::doc, Database};
use reqwest::StatusCode;
use warp::{path, Filter};

use crate::database::Mark;

use super::{filters::with_db, is_admin_uid};

impl Mark {
    fn new_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("mark")
            .and(warp::post())
            .and(warp::cookie("uid_schedule_token"))
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(add_mark)
    }

    fn delete_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("mark" / i64)
            .and(warp::delete())
            .and(warp::cookie("uid_schedule_token"))
            .and(with_db(db.clone()))
            .and_then(delete_mark)
    }

    fn update_request(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        path!("mark")
            .and(warp::put())
            .and(warp::cookie("uid_schedule_token"))
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(update_mark)
    }

    pub fn combined_filter(
        db: &Database,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        Self::new_request(db)
            .or(Self::delete_request(db))
            .or(Self::update_request(db))
    }
}

pub(crate) async fn add_mark(
    uid: String,
    mark: Vec<Mark>,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()).await {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    db.collection("marks").insert_many(mark, None).await.unwrap();
    Ok(StatusCode::OK)
}

pub(crate) async fn update_mark(
    uid: String,
    mark: Mark,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()).await {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    let update_result = db
        .collection::<Mark>("marks")
        .update_one(
            doc! {
                "_id": mark._id
            },
            doc! {
                "$set": {
                    "lesson_id": mark.lesson_id,
                    "user_id": mark.user_id,
                    "mark": mark.mark
                }
            },
            None,
        )
        .await
        .unwrap();
    if update_result.matched_count == 0 {
        return Ok(StatusCode::BAD_REQUEST);
    }
    Ok(StatusCode::OK)
}

pub(crate) async fn delete_mark(
    id: i64,
    uid: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    if !is_admin_uid(uid, db.clone()).await {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    let marks_deleted = db
        .collection::<Mark>("marks")
        .delete_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await
        .unwrap();

    if marks_deleted.deleted_count == 0 {
        return Ok(StatusCode::BAD_REQUEST);
    }
    Ok(StatusCode::OK)
}
