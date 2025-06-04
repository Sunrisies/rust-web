# 构建阶段
FROM rust:slim AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# 运行阶段
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/web-server .
CMD ["./web-server"]