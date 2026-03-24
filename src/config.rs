use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]//强制蛇形命名匹配
pub struct AppConfig {
    pub server: ServerConfig,
    pub ai: AiConfig,
    pub admin_username: String,  // 顶层字段，和 config.toml 对齐
    pub admin_password: String,  // 顶层字段，和 config.toml 对齐
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expire_hours: f64,
    pub rate_limit_per_minute: u32,
}

#[derive(Debug, Deserialize)]
pub struct AiConfig {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub timeout: u64,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let content = std::fs::read_to_string("config.toml")?;
        let cfg = toml::from_str(&content)?;
        Ok(cfg)
    }
}
// pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
//     AppConfig::load().unwrap()
// });
pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            jwt_secret: "your_very_secure_secret_key_123456".to_string(),
            jwt_expire_hours: 24.0,
            rate_limit_per_minute: 60,
        },
        ai: AiConfig {
            api_url: "https://api.deepseek.com/v1/chat/completions".to_string(),
            api_key: "sk-48d185e1ce89487484f35503320130de".to_string(),
            model: "deepseek-chat".to_string(),
            timeout: 30,
        },
        admin_username: "admin".to_string(),
        admin_password: "Admin@123".to_string(),
    }
});