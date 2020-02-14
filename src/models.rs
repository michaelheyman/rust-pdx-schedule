#[derive(Queryable)]
pub struct Term {
    pub date: i32,
    pub description: String,
}

#[derive(Queryable)]
pub struct Instructor {
    pub id: i32,
    pub full_name: String,
    pub first_name: String,
    pub last_name: String,
    pub rating: f64,
    pub url: String,
}
