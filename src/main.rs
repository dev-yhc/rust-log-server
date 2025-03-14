use std::sync::RwLock;
use std::io::Write;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use log::{info, LevelFilter};
use chrono::Local;
use env_logger::Builder;
use env_logger::Target;
use std::fs::OpenOptions;

mod config;

async fn hello() -> impl Responder {
    info!("/hello");
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
    // 로그 파일 생성
    std::fs::create_dir_all("logs")?;
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs/application.log")?;

    // 로거 설정
    Builder::new()
        .target(Target::Pipe(Box::new(log_file)))
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    info!("Application started");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("rust-log-server"),
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
    .bind(("0.0.0.0", 8080))?
    .shutdown_timeout(30) // <-- Allow 30 seconds for graceful shutdown
    .run()
    .await
}