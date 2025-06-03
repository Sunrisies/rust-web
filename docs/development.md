# 开发指南

本文档提供了项目的开发指南，包括环境设置、编码规范、测试指南和常见问题解决方案。

## 目录

1. [开发环境设置](#开发环境设置)
2. [项目结构](#项目结构)
3. [编码规范](#编码规范)
4. [测试指南](#测试指南)
5. [数据库管理](#数据库管理)
6. [错误处理](#错误处理)
7. [日志记录](#日志记录)
8. [性能优化](#性能优化)
9. [常见问题](#常见问题)

## 开发环境设置

### 必需组件

1. **Rust 工具链**
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 更新到最新版本
rustup update

# 安装 nightly 工具链（可选）
rustup install nightly
```

2. **MySQL**
```bash
# macOS (使用 Homebrew)
brew install mysql

# Ubuntu
sudo apt-get install mysql-server

# 启动 MySQL 服务
sudo service mysql start  # Linux
brew services start mysql  # macOS
```

3. **开发工具**
- VS Code 或其他 IDE
- Rust 插件（rust-analyzer）
- MySQL 客户端工具

### 项目设置

1. **克隆仓库**
```bash
git clone <repository-url>
cd mysql_user_crud
```

2. **配置环境变量**
```bash
cp .env.test .env
```

编辑 `.env` 文件：
```env
DATABASE_URL=mysql://username:password@localhost:3306/dbname
RUST_LOG=debug
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

3. **安装开发依赖**
```bash
# 安装 cargo-watch（用于开发时自动重新编译）
cargo install cargo-watch

# 安装 diesel_cli（用于数据库迁移）
cargo install diesel_cli --no-default-features --features mysql
```

## 项目结构

```
src/
├── api/                    # API 相关代码
│   ├── handlers.rs         # 请求处理器
│   ├── routes.rs           # 路由配置
│   └── mod.rs             # 模块声明
├── db/                     # 数据库相关代码
│   ├── database.rs         # 数据库操作
│   └── mod.rs             # 模块声明
├── models/                 # 数据模型
│   ├── user.rs            # 用户模型
│   └── mod.rs             # 模块声明
├── lib.rs                  # 库入口
└── main.rs                # 应用入口
```

## 编码规范

### 命名约定

1. **变量和函数**
- 使用蛇形命名法（snake_case）
- 描述性但简洁
```rust
let user_count = 0;
fn get_user_by_id(id: i32) -> Result<User, Error> {
    // ...
}
```

2. **类型和特征**
- 使用大驼峰命名法（PascalCase）
```rust
struct User {
    // ...
}

trait DatabaseConnection {
    // ...
}
```

3. **常量**
- 使用全大写蛇形命名法
```rust
const MAX_CONNECTIONS: u32 = 100;
```

### 代码格式化

使用 `rustfmt` 格式化代码：
```bash
# 格式化整个项目
cargo fmt

# 检查格式化问题但不修改
cargo fmt -- --check
```

### 代码检查

使用 `clippy` 进行代码检查：
```bash
cargo clippy
```

### 文档注释

- 为公共 API 添加文档注释
- 包含示例代码（如果适用）

```rust
/// 根据 ID 获取用户
///
/// # Arguments
///
/// * `id` - 用户 ID
///
/// # Returns
///
/// 返回包含用户信息的 Result
///
/// # Examples
///
/// ```
/// let user = get_user_by_id(1)?;
/// println!("User: {}", user.username);
/// ```
pub fn get_user_by_id(id: i32) -> Result<User, Error> {
    // ...
}
```

## 测试指南

### 单元测试

1. **测试结构**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        // ...
    }
}
```

2. **运行测试**
```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_user_creation

# 显示测试输出
cargo test -- --nocapture
```

### 集成测试

1. **设置测试数据库**
```bash
# 创建测试数据库
mysql -u root -p -e "CREATE DATABASE test_db;"

# 运行迁移
DATABASE_URL=mysql://root:password@localhost/test_db diesel migration run
```

2. **编写集成测试**
```rust
// tests/api_tests.rs
use mysql_user_crud;

#[actix_rt::test]
async fn test_create_user() {
    // ...
}
```

### 性能测试

使用 `criterion` 进行基准测试：

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_user_creation(c: &mut Criterion) {
    c.bench_function("create_user", |b| {
        b.iter(|| {
            // 测试代码
        })
    });
}

criterion_group!(benches, benchmark_user_creation);
criterion_main!(benches);
```

## 数据库管理

### 迁移

1. **创建新迁移**
```bash
diesel migration generate create_users
```

2. **运行迁移**
```bash
diesel migration run
```

3. **回滚迁移**
```bash
diesel migration revert
```

### 连接池管理

```rust
use mysql::Pool;

pub fn create_pool(database_url: &str) -> Pool {
    Pool::new(database_url).expect("Failed to create pool")
}
```

## 错误处理

### 自定义错误类型

```rust
#[derive(Debug)]
pub enum AppError {
    DatabaseError(mysql::Error),
    ValidationError(String),
    NotFound(String),
}

impl std::error::Error for AppError {}
```

### 错误转换

```rust
impl From<mysql::Error> for AppError {
    fn from(err: mysql::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
```

## 日志记录

### 配置日志

```rust
use env_logger::{Builder, Env};

fn setup_logger() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
}
```

### 使用日志

```rust
log::info!("Server starting on port {}", port);
log::error!("Database error: {}", err);
log::debug!("Processing request: {:?}", req);
```

## 性能优化

### 编译优化

在 `Cargo.toml` 中：
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### 数据库优化

1. **索引优化**
```sql
CREATE INDEX idx_username ON users(username);
```

2. **连接池配置**
```rust
let pool = Pool::new(database_url)
    .with_max_connections(100)
    .with_min_connections(5);
```

## 常见问题

### 1. 数据库连接问题

**问题**: 无法连接到数据库
**解决方案**:
- 检查数据库 URL 格式
- 确保 MySQL 服务正在运行
- 验证用户权限

### 2. 编译错误

**问题**: 依赖项版本冲突
**解决方案**:
- 更新 `Cargo.lock`
- 检查依赖项版本兼容性
- 清理并重新构建项目

### 3. 性能问题

**问题**: API 响应缓慢
**解决方案**:
- 检查数据库查询性能
- 优化数据库索引
- 调整连接池配置
- 使用异步操作

## 贡献指南

1. **分支命名**
- 功能分支: `feature/description`
- 修复分支: `fix/description`
- 文档分支: `docs/description`

2. **提交消息格式**
```
<type>(<scope>): <subject>

<body>

<footer>
```

示例：
```
feat(user): add email validation

- Add email format validation
- Add unique email constraint

Closes #123
```

3. **代码审查清单**
- 代码是否遵循项目规范
- 是否包含适当的测试
- 文档是否更新
- 性能影响是否可接受