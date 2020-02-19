use diesel::prelude::*;

use self::models::*;
use crate::models;

pub fn get_terms(
    conn: &SqliteConnection,
) -> Result<Option<Vec<models::Term>>, diesel::result::Error> {
    use crate::schema::term::dsl::*;

    let results = term.load::<models::Term>(conn).optional()?;

    Ok(results)
}

pub fn get_classes(
    conn: &SqliteConnection,
) -> Result<Option<Vec<ClassOfferingResult>>, diesel::result::Error> {
    use crate::schema::*;

    let results = classoffering::table
        .inner_join(course::table)
        .inner_join(instructor::table)
        .inner_join(term::table)
        .load::<ClassOfferingResult>(conn)
        .optional()?;

    Ok(results)
}
