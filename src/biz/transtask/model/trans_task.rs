use rbdc::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{common::model::entity::BaseEntity, sys::user::model::user::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransTask {
    pub trans_task_id: Option<i64>,
    pub data_source_id: i64,
    pub table_name: String,
    pub table_comment: String,
    pub remark: String,
    pub row_count: u16,
    pub last_trans_time: DateTime,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}

impl TransTask {
    pub fn from_create_bo(bo: TransTaskCreateBo, user: &User) -> Self {
        TransTask {
            trans_task_id: None,
            data_source_id: bo.data_source_id.unwrap(),
            table_name: bo.table_name,
            table_comment: bo.table_comment,
            remark: bo.remark,
            row_count: 0,
            last_trans_time: DateTime::now(),
            base_entity: BaseEntity::new(user.get_user_id()),
        }
    }

    pub fn from_update_bo(self: &mut Self, bo: TransTaskUpdateBo, user: &User) {
        self.trans_task_id = bo.trans_task_id;
        self.data_source_id = bo.data_source_id.unwrap();
        self.table_name = bo.table_name;
        self.table_comment = bo.table_comment;
        self.remark = bo.remark;
        self.base_entity.update(user.get_user_id());
    }

    pub fn to_list_vo(&self) -> TransTaskListVo {
        TransTaskListVo {
            trans_task_id: self.trans_task_id.unwrap(),
            data_source_id: self.data_source_id,
            table_name: self.table_name.clone(),
            table_comment: self.table_comment.clone(),
            remark: self.remark.clone(),
            row_count: self.row_count,
            last_trans_time: self.last_trans_time.clone(),
            base_entity: self.base_entity.clone(),
        }
    }

    pub fn to_detail_vo(&self) -> TransTaskDetailVo {
        TransTaskDetailVo {
            trans_task_id: self.trans_task_id.unwrap(),
            data_source_id: self.data_source_id,
            table_name: self.table_name.clone(),
            table_comment: self.table_comment.clone(),
            remark: self.remark.clone(),
            row_count: self.row_count,
            last_trans_time: self.last_trans_time.clone(),
            base_entity: self.base_entity.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct TransTaskCreateBo {
    #[validate(required)]
    pub data_source_id: Option<i64>,
    #[validate(length(min = 1, message = "table_name cannot be empty"))]
    pub table_name: String,
    #[validate(length(min = 1, message = "table_comment cannot be empty"))]
    pub table_comment: String,
    pub remark: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct TransTaskUpdateBo {
    #[validate(required)]
    pub trans_task_id: Option<i64>,
    #[validate(required)]
    pub data_source_id: Option<i64>,
    #[validate(length(min = 1, message = "table_name cannot be empty"))]
    pub table_name: String,
    #[validate(length(min = 1, message = "table_comment cannot be empty"))]
    pub table_comment: String,
    pub remark: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct TransTaskListVo {
    pub trans_task_id: i64,
    pub data_source_id: i64,
    pub table_name: String,
    pub table_comment: String,
    pub remark: String,
    pub row_count: u16,
    pub last_trans_time: DateTime,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct TransTaskDetailVo {
    pub trans_task_id: i64,
    pub data_source_id: i64,
    pub table_name: String,
    pub table_comment: String,
    pub remark: String,
    pub row_count: u16,
    pub last_trans_time: DateTime,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}
