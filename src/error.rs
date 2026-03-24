use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("参数错误：{0}")]
    BadRequest(String),
    #[error("未登录或Token已过期")]
    Unauthorized,
    #[error("权限不足")]
    Forbidden,
    #[error("服务器内部错误")]
    InternalError,
    #[error("AI服务调用失败")]
    AiServiceError,
    #[error("用户名或密码错误")]
    UserNotFoundOrPasswordWrong,
    #[error("请求过于频繁")]
    TooManyRequests,
    // #[error("internal error")]
    // InternalError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AiServiceError => StatusCode::BAD_GATEWAY,
            AppError::UserNotFoundOrPasswordWrong => StatusCode::UNAUTHORIZED,
            AppError::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let resp = crate::model::ApiResponse::<()> {
            code: self.status_code().as_u16(),
            msg: self.to_string(),
            data: None,
        };
        HttpResponse::build(self.status_code()).json(resp)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> Self {
        AppError::InternalError
    }
}

// 让 JWT 错误能自动转 AppError
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized
    }
}