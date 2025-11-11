use std::sync::Arc;

use crate::user::model::user::User;
use crate::util::jwt_util::JwtToken;
use crate::{app::AppState, error::error::AppError};

use axum::extract::{FromRef, FromRequestParts};
use axum::http;
use axum::http::request::Parts;

// 当前用户提取器
pub struct CurrentUser(pub User);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头获取用户ID
        let token = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::JwtTokenError(String::from(
                "请求头缺少 Authorization 字段",
            )))?;

        // 检查是否以 "Bearer " 开头
        let token = token
            .strip_prefix("Bearer ")
            .ok_or(AppError::JwtTokenError(format!("token格式错误:{token}")))?;
        let username = JwtToken::get_username("123", token);

        // 获取应用状态
        let app_state = Arc::<AppState>::from_ref(state);

        // 查询用户信息
        let user = app_state
            .services
            .user_service
            .get_user_by_username(&username)
            .await
            .ok_or(AppError::JwtTokenError(format!("用户不存在:{username}")))?;

        Ok(CurrentUser(user))
    }
}
