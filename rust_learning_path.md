# Rust 新手学习路线指南

## 1. 环境准备
- 安装Rust: 使用官方推荐的 `rustup` 工具
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- 验证安装:
  ```bash
  rustc --version
  cargo --version
  ```
- 配置IDE: 推荐使用VS Code + Rust Analyzer插件

## 2. 基础语法学习
- 变量与可变性
- 数据类型（标量、复合）
- 函数与模块系统
- 控制流（if/else, loop, while, for）
- 所有权系统（Rust核心概念）
  - 所有权规则
  - 引用与借用
  - 切片类型

## 3. 中级概念
- 结构体与方法
- 枚举与模式匹配
- 常用集合（String, Vec, HashMap）
- 错误处理（Result和Option）
- 泛型与trait
- 生命周期基础

## 4. 进阶主题
- 并发编程
  - 线程
  - 消息传递（channel）
  - 共享状态（Mutex, Arc）
- 智能指针（Box, Rc, RefCell）
- 宏基础
- unsafe Rust简介

## 5. 项目实践
- 使用Cargo管理项目
- 常用crate推荐：
  - `serde` - 序列化/反序列化
  - `reqwest` - HTTP客户端
  - `tokio` - 异步运行时
  - `clap` - 命令行参数解析
- 项目建议：
  1. 命令行工具（如TODO应用）
  2. 简单的HTTP服务
  3. 多线程数据处理

## 6. 学习资源
- 官方文档：
  - [The Rust Programming Language](https://doc.rust-lang.org/book/)（"The Book"）
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 中文资源：
  - [Rust语言圣经](https://course.rs/)
  - [Rust程序设计](https://kaisery.github.io/trpl-zh-cn/)
- 社区：
  - Rust中文论坛: https://rustcc.cn/
  - Rust官方用户论坛: https://users.rust-lang.org/

## 7. 学习建议
1. 多动手写代码，从简单项目开始
2. 遇到编译错误不要怕，Rust编译器非常友好
3. 理解所有权系统是掌握Rust的关键
4. 参与开源项目或阅读优秀Rust项目源码
5. 保持耐心，Rust学习曲线前期较陡峭但值得

> 提示：可以按照这个路线每周学习一个主题，配合实践项目巩固知识。