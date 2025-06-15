# 部署指南

本文档提供了在不同环境中部署 Rust Web 项目的详细说明。

## 目录

- [环境要求](#环境要求)
- [本地部署](#本地部署)
- [Docker 部署](#docker-部署)
- [生产环境部署](#生产环境部署)
- [CI/CD 集成](#cicd-集成)
- [监控与日志](#监控与日志)
- [备份与恢复](#备份与恢复)
- [故障排除](#故障排除)

## 环境要求

### 基本要求

- Rust 1.70.0 或更高版本
- MySQL 8.0 或更高版本
- 操作系统: Linux, macOS, 或 Windows
- 至少 1GB RAM
- 至少 1GB 磁盘空间

### 推荐配置

- 2 核 CPU
- 4GB RAM
- 10GB SSD 存储
- Ubuntu 22.04 LTS 或 Debian 11

## 本地部署

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

### 2. 安装 MySQL

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install mysql-server
sudo systemctl start mysql
sudo systemctl enable mysql
```

#### macOS
```bash
brew install mysql
brew services start mysql
```

#### Windows
从 [MySQL 官网](https://dev.mysql.com/downloads/installer/) 下载并安装 MySQL Installer。

### 3. 配置数据库

```bash
# 登录 MySQL
mysql -u root -p

# 创建数据库和用户
CREATE DATABASE rust_web;
CREATE USER 'rust_web_user'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON rust_web.* TO 'rust_web_user'@'localhost';
FLUSH PRIVILEGES;
EXIT;

# 导入初始数据
mysql -u rust_web_user -p rust_web < sql/user.sql
```

### 4. 配置环境变量

```bash
cp .env.example .env
```

编辑 `.env` 文件，设置以下变量：

```
DATABASE_URL=mysql://rust_web_user:your_password@localhost/rust_web
JWT_SECRET=your_jwt_secret
JWT_EXPIRATION=3600
SERVER_HOST=0.0.0.0
SERVER_PORT=18080
LOG_LEVEL=info
```

### 5. 构建和运行

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/rust-web
```

## Docker 部署

### 1. 安装 Docker 和 Docker Compose

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install docker.io docker-compose
sudo systemctl start docker
sudo systemctl enable docker
```

#### macOS
```bash
brew install docker docker-compose
```

#### Windows
从 [Docker 官网](https://www.docker.com/products/docker-desktop) 下载并安装 Docker Desktop。

### 2. 配置 Docker 环境

创建 `docker-compose.yml` 文件：

```yaml
version: '3'

services:
  app:
    build: .
    ports:
      - "18080:18080"
    environment:
      - DATABASE_URL=mysql://rust_web_user:your_password@db/rust_web
      - JWT_SECRET=your_jwt_secret
      - JWT_EXPIRATION=3600
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=18080
      - LOG_LEVEL=info
    depends_on:
      - db
    restart: always

  db:
    image: mysql:8.0
    ports:
      - "3306:3306"
    environment:
      - MYSQL_ROOT_PASSWORD=root_password
      - MYSQL_DATABASE=rust_web
      - MYSQL_USER=rust_web_user
      - MYSQL_PASSWORD=your_password
    volumes:
      - mysql_data:/var/lib/mysql
      - ./sql/user.sql:/docker-entrypoint-initdb.d/user.sql
    restart: always

volumes:
  mysql_data:
```

### 3. 构建和运行 Docker 容器

```bash
docker-compose up -d
```

### 4. 查看日志

```bash
docker-compose logs -f app
```

## 生产环境部署

### 1. 服务器准备

- 配置防火墙，只开放必要端口（如 80, 443, 22）
- 设置 SSH 密钥认证
- 更新系统包
- 安装必要工具（如 fail2ban, ufw）

### 2. 使用 Nginx 作为反向代理

安装 Nginx：

```bash
sudo apt update
sudo apt install nginx
```

配置 Nginx：

```bash
sudo nano /etc/nginx/sites-available/rust-web
```

添加以下配置：

```nginx
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:18080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

启用站点并重启 Nginx：

```bash
sudo ln -s /etc/nginx/sites-available/rust-web /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### 3. 配置 SSL（使用 Let's Encrypt）

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d your-domain.com
```

### 4. 创建系统服务

```bash
sudo nano /etc/systemd/system/rust-web.service
```

添加以下内容：

```ini
[Unit]
Description=Rust Web Service
After=network.target

[Service]
User=www-data
Group=www-data
WorkingDirectory=/path/to/rust-web
Environment="DATABASE_URL=mysql://rust_web_user:your_password@localhost/rust_web"
Environment="JWT_SECRET=your_jwt_secret"
Environment="JWT_EXPIRATION=3600"
Environment="SERVER_HOST=127.0.0.1"
Environment="SERVER_PORT=18080"
Environment="LOG_LEVEL=info"
ExecStart=/path/to/rust-web/target/release/rust-web
Restart=always

[Install]
WantedBy=multi-user.target
```

启用并启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-web
sudo systemctl start rust-web
```

## CI/CD 集成

### GitHub Actions

创建 `.github/workflows/ci.yml` 文件：

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mysql:8.0
        env:
          MYSQL_ROOT_PASSWORD: root_password
          MYSQL_DATABASE: rust_web_test
          MYSQL_USER: rust_web_user
          MYSQL_PASSWORD: test_password
        ports:
          - 3306:3306
        options: --health-cmd="mysqladmin ping" --health-interval=10s --health-timeout=5s --health-retries=3

    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        profile: minimal
        toolchain: stable
        override: true
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    - name: Run tests
      run: cargo test
      env:
        DATABASE_URL: mysql://rust_web_user:test_password@localhost/rust_web_test
        JWT_SECRET: test_secret
        
  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Deploy to production
      uses: appleboy/ssh-action@master
      with:
        host: ${{ secrets.SSH_HOST }}
        username: ${{ secrets.SSH_USERNAME }}
        key: ${{ secrets.SSH_PRIVATE_KEY }}
        script: |
          cd /path/to/rust-web
          git pull
          cargo build --release
          sudo systemctl restart rust-web
```

## 监控与日志

### 日志配置

在 `.env` 文件中设置日志级别：

```
LOG_LEVEL=info  # 可选值: trace, debug, info, warn, error
```

### 使用 Prometheus 监控

1. 安装 Prometheus 和 Grafana：

```bash
# 安装 Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.40.0/prometheus-2.40.0.linux-amd64.tar.gz
tar xvfz prometheus-2.40.0.linux-amd64.tar.gz
cd prometheus-2.40.0.linux-amd64

# 安装 Grafana
sudo apt-get install -y apt-transport-https software-properties-common
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
echo "deb https://packages.grafana.com/oss/deb stable main" | sudo tee -a /etc/apt/sources.list.d/grafana.list
sudo apt-get update
sudo apt-get install grafana
```

2. 配置 Prometheus：

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'rust-web'
    static_configs:
      - targets: ['localhost:18080']
```

3. 启动 Prometheus 和 Grafana：

```bash
# 启动 Prometheus
./prometheus --config.file=prometheus.yml

# 启动 Grafana
sudo systemctl start grafana-server
sudo systemctl enable grafana-server
```

## 备份与恢复

### 数据库备份

创建自动备份脚本 `backup.sh`：

```bash
#!/bin/bash
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR="/path/to/backups"
MYSQL_USER="rust_web_user"
MYSQL_PASSWORD="your_password"
DATABASE="rust_web"

# 创建备份目录
mkdir -p $BACKUP_DIR

# 备份数据库
mysqldump -u $MYSQL_USER -p$MYSQL_PASSWORD $DATABASE > $BACKUP_DIR/$DATABASE\_$TIMESTAMP.sql

# 压缩备份文件
gzip $BACKUP_DIR/$DATABASE\_$TIMESTAMP.sql

# 删除7天前的备份
find $BACKUP_DIR -name "*.sql.gz" -type f -mtime +7 -delete
```

设置定时任务：

```bash
chmod +x backup.sh
crontab -e
```

添加以下内容：

```
0 2 * * * /path/to/backup.sh
```

### 数据库恢复

```bash
gunzip < /path/to/backups/rust_web_20230101_020000.sql.gz | mysql -u rust_web_user -p rust_web
```

## 故障排除

### 常见问题

1. **服务无法启动**
   - 检查环境变量是否正确设置
   - 检查数据库连接是否可用
   - 查看日志文件获取详细错误信息

2. **数据库连接错误**
   - 确认 MySQL 服务正在运行
   - 验证数据库用户名和密码
   - 检查防火墙设置是否允许数据库连接

3. **性能问题**
   - 检查服务器资源使用情况（CPU, 内存, 磁盘 I/O）
   - 优化数据库查询
   - 考虑增加服务器资源或水平扩展

### 日志查看

```bash
# 查看系统服务日志
sudo journalctl -u rust-web

# 查看 Docker 容器日志
docker logs rust-web-app

# 查看 Nginx 日志
sudo tail -f /var/log/nginx/error.log
```

### 联系支持

如果您遇到无法解决的问题，请通过以下方式联系支持团队：

- 提交 GitHub Issue
- 发送邮件至 support@example.com
- 在工作时间拨打技术支持热线：+1-234-567-8900