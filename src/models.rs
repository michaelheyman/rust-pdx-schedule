extern crate chrono;
extern crate serde;
use self::serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Term {
    pub date: i32,
    pub description: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Instructor {
    pub id: i32,
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub rating: Option<f64>,
    pub url: Option<String>,
    pub timestamp: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Course {
    pub id: i32,
    pub name: String,
    pub class: String,
    pub discipline: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ClassOffering {
    pub class_offering_id: i32,
    pub course_id: i32,
    pub instructor_id: Option<i32>,
    pub term: Option<i32>,
    pub credits: i32,
    pub days: Option<String>,
    pub time: Option<String>,
    pub crn: i32,
    pub timestamp: Option<chrono::NaiveDateTime>,
}
