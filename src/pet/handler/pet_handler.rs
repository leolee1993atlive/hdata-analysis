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
    pet::model::pet::{Pet, PetCreateBo, PetListVo, PetUpdateBo},
};

pub async fn list_pet(State(state): State<Arc<AppState>>) -> R<Vec<PetListVo>> {
    let result = state.services.pet_service.list().await;

    if result.is_empty() {
        R::ok_with_data(Vec::new())
    } else {
        R::ok_with_data(result)
    }
}

pub async fn get_pet_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<Pet> {
    let result = state.services.pet_service.get_pet_by_id(id).await;

    match result {
        Some(pet) => R::ok_with_data(pet),
        None => R::error_with_message(String::from("查询失败")),
    }
}

pub async fn create_pet(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<PetCreateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .pet_service
        .insert_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("创建失败"))
    }
}

pub async fn update_pet(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Json(bo): Json<PetUpdateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state
        .services
        .pet_service
        .update_by_bo(bo, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("更新失败"))
    }
}

pub async fn delete_pet(
    State(state): State<Arc<AppState>>,
    CurrentUser(current_user): CurrentUser,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state
        .services
        .pet_service
        .delete_by_id(id, &current_user)
        .await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}

pub async fn true_delete_pet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state.services.pet_service.true_delete_by_id(id).await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}
