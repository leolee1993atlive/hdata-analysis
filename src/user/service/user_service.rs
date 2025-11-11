use crate::{
    app::Infrastructure,
    user::{
        model::user::{User, UserCreateBo, UserListVo, UserUpdateBo},
        repository::user_repo::UserRepository,
    },
};

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            user_repo: UserRepository::new(infra.batis.clone()),
        }
    }

    pub async fn list(&self) -> Vec<UserListVo> {
        match self.user_repo.select_all().await {
            Ok(result) => {
                result.into_iter().map(|user| user.to_list_vo()).collect()
            }
            Err(e) => {
                tracing::error!("查询列表失败: {:?}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_user_by_id(&self, id: i64) -> Option<User> {
        match self.user_repo.select_by_id(&id).await {
            Ok(Some(user)) => Some(user),
            Ok(None) => {
                tracing::error!("根据ID查询失败: {:?}", id);
                None
            }
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                None
            }
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Option<User> {
        match self.user_repo.select_by_username(username).await {
            Ok(Some(user)) => Some(user),
            Ok(None) => {
                tracing::error!("根据用户名查询失败: {:?}", username);
                None
            }
            Err(e) => {
                tracing::error!("根据用户名查询失败: {:?}", e);
                None
            }
        }
    }

    pub async fn insert_by_bo(
        &self,
        bo: UserCreateBo,
        current_user: &User,
    ) -> bool {
        let user = User::from_create_bo(bo, current_user);
        match self.user_repo.insert(&user).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("创建失败: {:?}", e);
                false
            }
        }
    }

    pub async fn update_by_bo(
        &self,
        bo: UserUpdateBo,
        current_user: &User,
    ) -> bool {
        let user_id = &bo.user_id.unwrap();
        let mut user = match self.user_repo.select_by_id(user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                tracing::error!("根据ID查询失败: {:?}", user_id);
                return false;
            }
            Err(e) => {
                tracing::error!("根据ID查询失败: {:?}", e);
                return false;
            }
        };

        user.from_update_bo(bo, current_user);
        match self.user_repo.update_by_id(&user, user_id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("更新失败: {:?}", e);
                false
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64, current_user: &User) -> bool {
        // 先查询实体
        let mut user = match self.user_repo.select_by_id(&id).await {
            Ok(Some(user)) => user,
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
        user.base_entity.delete(current_user.get_user_id());

        // 更新实体（软删除）
        match self.user_repo.update_by_id(&user, &id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }

    pub async fn true_delete_by_id(&self, id: i64) -> bool {
        match self.user_repo.delete_by_id(&id).await {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("删除失败: {:?}", e);
                false
            }
        }
    }
}
