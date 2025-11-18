use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};

use crate::{
    app::AppState,
    biz::transtask::handler::trans_task_handler::{
        create_trans_task, delete_trans_task, get_trans_task_by_id,
        list_trans_task, true_delete_trans_task, update_trans_task,
    },
};

pub fn trans_task_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/transtask",
            get(list_trans_task)
                .post(create_trans_task)
                .put(update_trans_task),
        )
        .route(
            "/transtask/{id}",
            get(get_trans_task_by_id).delete(delete_trans_task),
        )
        .route("/transtask/delete/{id}", delete(true_delete_trans_task))
}
