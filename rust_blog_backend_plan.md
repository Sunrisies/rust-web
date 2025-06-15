# Rust博客后端服务开发计划

## 技术栈选择

### 核心框架与库
- **Web框架**: [Actix-web](https://actix.rs/) - 高性能、异步Rust Web框架
- **ORM**: [Diesel](https://diesel.rs/) 或 [SQLx](https://github.com/launchbadge/sqlx) - 类型安全的SQL交互
- **认证**: [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT认证实现
- **序列化/反序列化**: [Serde](https://serde.rs/) - 处理JSON数据
- **日志**: [log](https://github.com/rust-lang/log) + [env_logger](https://github.com/env-logger-rs/env_logger/) - 日志记录
- **配置**: [config](https://github.com/mehcode/config-rs) - 配置管理
- **密码哈希**: [argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) - 安全密码存储
- **验证**: [validator](https://github.com/Keats/validator) - 数据验证

### 数据库
- **主数据库**: PostgreSQL - 强大的关系型数据库，适合博客系统
- **缓存**: Redis - 用于缓存和会话管理

## 项目结构

```
rust-blog-backend/
├── src/
│   ├── api/                 # API路由和处理器
│   │   ├── auth.rs          # 认证相关API
│   │   ├── posts.rs         # 文章相关API
│   │   ├── comments.rs      # 评论相关API
│   │   ├── users.rs         # 用户相关API
│   │   ├── tags.rs          # 标签相关API
│   │   └── mod.rs           # API模块导出
│   ├── config/              # 配置相关代码
│   │   ├── database.rs      # 数据库配置
│   │   ├── server.rs        # 服务器配置
│   │   └── mod.rs           # 配置模块导出
│   ├── db/                  # 数据库交互
│   │   ├── models/          # 数据库模型
│   │   ├── schema.rs        # 数据库模式(Diesel)
│   │   └── mod.rs           # 数据库模块导出
│   ├── middleware/          # 中间件
│   │   ├── auth.rs          # 认证中间件
│   │   ├── logger.rs        # 日志中间件
│   │   └── mod.rs           # 中间件模块导出
│   ├── services/            # 业务逻辑
│   │   ├── auth_service.rs  # 认证服务
│   │   ├── post_service.rs  # 文章服务
│   │   └── mod.rs           # 服务模块导出
│   ├── utils/               # 工具函数
│   │   ├── error.rs         # 错误处理
│   │   ├── jwt.rs           # JWT工具
│   │   └── mod.rs           # 工具模块导出
│   ├── main.rs              # 应用入口
│   └── lib.rs               # 库入口
├── migrations/              # 数据库迁移文件
├── tests/                   # 集成测试
├── Cargo.toml               # 项目依赖
├── Cargo.lock               # 依赖锁定文件
├── .env                     # 环境变量
├── .env.example             # 环境变量示例
├── Dockerfile               # Docker配置
├── docker-compose.yml       # Docker Compose配置
└── README.md                # 项目说明
```

## 数据库设计

### 表结构

1. **users** - 用户表
   ```sql
   CREATE TABLE users (
       id SERIAL PRIMARY KEY,
       username VARCHAR(50) NOT NULL UNIQUE,
       email VARCHAR(100) NOT NULL UNIQUE,
       password_hash VARCHAR(255) NOT NULL,
       display_name VARCHAR(100),
       bio TEXT,
       avatar_url VARCHAR(255),
       role VARCHAR(20) NOT NULL DEFAULT 'user',
       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
   );
   ```

2. **posts** - 文章表
   ```sql
   CREATE TABLE posts (
       id SERIAL PRIMARY KEY,
       title VARCHAR(255) NOT NULL,
       slug VARCHAR(255) NOT NULL UNIQUE,
       content TEXT NOT NULL,
       excerpt TEXT,
       featured_image VARCHAR(255),
       published BOOLEAN NOT NULL DEFAULT false,
       author_id INTEGER NOT NULL REFERENCES users(id),
       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
       published_at TIMESTAMP
   );
   ```

3. **tags** - 标签表
   ```sql
   CREATE TABLE tags (
       id SERIAL PRIMARY KEY,
       name VARCHAR(50) NOT NULL UNIQUE,
       slug VARCHAR(50) NOT NULL UNIQUE,
       description TEXT,
       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
   );
   ```

4. **post_tags** - 文章标签关联表
   ```sql
   CREATE TABLE post_tags (
       post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
       tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
       PRIMARY KEY (post_id, tag_id)
   );
   ```

5. **comments** - 评论表
   ```sql
   CREATE TABLE comments (
       id SERIAL PRIMARY KEY,
       content TEXT NOT NULL,
       post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
       user_id INTEGER NOT NULL REFERENCES users(id),
       parent_id INTEGER REFERENCES comments(id) ON DELETE CASCADE,
       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
       updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
   );
   ```

6. **likes** - 点赞表
   ```sql
   CREATE TABLE likes (
       user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
       post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
       PRIMARY KEY (user_id, post_id)
   );
   ```

## API设计

### 认证API

1. **注册**
   - 路径: `POST /api/auth/register`
   - 请求体:
     ```json
     {
       "username": "user123",
       "email": "user@example.com",
       "password": "securepassword"
     }
     ```
   - 响应:
     ```json
     {
       "status": "success",
       "message": "User registered successfully",
       "data": {
         "id": 1,
         "username": "user123",
         "email": "user@example.com"
       }
     }
     ```

2. **登录**
   - 路径: `POST /api/auth/login`
   - 请求体:
     ```json
     {
       "username": "user123",
       "password": "securepassword"
     }
     ```
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Login successful",
       "data": {
         "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
         "user": {
           "id": 1,
           "username": "user123",
           "email": "user@example.com",
           "role": "user"
         }
       }
     }
     ```

### 文章API

1. **获取文章列表**
   - 路径: `GET /api/posts`
   - 查询参数: `page`, `limit`, `tag`, `search`
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "posts": [
           {
             "id": 1,
             "title": "Hello World",
             "slug": "hello-world",
             "excerpt": "This is my first post",
             "featured_image": "https://example.com/image.jpg",
             "author": {
               "id": 1,
               "username": "user123",
               "display_name": "John Doe"
             },
             "tags": ["rust", "programming"],
             "published_at": "2023-01-01T12:00:00Z",
             "comment_count": 5,
             "like_count": 10
           }
         ],
         "total": 50,
         "page": 1,
         "limit": 10
       }
     }
     ```

2. **获取单篇文章**
   - 路径: `GET /api/posts/{slug}`
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "id": 1,
         "title": "Hello World",
         "slug": "hello-world",
         "content": "# Hello World\n\nThis is my first post...",
         "featured_image": "https://example.com/image.jpg",
         "author": {
           "id": 1,
           "username": "user123",
           "display_name": "John Doe",
           "bio": "Rust developer"
         },
         "tags": [
           {"id": 1, "name": "rust", "slug": "rust"},
           {"id": 2, "name": "programming", "slug": "programming"}
         ],
         "published_at": "2023-01-01T12:00:00Z",
         "comment_count": 5,
         "like_count": 10,
         "liked_by_user": false
       }
     }
     ```

3. **创建文章**
   - 路径: `POST /api/posts`
   - 权限: 需要认证
   - 请求体:
     ```json
     {
       "title": "New Post",
       "content": "# New Post\n\nThis is a new post...",
       "excerpt": "This is a new post",
       "featured_image": "https://example.com/image.jpg",
       "tags": ["rust", "web"],
       "published": true
     }
     ```
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Post created successfully",
       "data": {
         "id": 2,
         "title": "New Post",
         "slug": "new-post"
       }
     }
     ```

4. **更新文章**
   - 路径: `PUT /api/posts/{id}`
   - 权限: 需要认证，只有作者或管理员可以更新
   - 请求体: (与创建文章相同)
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Post updated successfully",
       "data": {
         "id": 2,
         "title": "Updated Post",
         "slug": "updated-post"
       }
     }
     ```

5. **删除文章**
   - 路径: `DELETE /api/posts/{id}`
   - 权限: 需要认证，只有作者或管理员可以删除
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Post deleted successfully"
     }
     ```

### 评论API

1. **获取文章评论**
   - 路径: `GET /api/posts/{post_id}/comments`
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "comments": [
           {
             "id": 1,
             "content": "Great post!",
             "user": {
               "id": 2,
               "username": "user456",
               "display_name": "Jane Smith"
             },
             "created_at": "2023-01-02T10:30:00Z",
             "replies": [
               {
                 "id": 2,
                 "content": "Thanks!",
                 "user": {
                   "id": 1,
                   "username": "user123",
                   "display_name": "John Doe"
                 },
                 "created_at": "2023-01-02T11:00:00Z"
               }
             ]
           }
         ]
       }
     }
     ```

2. **添加评论**
   - 路径: `POST /api/posts/{post_id}/comments`
   - 权限: 需要认证
   - 请求体:
     ```json
     {
       "content": "This is a comment",
       "parent_id": null
     }
     ```
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Comment added successfully",
       "data": {
         "id": 3,
         "content": "This is a comment",
         "created_at": "2023-01-03T09:15:00Z"
       }
     }
     ```

### 用户API

1. **获取当前用户信息**
   - 路径: `GET /api/users/me`
   - 权限: 需要认证
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "id": 1,
         "username": "user123",
         "email": "user@example.com",
         "display_name": "John Doe",
         "bio": "Rust developer",
         "avatar_url": "https://example.com/avatar.jpg",
         "role": "user",
         "created_at": "2023-01-01T00:00:00Z"
       }
     }
     ```

2. **更新用户信息**
   - 路径: `PUT /api/users/me`
   - 权限: 需要认证
   - 请求体:
     ```json
     {
       "display_name": "John Smith",
       "bio": "Rust and Web developer",
       "avatar_url": "https://example.com/new-avatar.jpg"
     }
     ```
   - 响应:
     ```json
     {
       "status": "success",
       "message": "Profile updated successfully",
       "data": {
         "display_name": "John Smith",
         "bio": "Rust and Web developer",
         "avatar_url": "https://example.com/new-avatar.jpg"
       }
     }
     ```

### 标签API

1. **获取所有标签**
   - 路径: `GET /api/tags`
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "tags": [
           {
             "id": 1,
             "name": "Rust",
             "slug": "rust",
             "description": "Rust programming language",
             "post_count": 15
           },
           {
             "id": 2,
             "name": "Web Development",
             "slug": "web-development",
             "description": "Web development topics",
             "post_count": 25
           }
         ]
       }
     }
     ```

