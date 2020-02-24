#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

const DATABASE_PATH: &str = "dist/app.db";

/// Gets all classes.
#[get("api/classes/{term}")]
async fn get_classes(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let classes = web::block(move || actions::get_classes(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(classes) = classes {
        Ok(HttpResponse::Ok().json(classes))
    } else {
        let res = HttpResponse::NotFound().body(format!("No classes found"));
        Ok(res)
    }
}

/// Gets all courses.
#[get("/api/courses")]
async fn get_courses(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let courses = web::block(move || actions::get_courses(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(courses) = courses {
        Ok(HttpResponse::Ok().json(courses))
    } else {
        let res = HttpResponse::NotFound().body(format!("No courses found"));
        Ok(res)
    }
}

/// Gets all instructors.
#[get("/api/instructors")]
async fn get_instructors(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let instructors = web::block(move || actions::get_instructors(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(instructors) = instructors {
        Ok(HttpResponse::Ok().json(instructors))
    } else {
        let res = HttpResponse::NotFound().body(format!("No instructors found"));
        Ok(res)
    }
}

/// Gets all terms.
#[get("/api/terms")]
async fn get_terms(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let terms = web::block(move || actions::get_terms(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(terms) = terms {
        Ok(HttpResponse::Ok().json(terms))
    } else {
        let res = HttpResponse::NotFound().body(format!("No terms found"));
        Ok(res)
    }
}

fn create_database_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or(DATABASE_PATH.to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = create_database_connection_pool();

    let bind = "localhost:8080";

    println!("Actix running at: http://{}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_classes)
            .service(get_courses)
            .service(get_instructors)
            .service(get_terms)
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(&bind)?
    .run()
    .await
}
