# 部署指南

本文档提供了将 MySQL User CRUD API 部署到生产环境的详细说明，包括环境配置、部署选项、监控和维护指南。

## 目录

1. [部署准备](#部署准备)
2. [Docker 部署](#docker-部署)
3. [手动部署](#手动部署)
4. [环境配置](#环境配置)
5. [数据库配置](#数据库配置)
6. [安全配置](#安全配置)
7. [监控设置](#监控设置)
8. [日志管理](#日志管理)
9. [备份策略](#备份策略)
10. [维护计划](#维护计划)
11. [故障排除](#故障排除)

## 部署准备

### 系统要求

- CPU: 2+ 核心
- 内存: 4GB+ RAM
- 磁盘: 20GB+ 可用空间
- 操作系统: Ubuntu 20.04+ / CentOS 8+ / macOS 12+

### 依赖检查清单

- [ ] Rust 1.70+ 已安装
- [ ] MySQL 8.0+ 已安装
- [ ] Docker 20.10+ (如果使用 Docker 部署)
- [ ] SSL 证书 (用于 HTTPS)
- [ ] 防火墙配置
- [ ] 系统监控工具

## Docker 部署

### 1. 构建 Docker 镜像

```bash
# 从项目根目录构建镜像
docker build -t mysql_user_crud:latest .
```

### 2. 运行容器

```bash
# 创建 Docker 网络
docker network create user-api-network

# 运行 MySQL 容器
docker run -d \
  --name mysql \
  --network user-api-network \
  -e MYSQL_ROOT_PASSWORD=your_root_password \
  -e MYSQL_DATABASE=user_db \
  -e MYSQL_USER=api_user \
  -e MYSQL_PASSWORD=api_password \
  -v mysql_data:/var/lib/mysql \
  mysql:8.0

# 运行 API 容器
docker run -d \
  --name user-api \
  --network user-api-network \
  -p 8080:8080 \
  -e DATABASE_URL=mysql://api_user:api_password@mysql:3306/user_db \
  -e RUST_LOG=info \
  mysql_user_crud:latest
```

### 3. Docker Compose 部署

创建 `docker-compose.yml`:

```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=mysql://api_user:api_password@mysql:3306/user_db
      - RUST_LOG=info
    depends_on:
      - mysql
    restart: unless-stopped
    networks:
      - user-api-network

  mysql:
    image: mysql:8.0
    environment:
      - MYSQL_ROOT_PASSWORD=your_root_password
      - MYSQL_DATABASE=user_db
      - MYSQL_USER=api_user
      - MYSQL_PASSWORD=api_password
    volumes:
      - mysql_data:/var/lib/mysql
    networks:
      - user-api-network

volumes:
  mysql_data:

networks:
  user-api-network:
```

运行:
```bash
docker-compose up -d
```

## 手动部署

### 1. 编译项目

```bash
# 生产环境编译
cargo build --release

# 复制二进制文件到部署目录
sudo cp target/release/mysql_user_crud /usr/local/bin/
```

### 2. 创建系统服务

创建 `/etc/systemd/system/user-api.service`:

```ini
[Unit]
Description=MySQL User CRUD API
After=network.target mysql.service

[Service]
Type=simple
User=api_user
Environment=DATABASE_URL=mysql://api_user:api_password@localhost:3306/user_db
Environment=RUST_LOG=info
Environment=SERVER_HOST=0.0.0.0
Environment=SERVER_PORT=8080
ExecStart=/usr/local/bin/mysql_user_crud
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

启动服务:
```bash
sudo systemctl enable user-api
sudo systemctl start user-api
```

## 环境配置

### 生产环境变量

```env
# 数据库配置
DATABASE_URL=mysql://user:password@host:3306/dbname
DATABASE_POOL_SIZE=10

# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info

# 安全配置
ENABLE_HTTPS=true
SSL_CERT_PATH=/path/to/cert.pem
SSL_KEY_PATH=/path/to/key.pem
```

### Nginx 反向代理配置

```nginx
server {
    listen 80;
    server_name api.example.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl;
    server_name api.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

## 数据库配置

### MySQL 优化设置

编辑 `my.cnf`:

```ini
[mysqld]
# 连接设置
max_connections = 1000
max_connect_errors = 10000

# 缓冲池设置
innodb_buffer_pool_size = 1G
innodb_buffer_pool_instances = 4

# 日志设置
slow_query_log = 1
slow_query_log_file = /var/log/mysql/slow.log
long_query_time = 2
```

### 数据库备份配置

创建备份脚本 `backup.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/path/to/backups"
DATE=$(date +%Y%m%d_%H%M%S)
mysqldump -u user -p'password' user_db > "$BACKUP_DIR/backup_$DATE.sql"
```

设置定时任务:
```bash
0 2 * * * /path/to/backup.sh
```

## 安全配置

### 防火墙设置

```bash
# UFW (Ubuntu)
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable

# firewalld (CentOS)
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --reload
```

### SSL 配置

使用 Let's Encrypt:
```bash
sudo certbot certonly --nginx -d api.example.com
```

## 监控设置

### 1. 健康检查端点

监控 URL: `https://api.example.com/health`

### 2. Prometheus 配置

```yaml
scrape_configs:
  - job_name: 'user-api'
    scrape_interval: 15s
    static_configs:
      - targets: ['api.example.com:8080']
```

### 3. Grafana 仪表板

导入推荐的仪表板模板，监控：
- 请求率
- 响应时间
- 错误率
- 系统资源使用情况

## 日志管理

### 1. 日志轮转配置

创建 `/etc/logrotate.d/user-api`:

```
/var/log/user-api/*.log {
    daily
    rotate 14
    compress
    delaycompress
    notifempty
    create 0640 api_user api_user
    sharedscripts
    postrotate
        systemctl reload user-api
    endscript
}
```

### 2. 集中式日志收集

使用 ELK Stack 或 Loki 进行日志聚合。

## 备份策略

### 1. 数据库备份

- 每日完整备份
- 每小时增量备份
- 保留 30 天的备份历史

### 2. 配置备份

- 定期备份环境配置
- 使用版本控制管理配置文件

## 维护计划

### 1. 定期维护任务

- 每日: 检查日志和监控警报
- 每周: 检查系统更新
- 每月: 审查性能指标
- 每季: 安全审计

### 2. 更新流程

1. 在测试环境验证更新
2. 创建备份
3. 执行更新
4. 验证功能
5. 回滚计划（如需要）

## 故障排除

### 1. 常见问题

#### 数据库连接问题
```bash
# 检查数据库连接
mysql -u api_user -p -h localhost user_db

# 检查服务日志
journalctl -u user-api -f
```

#### 性能问题
```bash
# 检查系统资源
top
iostat
vmstat

# 检查慢查询日志
tail -f /var/log/mysql/slow.log
```

### 2. 紧急联系人

创建 `EMERGENCY.md`:
```markdown
## 紧急联系人

1. 系统管理员: admin@example.com
2. 数据库管理员: dba@example.com
3. 安全团队: security@example.com
```

### 3. 回滚流程

```bash
# 1. 停止服务
sudo systemctl stop user-api

# 2. 恢复数据库
mysql -u root -p user_db < backup.sql

# 3. 恢复二进制文件
sudo cp /backup/mysql_user_crud.old /usr/local/bin/mysql_user_crud

# 4. 重启服务
sudo systemctl start user-api
```

## 性能调优

### 1. 系统调优

编辑 `/etc/sysctl.conf`:
```
# 网络调优
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 65535

# 文件描述符
fs.file-max = 2097152
```

### 2. 应用调优

调整环境变量:
```env
DATABASE_POOL_SIZE=20
RUST_MIN_THREADS=4
RUST_MAX_THREADS=32
```

## 扩展建议

### 1. 水平扩展

- 使用负载均衡器
- 部署多个 API 实例
- 主从数据库复制

### 2. 缓存策略

- 使用 Redis 缓存热点数据
- 实现请求限流
- CDN 集成（如需要）