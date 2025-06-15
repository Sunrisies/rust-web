# Rust Web 项目文档

欢迎查阅 Rust Web 项目的文档。本文档提供了项目的详细说明、架构设计、API参考、部署指南和开发指南。

## 文档目录

### [架构说明](architecture.md)
了解项目的整体架构设计、组件交互和技术选择。

### [API 文档](api.md)
详细的API端点说明，包括请求参数、响应格式和认证要求。

### [部署指南](deployment.md)
如何在不同环境中部署和运行项目的详细说明。

### [开发指南](development.md)
项目开发流程、代码规范和贡献指南。

## 快速链接

- [项目主页](../README.md)
- [GitHub 仓库](#) <!-- 替换为实际的GitHub仓库链接 -->

## 技术栈概览

- **Web 框架**: Actix-web 4.x
- **数据库**: MySQL
- **认证**: JWT + Google Authenticator
- **API 风格**: RESTful
- **实时通信**: Server-Sent Events (SSE)
- **容器化**: Docker

## 项目结构

```
src/
├── api/          # API 路由和处理器
│   ├── auth.rs   # 认证相关API
│   ├── user.rs   # 用户相关API
│   └── sse.rs    # 服务器发送事件API
├── config/       # 配置相关代码
│   ├── log.rs    # 日志配置
│   └── permission.rs # 权限配置
├── db/          # 数据库连接和操作
├── dto/         # 数据传输对象
├── error/       # 错误处理
├── middleware/  # 中间件
│   ├── auth.rs  # 认证中间件
│   └── logger.rs # 日志中间件
├── models/      # 数据模型
├── types/       # 类型定义
└── utils/       # 工具函数
    ├── jsonwebtoken.rs # JWT工具
    └── sse.rs   # SSE工具
```

## 环境要求

- Rust 1.70.0 或更高版本
- MySQL 8.0 或更高版本
- 操作系统: Linux, macOS, 或 Windows

## 获取帮助

如果您在使用过程中遇到任何问题，请查阅相关文档或提交Issue。