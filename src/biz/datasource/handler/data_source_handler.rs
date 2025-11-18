use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use crate::{
    app::AppState,
    biz::datasource::model::data_source::{
        DataSource, DataSourceCreateBo, DataSourceDetailVo, DataSourceListVo,
        DataSourceUpdateBo,
    },
    common::vo::response::R,
    middleware::extractors::CurrentUser,
};

pub async fn list_data_source(
    State(state): State<Arc<AppState>>,
) -> R<Vec<DataSourceListVo>> {
    let result = state.services.data_source_service.list().await;

    if result.is_empty() {
        R::ok_with_data(Vec::new())
    } else {
        R::ok_with_data(result)
    }
}

pub async fn get_data_source_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<DataSourceDetailVo> {
    let result = state
        .services
        .data_source_service
        .get_data_source_by_id(id)
        .await;

    match result {
        Some(data_source) => R::ok_with_data(data_source),
        None => R::error_with_message(String::from("查询失败")),
    }
}

pub async fn create_data_source(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<DataSourceCreateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .data_source_service
        .insert_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("创建失败"))
    }
}

pub async fn update_data_source(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<DataSourceUpdateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .data_source_service
        .update_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("更新失败"))
    }
}

pub async fn delete_data_source(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .data_source_service
        .delete_by_id(id, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}

pub async fn true_delete_data_source(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .data_source_service
        .true_delete_by_id(id)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}

pub async fn test_data_source(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<bool> {
    let (result, message) = state
        .services
        .data_source_service
        .test_data_source(id)
        .await;

    if result {
        R::ok_with_data_and_message(result, message)
    } else {
        R::error_with_message(message)
    }
}
