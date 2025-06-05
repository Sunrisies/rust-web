# 构建阶段
FROM rust:1.86.0-alpine AS builder

RUN apk add --no-cache build-base openssl perl

WORKDIR /app
COPY . .

# RUN apt-get update && apt-get install -y musl-tools && \
    # rustup target add x86_64-unknown-linux-musl && apt-get install libssl-dev -y &&  apt-get install pkg-config -y
RUN cargo build --release

# 运行阶段
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/mysql_user_crud .
COPY --from=builder /app/.env .
CMD ["./mysql_user_crud"]