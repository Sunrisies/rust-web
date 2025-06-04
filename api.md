根据 RESTful API 设计原则和 HTTP 状态码规范，以下是针对 CRUD 操作（创建、读取、更新、删除）的完整状态码使用指南：

### 一、创建操作（Create - POST）

| 场景 | 状态码 | 说明 |
|------|--------|------|
| 创建成功 | 201 Created | 资源创建成功 |
| 参数缺失 | 400 Bad Request | 缺少必要参数 |
| 参数类型错误 | 400 Bad Request | 参数格式/类型不正确 |
| 唯一约束冲突 | 409 Conflict | 用户名/邮箱等唯一字段重复 |
| 数据库错误 | 500 Internal Server Error | 数据库操作失败 |
| 权限不足 | 401 Unauthorized | 未认证或认证失败 |
| 禁止访问 | 403 Forbidden | 认证成功但权限不足 |

### 二、读取操作（Read - GET）

| 场景 | 状态码 | 说明 |
|------|--------|------|
| 查询成功 | 200 OK | 返回资源列表 |
| 单个资源查询成功 | 200 OK | 返回单个资源 |
| 资源不存在 | 404 Not Found | 请求的资源不存在 |
| 参数缺失 | 400 Bad Request | 缺少必要参数（如ID） |
| 参数类型错误 | 400 Bad Request | 参数格式/类型不正确 |
| 数据库错误 | 500 Internal Server Error | 数据库查询失败 |
| 权限不足 | 401 Unauthorized | 未认证或认证失败 |
| 禁止访问 | 403 Forbidden | 认证成功但权限不足 |

### 三、更新操作（Update - PUT/PATCH）

| 场景 | 状态码 | 说明 |
|------|--------|------|
| 更新成功 | 200 OK | 返回更新后的资源 |
| 资源不存在 | 404 Not Found | 要更新的资源不存在 |
| 参数缺失 | 400 Bad Request | 缺少必要参数 |
| 参数类型错误 | 400 Bad Request | 参数格式/类型不正确 |
| 唯一约束冲突 | 409 Conflict | 更新后导致唯一字段冲突 |
| 数据库错误 | 500 Internal Server Error | 数据库更新失败 |
| 权限不足 | 401 Unauthorized | 未认证或认证失败 |
| 禁止访问 | 403 Forbidden | 认证成功但权限不足 |

### 四、删除操作（Delete - DELETE）

| 场景 | 状态码 | 说明 |
|------|--------|------|
| 删除成功 | 204 No Content | 资源已删除（无返回内容） |
| 资源不存在 | 404 Not Found | 要删除的资源不存在 |
| 参数缺失 | 400 Bad Request | 缺少必要参数（如ID） |
| 参数类型错误 | 400 Bad Request | 参数格式/类型不正确 |
| 数据库错误 | 500 Internal Server Error | 数据库删除失败 |
| 权限不足 | 401 Unauthorized | 未认证或认证失败 |
| 禁止访问 | 403 Forbidden | 认证成功但权限不足 |

### 五、通用状态码

| 状态码 | 说明 |
|--------|------|
| 401 Unauthorized | 需要认证但未提供凭证 |
| 403 Forbidden | 认证成功但权限不足 |
| 405 Method Not Allowed | 不支持的HTTP方法 |
| 429 Too Many Requests | 请求过于频繁 |
| 503 Service Unavailable | 服务暂时不可用 |
