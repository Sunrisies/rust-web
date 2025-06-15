# API 文档

## 概述

本文档详细说明了 Rust Web 项目提供的所有 API 端点、请求参数、响应格式和认证要求。所有 API 返回 JSON 格式的响应，并使用标准的 HTTP 状态码表示请求结果。

## 基础 URL

所有 API 的基础 URL 为：`http://your-domain.com/api/v1`

## 认证

除了明确标记为公开的端点外，所有 API 都需要认证。认证通过 JWT Token 实现，Token 需要在请求头中通过 `Authorization` 字段提供：

```
Authorization: Bearer <your_jwt_token>
```

### 获取 Token

```
POST /auth/login
```

**请求体**:
```json
{
  "username": "your_username",
  "password": "your_password"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
  }
}
```

### 刷新 Token

```
POST /auth/refresh
```

**请求头**:
```
Authorization: Bearer <your_refresh_token>
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
  }
}
```

## 用户管理

### 获取当前用户信息

```
GET /users/me
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "id": 1,
    "username": "john_doe",
    "email": "john@example.com",
    "role": "admin",
    "created_at": "2023-01-01T00:00:00Z",
    "updated_at": "2023-01-01T00:00:00Z"
  }
}
```

### 获取用户列表

```
GET /users
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**查询参数**:
- `page`: 页码，默认为 1
- `limit`: 每页记录数，默认为 10
- `sort`: 排序字段，默认为 id
- `order`: 排序方向，asc 或 desc，默认为 asc

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "total": 100,
    "page": 1,
    "limit": 10,
    "users": [
      {
        "id": 1,
        "username": "john_doe",
        "email": "john@example.com",
        "role": "admin",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
      },
      // ... 更多用户
    ]
  }
}
```

### 创建用户

```
POST /users
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**请求体**:
```json
{
  "username": "new_user",
  "email": "new_user@example.com",
  "password": "secure_password",
  "role": "user"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "id": 101,
    "username": "new_user",
    "email": "new_user@example.com",
    "role": "user",
    "created_at": "2023-01-02T00:00:00Z",
    "updated_at": "2023-01-02T00:00:00Z"
  }
}
```

### 更新用户

```
PUT /users/{id}
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**请求体**:
```json
{
  "email": "updated_email@example.com",
  "role": "admin"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "id": 101,
    "username": "new_user",
    "email": "updated_email@example.com",
    "role": "admin",
    "created_at": "2023-01-02T00:00:00Z",
    "updated_at": "2023-01-02T01:00:00Z"
  }
}
```

### 删除用户

```
DELETE /users/{id}
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": null
}
```

## 双因素认证

### 启用双因素认证

```
POST /auth/2fa/enable
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "secret": "JBSWY3DPEHPK3PXP",
    "qr_code": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA..."
  }
}
```

### 验证双因素认证

```
POST /auth/2fa/verify
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**请求体**:
```json
{
  "code": "123456"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "enabled": true
  }
}
```

### 禁用双因素认证

```
POST /auth/2fa/disable
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**请求体**:
```json
{
  "code": "123456"
}
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "enabled": false
  }
}
```

## 实时通知

### 建立 SSE 连接

```
GET /sse
```

**请求头**:
```
Authorization: Bearer <your_jwt_token>
```

**响应**:
Server-Sent Events 流，事件格式如下：

```
event: message
data: {"id": "msg-123", "type": "notification", "content": "New message received"}

event: alert
data: {"id": "alert-456", "type": "system", "content": "System maintenance scheduled"}
```

## 错误响应

所有 API 在发生错误时返回统一的错误格式：

```json
{
  "code": 1001,
  "message": "Invalid credentials",
  "data": null
}
```

### 常见错误码

| 错误码 | 描述 |
|--------|------|
| 1001   | 认证失败 |
| 1002   | 权限不足 |
| 1003   | 资源不存在 |
| 1004   | 参数验证失败 |
| 1005   | 服务器内部错误 |
| 1006   | 请求频率超限 |

## 速率限制

API 实施了速率限制以防止滥用。限制信息通过以下响应头返回：

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1609459200
```

超过限制时，服务器返回 429 Too Many Requests 状态码。

## API 版本控制

API 版本通过 URL 路径控制（如 `/api/v1/users`）。当 API 发生不兼容变更时，会增加版本号。

## 请求示例

### cURL

```bash
# 登录
curl -X POST http://your-domain.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "john_doe", "password": "secure_password"}'

# 获取用户信息
curl -X GET http://your-domain.com/api/v1/users/me \
  -H "Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### JavaScript

```javascript
// 登录
fetch('http://your-domain.com/api/v1/auth/login', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    username: 'john_doe',
    password: 'secure_password'
  })
})
.then(response => response.json())
.then(data => console.log(data));

// 获取用户信息
fetch('http://your-domain.com/api/v1/users/me', {
  method: 'GET',
  headers: {
    'Authorization': 'Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...'
  }
})
.then(response => response.json())
.then(data => console.log(data));
```