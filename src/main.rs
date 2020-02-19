#[macro_use]
extern crate diesel;

use actix_web::http::StatusCode;
use actix_web::{
    get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

async fn hello_world(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

/// Gets all terms.
#[get("/api/terms")]
async fn get_terms(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let terms = web::block(move || actions::find_terms(&conn))
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

#[get("/")]
async fn client_page(req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = create_database_connection_pool();

    let bind = "localhost:8080";

    println!("Actix running at: http://{}", &bind);
    print_api_endpoints(&bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_terms)
            .service(get_classes)
            .service(client_page)
    })
    .bind(&bind)?
    .run()
    .await
}

fn print_api_endpoints(bind: &&str) {
    println!("API Endpoints:");
    println!("\tTerms: http://{}", &bind);
    println!("\tTerms: http://{}/welcome", &bind);
    println!("\tTerms: http://{}/api/terms", &bind);
    println!("\tClasses: http://{}/api/classes/202001", &bind);
}

fn create_database_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}
