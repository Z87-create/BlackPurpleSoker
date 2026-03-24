use crate::{error::AppError, model::User};

pub async fn verify(username: &str, password: &str) -> Result<User, AppError> {
    let admin_user = crate::config::CONFIG.admin_username.clone();
    let admin_pwd = crate::config::CONFIG.admin_password.clone();

    if username == admin_user && password == admin_pwd {
        Ok(User {
            username: admin_user,
            role: "admin".into(),
        })
    } else {
        Err(AppError::UserNotFoundOrPasswordWrong)
    }
}

pub async fn get_all() -> Result<Vec<User>, AppError> {
    Ok(vec![User {
        username: "admin".into(),
        role: "admin".into(),
    }])
}