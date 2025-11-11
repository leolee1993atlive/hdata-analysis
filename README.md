# PetClinic-RS 🐾

一个使用 Rust 构建的现代化宠物诊所管理系统，基于 Axum 框架开发，提供完整的 RESTful API 服务。

## ✨ 特性

- 🔐 **JWT 身份认证** - 安全的用户认证和授权机制
- 🐕 **宠物管理** - 完整的宠物信息 CRUD 操作
- 👥 **用户管理** - 用户注册、登录和权限管理
- 🏷️ **宠物类型管理** - 宠物分类和类型管理
- 🗄️ **多数据库支持** - 同时支持 MySQL 和 DuckDB
- 📊 **连接池管理** - RBatis ORM + r2d2 连接池双重保障
- 🚀 **高性能异步** - 基于 Tokio 的异步运行时
- 📝 **结构化日志** - 使用 tracing 进行日志记录

## 🛠️ 技术栈

### 核心框架
- **[Axum](https://github.com/tokio-rs/axum)** - 现代化的 Rust Web 框架
- **[Tokio](https://tokio.rs/)** - 异步运行时
- **[Serde](https://serde.rs/)** - 序列化/反序列化

### 数据库
- **[RBatis](https://github.com/rbatis/rbatis)** - Rust ORM 框架
- **[r2d2](https://github.com/sfackler/r2d2)** - 数据库连接池
- **MySQL** - 主数据库
- **DuckDB** - 分析型数据库

### 认证与安全
- **[jsonwebtoken](https://github.com/Keats/jsonwebtoken)** - JWT 令牌处理
- **Bearer Token** 认证机制

### 配置与日志
- **[config](https://github.com/mehcode/config-rs)** - 配置管理
- **[tracing](https://github.com/tokio-rs/tracing)** - 结构化日志

## 📁 项目结构

```
src/
├── main.rs                 # 应用入口点
├── auth/                   # 认证模块
│   ├── handler/           # 认证处理器
│   ├── model/             # 认证模型
│   ├── route/             # 认证路由
│   └── service/           # 认证服务
├── user/                   # 用户管理模块
│   ├── handler/           # 用户处理器
│   ├── model/             # 用户模型
│   ├── repository/        # 用户数据访问
│   ├── route/             # 用户路由
│   └── service/           # 用户服务
├── pet/                    # 宠物管理模块
│   ├── handler/           # 宠物处理器
│   ├── model/             # 宠物模型
│   ├── repository/        # 宠物数据访问
│   ├── route/             # 宠物路由
│   └── service/           # 宠物服务
├── common/                 # 公共模块
│   ├── macros/            # 宏定义
│   ├── model/             # 公共模型
│   └── vo/                # 值对象
├── config/                 # 配置模块
├── error/                  # 错误处理
├── middleware/             # 中间件
└── util/                   # 工具类
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- MySQL 8.0+
- DuckDB (可选)

### 安装步骤

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd petclinic-rs
   ```

2. **配置数据库**
   
   编辑 `config.toml` 文件：
   ```toml
   [server]
   port = 3000
   host = "localhost"
   
   [db]
   url = "localhost"
   port = 3306
   db_name = "petclinic"
   username = "your_username"
   password = "your_password"
   
   [redis]
   url = "localhost"
   port = 6379
   password = ""
   db = 10
   ```

3. **创建数据库**
   ```sql
   CREATE DATABASE petclinic;
   ```

4. **运行应用**
   ```bash
   cargo run
   ```

   应用将在 `http://localhost:3000` 启动

## 📚 API 文档

### 认证端点

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| POST | `/api/login` | 用户登录 | ❌ |

### 用户管理

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| GET | `/api/user` | 获取用户列表 | ✅ |
| GET | `/api/user/{id}` | 获取用户详情 | ✅ |
| POST | `/api/user` | 创建用户 | ✅ |
| PUT | `/api/user/{id}` | 更新用户 | ✅ |
| DELETE | `/api/user/{id}` | 删除用户 | ✅ |

### 宠物管理

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| GET | `/api/pet` | 获取宠物列表 | ✅ |
| GET | `/api/pet/{id}` | 获取宠物详情 | ✅ |
| POST | `/api/pet` | 添加宠物 | ✅ |
| PUT | `/api/pet/{id}` | 更新宠物信息 | ✅ |
| DELETE | `/api/pet/{id}` | 删除宠物 | ✅ |

### 宠物类型管理

| 方法 | 端点 | 描述 | 认证 |
|------|------|------|------|
| GET | `/api/pet_type` | 获取宠物类型列表 | ✅ |
| GET | `/api/pet_type/{id}` | 获取宠物类型详情 | ✅ |
| POST | `/api/pet_type` | 创建宠物类型 | ✅ |
| PUT | `/api/pet_type/{id}` | 更新宠物类型 | ✅ |
| DELETE | `/api/pet_type/{id}` | 删除宠物类型 | ✅ |

### 请求示例

**登录获取 Token**
```bash
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

**获取宠物列表**
```bash
curl -X GET http://localhost:3000/api/pet \
  -H "Authorization: Bearer <your-jwt-token>"
```

**添加新宠物**
```bash
curl -X POST http://localhost:3000/api/pet \
  -H "Authorization: Bearer <your-jwt-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Buddy",
    "birth_date": "2020-01-15",
    "pet_type_id": 1,
    "owner_id": 1
  }'
```

## 🔧 开发命令

```bash
# 运行开发服务器
cargo run

# 构建发布版本
cargo build --release

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 热重载开发 (需要安装 cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

## 🏗️ 架构设计

### 应用状态

```rust
pub struct AppState {
    pub batis: RBatis,              // RBatis ORM 实例
    pub pool: ConnectionPool,       // 连接池管理
}

pub struct ConnectionPool {
    pub rb_pool: Pool<MySqlConnectionManager>,    // MySQL 连接池
    pub duck_pool: Pool<DuckdbConnectionManager>, // DuckDB 连接池
}
```

### 数据模型

所有实体都继承 `BaseEntity`，包含通用字段：
- `version` - 版本号（乐观锁）
- `created_by` - 创建者
- `created_date` - 创建时间
- `last_modified_by` - 最后修改者
- `last_modified_date` - 最后修改时间
- `deleted_by` - 删除者（软删除）
- `deleted_date` - 删除时间（软删除）

## 🔐 认证机制

1. **登录流程**：用户提供用户名和密码
2. **Token 生成**：服务器验证凭据后生成 JWT Token
3. **Token 使用**：客户端在请求头中携带 `Authorization: Bearer <token>`
4. **权限验证**：中间件验证 Token 并检查用户权限

## 🤝 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Axum](https://github.com/tokio-rs/axum) - 优秀的 Rust Web 框架
- [RBatis](https://github.com/rbatis/rbatis) - 强大的 Rust ORM
- [Tokio](https://tokio.rs/) - 异步运行时支持

---

**Happy Coding! 🎉**