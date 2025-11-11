use serde::Deserialize;

// 配置结构体
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
}

// 服务器配置结构体
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

// 数据库配置结构体
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub url: String,
    pub port: i32,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

// Redis配置结构体
#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub port: i32,
    pub password: Option<String>,
    pub db: i32,
}
