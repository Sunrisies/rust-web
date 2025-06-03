# MySQL User CRUD API

这是一个使用 Rust 和 Actix-web 框架实现的 RESTful CRUD API 示例，用于用户管理系统。该项目展示了如何构建一个高性能、类型安全的 Web 服务，并与 MySQL 数据库进行交互。

## 功能特点

- RESTful API 设计
- MySQL 数据库集成
- 完整的 CRUD 操作
- 分页查询支持
- 错误处理和验证
- Docker 支持
- 性能优化配置

## 技术栈

- Rust 2021 Edition
- Actix-web 4.4.0 (Web 框架)
- MySQL 24.0.0 (数据库驱动)
- Serde (序列化/反序列化)
- Tokio (异步运行时)
- Docker (容器化)

## 项目结构

```
mysql_user_crud/
├── src/
│   ├── api/                    # API 相关代码
│   │   ├── handlers.rs         # 请求处理器
│   │   ├── routes.rs           # 路由配置
│   │   └── mod.rs             # 模块声明
│   ├── db/                     # 数据库相关代码
│   │   ├── database.rs         # 数据库操作
│   │   └── mod.rs             # 模块声明
│   ├── models/                 # 数据模型
│   │   ├── user.rs            # 用户模型
│   │   └── mod.rs             # 模块声明
│   ├── lib.rs                  # 库入口
│   └── main.rs                 # 应用入口
├── Cargo.toml                  # 项目配置和依赖
├── Dockerfile                  # Docker 配置
├── build.sh                    # 构建脚本
└── .env.test                   # 测试环境配置
```

## API 接口

### 用户管理 API

基础路径: `/api/users`

| 方法   | 路径     | 描述         | 请求体                  | 响应                    |
|--------|----------|--------------|------------------------|------------------------|
| GET    | /        | 获取用户列表   | 查询参数: page, limit   | 分页用户列表             |
| POST   | /        | 创建新用户     | 用户信息                | 创建的用户信息           |
| GET    | /{id}    | 获取单个用户   | -                      | 用户信息                |
| PUT    | /{id}    | 更新用户信息   | 更新的用户信息           | 更新后的用户信息         |
| DELETE | /{id}    | 删除用户      | -                      | 204 No Content        |

### 请求/响应示例

#### 创建用户
```http
POST /api/users
Content-Type: application/json

{
    "username": "john_doe",
    "email": "john@example.com",
    "age": 30
}
```

#### 分页获取用户
```http
GET /api/users?page=1&limit=10
```

响应:
```json
{
    "data": [...],
    "pagination": {
        "total": 100,
        "total_pages": 10,
        "current_page": 1,
        "limit": 10,
        "has_next": true,
        "has_previous": false
    }
}
```

## 数据库设计

用户表结构:
```sql
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    age INT
);
```

## 快速开始

### 本地开发

1. 克隆项目:
```bash
git clone <repository-url>
cd mysql_user_crud
```

2. 设置环境变量:
```bash
cp .env.test .env
# 编辑 .env 文件，设置数据库连接信息
```

3. 构建和运行:
```bash
cargo build
cargo run
```

### Docker 部署

1. 构建镜像:
```bash
docker build -t mysql_user_crud .
```

2. 运行容器:
```bash
docker run -d \
  -p 8080:8080 \
  -e DATABASE_URL=mysql://username:password@host:3306/dbname \
  mysql_user_crud
```

## 性能优化

项目包含多层面的性能优化：

1. **编译优化**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
debug = false
```

2. **Docker 优化**
- 多阶段构建
- 最小化基础镜像
- UPX 压缩

3. **运行时优化**
- 连接池管理
- 异步处理
- 分页查询

## 错误处理

系统实现了完整的错误处理机制：

- 输入验证
- 数据库错误处理
- HTTP 错误响应
- 自定义错误类型

## 测试

运行测试:
```bash
cargo test
```

## 部署建议

1. **环境配置**
   - 使用环境变量进行配置
   - 根据环境调整日志级别
   - 配置适当的数据库连接池大小

2. **监控**
   - 实现健康检查端点
   - 监控数据库连接状态
   - 跟踪请求响应时间

3. **安全性**
   - 启用 HTTPS
   - 实现速率限制
   - 添加请求验证

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

[MIT License](LICENSE)

## 维护者

[sunrise](3266420686@qq.com)