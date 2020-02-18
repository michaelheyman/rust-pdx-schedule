use diesel::prelude::*;

use crate::models;

pub fn find_terms(
    conn: &SqliteConnection,
) -> Result<Option<Vec<models::Term>>, diesel::result::Error> {
    use crate::schema::term::dsl::*;

    let results = term.load::<models::Term>(conn).optional()?;

    Ok(results)
}
