extern crate diesel;
extern crate rust_pdx_schedule;

use self::models::*;
use diesel::prelude::*;
use rust_pdx_schedule::*;

fn main() {
    use self::schema::course::dsl::*;

    let connection = establish_connection();
    let results = course
        .limit(5)
        .load::<Course>(&connection)
        .expect("Error loading courses");

    println!("Displaying {} courses", results.len());
    for c in results {
        println!("{}", c.id);
        println!("{}", c.name);
        println!("{}", c.number);
        println!("{}", c.discipline);
        println!("----------\n");
    }
}
