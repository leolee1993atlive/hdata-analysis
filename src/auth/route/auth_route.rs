use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{app::AppState, auth::handler::auth_handler::login};

pub fn auth_route() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(login))
}
