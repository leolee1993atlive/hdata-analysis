use crate::{
    app::Infrastructure,
    biz::datasource::{
        model::data_source::{
            DataSource, DataSourceCreateBo, DataSourceListVo,
            DataSourceUpdateBo,
        },
        repository::data_source_repo::DataSourceRepository,
    },
    sys::user::model::user::User,
};

#[derive(Clone)]
pub struct DataSourceService {
    data_source_repo: DataSourceRepository,
}

impl DataSourceService {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            data_source_repo: DataSourceRepository::new(infra.batis.clone()),
        }
    }

    pub async fn list(&self) -> Vec<DataSourceListVo> {
        match self.data_source_repo.select_all().await {
            Ok(result) => {
                result.into_iter().map(|pet| pet.to_list_vo()).collect()
            }
            Err(e) => {
                tracing::error!("查询列表失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_data_source_by_id(&self, id: i64) -> Option<DataSource> {
        match self.data_source_repo.select_by_id(&id).await {
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
        bo: DataSourceCreateBo,
        current_user: &User,
    ) -> bool {
        let pet = DataSource::from_create_bo(bo, current_user);
        match self.data_source_repo.insert(&pet).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("创建失败: {:?}", e);
                false
            }
        }
    }

    pub async fn update_by_bo(
        &self,
        bo: DataSourceUpdateBo,
        current_user: &User,
    ) -> bool {
        let data_source_id = &bo.data_source_id.unwrap();
        let mut pet =
            match self.data_source_repo.select_by_id(data_source_id).await {
                Ok(Some(pet)) => pet,
                Ok(None) => {
                    tracing::error!("根据ID查询失败: {:?}", data_source_id);
                    return false;
                }
                Err(e) => {
                    tracing::error!("根据ID查询失败: {:?}", e);
                    return false;
                }
            };

        pet.from_update_bo(bo, current_user);
        match self
            .data_source_repo
            .update_by_id(&pet, data_source_id)
            .await
        {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("更新失败: {:?}", e);
                false
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64, current_user: &User) -> bool {
        // 先查询实体
        let mut pet = match self.data_source_repo.select_by_id(&id).await {
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
        match self.data_source_repo.update_by_id(&pet, &id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }

    pub async fn true_delete_by_id(&self, id: i64) -> bool {
        match self.data_source_repo.delete_by_id(&id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }
}
