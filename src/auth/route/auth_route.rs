use crate::app::AppState;
use crate::auth::handler::auth_handler::login;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn auth_route() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(login))
}
