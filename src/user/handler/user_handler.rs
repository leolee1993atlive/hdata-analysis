use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use crate::{
    app::AppState,
    common::vo::response::R,
    middleware::extractors::CurrentUser,
    user::model::user::{User, UserCreateBo, UserListVo, UserUpdateBo},
};

pub async fn list_user(
    State(state): State<Arc<AppState>>,
) -> R<Vec<UserListVo>> {
    let result = state.services.user_service.list().await;

    if result.is_empty() {
        R::ok_with_data(Vec::new())
    } else {
        R::ok_with_data(result)
    }
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<User> {
    let result = state.services.user_service.get_user_by_id(id).await;

    match result {
        Some(user) => R::ok_with_data(user),
        None => R::error_with_message(String::from("查询失败")),
    }
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<UserCreateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .user_service
        .insert_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("创建失败"))
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<UserUpdateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .user_service
        .update_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("更新失败"))
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .user_service
        .delete_by_id(id, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}

pub async fn true_delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state.services.user_service.true_delete_by_id(id).await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}
