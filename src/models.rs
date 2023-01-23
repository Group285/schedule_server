use chrono::prelude::*;
use rorm::{Model, Patch};

#[derive(Clone, Debug, Patch)]
#[rorm(model = "Students")]
struct Student {
    #[rorm(id)]
    id: usize,
    name: String,
}

#[derive(Clone, Debug, Patch)]
#[rorm(model = "Subjects")]
struct Subject {
    #[rorm(id)]
    id: usize,
    datetime: DateTime<Utc>,
    sort: usize,
    name: String,
    teacher_id: usize,
}

#[derive(Clone, Debug, Patch)]
#[rorm(model = "Teachers")]
struct Teacher {
    #[rorm(id)]
    id: usize,
    name: String,
}
