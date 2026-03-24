use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            msg: "success".into(),
            data: Some(data),
        }
    }

    pub fn err(code: u16, msg: String) -> Self {
        Self { code, msg, data: None }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiQueryReq {
    pub question: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub choices: Vec<AiChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiChoice {
    pub message: AiMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMessage {
    pub content: String,
}