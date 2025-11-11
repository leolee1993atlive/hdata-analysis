use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{common::model::entity::BaseEntity, sys::user::model::user::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    pub data_source_id: Option<i64>,
    pub code: String,
    pub name: String,
    pub remark: String,
    pub db_type: String,
    pub db_driver: String,
    pub db_url: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}

impl DataSource {
    pub fn from_create_bo(bo: DataSourceCreateBo, user: &User) -> Self {
        Self {
            data_source_id: None,
            code: bo.code,
            name: bo.name,
            remark: bo.remark,
            db_type: bo.db_type,
            db_driver: bo.db_driver,
            db_url: bo.db_url,
            db_name: bo.db_name,
            db_username: bo.db_username,
            db_password: bo.db_password,
            base_entity: BaseEntity::new(user.get_user_id()),
        }
    }

    pub fn from_update_bo(
        self: &mut Self,
        bo: DataSourceUpdateBo,
        user: &User,
    ) {
        self.data_source_id = bo.data_source_id;
        self.code = bo.code;
        self.name = bo.name;
        self.remark = bo.remark;
        self.db_type = bo.db_type;
        self.db_driver = bo.db_driver;
        self.db_url = bo.db_url;
        self.db_name = bo.db_name;
        self.db_username = bo.db_username;
        self.db_password = bo.db_password;
        self.base_entity.update(user.get_user_id());
    }

    pub fn to_list_vo(&self) -> DataSourceListVo {
        DataSourceListVo {
            data_source_id: self.data_source_id.unwrap(),
            code: self.code.clone(),
            name: self.name.clone(),
            remark: self.remark.clone(),
            db_type: self.db_type.clone(),
            db_name: self.db_name.clone(),
            base_entity: self.base_entity.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct DataSourceCreateBo {
    #[validate(length(min = 1, message = "code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,
    pub remark: String,
    #[validate(length(min = 1, message = "db_type cannot be empty"))]
    pub db_type: String,
    #[validate(length(min = 1, message = "db_driver cannot be empty"))]
    pub db_driver: String,
    #[validate(length(min = 1, message = "db_url cannot be empty"))]
    pub db_url: String,
    #[validate(length(min = 1, message = "db_name cannot be empty"))]
    pub db_name: String,
    #[validate(length(min = 1, message = "db_username cannot be empty"))]
    pub db_username: String,
    #[validate(length(min = 1, message = "db_password cannot be empty"))]
    pub db_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct DataSourceUpdateBo {
    #[validate(required)]
    pub data_source_id: Option<i64>,
    #[validate(length(min = 1, message = "code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "name cannot be empty"))]
    pub name: String,
    pub remark: String,
    #[validate(length(min = 1, message = "db_type cannot be empty"))]
    pub db_type: String,
    #[validate(length(min = 1, message = "db_driver cannot be empty"))]
    pub db_driver: String,
    #[validate(length(min = 1, message = "db_url cannot be empty"))]
    pub db_url: String,
    #[validate(length(min = 1, message = "db_name cannot be empty"))]
    pub db_name: String,
    #[validate(length(min = 1, message = "db_username cannot be empty"))]
    pub db_username: String,
    #[validate(length(min = 1, message = "db_password cannot be empty"))]
    pub db_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct DataSourceListVo {
    pub data_source_id: i64,
    pub code: String,
    pub name: String,
    pub remark: String,
    pub db_type: String,
    pub db_name: String,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}
