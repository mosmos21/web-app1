extern crate env_logger;

use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use serde::{Deserialize};

#[derive(Deserialize)]
struct PostEchoParams {
    value: String,
}

fn echo (params: web::Json<PostEchoParams>) -> HttpResponse {
    let value = &params.value;
    HttpResponse::Ok().body(format!("echo from server: {}", value))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/echo").route(web::post().to(echo)))
    })
        .bind(format!("0.0.0.0:{}",port))?
        .run()
        .await
}

