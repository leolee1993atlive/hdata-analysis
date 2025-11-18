use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get},
};

use crate::{
    app::AppState,
    biz::datasource::handler::data_source_handler::{
        create_data_source, delete_data_source, get_data_source_by_id,
        list_data_source, test_data_source, true_delete_data_source,
        update_data_source,
    },
};

pub fn data_source_route() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/datasource",
            get(list_data_source)
                .post(create_data_source)
                .put(update_data_source),
        )
        .route(
            "/datasource/{id}",
            get(get_data_source_by_id).delete(delete_data_source),
        )
        .route("/datasource/delete/{id}", delete(true_delete_data_source))
        .route("/datasource/test/{id}", get(test_data_source))
}
