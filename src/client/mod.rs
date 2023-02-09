mod rawdata;

use std::time::Duration;

use chrono::prelude::*;
use log::{info, error, debug};
use serde::Deserialize;
use tokio::{sync::mpsc, time::interval};
use serde_json::Value;
use crate::client::rawdata::RawData;
use crate::database::{Classroom, Lesson, Subject, Teacher};

pub fn get_lessons(from: i64, to: i64) -> Vec<Lesson> {
    todo!()
}

