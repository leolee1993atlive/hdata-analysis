use std::sync::Arc;

use axum::{Json, extract::State};
use redis::Commands;
use serde_json::json;

use crate::{
    app::AppState,
    auth::model::auth::{AuthBody, AuthPayload},
    common::vo::response::R,
    error::error::AuthError,
    util::jwt_util::JwtToken,
};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<R<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    tracing::info!("尝试登录用户: {}", payload.username);
    let user = state
        .services
        .user_service
        .get_user_by_username(&payload.username)
        .await;
    tracing::debug!("select_by_username: {:?}", json!(user));

    // 检查查询是否成功
    let user = match user {
        Some(u) => {
            tracing::info!("找到用户: {}", payload.username);
            u
        }
        None => {
            tracing::error!("用户不存在: {}", payload.username);
            return Err(AuthError::WrongCredentials);
        }
    };

    // 验证用户是否存在以及密码是否正确
    if user.password != payload.password
        && !bcrypt::verify(payload.password.as_bytes(), &user.password).unwrap()
    {
        tracing::error!("密码错误: {}", payload.username);
        return Err(AuthError::WrongCredentials);
    }

    // Create the authorization token
    let token = JwtToken::new(user.user_id.unwrap(), &user.username, vec![]);
    let token_str = token.create_token("123").unwrap();
    let token_key = format!("token::{}", payload.username);
    let mut redis_pool = state.infra.pool.redis_pool.get().unwrap();
    redis_pool
        .set::<String, String, ()>(token_key, token_str.clone())
        .unwrap();

    // Send the authorized token
    Ok(R::ok_with_data(AuthBody::new(token_str)))
}