2. **获取标签详情**
   - 路径: `GET /api/tags/{slug}`
   - 响应:
     ```json
     {
       "status": "success",
       "data": {
         "id": 1,
         "name": "Rust",
         "slug": "rust",
         "description": "Rust programming language",
         "post_count": 15,
         "recent_posts": [
           {
             "id": 1,
             "title": "Getting Started with Rust",
             "slug": "getting-started-with-rust",
             "excerpt": "Learn the basics of Rust..."
           }
         ]
       }
     }
     ```

## 开发时间表（每天2小时）

### 第一周：项目设置和基础功能（12小时）

| 日期 | 任务 | 预计时间 | 完成情况 |
|------|------|----------|----------|
| 第1天 | 项目初始化、依赖配置、数据库设计 | 2小时 | □ |
| 第2天 | 数据库迁移、模型定义 | 2小时 | □ |
| 第3天 | 用户认证系统（注册功能） | 2小时 | □ |
| 第4天 | 用户认证系统（登录功能、JWT） | 2小时 | □ |
| 第5天 | 错误处理、中间件设置 | 2小时 | □ |
| 第6天 | 用户信息API实现 | 2小时 | □ |

### 第二周：文章功能实现（12小时）

| 日期 | 任务 | 预计时间 | 完成情况 |
|------|------|----------|----------|
| 第7天 | 文章CRUD API - 创建和读取 | 2小时 | □ |
| 第8天 | 文章CRUD API - 更新和删除 | 2小时 | □ |
| 第9天 | 文章列表分页和过滤 | 2小时 | □ |
| 第10天 | 文章搜索功能 | 2小时 | □ |
| 第11天 | 标签系统实现 | 2小时 | □ |
| 第12天 | 文章标签关联功能 | 2小时 | □ |

