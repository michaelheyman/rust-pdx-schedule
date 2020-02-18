#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod models;
mod schema;

async fn hello_world(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let bind = "localhost:8080";

    println!("Actix running at: http://{}", &bind);

    HttpServer::new(move || App::new().route("/", web::get().to(hello_world)))
        .bind(&bind)?
        .run()
        .await
}
