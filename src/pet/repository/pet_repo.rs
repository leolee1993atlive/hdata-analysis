use std::sync::Arc;

use crate::pet::model::pet::Pet;
use rbatis::{RBatis, crud, impl_delete, impl_select, impl_update};
use rbdc::db::ExecResult;

#[derive(Clone)]
pub struct PetRepository {
    rb: Arc<RBatis>,
}

impl PetRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    pub async fn select_all(&self) -> Result<Vec<Pet>, rbatis::Error> {
        Pet::select_all(&*self.rb).await
    }

    pub async fn select_by_id(
        &self,
        pet_id: &i64,
    ) -> Result<Option<Pet>, rbatis::Error> {
        Pet::select_by_id(&*self.rb, pet_id).await
    }

    pub async fn insert(&self, pet: &Pet) -> Result<ExecResult, rbatis::Error> {
        Pet::insert(&*self.rb, pet).await
    }

    pub async fn update_by_id(
        &self,
        pet: &Pet,
        pet_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        Pet::update_by_id(&*self.rb, pet, pet_id).await
    }

    pub async fn delete_by_id(
        &self,
        pet_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        Pet::delete_by_id(&*self.rb, pet_id).await
    }
}

crud!(Pet {});
impl_select!(
    Pet{select_by_name(name: &str) -> Option => "`where name = #{name} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_select!(
    Pet{select_by_id(pet_id: &i64) -> Option => "`where pet_id = #{pet_id} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_update!(
    Pet{update_by_id(pet_id: &i64) => "`where pet_id = #{pet_id} and deleted_by is null and deleted_date is null`"}
);
impl_delete!(
    Pet{delete_by_id(pet_id: &i64) => "`where pet_id = #{pet_id} and deleted_by is null and deleted_date is null`"}
);