### 第三周：评论和互动功能（12小时）

| 日期 | 任务 | 预计时间 | 完成情况 |
|------|------|----------|----------|
| 第13天 | 评论系统基础功能 | 2小时 | □ |
| 第14天 | 评论嵌套回复功能 | 2小时 | □ |
| 第15天 | 评论分页和排序 | 2小时 | □ |
| 第16天 | 点赞功能实现 | 2小时 | □ |
| 第17天 | 用户文章收藏功能 | 2小时 | □ |
| 第18天 | 文章统计功能（阅读量、点赞数） | 2小时 | □ |

### 第四周：性能优化和高级功能（12小时）

| 日期 | 任务 | 预计时间 | 完成情况 |
|------|------|----------|----------|
| 第19天 | Redis缓存集成 | 2小时 | □ |
| 第20天 | 热门文章缓存实现 | 2小时 | □ |
| 第21天 | 数据库查询优化 | 2小时 | □ |
| 第22天 | API响应缓存 | 2小时 | □ |
| 第23天 | 图片上传功能 | 2小时 | □ |
| 第24天 | 用户头像处理 | 2小时 | □ |

### 第五周：测试和文档（12小时）

| 日期 | 任务 | 预计时间 | 完成情况 |
|------|------|----------|----------|
| 第25天 | 单元测试编写 - 模型和服务 | 2小时 | □ |
| 第26天 | 单元测试编写 - API和中间件 | 2小时 | □ |
| 第27天 | 集成测试编写 | 2小时 | □ |
| 第28天 | API文档生成 | 2小时 | □ |
| 第29天 | Swagger/OpenAPI集成 | 2小时 | □ |
| 第30天 | 部署文档编写 | 2小时 | □ |

## 优先级建议

### 高优先级（核心功能）
1. 用户认证系统
2. 文章CRUD功能
3. 数据库设计和迁移
4. 错误处理机制
5. 基本的API文档

### 中优先级（重要功能）
1. 评论系统
2. 标签系统
3. 文章搜索
4. 缓存机制
5. 单元测试

### 低优先级（可后期添加）
1. 图片上传
2. 用户头像
3. 高级统计功能
4. Swagger集成
5. 性能优化

## 开发注意事项

1. **代码质量**
   - 遵循Rust编码规范
   - 使用clippy进行代码检查
   - 编写清晰的文档注释
   - 保持代码简洁和可维护性

2. **安全性**
   - 正确处理用户认证和授权
   - 防止SQL注入
   - 安全存储密码（使用argon2）
   - 实施适当的速率限制

3. **性能**
   - 合理使用数据库索引
   - 实现必要的缓存机制
   - 优化数据库查询
   - 处理并发请求

4. **测试**
   - 编写单元测试和集成测试
   - 测试错误处理
   - 测试边界情况
   - 进行性能测试

5. **部署**
   - 准备Docker配置
   - 设置CI/CD流程
   - 配置日志记录
   - 准备监控方案