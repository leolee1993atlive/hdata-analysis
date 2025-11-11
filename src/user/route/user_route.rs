use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};

use crate::{
    app::AppState,
    user::handler::user_handler::{
        create_user, delete_user, get_user_by_id, list_user, true_delete_user,
        update_user,
    },
};

pub fn user_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/user", get(list_user).post(create_user).put(update_user))
        .route("/user/{id}", get(get_user_by_id).delete(delete_user))
        .route("/user/delete/{id}", delete(true_delete_user))
}
