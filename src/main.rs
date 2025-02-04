#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, http::header, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

const DATABASE_PATH: &str = "dist/app.db";

/// Gets class by id.
#[get("/api/class/{class_id}")]
async fn get_class(
    pool: web::Data<DbPool>,
    class_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let class_id = class_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let class = web::block(move || actions::get_class(class_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(class) = class {
        Ok(HttpResponse::Ok().json(class))
    } else {
        let res = HttpResponse::NotFound().body(format!("Class with id '{}' not found", class_id));
        Ok(res)
    }
}

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

/// Gets course by id.
#[get("/api/course/{course_id}")]
async fn get_course(
    pool: web::Data<DbPool>,
    course_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let course_id = course_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let course = web::block(move || actions::get_course(course_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(course) = course {
        Ok(HttpResponse::Ok().json(course))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("Course with id '{}' not found", course_id));
        Ok(res)
    }
}

/// Gets instructor by id.
#[get("/api/instructor/{instructor_id}")]
async fn get_instructor(
    pool: web::Data<DbPool>,
    instructor_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let instructor_id = instructor_id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let instructor = web::block(move || actions::get_instructor(instructor_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(instructor) = instructor {
        Ok(HttpResponse::Ok().json(instructor))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("Instructor with id '{}' not found", instructor_id));
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
            .wrap(
                Cors::new()
                    //                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://[::1]:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(get_class)
            .service(get_classes)
            .service(get_course)
            .service(get_courses)
            .service(get_instructor)
            .service(get_instructors)
            .service(get_terms)
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(&bind)?
    .run()
    .await
}
