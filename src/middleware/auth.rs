use crate::common::vo::response::R;
use crate::error::error::AppError;
use crate::util::jwt_util::JwtToken;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http, response};

pub async fn auth(
    mut req: Request,
    next: Next,
) -> Result<response::Response, StatusCode> {
    tracing::info!("req {:?}", req.uri());
    let path = req.uri().to_string();
    if path.eq("/login") {
        return Ok(next.run(req).await);
    }
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if auth_header.is_none() {
        return Ok(R::<()>::error_with_code_and_message(
            401,
            String::from("请求头缺少 Authorization 字段"),
        )
        .into_response());
    }
    let authorization = auth_header.unwrap();

    let token = authorization.to_string().replace("Bearer ", "");
    let jwt_token_e = JwtToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => data,
        Err(err) => {
            let er = match err {
                AppError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };
            return Ok(R::<()>::error_with_code_and_message(
                401,
                format!("Token 校验失败: {}", er),
            )
            .into_response());
        }
    };

    let mut flag: bool = true;
    for token_permission in &jwt_token.permissions {
        if token_permission.to_string().replace("/api", "") == path {
            flag = true;
            break;
        }
    }

    if flag {
        req.headers_mut()
            .insert("user_id", jwt_token.id.to_string().parse().unwrap());
        return Ok(next.run(req).await);
    }

    Ok(R::<()>::error_with_code_and_message(
        401,
        format!("用户还没有授权url:{path}"),
    )
    .into_response())
}
