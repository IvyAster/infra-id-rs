# IvyAster/infra-id-rs

**一个简洁的由Rust实现的Snowflake ID生成服务**

## 技术栈

- Rust: 1.90
- actix-web
- serde
- serde_json
- config
- trace
- anyhow

## 构建

使用镜像如下:
- rust:alpine3.22 用于构建 大小1G左右
- alpine:latest 用于运行 大小8MB
- nginx:alpine 用于组成集群 大小52MB
- IvyAster/infra-id-rs:alpine-1.0 构建完成的可运行镜像, 大小17MB

```bash
source docker_build.sh alpine-1.0
# 会生成镜像 IvyAster/infra-id-rs:alpine-1.0
```

## 接口
### 生成单个id
```bash
curl -v http://127.0.0.1:8080/api/id
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
* using HTTP/1.x
> GET /api/id HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 63
< content-type: application/json
< date: Tue, 30 Sep 2025 05:05:58 GMT
<
* Connection #0 to host 127.0.0.1 left intact
{"code":"200","message":"success","data":"7378656335472099328"}
```

```bash
curl -v http://127.0.0.1:8080/id
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
* using HTTP/1.x
> GET /id HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 19
< date: Tue, 30 Sep 2025 05:06:55 GMT
<
* Connection #0 to host 127.0.0.1 left intact
7378656575587614720
```

### 生成批量id
```bash
curl -v http://127.0.0.1:8080/api/ids/10
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
* using HTTP/1.x
> GET /api/ids/10 HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 263
< content-type: application/json
< date: Tue, 30 Sep 2025 05:07:37 GMT
<
* Connection #0 to host 127.0.0.1 left intact
{"code":"200","message":"success","data":["7378656748590071808","7378656748590071809","7378656748590071810","7378656748590071811","7378656748590071812","7378656748590071813","7378656748590071814","7378656748590071815","7378656748590071816","7378656748590071817"]}
```

```bash
curl -v http://127.0.0.1:8080/ids/10
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
* using HTTP/1.x
> GET /ids/10 HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 221
< content-type: application/json
< date: Tue, 30 Sep 2025 05:07:28 GMT
<
* Connection #0 to host 127.0.0.1 left intact
["7378656712925904896","7378656712925904897","7378656712925904898","7378656712925904899","7378656712925904900","7378656712925904901","7378656712925904902","7378656712925904903","7378656712925904904","7378656712925904905"]
```

### 解析id
```bash
curl -v http://127.0.0.1:8080/api/id/struct/7378656748590071808
*   Trying 127.0.0.1:8080...
* Connected to 127.0.0.1 (127.0.0.1) port 8080
* using HTTP/1.x
> GET /api/id/struct/7378656748590071808 HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/8.12.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 96
< content-type: application/json
< date: Tue, 30 Sep 2025 05:09:10 GMT
<
* Connection #0 to host 127.0.0.1 left intact
{"code":"200","message":"success","data":{"timestamp":1759208857677,"worker_id":0,"sequence":0}}
```

## 使用

### 简单使用
环境变量:
- ID_CONFIG__WORKER_ID=0..1023 机器标识
- SERVER_CONFIG__HOST="0.0.0.0" http绑定地址
- SERVER_CONFIG__PORT=8080 端口
- LOG_CONFIG__LEVEL="error|warn|info|trace" 日志等级
- LOG_CONFIG__LOCATION="./logs" 日志位置
- LOG_CONFIG__FILE_PREFIX="infra-id" 日志文件前缀
- LOG_CONFIG__APPENDER="file|console|all" 日志输出位置

```yaml
services:
  id:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id
    ports:
      - "8000:8080"
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=0
      - SERVER_CONFIG__PORT=8080
```

### 使用nginx组成多实例负载
- nginx配置   [看这里](./scripts/id.conf)
- docker-compose配置 [看这里](./scripts/docker-compose-group.yml)
```yaml
services:
  id-0:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-0
    ports:
      - "8000:8080"
    networks:
      id-group:
        ipv4_address: 172.28.0.100
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=0
      - SERVER_CONFIG__PORT=8080
  id-1:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-1
    ports:
      - "8001:8080"
    networks:
      id-group:
        ipv4_address: 172.28.0.101
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=1
      - SERVER_CONFIG__PORT=8080
  id-2:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-2
    ports:
      - "8002:8080"
    networks:
      id-group:
        ipv4_address: 172.28.0.102
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=2
      - SERVER_CONFIG__PORT=8080
  id-3:
    image: IvyAster/infra-id-rs:alpine-1.0
    container_name: id-3
    ports:
      - "8003:8080"
    networks:
      id-group:
        ipv4_address: 172.28.0.103
    restart: always
    environment:
      - ID_CONFIG__WORKER_ID=3
      - SERVER_CONFIG__PORT=8080
  proxy:
    image: nginx:alpine
    restart: always
    container_name: proxy
    depends_on:
      - id-0
      - id-1
      - id-2
      - id-3
    networks:
      id-group:
        ipv4_address: 172.28.0.200
    ports:
      - "8100:8100"
    volumes:
      - ./id.conf:/etc/nginx/conf.d/id.conf

# 定义自定义网络
networks:
  id-group:
    driver: bridge
    name: id-group
    ipam:
      config:
        - subnet: 172.28.0.0/16
          gateway: 172.28.0.1
```

