use std::sync::Arc;

use rbatis::{RBatis, crud, impl_delete, impl_select, impl_update};
use rbdc::db::ExecResult;

use crate::biz::transtask::model::trans_task::TransTask;

#[derive(Clone)]
pub struct TransTaskRepository {
    rb: Arc<RBatis>,
}

impl TransTaskRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    pub async fn select_all(&self) -> Result<Vec<TransTask>, rbatis::Error> {
        TransTask::select_all(&*self.rb).await
    }

    pub async fn select_by_id(
        &self,
        trans_task_id: &i64,
    ) -> Result<Option<TransTask>, rbatis::Error> {
        TransTask::select_by_id(&*self.rb, trans_task_id).await
    }

    pub async fn insert(
        &self,
        data_source: &TransTask,
    ) -> Result<ExecResult, rbatis::Error> {
        TransTask::insert(&*self.rb, data_source).await
    }

    pub async fn update_by_id(
        &self,
        data_source: &TransTask,
        trans_task_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        TransTask::update_by_id(&*self.rb, data_source, trans_task_id).await
    }

    pub async fn delete_by_id(
        &self,
        trans_task_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        TransTask::delete_by_id(&*self.rb, trans_task_id).await
    }
}

crud!(TransTask {});
impl_select!(
    TransTask{select_by_id(trans_task_id: &i64) -> Option => "`where trans_task_id = #{trans_task_id} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_update!(
    TransTask{update_by_id(trans_task_id: &i64) => "`where trans_task_id = #{trans_task_id} and deleted_by is null and deleted_date is null`"}
);
impl_delete!(
    TransTask{delete_by_id(trans_task_id: &i64) => "`where trans_task_id = #{trans_task_id} and deleted_by is null and deleted_date is null`"}
);
