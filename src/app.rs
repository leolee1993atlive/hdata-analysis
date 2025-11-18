use std::sync::Arc;

use r2d2::Pool;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;

use crate::{
    biz::{
        datasource::service::data_source_service::DataSourceService,
        pet::service::{
            pet_service::PetService, pet_type_service::PetTypeService,
        },
        transtask::service::trans_task_service::TransTaskService,
    },
    config::config::AppConfig,
    sys::user::service::user_service::UserService,
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
}

#[derive(Clone)]
pub struct ServiceContainer {
    pub user_service: UserService,
    pub pet_service: PetService,
    pub pet_type_service: PetTypeService,
    pub data_source_service: DataSourceService,
    pub trans_task_service: TransTaskService,
}

impl ServiceContainer {
    pub fn new(infra: &Infrastructure) -> Self {
        Self {
            user_service: UserService::new(infra),
            pet_service: PetService::new(infra),
            pet_type_service: PetTypeService::new(infra),
            data_source_service: DataSourceService::new(infra),
            trans_task_service: TransTaskService::new(infra),
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

    // 创建共享应用状态，包含RBatis和r2d2连接池
    let infra = Infrastructure {
        batis: Arc::new(rb.clone()),
        pool: Arc::new(ConnectionPool {
            redis_pool: redis_pool,
        }),
    };
    let services = ServiceContainer::new(&infra);
    Arc::new(AppState {
        infra: infra,
        services: services,
    })
}
