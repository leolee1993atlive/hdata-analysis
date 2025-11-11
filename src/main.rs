mod app;
mod auth;
mod common;
mod config;
mod error;
mod middleware;
mod pet;
mod user;
mod util;

use ::config::{Config, File};
use axum::Router;
use axum::middleware::from_fn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::auth::route::auth_route::auth_route;
use crate::config::config::AppConfig;
use crate::middleware::auth::auth;
use crate::pet::route::pet_route::pet_route;
use crate::pet::route::pet_type_route::pet_type_route;
use crate::user::route::user_route::user_route;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    format!("{}=debug", env!("CARGO_CRATE_NAME")).into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载和解析配置文件
    let config = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<AppConfig>()
        .unwrap();

    let app_state = app::init_app(&config).await;

    let app = Router::new().nest(
        "/api",
        Router::new()
            .merge(auth_route())
            .merge(user_route())
            .merge(pet_route())
            .merge(pet_type_route())
            .route_layer(from_fn(auth))
            .with_state(app_state),
    );

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.server.host, config.server.port
    ))
    .await
    .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
