# 开发指南

本文档提供了 Rust Web 项目的开发流程、代码规范和贡献指南。

## 目录

- [开发环境设置](#开发环境设置)
- [项目结构](#项目结构)
- [编码规范](#编码规范)
- [开发工作流](#开发工作流)
- [测试指南](#测试指南)
- [文档编写](#文档编写)
- [版本控制](#版本控制)
- [贡献指南](#贡献指南)
- [常见问题](#常见问题)

## 开发环境设置

### 安装必要工具

1. **Rust 和 Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   rustup default stable
   ```

2. **MySQL**
   - Ubuntu/Debian: `sudo apt install mysql-server`
   - macOS: `brew install mysql`
   - Windows: 从 [MySQL 官网](https://dev.mysql.com/downloads/installer/) 下载安装程序

3. **开发工具**
   - 推荐 IDE: VS Code 或 IntelliJ IDEA 配合 Rust 插件
   - 安装 VS Code 插件: rust-analyzer, crates, Better TOML

### 配置开发环境

1. **克隆仓库**
   ```bash
   git clone [项目地址]
   cd rust-web
   ```

2. **安装开发依赖**
   ```bash
   # 安装 cargo-watch 用于自动重载
   cargo install cargo-watch
   
   # 安装 sqlx-cli 用于数据库迁移
   cargo install sqlx-cli
   
   # 安装 cargo-audit 用于安全审计
   cargo install cargo-audit
   ```

3. **配置环境变量**
   ```bash
   cp .env.example .env.development
   ```
   
   编辑 `.env.development` 文件，设置开发环境变量：
   ```
   DATABASE_URL=mysql://rust_web_user:your_password@localhost/rust_web_dev
   JWT_SECRET=dev_secret
   JWT_EXPIRATION=3600
   SERVER_HOST=127.0.0.1
   SERVER_PORT=18080
   LOG_LEVEL=debug
   ```

4. **初始化开发数据库**
   ```bash
   # 创建开发数据库
   mysql -u root -p -e "CREATE DATABASE rust_web_dev; CREATE USER 'rust_web_user'@'localhost' IDENTIFIED BY 'your_password'; GRANT ALL PRIVILEGES ON rust_web_dev.* TO 'rust_web_user'@'localhost'; FLUSH PRIVILEGES;"
   
   # 导入初始数据
   mysql -u rust_web_user -p rust_web_dev < sql/user.sql
   ```

## 项目结构

```
rust-web/
├── .github/            # GitHub 配置文件
├── docs/               # 项目文档
├── sql/                # SQL 脚本
├── src/                # 源代码
│   ├── api/            # API 路由和处理器
│   │   ├── auth.rs     # 认证相关 API
│   │   ├── user.rs     # 用户相关 API
│   │   └── sse.rs      # 服务器发送事件 API
│   ├── config/         # 配置相关代码
│   │   ├── log.rs      # 日志配置
│   │   └── permission.rs # 权限配置
│   ├── db/             # 数据库连接和操作
│   ├── dto/            # 数据传输对象
│   ├── error/          # 错误处理
│   ├── middleware/     # 中间件
│   │   ├── auth.rs     # 认证中间件
│   │   └── logger.rs   # 日志中间件
│   ├── models/         # 数据模型
│   ├── types/          # 类型定义
│   ├── utils/          # 工具函数
│   │   ├── jsonwebtoken.rs # JWT 工具
│   │   └── sse.rs      # SSE 工具
│   ├── lib.rs          # 库入口
│   └── main.rs         # 应用入口
├── tests/              # 集成测试
├── .env.example        # 环境变量示例
├── .gitignore          # Git 忽略文件
├── Cargo.toml          # 项目依赖
├── Cargo.lock          # 依赖锁定文件
├── Dockerfile          # Docker 配置
├── docker-compose.yml  # Docker Compose 配置
└── README.md           # 项目说明
```

## 编码规范

### Rust 代码风格

1. **命名约定**
   - 使用蛇形命名法（snake_case）命名变量和函数
   - 使用大驼峰命名法（PascalCase）命名类型和特性
   - 使用全大写蛇形命名法（SCREAMING_SNAKE_CASE）命名常量
   - 模块名使用蛇形命名法

2. **代码格式化**
   - 使用 `rustfmt` 格式化代码：`cargo fmt`
   - 提交前运行 `cargo fmt --check` 确保代码格式正确

3. **代码质量**
   - 使用 `clippy` 检查代码质量：`cargo clippy`
   - 修复所有 clippy 警告或使用 `#[allow(...)]` 注解说明原因

### 注释规范

1. **文档注释**
   - 为公共 API 提供文档注释（`///` 或 `/** ... */`）
   - 包含参数、返回值和示例说明
   - 示例：
     ```rust
     /// 验证用户凭证并返回 JWT token
     ///
     /// # Arguments
     ///
     /// * `credentials` - 包含用户名和密码的凭证
     ///
     /// # Returns
     ///
     /// 成功时返回 JWT token，失败时返回认证错误
     ///
     /// # Examples
     ///
     /// ```
     /// let credentials = Credentials { username: "user", password: "pass" };
     /// let token = authenticate(credentials).await?;
     /// ```
     pub async fn authenticate(credentials: Credentials) -> Result<String, AuthError> {
         // 实现...
     }
     ```

2. **代码注释**
   - 使用 `//` 注释解释复杂逻辑
   - 避免注释明显的代码
   - 保持注释与代码同步更新

### 错误处理

1. **使用 `Result` 类型**
   - 函数应返回 `Result<T, E>` 而不是 panic
   - 使用 `?` 运算符传播错误

2. **自定义错误类型**
   - 使用 `thiserror` 创建自定义错误类型
   - 实现错误转换和显示特性
   - 示例：
     ```rust
     use thiserror::Error;
     
     #[derive(Error, Debug)]
     pub enum AppError {
         #[error("Authentication error: {0}")]
         AuthError(String),
         
         #[error("Database error: {0}")]
         DbError(#[from] sqlx::Error),
         
         #[error("Validation error: {0}")]
         ValidationError(String),
     }
     ```

## 开发工作流

### 本地开发

1. **启动开发服务器**
   ```bash
   # 使用开发环境变量
   export $(cat .env.development | xargs)
   
   # 启动带有自动重载的服务器
   cargo watch -x run
   ```

2. **实时编译检查**
   ```bash
   # 在另一个终端窗口运行
   cargo watch -c -x check
   ```

### 分支策略

1. **主分支**
   - `main`: 稳定版本，只接受经过测试的合并请求
   - `develop`: 开发分支，集成已完成的功能

2. **功能分支**
   - 从 `develop` 分支创建功能分支
   - 命名格式：`feature/feature-name`
   - 完成后合并回 `develop` 分支

3. **修复分支**
   - 从 `main` 分支创建修复分支
   - 命名格式：`hotfix/issue-description`
   - 完成后同时合并到 `main` 和 `develop` 分支

### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

类型（type）:
- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更改
- `style`: 不影响代码含义的更改（空格、格式等）
- `refactor`: 既不修复错误也不添加功能的代码更改
- `perf`: 提高性能的代码更改
- `test`: 添加或修正测试
- `build`: 影响构建系统或外部依赖的更改
- `ci`: 更改 CI 配置文件和脚本

示例：
```
feat(auth): 添加双因素认证支持

- 添加 Google Authenticator 集成
- 更新用户模型以支持 2FA
- 添加 2FA 设置和验证 API

Closes #123
```

## 测试指南

### 测试类型

1. **单元测试**
   - 测试单个函数或模块
   - 放在与被测代码相同的文件中
   - 使用 `#[cfg(test)]` 标记测试模块

2. **集成测试**
   - 测试多个组件的交互
   - 放在 `tests/` 目录下
   - 每个测试文件是一个独立的 crate

3. **API 测试**
   - 测试 HTTP API 端点
   - 使用 `actix-rt` 和 `reqwest` 发送请求

### 编写测试

1. **单元测试示例**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_validate_password() {
           assert!(validate_password("StrongP@ss123"));
           assert!(!validate_password("weak"));
       }
   }
   ```

2. **集成测试示例**
   ```rust
   // tests/api_auth.rs
   use rust_web::{create_app, config::Config};
   use actix_web::{test, App};
   
   #[actix_rt::test]
   async fn test_login_endpoint() {
       let config = Config::from_env().unwrap();
       let app = test::init_service(
           App::new()
               .configure(|cfg| create_app(cfg, &config))
       ).await;
       
       let req = test::TestRequest::post()
           .uri("/api/v1/auth/login")
           .set_json(&json!({
               "username": "test_user",
               "password": "test_password"
           }))
           .to_request();
           
       let resp = test::call_service(&app, req).await;
       assert_eq!(resp.status(), 200);
       
       let body: Value = test::read_body_json(resp).await;
       assert_eq!(body["code"], 0);
       assert!(body["data"]["token"].is_string());
   }
   ```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_login_endpoint

# 运行特定模块的测试
cargo test --package rust-web --lib -- api::auth::tests

# 显示测试输出
cargo test -- --nocapture
```

### 测试覆盖率

使用 `grcov` 生成测试覆盖率报告：

```bash
# 安装 grcov
cargo install grcov

# 安装 llvm-tools-preview
rustup component add llvm-tools-preview

# 运行带覆盖率的测试
RUSTFLAGS="-Zinstrument-coverage" \
LLVM_PROFILE_FILE="rust-web-%p-%m.profraw" \
cargo test

# 生成覆盖率报告
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
```

## 文档编写

### 代码文档

1. **生成文档**
   ```bash
   cargo doc --no-deps --open
   ```

2. **文档测试**
   - 文档注释中的代码示例会被自动测试
   - 确保示例代码是正确的并且能够运行

### 项目文档

1. **README.md**
   - 项目概述
   - 快速开始指南
   - 基本用法示例

2. **docs/ 目录**
   - 详细的用户指南
   - API 文档
   - 架构说明
   - 部署指南
   - 贡献指南

## 版本控制

遵循 [语义化版本控制](https://semver.org/) 规范：

- **主版本号**：当你做了不兼容的 API 修改
- **次版本号**：当你做了向下兼容的功能性新增
- **修订号**：当你做了向下兼容的问题修正

### 版本发布流程

1. 更新 `Cargo.toml` 中的版本号
2. 更新 `CHANGELOG.md`
3. 创建版本标签：`git tag v1.0.0`
4. 推送标签：`git push origin v1.0.0`

## 贡献指南

### 提交贡献

1. **Fork 仓库**
   - 在 GitHub 上 fork 项目仓库

2. **克隆 fork 的仓库**
   ```bash
   git clone https://github.com/your-username/rust-web.git
   cd rust-web
   ```

3. **创建功能分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **开发功能**
   - 编写代码
   - 添加测试
   - 更新文档

5. **提交更改**
   ```bash
   git add .
   git commit -m "feat: 添加新功能"
   ```

6. **推送到 fork 的仓库**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **创建合并请求**
   - 在 GitHub 上创建从你的功能分支到主仓库 develop 分支的合并请求
   - 填写详细的描述，包括实现的功能和解决的问题

### 代码审查

1. **审查标准**
   - 代码质量：遵循编码规范
   - 测试覆盖：新功能必须有测试
   - 文档：更新相关文档
   - 性能：不引入性能问题

2. **审查流程**
   - 至少一名维护者必须批准更改
   - CI 测试必须通过
   - 所有讨论问题必须解决

### 报告问题

1. **提交 Issue**
   - 使用 Issue 模板
   - 提供详细的问题描述
   - 包括复现步骤
   - 附上相关日志和截图

2. **Issue 分类**
   - Bug：软件缺陷
   - Feature：功能请求
   - Documentation：文档改进
   - Question：使用问题

## 常见问题

### 开发问题

1. **数据库连接失败**
   - 检查 `.env` 文件中的数据库配置
   - 确保 MySQL 服务正在运行
   - 验证数据库用户权限

2. **依赖问题**
   - 运行 `cargo update` 更新依赖
   - 检查 `Cargo.lock` 是否提交到版本控制
   - 使用 `cargo clean` 清理构建缓存

3. **编译错误**
   - 运行 `cargo check` 获取详细错误信息
   - 更新 Rust 工具链：`rustup update`
   - 检查是否使用了不稳定特性

### 调试技巧

1. **日志调试**
   - 设置环境变量 `RUST_LOG=debug`
   - 使用 `log::debug!()` 输出调试信息
   - 查看日志输出

2. **使用调试器**
   - 安装 `rust-gdb` 或 `rust-lldb`
   - 添加断点：`#[breakpoint]`
   - 运行调试：`rust-gdb target/debug/rust-web`

3. **性能分析**
   - 使用 `cargo flamegraph` 生成火焰图
   - 使用 `criterion` 进行基准测试
   - 分析内存使用：`valgrind`

### 常见错误解决

1. **权限错误**
   - 检查 JWT 令牌是否有效
   - 验证用户角色和权限
   - 查看认证中间件日志

2. **数据库查询错误**
   - 检查 SQL 语句语法
   - 验证表结构和字段名
   - 确保数据类型匹配

3. **API 响应错误**
   - 检查请求格式和参数
   - 验证内容类型头
   - 查看服务器错误日志

## 附录

### 有用的命令

```bash
# 检查代码格式
cargo fmt --check

# 运行 clippy 检查
cargo clippy -- -D warnings

# 运行安全审计
cargo audit

# 生成依赖图
cargo install cargo-deps
cargo deps --all-deps | dot -Tpng > deps.png

# 分析二进制大小
cargo bloat --release

# 检查未使用的依赖
cargo install cargo-udeps
cargo +nightly udeps
```

### 推荐资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- [Actix Web 文档](https://actix.rs/docs/)
- [SQLx 文档](https://github.com/launchbadge/sqlx)
- [Rust 设计模式](https://rust-unofficial.github.io/patterns/)