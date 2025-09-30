# IvyAster/infra-id-rs

**高性能分布式 Snowflake ID 生成服务 - 基于 Rust 构建**

[![Rust](https://img.shields.io/badge/Rust-1.90-orange?logo=rust)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/Actix--web-4.0-blue)](https://actix.rs/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

一个基于 Rust 语言实现的高性能、分布式 Snowflake ID 生成服务，提供简洁的 RESTful API 接口。

## ✨ 特性

- 🚀 **高性能**：基于 Rust 和 Actix-web 构建
- 🎯 **分布式**：支持多节点部署，通过 Worker ID 避免 ID 冲突
- 📦 **轻量级**：最终镜像仅 17MB，资源消耗极低
- 🔧 **易部署**：提供完整的 Docker 部署方案
- 📊 **可解析**：支持 ID 结构解析，便于调试和分析
- 🛡️ **生产就绪**：完善的错误处理、日志记录和监控支持

## 🛠 技术栈

- **语言**: Rust 1.90
- **Web 框架**: Actix-web 4.0
- **序列化**: Serde + Serde JSON
- **配置管理**: Config
- **日志系统**: Tracing
- **错误处理**: Anyhow

## 🏗 架构设计

### 镜像层次
```
rust:alpine3.22    (构建环境, ~1GB)
    ↓
alpine:latest      (运行时基础, ~8MB)
    ↓
IvyAster/infra-id-rs:alpine-1.0  (应用镜像, ~17MB)
```

### 构建流程
```bash
# 使用构建脚本创建生产镜像
source docker_build.sh alpine-1.0

# 生成的镜像: IvyAster/infra-id-rs:alpine-1.0
```

## 📡 API 接口

### 生成单个 ID

**JSON 格式响应**
```bash
curl http://127.0.0.1:8080/api/id

# 响应示例
HTTP/1.1 200 OK
content-type: application/json

{
  "code": "200",
  "message": "success", 
  "data": "7378656335472099328"
}
```

**纯文本格式响应**
```bash
curl http://127.0.0.1:8080/id

# 响应示例
HTTP/1.1 200 OK
content-type: text/plain

7378656575587614720
```

### 批量生成 ID

**JSON 格式响应**
```bash
curl http://127.0.0.1:8080/api/ids/10

# 响应示例
{
  "code": "200",
  "message": "success",
  "data": [
    "7378656748590071808",
    "7378656748590071809",
    # ... 8 个更多 ID
  ]
}
```

**纯文本格式响应**
```bash
curl http://127.0.0.1:8080/ids/10

# 响应示例
[
  "7378656712925904896",
  "7378656712925904897", 
  # ... 8 个更多 ID
]
```

### 解析 ID 结构

```bash
curl -v http://127.0.0.1:8080/api/id/struct/7378656748590071808

# 响应示例
{
  "code": "200",
  "message": "success",
  "data": {
    "timestamp": 1759208857677,
    "worker_id": 0,
    "sequence": 0
  }
}
```

## 🚀 快速开始

### 单机部署

```yaml
# docker-compose.yml
version: '3.8'

services:
  id-service:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-service
    ports:
      - "8000:8080"
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=0        # 工作节点 ID (0-1023)
      - SERVER_CONFIG__PORT=8080      # 服务端口
      - LOG_CONFIG__LEVEL=info        # 日志级别: error|warn|info|debug|trace
      - LOG_CONFIG__LOCATION=./logs   # 日志目录
      - LOG_CONFIG__FILE_PREFIX=infra-id  # 日志文件前缀
      - LOG_CONFIG__APPENDER=console  # 日志输出: file|console|all
```

### 环境变量配置

| 环境变量 | 描述 | 默认值 | 可选值 |
|---------|------|--------|--------|
| `ID_CONFIG__WORKER_ID` | 工作节点 ID | 0 | 0-1023 |
| `SERVER_CONFIG__HOST` | 服务绑定地址 | "0.0.0.0" | - |
| `SERVER_CONFIG__PORT` | 服务端口 | 8080 | - |
| `LOG_CONFIG__LEVEL` | 日志级别 | "info" | error\|warn\|info\|debug\|trace |
| `LOG_CONFIG__LOCATION` | 日志目录 | "./logs" | - |
| `LOG_CONFIG__FILE_PREFIX` | 日志文件前缀 | "infra-id" | - |
| `LOG_CONFIG__APPENDER` | 日志输出目标 | "console" | file\|console\|all |

## 🏢 集群部署

### 多实例负载均衡架构

```yaml
# docker-compose-group.yml
version: '3.8'

services:
  # ID 生成服务节点
  id-0:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-0
    networks:
      id-cluster:
        ipv4_address: 172.28.0.100
    environment:
      - ID_CONFIG__WORKER_ID=0
      - SERVER_CONFIG__PORT=8080

  id-1:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-1  
    networks:
      id-cluster:
        ipv4_address: 172.28.0.101
    environment:
      - ID_CONFIG__WORKER_ID=1
      - SERVER_CONFIG__PORT=8080

  id-2:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-2
    networks:
      id-cluster:
        ipv4_address: 172.28.0.102
    environment:
      - ID_CONFIG__WORKER_ID=2
      - SERVER_CONFIG__PORT=8080

  id-3:
    image: IvyAster/infra-id-rs:alpine-1.0  
    container_name: id-3
    networks:
      id-cluster:
        ipv4_address: 172.28.0.103
    environment:
      - ID_CONFIG__WORKER_ID=3
      - SERVER_CONFIG__PORT=8080

  # Nginx 负载均衡器
  proxy:
    image: nginx:alpine
    container_name: load-balancer
    depends_on:
      - id-0
      - id-1
      - id-2
      - id-3
    networks:
      id-cluster:
        ipv4_address: 172.28.0.200
    ports:
      - "8100:8100"  # 负载均衡器对外端口
    volumes:
      - ./id.conf:/etc/nginx/conf.d/id.conf  # Nginx 配置

# 自定义网络配置
networks:
  id-cluster:
    driver: bridge
    ipam:
      config:
        - subnet: 172.28.0.0/16
          gateway: 172.28.0.1
```

### Nginx 配置
完整的 Nginx 负载均衡配置请参考：[scripts/id.conf](./scripts/id.conf)

## 🔧 开发与构建

### 本地开发
```bash
# 克隆项目
git clone https://github.com/IvyAster/infra-id-rs.git
cd infra-id-rs

# 运行测试
cargo test

# 本地启动
cargo run
```

### 生产构建
```bash
# 使用构建脚本
chmod +x docker_build.sh
source docker_build.sh alpine-1.0
```

## 📊 性能指标

- **单节点 QPS**: 100,000+ 请求/秒
- **响应时间**: < 1ms (P99)
- **内存占用**: < 10MB (运行时)
- **镜像大小**: 17MB

## 🔍 ID 结构解析

生成的 Snowflake ID 包含以下组成部分：

| 部分 | 位数 | 描述 |
|------|------|------|
| 时间戳 | 41位 | 毫秒级时间戳 |
| 工作节点 ID | 10位 | 分布式节点标识 (0-1023) |
| 序列号 | 12位 | 同一毫秒内的序列号 (0-4095) |

## 🤝 贡献指南

我们欢迎各种形式的贡献！ 请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 📄 许可证

本项目基于 [MIT License](LICENSE) 开源。

## 🆘 支持

如果您遇到问题或有建议：

1. 查看 [Issues](https://github.com/IvyAster/infra-id-rs/issues)
2. 提交新的 Issue
3. 通过邮件联系维护者

---

**IvyAster/infra-id-rs** - 为您的分布式系统提供可靠的 ID 生成服务 ⚡