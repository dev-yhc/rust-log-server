use std::sync::RwLock;
use std::io::Write;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use log::{info, LevelFilter};
use std::fs::File;
use chrono::Local;
use env_logger::Builder;
use env_logger::Target;

mod config;
use config::AppConfig;

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
    // TODO: upload config
    let config = AppConfig::load().expect("Failed to load config");
    // 로그 파일 생성
    std::fs::create_dir_all("logs")?;
    let log_file = File::create("./logs/application.log")?;

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
                app_name: config.app_name.clone(),
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
    .bind(("127.0.0.1", config.port))?
    .shutdown_timeout(30) // <-- Allow 30 seconds for graceful shutdown
    .run()
    .await
}