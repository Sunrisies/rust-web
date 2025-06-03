# MySQL User CRUD API

这是一个基于 Rust 和 Actix-web 实现的用户管理 RESTful API 系统，提供用户的增删改查功能，支持分页查询。

## 环境要求

- Rust
- MySQL
- 环境变量配置（.env 文件）

## 配置说明

1. 创建 `.env` 文件并配置数据库连接信息：

```env
DATABASE_URL=mysql://username:password@localhost:3306/database_name
```

2. 运行服务器：

```bash
cargo run
```

默认服务器运行在 `http://localhost:8080`

## API 端点

### 1. 获取用户列表（支持分页）

```
GET /api/users
```

查询参数：
- `page`: 页码（可选，默认值：1）
- `limit`: 每页数量（可选，默认值：10，最大值：100）

示例请求：
```bash
# 使用默认分页参数
curl http://localhost:8080/api/users

# 自定义分页参数
curl http://localhost:8080/api/users?page=2&limit=20
```

成功响应：
```json
{
  "data": [
    {
      "id": 1,
      "username": "user1",
      "email": "user1@example.com",
      "age": 25
    },
    {
      "id": 2,
      "username": "user2",
      "email": "user2@example.com",
      "age": 30
    }
  ],
  "pagination": {
    "total": 50,
    "total_pages": 5,
    "current_page": 1,
    "limit": 10,
    "has_next": true,
    "has_previous": false
  }
}
```

### 2. 创建新用户

```
POST /api/users
```

请求体：
```json
{
  "username": "newuser",
  "email": "newuser@example.com",
  "age": 25
}
```

示例请求：
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"newuser","email":"newuser@example.com","age":25}'
```

成功响应：
```json
{
  "id": 3,
  "username": "newuser",
  "email": "newuser@example.com",
  "age": 25
}
```

### 3. 获取特定用户

```
GET /api/users/{id}
```

示例请求：
```bash
curl http://localhost:8080/api/users/1
```

成功响应：
```json
{
  "id": 1,
  "username": "user1",
  "email": "user1@example.com",
  "age": 25
}
```

### 4. 更新用户

```
PUT /api/users/{id}
```

请求体：
```json
{
  "username": "updateduser",
  "email": "updated@example.com",
  "age": 26
}
```

示例请求：
```bash
curl -X PUT http://localhost:8080/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"username":"updateduser","email":"updated@example.com","age":26}'
```

成功响应：
```json
{
  "id": 1,
  "username": "updateduser",
  "email": "updated@example.com",
  "age": 26
}
```

### 5. 删除用户

```
DELETE /api/users/{id}
```

示例请求：
```bash
curl -X DELETE http://localhost:8080/api/users/1
```

成功响应：
- 状态码：204 No Content
- 无响应体

## 错误处理

API 使用统一的错误响应格式：

```json
{
  "error": "错误信息描述"
}
```

常见错误情况：

1. 分页参数无效：
```bash
# 无效的页码
curl http://localhost:8080/api/users?page=0
响应：{"error": "页码必须大于0"}

# 过大的每页数量会自动调整为最大允许值(100)
curl http://localhost:8080/api/users?limit=200
```

2. 用户名已存在：
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"existing_user","email":"test@example.com","age":25}'
响应：{"error": "用户名 'existing_user' 已存在"}
```

3. 用户不存在：
```bash
curl http://localhost:8080/api/users/999
响应：{"error": "ID为999的用户不存在"}
```

4. 无效的输入数据：
```bash
# 空用户名
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"","email":"test@example.com","age":25}'
响应：{"error": "用户名不能为空"}

# 空邮箱
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"newuser","email":"","age":25}'
响应：{"error": "邮箱不能为空"}
```

## 注意事项

1. 分页参数限制：
   - 页码(page)必须大于0
   - 每页数量(limit)范围：1-100
   - 超出范围的limit会自动调整到最近的有效值

2. 用户名唯一性：
   - 系统不允许重复的用户名
   - 创建和更新用户时都会检查用户名唯一性

3. 数据验证：
   - 用户名和邮箱不能为空
   - ID必须为正整数
   - 年龄字段可选

4. 错误处理：
   - 所有错误响应都包含详细的错误信息
   - 使用适当的HTTP状态码表示不同类型的错误