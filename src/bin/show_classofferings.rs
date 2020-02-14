extern crate diesel;
extern crate rust_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;

fn main() {
    use self::schema::classoffering::dsl::*;

    let connection = establish_connection();
    let results = classoffering
        .limit(5)
        .load::<ClassOffering>(&connection)
        .expect("Error loading class offerings");

    println!("Displaying {} class offerings", results.len());
    for c in results {
        println!("{}", c.class_offering_id);
        println!("{}", c.course_id);
        println!("{:?}", c.instructor_id);
        println!("{:?}", c.term);
        println!("{}", c.credits);
        println!("{:?}", c.days);
        println!("{:?}", c.time);
        println!("{}", c.crn);
        println!("{:?}", c.timestamp);
        println!("----------\n");
    }
}
