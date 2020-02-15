extern crate diesel;
extern crate rust_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;

fn main() {
    use self::schema::*;

    let connection = establish_connection();
    let query : Vec<(ClassOffering, Instructor, Course, Term)> = classoffering::table
        .inner_join(instructor::table)
        .inner_join(course::table)
        .inner_join(term::table)
        .limit(5)
        .load(&connection)
        .expect("Error loading photos");

    println!("Displaying {} class offerings join", query.len());
    for (co, i, c , t) in query {
        println!("{:?}", co.class_offering_id);
        println!("{:?}", i.full_name);
        println!("{:?}", c.class);
        println!("{:?}", c.name);
        println!("{:?}", t.date);
    }
}
