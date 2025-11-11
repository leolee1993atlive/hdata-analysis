use std::sync::Arc;

use duckdb::DuckdbConnectionManager;
use r2d2::Pool;
use r2d2_mysql::{MySqlConnectionManager, mysql::OptsBuilder};
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;

use crate::{
    config::config::AppConfig,
    pet::service::{pet_service::PetService, pet_type_service::PetTypeService},
    user::service::user_service::UserService,
};

// 定义应用状态结构体，包含数据库连接池
#[derive(Clone)]
pub struct AppState {
    pub infra: Infrastructure,
    pub services: ServiceContainer,
}

#[derive(Clone)]
pub struct Infrastructure {
    pub batis: Arc<RBatis>,
    pub pool: Arc<ConnectionPool>,
}

#[derive(Clone)]
pub struct ConnectionPool {
    pub redis_pool: Pool<redis::Client>,
    pub rb_pool: Pool<MySqlConnectionManager>,
    pub duck_pool: Pool<DuckdbConnectionManager>,
}

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_service: UserService,
    pub pet_service: PetService,
    pub pet_type_service: PetTypeService,
}

impl ServiceContainer {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            user_service: UserService::new(infra),
            pet_service: PetService::new(infra),
            pet_type_service: PetTypeService::new(infra),
        }
    }
}

pub async fn init_app(config: &AppConfig) -> Arc<AppState> {
    // 创建Redis示例
    let redis_url = match &config.redis.password {
        Some(password) => format!(
            "redis://:{}@{}:{}/{}",
            password, config.redis.url, config.redis.port, config.redis.db
        ),
        None => format!(
            "redis://{}:{}/{}",
            config.redis.url, config.redis.port, config.redis.db
        ),
    };
    let redis_client = redis::Client::open(redis_url).unwrap();
    let redis_pool = Pool::builder().build(redis_client).unwrap();

    // 创建RBatis实例
    let rb = RBatis::new();
    let dsn = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db.username,
        config.db.password,
        config.db.url,
        config.db.port,
        config.db.db_name,
    );
    match rb.link(MysqlDriver {}, dsn.as_str()).await {
        Ok(_) => tracing::info!(
            "Successfully connected to database with RBatis: {}",
            config.db.db_name
        ),
        Err(e) => {
            tracing::error!("Failed to connect to database with RBatis: {}", e);
            panic!("RBatis database connection failed: {}", e);
        }
    }

    // 创建MySQL连接选项用于r2d2连接池
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(config.db.url.clone()))
        .tcp_port(config.db.port as u16)
        .user(Some(config.db.username.clone()))
        .pass(Some(config.db.password.clone()))
        .db_name(Some(config.db.db_name.clone()));

    // 创建连接管理器
    let rb_manager = MySqlConnectionManager::new(opts);
    // 创建r2d2连接池
    let rb_pool = match Pool::new(rb_manager) {
        Ok(pool) => {
            tracing::info!(
                "Successfully created r2d2 database connection pool for: {}",
                config.db.db_name
            );
            pool
        }
        Err(e) => {
            tracing::error!(
                "Failed to create r2d2 database connection pool: {}",
                e
            );
            panic!("r2d2 database connection pool creation failed: {}", e);
        }
    };

    let duckdb_path = format!("{}_duck.duckdb", config.db.db_name);
    let duck_manager = match DuckdbConnectionManager::file(duckdb_path) {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!(
                "Failed to create duckdb connection manager: {}",
                e
            );
            panic!("duckdb connection manager creation failed: {}", e);
        }
    };
    let duck_pool = match Pool::builder().max_size(10).build(duck_manager) {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to create duckdb connection pool: {}", e);
            panic!("duckdb connection pool creation failed: {}", e);
        }
    };

    // 创建共享应用状态，包含RBatis和r2d2连接池
    let infra = Infrastructure {
        batis: Arc::new(rb.clone()),
        pool: Arc::new(ConnectionPool {
            redis_pool: redis_pool,
            rb_pool: rb_pool,
            duck_pool: duck_pool,
        }),
    };
    let services = ServiceContainer::new(&infra);
    Arc::new(AppState {
        infra: infra,
        services: services,
    })
}
