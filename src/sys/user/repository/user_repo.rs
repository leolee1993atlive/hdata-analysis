use std::sync::Arc;

use crate::sys::user::model::user::User;
use rbatis::{RBatis, crud, impl_delete, impl_select, impl_update};
use rbdc::db::ExecResult;

#[derive(Clone)]
pub struct UserRepository {
    rb: Arc<RBatis>,
}

impl UserRepository {
    pub fn new(rb: Arc<RBatis>) -> Self {
        Self { rb }
    }

    pub async fn select_all(&self) -> Result<Vec<User>, rbatis::Error> {
        User::select_all(&*self.rb).await
    }

    pub async fn select_by_id(
        &self,
        user_id: &i64,
    ) -> Result<Option<User>, rbatis::Error> {
        User::select_by_id(&*self.rb, user_id).await
    }

    pub async fn select_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, rbatis::Error> {
        User::select_by_username(&*self.rb, username).await
    }

    pub async fn insert(
        &self,
        user: &User,
    ) -> Result<ExecResult, rbatis::Error> {
        User::insert(&*self.rb, user).await
    }

    pub async fn update_by_id(
        &self,
        user: &User,
        user_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        User::update_by_id(&*self.rb, user, user_id).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &i64,
    ) -> Result<ExecResult, rbatis::Error> {
        User::delete_by_id(&*self.rb, user_id).await
    }
}

crud!(User {});
impl_select!(
    User{select_by_username(username: &str) -> Option => "`where username = #{username} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_select!(
    User{select_by_id(user_id: &i64) -> Option => "`where user_id = #{user_id} and deleted_by is null and deleted_date is null limit 1`"}
);
impl_update!(
    User{update_by_id(user_id: &i64) => "`where user_id = #{user_id} and deleted_by is null and deleted_date is null`"}
);
impl_delete!(
    User{delete_by_id(user_id: &i64) => "`where user_id = #{user_id} and deleted_by is null and deleted_date is null`"}
);
