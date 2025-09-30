# 构建阶段
FROM rust:alpine3.22 AS builder

# Alpine下需要安装musl-dev来构建（如果你依赖一些C库的话）
RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

# 运行时阶段
FROM alpine:latest

# 安装运行时依赖（例如，ca-certificates，如果你需要SSL）
RUN apk add --no-cache ca-certificates

# 创建非root用户
RUN addgroup -S app && adduser -S app -G app

USER app
WORKDIR /app

COPY --from=builder /app/target/release/infra-id-rs .
COPY app.toml .

EXPOSE 8080

CMD ["./infra-id-rs"]