use crate::{
    app::Infrastructure,
    biz::pet::{
        model::pet::{Pet, PetCreateBo, PetListVo, PetUpdateBo},
        repository::pet_repo::PetRepository,
    },
    sys::user::model::user::User,
};

#[derive(Clone)]
pub struct PetService {
    pet_repo: PetRepository,
}

impl PetService {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            pet_repo: PetRepository::new(infra.batis.clone()),
        }
    }

    pub async fn list(&self) -> Vec<PetListVo> {
        match self.pet_repo.select_all().await {
            Ok(result) => {
                result.into_iter().map(|pet| pet.to_list_vo()).collect()
            }
            Err(e) => {
                tracing::error!("查询列表失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_pet_by_id(&self, id: i64) -> Option<Pet> {
        match self.pet_repo.select_by_id(&id).await {
            Ok(Some(pet)) => Some(pet),
            Ok(None) => None,
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                None
            }
        }
    }

    pub async fn insert_by_bo(
        &self,
        bo: PetCreateBo,
        current_user: &User,
    ) -> bool {
        let pet = Pet::from_create_bo(bo, current_user);
        match self.pet_repo.insert(&pet).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("创建失败: {:?}", e);
                false
            }
        }
    }

    pub async fn update_by_bo(
        &self,
        bo: PetUpdateBo,
        current_user: &User,
    ) -> bool {
        let pet_id = &bo.pet_id.unwrap();
        let mut pet = match self.pet_repo.select_by_id(pet_id).await {
            Ok(Some(pet)) => pet,
            Ok(None) => {
                tracing::error!("根据ID查询失败: {:?}", pet_id);
                return false;
            }
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                return false;
            }
        };

        pet.from_update_bo(bo, current_user);
        match self.pet_repo.update_by_id(&pet, pet_id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("更新失败: {:?}", e);
                false
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64, current_user: &User) -> bool {
        // 先查询实体
        let mut pet = match self.pet_repo.select_by_id(&id).await {
            Ok(Some(pet)) => pet,
            Ok(None) => {
                tracing::error!("根据ID查询失败: {:?}", id);
                return false;
            }
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                return false;
            }
        };

        // 设置删除信息
        pet.base_entity.delete(current_user.get_user_id());

        // 更新实体（软删除）
        match self.pet_repo.update_by_id(&pet, &id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }

    pub async fn true_delete_by_id(&self, id: i64) -> bool {
        match self.pet_repo.delete_by_id(&id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }
}
