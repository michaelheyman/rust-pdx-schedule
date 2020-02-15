extern crate diesel;
extern crate rust_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;

fn main() {
    use self::schema::*;

    let connection = establish_connection();
    let query : Vec<(ClassOffering, Instructor)> = classoffering::table
        .inner_join(instructor::table)
        .limit(5)
        .load(&connection)
        .expect("Error loading photos");

    println!("Displaying {} class offerings join", query.len());
    for (co, i) in query {
        println!("{:?}", co.class_offering_id);
        println!("{:?}", i.full_name);
    }
}
