use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::common::vo::response::R;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("用户或密码错误")]
    WrongCredentials,
    #[error("用户或密码不能为空")]
    MissingCredentials,
    #[error("创建Token失败")]
    TokenCreation,
    #[error("Token无效或已失效")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials")
            }
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials")
            }
            AuthError::TokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error")
            }
            AuthError::InvalidToken => {
                (StatusCode::BAD_REQUEST, "Invalid token")
            }
        };
        let body = R::<()>::error_with_code_and_message(
            status.as_u16(),
            message.to_string(),
        );
        (status, body).into_response()
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("JWT错误：{0}")]
    JwtTokenError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DbError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::BusinessError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::JwtTokenError(e) => {
                (StatusCode::UNAUTHORIZED, e.to_string())
            }
        };
        let body = R::<()>::error_with_code_and_message(
            status.as_u16(),
            message.to_string(),
        );
        (status, body).into_response()
    }
}
