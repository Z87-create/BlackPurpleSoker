use actix_web::{web, App, HttpServer};
use anyhow::Result;

mod auth;
mod config;
mod controller;
mod error;
mod model;
mod service;
mod user_manager;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .init();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            
            // 直接注册路由
            .service(controller::login_api)
            .service(controller::ai_query)
            .service(controller::user_list_api)
            
            // 静态页面
            .service(
                actix_files::Files::new("/", "./static")
                    .prefer_utf8(true)
                    .index_file("index.html")
            )
    })
    .bind((
        config::CONFIG.server.host.clone(),
        config::CONFIG.server.port.clone()
    ))?
    .run()
    .await?;

    Ok(())
}