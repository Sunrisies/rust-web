# API 文档

## 概述

本文档详细说明了 MySQL User CRUD API 的所有端点、请求/响应格式、错误处理和使用示例。

## 基础信息

- **基础 URL**: `http://localhost:8080/api`
- **内容类型**: `application/json`
- **字符编码**: UTF-8

## 认证

目前 API 不需要认证。未来版本可能会添加认证机制。

## 通用响应格式

### 成功响应

```json
{
    "data": <response_data>,
    "message": "操作成功描述"
}
```

### 错误响应

```json
{
    "error": {
        "code": "错误代码",
        "message": "错误描述"
    }
}
```

## API 端点

### 1. 获取用户列表

获取分页的用户列表。

**请求**:
```http
GET /api/users?page=1&limit=10
```

**查询参数**:
- `page`: 页码（默认值：1）
- `limit`: 每页记录数（默认值：10，最大值：100）

**成功响应** (200 OK):
```json
{
    "data": [
        {
            "id": 1,
            "username": "john_doe",
            "email": "john@example.com",
            "age": 30
        },
        // ... 更多用户
    ],
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

### 2. 获取单个用户

根据 ID 获取单个用户的详细信息。

**请求**:
```http
GET /api/users/{id}
```

**路径参数**:
- `id`: 用户 ID

**成功响应** (200 OK):
```json
{
    "data": {
        "id": 1,
        "username": "john_doe",
        "email": "john@example.com",
        "age": 30
    }
}
```

**错误响应** (404 Not Found):
```json
{
    "error": {
        "code": "USER_NOT_FOUND",
        "message": "用户不存在"
    }
}
```

### 3. 创建用户

创建新用户。

**请求**:
```http
POST /api/users
Content-Type: application/json

{
    "username": "john_doe",
    "email": "john@example.com",
    "age": 30
}
```

**请求体字段**:
- `username`: 用户名（必需，字符串，2-50字符）
- `email`: 电子邮件（必需，有效的电子邮件格式）
- `age`: 年龄（可选，整数，0-150）

**成功响应** (201 Created):
```json
{
    "data": {
        "id": 1,
        "username": "john_doe",
        "email": "john@example.com",
        "age": 30
    },
    "message": "用户创建成功"
}
```

**错误响应** (400 Bad Request):
```json
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "验证错误",
        "details": {
            "username": "用户名已存在",
            "email": "无效的电子邮件格式"
        }
    }
}
```

### 4. 更新用户

更新现有用户的信息。

**请求**:
```http
PUT /api/users/{id}
Content-Type: application/json

{
    "username": "john_doe_updated",
    "email": "john_new@example.com",
    "age": 31
}
```

**路径参数**:
- `id`: 用户 ID

**请求体字段**:
- `username`: 用户名（可选，字符串，2-50字符）
- `email`: 电子邮件（可选，有效的电子邮件格式）
- `age`: 年龄（可选，整数，0-150）

**成功响应** (200 OK):
```json
{
    "data": {
        "id": 1,
        "username": "john_doe_updated",
        "email": "john_new@example.com",
        "age": 31
    },
    "message": "用户更新成功"
}
```

**错误响应** (404 Not Found):
```json
{
    "error": {
        "code": "USER_NOT_FOUND",
        "message": "用户不存在"
    }
}
```

### 5. 删除用户

删除指定的用户。

**请求**:
```http
DELETE /api/users/{id}
```

**路径参数**:
- `id`: 用户 ID

**成功响应** (204 No Content)

**错误响应** (404 Not Found):
```json
{
    "error": {
        "code": "USER_NOT_FOUND",
        "message": "用户不存在"
    }
}
```

## 错误代码

| 错误代码 | HTTP 状态码 | 描述 |
|----------|------------|------|
| VALIDATION_ERROR | 400 | 请求数据验证失败 |
| USER_NOT_FOUND | 404 | 请求的用户不存在 |
| DATABASE_ERROR | 500 | 数据库操作错误 |
| INTERNAL_ERROR | 500 | 内部服务器错误 |

## 数据验证规则

### 用户名 (username)
- 必须是 2-50 个字符
- 只能包含字母、数字、下划线
- 必须唯一

### 电子邮件 (email)
- 必须是有效的电子邮件格式
- 最大长度 255 字符
- 必须唯一

### 年龄 (age)
- 必须是 0-150 之间的整数
- 可选字段

## 速率限制

目前 API 没有实现速率限制。未来版本可能会添加以下限制：
- 每个 IP 地址每分钟最多 60 个请求
- 超过限制时返回 429 Too Many Requests 状态码

## 分页

所有返回列表的端点都支持分页：

- `page`: 页码，从 1 开始
- `limit`: 每页记录数，默认 10，最大 100

分页响应包含以下元数据：
- `total`: 总记录数
- `total_pages`: 总页数
- `current_page`: 当前页码
- `limit`: 每页记录数
- `has_next`: 是否有下一页
- `has_previous`: 是否有上一页

## 使用示例

### cURL 示例

1. 获取用户列表：
```bash
curl -X GET "http://localhost:8080/api/users?page=1&limit=10"
```

2. 创建新用户：
```bash
curl -X POST "http://localhost:8080/api/users" \
     -H "Content-Type: application/json" \
     -d '{
         "username": "john_doe",
         "email": "john@example.com",
         "age": 30
     }'
```

3. 更新用户：
```bash
curl -X PUT "http://localhost:8080/api/users/1" \
     -H "Content-Type: application/json" \
     -d '{
         "age": 31
     }'
```

4. 删除用户：
```bash
curl -X DELETE "http://localhost:8080/api/users/1"
```

### Python 示例

```python
import requests

# 获取用户列表
response = requests.get('http://localhost:8080/api/users', params={'page': 1, 'limit': 10})
users = response.json()

# 创建用户
new_user = {
    'username': 'john_doe',
    'email': 'john@example.com',
    'age': 30
}
response = requests.post('http://localhost:8080/api/users', json=new_user)
created_user = response.json()

# 更新用户
updates = {'age': 31}
response = requests.put(f'http://localhost:8080/api/users/1', json=updates)
updated_user = response.json()

# 删除用户
response = requests.delete('http://localhost:8080/api/users/1')
```

## 最佳实践

1. 使用 HTTPS 进行安全通信
2. 实现适当的错误处理
3. 遵循 API 的分页建议
4. 缓存不经常变化的数据
5. 使用合适的 HTTP 方法和状态码