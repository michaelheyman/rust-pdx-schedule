extern crate diesel;
extern crate rust_pdx_schedule;
extern crate serde;
extern crate serde_json;

use self::models::*;
use diesel::prelude::*;
use rust_pdx_schedule::*;
use serde_json::{json, Value};

type ClassOfferingResult = (ClassOffering, Instructor, Course, Term);

fn main() {
    use self::schema::*;

    let connection = establish_connection();
    let query: Vec<ClassOfferingResult> = classoffering::table
        .inner_join(instructor::table)
        .inner_join(course::table)
        .inner_join(term::table)
        .limit(5)
        .load(&connection)
        .expect("Error loading photos");

    println!("Displaying {} class offerings join", query.len());
    for class in query {
        let json_object = create_json_object(class);

        println!("{}", serde_json::to_string_pretty(&json_object).unwrap());
    }
}

fn create_json_object(class_offering_result: ClassOfferingResult) -> Value {
    let class_offering = class_offering_result.0;
    let instructor = class_offering_result.1;
    let course = class_offering_result.2;
    let term = class_offering_result.3;

    json!({
        "id": class_offering.id,
        "credits": class_offering.credits,
        "days": class_offering.days,
        "time": class_offering.time,
        "crn": class_offering.crn,
        "timestamp": class_offering.timestamp,
        "course": course,
        "instructor": instructor,
        "term": term
    })
}
