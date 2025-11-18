use r2d2::Pool;
use r2d2_mysql::{MySqlConnectionManager, mysql::OptsBuilder};

use crate::{
    app::Infrastructure,
    biz::datasource::{
        model::data_source::{
            DataSource, DataSourceCreateBo, DataSourceDetailVo,
            DataSourceListVo, DataSourceUpdateBo,
        },
        repository::data_source_repo::DataSourceRepository,
    },
    sys::user::model::user::User,
    util::crypto_util::decrypt_password,
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
            Ok(result) => result
                .into_iter()
                .map(|entity| entity.to_list_vo())
                .collect(),
            Err(e) => {
                tracing::error!("查询列表失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_data_source_by_id(
        &self,
        id: i64,
    ) -> Option<DataSourceDetailVo> {
        match self.data_source_repo.select_by_id(&id).await {
            Ok(Some(entity)) => Some(entity.to_detail_vo()),
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
        let entity = DataSource::from_create_bo(bo, current_user);
        match self.data_source_repo.insert(&entity).await {
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
        let mut entity =
            match self.data_source_repo.select_by_id(data_source_id).await {
                Ok(Some(entity)) => entity,
                Ok(None) => {
                    tracing::error!("根据ID查询失败: {:?}", data_source_id);
                    return false;
                }
                Err(e) => {
                    tracing::error!("根据ID查询失败: {:?}", e);
                    return false;
                }
            };

        entity.from_update_bo(bo, current_user);
        match self
            .data_source_repo
            .update_by_id(&entity, data_source_id)
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
        let mut entity = match self.data_source_repo.select_by_id(&id).await {
            Ok(Some(entity)) => entity,
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
        entity.base_entity.delete(current_user.get_user_id());

        // 更新实体（软删除）
        match self.data_source_repo.update_by_id(&entity, &id).await {
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

    pub async fn test_data_source(&self, id: i64) -> (bool, String) {
        let entity = self.get_data_source_by_id(id).await;
        match entity {
            Some(data_source) => {
                // 解密数据库密码
                let decrypted_password =
                    match decrypt_password(&data_source.db_password) {
                        Ok(password) => password,
                        Err(e) => {
                            tracing::error!("密码解密失败: {}", e);
                            return (false, format!("密码解密失败: {}", e));
                        }
                    };

                // 创建MySQL连接选项用于r2d2连接池
                let opts = OptsBuilder::new()
                    .ip_or_hostname(Some(data_source.db_host.clone()))
                    .tcp_port(data_source.db_port)
                    .user(Some(data_source.db_username.clone()))
                    .pass(Some(decrypted_password))
                    .db_name(Some(data_source.db_name.clone()));

                // 创建连接管理器
                let rb_manager = MySqlConnectionManager::new(opts);
                // 创建r2d2连接池
                match Pool::new(rb_manager) {
                    Ok(_) => {
                        tracing::info!(
                            "Successfully created r2d2 database connection pool for: {}",
                            data_source.db_name
                        );
                        (
                            true,
                            String::from(
                                "Connection pool created successfully",
                            ),
                        )
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to create r2d2 database connection pool: {}",
                            e
                        );
                        (false, String::from("Connection pool creation failed"))
                    }
                }
            }
            None => (false, String::from("Data source not found")),
        }
    }
}
