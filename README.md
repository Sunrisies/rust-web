# Rust Web 项目

这是一个基于 Rust 和 Actix-web 框架构建的现代化 Web 服务项目。项目实现了完整的用户认证、权限管理、实时通知等功能。

## 功能特性

- 用户认证和授权（JWT + Google Authenticator）
- 基于角色的权限管理系统
- 实时事件通知（Server-Sent Events）
- RESTful API 设计
- 数据库集成（MySQL）
- 完整的错误处理机制
- CORS 支持
- 日志系统
- 中间件支持

## 技术栈

- **框架**: Actix-web 4.x
- **数据库**: MySQL
- **认证**: JWT (JSON Web Tokens) + Google Authenticator
- **实时通信**: Server-Sent Events (SSE)
- **日志**: env_logger
- **配置**: dotenv
- **序列化**: serde
- **错误处理**: thiserror

## 快速开始

### 环境要求

- Rust 1.70.0 或更高版本
- MySQL 8.0 或更高版本
- 操作系统: Linux, macOS, 或 Windows

### 安装

1. 克隆项目：
```bash
git clone [项目地址]
cd rust-web
```

2. 配置环境变量：
```bash
cp .env.example .env
# 编辑 .env 文件，设置必要的环境变量
```

3. 初始化数据库：
```bash
# 运行 sql/user.sql 中的数据库初始化脚本
```

4. 构建和运行：
```bash
cargo build --release
cargo run
```

服务器默认运行在 http://0.0.0.0:18080

## 项目结构

```
src/
├── api/          # API 路由和处理器
├── config/       # 配置相关代码
├── db/          # 数据库连接和操作
├── dto/         # 数据传输对象
├── error/       # 错误处理
├── middleware/  # 中间件
├── models/      # 数据模型
├── types/       # 类型定义
└── utils/       # 工具函数
```

## 文档

详细文档请参考 `docs` 目录：

- [API 文档](docs/api.md)
- [架构说明](docs/architecture.md)
- [部署指南](docs/deployment.md)
- [开发指南](docs/development.md)

## Docker 支持

项目提供了 Docker 支持，可以使用以下命令构建和运行：

```bash
# 构建 Docker 镜像
docker build -t rust-web .

# 运行容器
docker run -p 18080:18080 rust-web
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件