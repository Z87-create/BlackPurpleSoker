use actix_web::{get, post, web, Responder};
use actix_files::Files;

use crate::auth::create_token;
use crate::error::AppError;
use crate::model::{ApiResponse, AiQueryReq, LoginReq};
use crate::user_manager::verify;
use crate::service;

#[post("/api/login")]
async fn login_api(body: web::Json<LoginReq>) -> Result<impl Responder, AppError> {
    let user = verify(&body.username, &body.password).await?;
    let token = create_token(&user.username, &user.role)?;
    Ok(web::Json(ApiResponse::ok(token)))
}

#[post("/api/ai")]
async fn ai_query(body: web::Json<AiQueryReq>) -> Result<impl Responder, AppError> {
    let answer = service::ai_query(&body.question).await?;
    Ok(web::Json(ApiResponse::ok(answer)))
}

#[get("/api/users")]
async fn user_list_api() -> Result<impl Responder, AppError> {
    let users = crate::user_manager::get_all().await?;
    Ok(web::Json(ApiResponse::ok(users)))
}