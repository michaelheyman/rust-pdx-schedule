extern crate diesel;
extern crate rust_diesel;
extern crate serde;
extern crate serde_json;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;
use serde_json::{json, Value};

fn main() {
    use self::schema::*;

    let connection = establish_connection();
    let query: Vec<(ClassOffering, Instructor, Course, Term)> = classoffering::table
        .inner_join(instructor::table)
        .inner_join(course::table)
        .inner_join(term::table)
        .limit(1)
        .load(&connection)
        .expect("Error loading photos");

    println!("Displaying {} class offerings join", query.len());
    for (co, i, c, t) in query {
        let json_object = create_json_object(co, i, c, t);

        println!("{}", serde_json::to_string_pretty(&json_object).unwrap());
    }
}

fn create_json_object(co: ClassOffering, i: Instructor, c: Course, t: Term) -> Value {
    json!({
        "id": co.class_offering_id,
        "credits": co.credits,
        "days": co.days,
        "time": co.time,
        "crn": co.crn,
        "timestamp": co.timestamp,
        "course": c,
        "instructor": i,
        "term": t
    })
}
