use crate::{
    app::Infrastructure,
    biz::pet::{
        model::pet_type::{
            PetType, PetTypeCreateBo, PetTypeListVo, PetTypeUpdateBo,
        },
        repository::pet_type_repo::PetTypeRepository,
    },
};

#[derive(Clone)]
pub struct PetTypeService {
    pet_type_repo: PetTypeRepository,
}

impl PetTypeService {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            pet_type_repo: PetTypeRepository::new(infra.batis.clone()),
        }
    }

    pub async fn list(&self) -> Vec<PetTypeListVo> {
        match self.pet_type_repo.select_all().await {
            Ok(result) => {
                result.into_iter().map(|pet| pet.to_list_vo()).collect()
            }
            Err(e) => {
                tracing::error!("查询列表失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_pet_type_by_id(&self, id: i64) -> Option<PetType> {
        match self.pet_type_repo.select_by_id(&id).await {
            Ok(Some(pet_type)) => Some(pet_type),
            Ok(None) => None,
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                None
            }
        }
    }

    pub async fn insert_by_bo(&self, bo: PetTypeCreateBo) -> bool {
        let pet_type = PetType::from_create_bo(bo);
        match self.pet_type_repo.insert(&pet_type).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("创建失败: {:?}", e);
                false
            }
        }
    }

    pub async fn update_by_bo(&self, bo: PetTypeUpdateBo) -> bool {
        let pet_type_id = &bo.pet_type_id.unwrap();
        let mut pet_type =
            match self.pet_type_repo.select_by_id(pet_type_id).await {
                Ok(Some(pet_type)) => pet_type,
                Ok(None) => {
                    tracing::error!("根据ID查询失败: {:?}", pet_type_id);
                    return false;
                }
                Err(e) => {
                    tracing::error!("根据ID查询失败: {:?}", e);
                    return false;
                }
            };

        pet_type.from_update_bo(bo);
        match self
            .pet_type_repo
            .update_by_id(&pet_type, pet_type_id)
            .await
        {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("更新失败: {:?}", e);
                false
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64) -> bool {
        match self.pet_type_repo.delete_by_id(&id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }
}
