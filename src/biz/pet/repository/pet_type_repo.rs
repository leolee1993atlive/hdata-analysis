use std::sync::Arc;

use crate::biz::pet::model::pet_type::PetType;
use rbatis::{RBatis, crud, impl_delete, impl_select, impl_update};
use rbdc::db::ExecResult;

#[derive(Clone)]
pub struct PetTypeRepository {
    rb: Arc<RBatis>,
}

impl PetTypeRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    pub async fn select_all(&self) -> Result<Vec<PetType>, rbatis::Error> {
        PetType::select_all(&*self.rb).await
    }

    pub async fn select_by_id(
        &self,
        pet_type_id: &i64,
    ) -> Result<Option<PetType>, rbatis::Error> {
        PetType::select_by_id(&*self.rb, pet_type_id).await
    }

    pub async fn insert(
        &self,
        pet_type: &PetType,
    ) -> Result<ExecResult, rbatis::Error> {
        PetType::insert(&*self.rb, pet_type).await
    }

    pub async fn update_by_id(
        &self,
        pet_type: &PetType,
        pet_type_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        PetType::update_by_id(&*self.rb, pet_type, pet_type_id).await
    }

    pub async fn delete_by_id(
        &self,
        pet_type_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        PetType::delete_by_id(&*self.rb, pet_type_id).await
    }
}

crud!(PetType {});
impl_select!(
    PetType{select_by_id(pet_type_id: &i64) -> Option => "`where pet_type_id = #{pet_type_id} limit 1`"}
);
impl_update!(
    PetType{update_by_id(pet_type_id: &i64) => "`where pet_type_id = #{pet_type_id}`"}
);
impl_delete!(
    PetType{delete_by_id(pet_type_id: &i64) => "`where pet_type_id = #{pet_type_id}`"}
);
