use log::{error, info};
use serde::Deserialize;
use serde_json::Value;
use crate::database::{Classroom, Lesson, Subject};

// example rawdata json from v4 api (https://production.collegeschedule.ru:2096/v4/schedule?from=1675875600&to=1675875600&groupId=34&classroomId=28&subjectId=952):
// [
//   {
//     "group": {
//       "title": "85",
//       "strong": "",
//       "pretty": "9ИС-285",
//       "course": 2,
//       "education": "NOT_FULL",
//       "startYear": 2021,
//       "type": "FREE",
//       "specialtyId": 5,
//       "buildingId": 1,
//       "id": 34,
//       "createdAt": 1663865341
//     },
//     "date": 1675875600,
//     "sort": 2,
//     "subgroup": null,
//     "subject": {
//       "id": 952,
//       "createdAt": 1663865340,
//       "title": "Компьютерные сети",
//       "short": "Компьютерные сети"
//     },
//     "teacher": {
//       "type": "TEACHER",
//       "firstName": "Елизавета",
//       "secondName": "Балаева",
//       "thirdName": "Максимовна",
//       "full": "Балаева Елизавета Максимовна",
//       "pretty": "Балаева Е. М.",
//       "id": 123,
//       "createdAt": 1661084419
//     },
//     "classroom": {
//       "title": "307",
//       "floor": 3,
//       "hasComputers": true,
//       "inside": true,
//       "buildingId": 1,
//       "id": 28,
//       "createdAt": 1663865340
//     },
//     "groupId": 34,
//     "subjectId": 952,
//     "teacherId": 123,
//     "classroomId": 28,
//     "start": 745,
//     "startTitle": "12:25",
//     "end": 840,
//     "endTitle": "14:00",
//     "id": 2508422,
//     "createdAt": 1675407234
//   }
// ]


#[allow(non_snake_case)]
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct RawData {
    id: i64,
    createdAt: i64,

    group: Value,
    classroom: Value,
    teacher: Value,
    subject: Value,

    // INFO: may be always null, need to check further
    subgroup: Value,

    date: i64,
    sort: i64,

    groupId: i64,
    subjectId: i64,
    teacherId: i64,
    classroomId: i64,

    start: i64,
    startTitle: i64,

    end: i64,
    endTitle: i64,
}

impl RawData {
    /// returns a lesson formatted data
    pub fn get_lesson(&self) -> Lesson {
        Lesson {
            id: self.id,
            sort: self.sort,
            date: self.date,
            start: self.start * 60,
            end: self.end * 60,
            subject_id: self.subjectId,
            classroom: self.get_classroom(),
        }
    }

    /// returns a subject formatted data
    pub fn get_subject(&self) -> Subject {
        Subject {
            id: self.subjectId,
            subject: self.subject["title"]
                .as_str()
                .unwrap()
                .to_owned(),
            teacher_id: self.teacherId,
            teacher_name: self.teacher["full"]
                .as_str()
                .unwrap()
                .to_owned(),
        }
    }

    /// returns a classroom formatted data
    fn get_classroom(&self) -> Classroom {
        Classroom {
            id: self.classroomId,
            title: self.classroom["title"]
                .as_str()
                .unwrap()
                .to_owned(),
            has_computers: self.classroom["hasComputers"]
                .as_bool()
                .unwrap(),
        }
    }

    /// ## get data from https://production.collegeschedule.ru:2096/v4/schedule
    /// returns Some(...) if `from` and `to` are valid, None if any error appears
    /// * `from: i64` - date in epoch to search from
    /// * `to: i64` - date in epoch to search to
    pub async fn get(from: i64, to: i64) -> Option<Vec<Self>> {
        if let Ok(response) = reqwest::get(format!(
            "https://production.collegeschedule.ru:2096/v4/schedule?from={}&to={}&groupId=34&titles=true",
            from, to
        ))
            .await
        {
            if let Ok(text) = response.text().await {
                let json: Vec<Self> = serde_json::from_str(text.as_str()).unwrap();
                if !json.is_empty() {
                    info!("send response successfully");
                    return Some(json);
                }
            }
        }
        error!("send response failed");
        None
    }

}
