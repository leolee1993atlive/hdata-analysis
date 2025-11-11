use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

// 添加一个通用的 API 响应枚举
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum R<T> {
    NoDataSuccess(ApiResponse<()>),
    Success(ApiResponse<T>),
    Error(ApiResponse<()>),
    DataError(ApiResponse<T>),
}

impl<T> IntoResponse for R<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            R::NoDataSuccess(r) => r.into_response(),
            R::Success(r) => r.into_response(),
            R::Error(r) => r.into_response(),
            R::DataError(r) => r.into_response(),
        }
    }
}

impl<T> R<T> {
    pub fn ok() -> Self {
        R::NoDataSuccess(ApiResponse::<()>::ok())
    }

    pub fn ok_with_data(data: T) -> Self {
        R::Success(ApiResponse::ok_with_data(data))
    }

    pub fn ok_with_data_and_message(data: T, message: String) -> Self {
        R::Success(ApiResponse::ok_with_data_and_message(data, message))
    }

    pub fn error() -> Self {
        R::Error(ApiResponse::<()>::error())
    }

    pub fn error_with_message(message: String) -> Self {
        R::Error(ApiResponse::<()>::error_with_code_and_message(500, message))
    }

    pub fn error_with_code_and_message(code: u16, message: String) -> Self {
        R::Error(ApiResponse::<()>::error_with_code_and_message(
            code, message,
        ))
    }

    pub fn error_with_data(data: T) -> Self {
        R::DataError(ApiResponse::error_with_data(data))
    }

    pub fn error_with_code_and_data(code: u16, data: T) -> Self {
        R::DataError(ApiResponse::error_with_code_and_data(code, data))
    }

    pub fn error_with_code_and_message_and_data(
        code: u16,
        messages: String,
        data: T,
    ) -> Self {
        R::DataError(ApiResponse::error_with_code_and_message_and_data(
            code, messages, data,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok() -> ApiResponse<()> {
        ApiResponse {
            code: 200,
            message: "Success".to_string(),
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> ApiResponse<T> {
        ApiResponse {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn ok_with_message(message: String) -> ApiResponse<()> {
        ApiResponse {
            code: 200,
            message,
            data: None,
        }
    }

    pub fn ok_with_data_and_message(
        data: T,
        message: String,
    ) -> ApiResponse<T> {
        ApiResponse {
            code: 200,
            message,
            data: Some(data),
        }
    }

    pub fn error() -> ApiResponse<()> {
        ApiResponse {
            code: 500,
            message: "Internal Server Error".to_string(),
            data: None,
        }
    }

    pub fn error_with_message(message: String) -> ApiResponse<()> {
        ApiResponse {
            code: 500,
            message,
            data: None,
        }
    }

    pub fn error_with_code_and_message(
        code: u16,
        message: String,
    ) -> ApiResponse<()> {
        ApiResponse {
            code,
            message,
            data: None,
        }
    }

    pub fn error_with_data(data: T) -> ApiResponse<T> {
        ApiResponse {
            code: 500,
            message: "Internal Server Error".to_string(),
            data: Some(data),
        }
    }

    pub fn error_with_code_and_data(code: u16, data: T) -> ApiResponse<T> {
        ApiResponse {
            code,
            message: "Internal Server Error".to_string(),
            data: Some(data),
        }
    }

    pub fn error_with_code_and_message_and_data(
        code: u16,
        messages: String,
        data: T,
    ) -> ApiResponse<T> {
        ApiResponse {
            code,
            message: messages,
            data: Some(data),
        }
    }

    pub fn is_success(&self) -> bool {
        self.code == 200
    }

    pub fn is_error(&self) -> bool {
        self.code != 200
    }
}

// 实现 IntoResponse，让 R<T> 可以直接作为响应返回
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
