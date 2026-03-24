// service.rs 【正确最终版】
use crate::{error::AppError, model::*};
use reqwest::Client;

pub async fn ai_query(question: &str) -> Result<String, AppError> {
    let client = Client::new();
    let url = &crate::config::CONFIG.ai.api_url;
    let key = &crate::config::CONFIG.ai.api_key;

    let body = serde_json::json!({
        "model": "deepseek-chat",
        "messages": [{"role":"user","content":question}]
    });

    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::AiServiceError)?;

    let result: AiChatResponse = resp
        .json()
        .await
        .map_err(|_| AppError::AiServiceError)?;

    Ok(result.choices[0].message.content.clone())
}