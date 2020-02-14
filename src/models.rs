#[derive(Queryable)]
pub struct Term {
    pub date: i32,
    pub description: String,
}

#[derive(Queryable)]
pub struct Instructor {
    pub id: i32,
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub rating: Option<f64>,
    pub url: Option<String>,
}
