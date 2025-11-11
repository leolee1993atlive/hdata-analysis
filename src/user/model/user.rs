use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::model::entity::BaseEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: Option<i64>,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub active: String,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}

impl User {
    pub fn from_create_bo(bo: UserCreateBo, user: &User) -> Self {
        Self {
            user_id: None,
            username: bo.username,
            password: hash_password(&bo.password),
            first_name: bo.first_name,
            last_name: bo.last_name,
            email: bo.email,
            active: bo.active,
            base_entity: BaseEntity::new(user.get_user_id()),
        }
    }

    pub fn from_update_bo(self: &mut Self, bo: UserUpdateBo, user: &User) {
        self.user_id = bo.user_id;
        self.username = bo.username;
        self.first_name = bo.first_name;
        self.last_name = bo.last_name;
        self.email = bo.email;
        self.active = bo.active;
        self.base_entity.update(user.get_user_id());
    }

    pub fn to_list_vo(&self) -> UserListVo {
        UserListVo {
            user_id: self.user_id.unwrap(),
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            active: self.active.clone(),
            base_entity: self.base_entity.clone(),
        }
    }

    pub fn get_user_id(&self) -> i64 {
        self.user_id.unwrap_or(0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserCreateBo {
    #[validate(length(min = 1, message = "username cannot be empty"))]
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub active: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate)]
pub struct UserUpdateBo {
    #[validate(required)]
    pub user_id: Option<i64>,
    #[validate(length(min = 1, message = "username cannot be empty"))]
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub active: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserListVo {
    pub user_id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub active: String,
    #[serde(flatten)]
    pub base_entity: BaseEntity,
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}
