use crate::{
    app::Infrastructure,
    biz::transtask::{
        model::trans_task::{
            TransTask, TransTaskCreateBo, TransTaskDetailVo, TransTaskListVo,
            TransTaskUpdateBo,
        },
        repository::trans_task_repo::TransTaskRepository,
    },
    sys::user::model::user::User,
};

#[derive(Clone)]
pub struct TransTaskService {
    trans_task_repo: TransTaskRepository,
}

impl TransTaskService {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            trans_task_repo: TransTaskRepository::new(infra.batis.clone()),
        }
    }

    pub async fn list(&self) -> Vec<TransTaskListVo> {
        match self.trans_task_repo.select_all().await {
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

    pub async fn get_trans_task_by_id(
        &self,
        id: i64,
    ) -> Option<TransTaskDetailVo> {
        match self.trans_task_repo.select_by_id(&id).await {
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
        bo: TransTaskCreateBo,
        current_user: &User,
    ) -> bool {
        let entity = TransTask::from_create_bo(bo, current_user);
        match self.trans_task_repo.insert(&entity).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("创建失败: {:?}", e);
                false
            }
        }
    }

    pub async fn update_by_bo(
        &self,
        bo: TransTaskUpdateBo,
        current_user: &User,
    ) -> bool {
        let trans_task_id = &bo.trans_task_id.unwrap();
        let mut entity =
            match self.trans_task_repo.select_by_id(trans_task_id).await {
                Ok(Some(entity)) => entity,
                Ok(None) => {
                    tracing::error!("根据ID查询失败: {:?}", trans_task_id);
                    return false;
                }
                Err(e) => {
                    tracing::error!("根据ID查询失败: {:?}", e);
                    return false;
                }
            };

        entity.from_update_bo(bo, current_user);
        match self
            .trans_task_repo
            .update_by_id(&entity, trans_task_id)
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
        let mut entity = match self.trans_task_repo.select_by_id(&id).await {
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
        match self.trans_task_repo.update_by_id(&entity, &id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }

    pub async fn true_delete_by_id(&self, id: i64) -> bool {
        match self.trans_task_repo.delete_by_id(&id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }
}
