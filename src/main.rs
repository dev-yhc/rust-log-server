use std::sync::RwLock;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppState {
    app_name: String,
    counter: RwLock<i32>,
}

async fn _healthcheck() -> impl Responder {
    HttpResponse::Ok().body("UP")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: upload config
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("log server"),
                counter: RwLock::new(0)
            }))
            .service(
                web::scope("/_healthcheck")
                .route("", web::get().to(_healthcheck))
            )
            .service(
                web::scope("/v1/log")
                .route("", web::get().to(hello))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}