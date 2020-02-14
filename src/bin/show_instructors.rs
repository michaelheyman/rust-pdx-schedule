extern crate diesel;
extern crate rust_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;

fn main() {
    use self::schema::instructor::dsl::*;

    let connection = establish_connection();
    let results = instructor
        .limit(5)
        .load::<Instructor>(&connection)
        .expect("Error loading instructors");

    println!("Displaying {} instructors", results.len());
    for i in results {
        println!("{}", i.id);
        println!("{}", i.full_name);
        println!("{:?}", i.first_name);
        println!("{:?}", i.last_name);
        println!("{:?}", i.rating);
        println!("{:?}", i.url);
        println!("----------\n");
    }
}
