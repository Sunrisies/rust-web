# 构建阶段
FROM rust:1.72-slim-bullseye as builder

# 安装 UPX 和其他必要工具
RUN apt-get update && \
    apt-get install -y upx pkg-config libssl-dev default-libmysqlclient-dev && \
    rm -rf /var/lib/apt/lists/*

# 创建新的空项目
WORKDIR /usr/src/app
RUN cargo new --bin mysql_user_crud
WORKDIR /usr/src/app/mysql_user_crud

# 首先复制依赖相关文件
COPY Cargo.toml Cargo.lock ./

# 构建依赖
RUN cargo build --release
RUN rm src/*.rs
RUN rm target/release/deps/mysql_user_crud*

# 复制源代码
COPY . .

# 构建应用
RUN cargo build --release

# 使用 UPX 压缩二进制文件
RUN upx --best --lzma target/release/mysql_user_crud

# 运行阶段
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y --no-install-recommends default-libmysqlclient-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -U -s /bin/false appuser

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /usr/src/app/mysql_user_crud/target/release/mysql_user_crud .

# 复制配置文件（如果有的话）
COPY --from=builder /usr/src/app/mysql_user_crud/.env* ./

# 设置权限
RUN chown -R appuser:appuser /app

# 切换到非 root 用户
USER appuser

# 暴露端口
EXPOSE 8080

# 设置环境变量
ENV RUST_LOG=info

# 运行应用
CMD ["./mysql_user_crud"]