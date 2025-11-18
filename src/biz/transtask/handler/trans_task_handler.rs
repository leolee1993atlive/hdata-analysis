use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use crate::{
    app::AppState,
    biz::transtask::model::trans_task::{
        TransTaskCreateBo, TransTaskDetailVo, TransTaskListVo,
        TransTaskUpdateBo,
    },
    common::vo::response::R,
    middleware::extractors::CurrentUser,
};

pub async fn list_trans_task(
    State(state): State<Arc<AppState>>,
) -> R<Vec<TransTaskListVo>> {
    let result = state.services.trans_task_service.list().await;

    if result.is_empty() {
        R::ok_with_data(Vec::new())
    } else {
        R::ok_with_data(result)
    }
}

pub async fn get_trans_task_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<TransTaskDetailVo> {
    let result = state
        .services
        .trans_task_service
        .get_trans_task_by_id(id)
        .await;

    match result {
        Some(trans_task) => R::ok_with_data(trans_task),
        None => R::error_with_message(String::from("查询失败")),
    }
}

pub async fn create_trans_task(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<TransTaskCreateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .trans_task_service
        .insert_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("创建失败"))
    }
}

pub async fn update_trans_task(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<TransTaskUpdateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .trans_task_service
        .update_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("更新失败"))
    }
}

pub async fn delete_trans_task(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .trans_task_service
        .delete_by_id(id, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}

pub async fn true_delete_trans_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .trans_task_service
        .true_delete_by_id(id)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}
