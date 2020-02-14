extern crate diesel;
extern crate rust_diesel;

use self::models::*;
use diesel::prelude::*;
use rust_diesel::*;

fn main() {
    use self::schema::term::dsl::*;

    let connection = establish_connection();
    let results = term
        .limit(5)
        .load::<Term>(&connection)
        .expect("Error loading terms");

    println!("Displaying {} terms", results.len());
    for t in results {
        println!("{}", t.date);
        println!("----------\n");
        println!("{}", t.description);
    }
}
