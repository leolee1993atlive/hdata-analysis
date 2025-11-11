use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use validator::Validate;

use crate::{
    app::AppState,
    common::vo::response::R,
    pet::model::pet_type::{
        PetType, PetTypeCreateBo, PetTypeListVo, PetTypeUpdateBo,
    },
};

pub async fn list_pet_type(
    State(state): State<Arc<AppState>>,
) -> R<Vec<PetTypeListVo>> {
    let result = state.services.pet_type_service.list().await;

    if result.is_empty() {
        R::ok_with_data(Vec::new())
    } else {
        R::ok_with_data(result)
    }
}

pub async fn get_pet_type_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<PetType> {
    let result = state.services.pet_type_service.get_pet_type_by_id(id).await;

    match result {
        Some(pet_type) => R::ok_with_data(pet_type),
        None => R::error_with_message(String::from("查询失败")),
    }
}

pub async fn create_pet_type(
    State(state): State<Arc<AppState>>,
    Json(bo): Json<PetTypeCreateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state.services.pet_type_service.insert_by_bo(bo).await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("创建失败"))
    }
}

pub async fn update_pet_type(
    State(state): State<Arc<AppState>>,
    Json(bo): Json<PetTypeUpdateBo>,
) -> R<()> {
    if let Err(e) = &bo.validate() {
        return R::error_with_message(e.to_string());
    }

    let result = state.services.pet_type_service.update_by_bo(bo).await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("更新失败"))
    }
}

pub async fn delete_pet_type(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> R<()> {
    let result = state.services.pet_type_service.delete_by_id(id).await;

    if result {
        R::ok()
    } else {
        R::error_with_message(String::from("删除失败"))
    }
}
