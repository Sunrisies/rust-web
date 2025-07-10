#!/bin/bash

# 配置变量
IMAGE_NAME="rust-web:latest"
LOCAL_TAR_FILE="rust-web.tar"
REMOTE_USER="root"
REMOTE_HOST="api.chaoyang1024.top"
REMOTE_DIR="/home/docker/server"
CONTAINER_NAME="rust-web"
PORT_MAPPING="18080:18080"
VOLUME_MAPPING="/home/docker/server/data1:/home/app/logs"

# 日志函数
log() {
  echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

# 检查命令是否执行成功
check_success() {
  if [ $? -ne 0 ]; then
    log "错误: $1 执行失败"
    exit 1
  fi
}


# 构建 Docker 镜像
log "正在构建 Docker 镜像..."
docker build -t $IMAGE_NAME .
if [ $? -ne 0 ]; then
  log "Docker 构建失败，正在回退版本..."
  node -e "require('./scripts/version-manager').revertVersion('$CURRENT_VERSION')"
  exit 1
fi

# 保存 Docker 镜像为 tar 文件
log "正在将 Docker 镜像保存为 tar 文件..."
docker save -o $LOCAL_TAR_FILE $IMAGE_NAME
if [ $? -ne 0 ]; then
  log "Docker 镜像保存失败，正在回退版本..."
  node -e "require('./scripts/version-manager').revertVersion('$CURRENT_VERSION')"
  exit 1
fi

# 上传文件到远程服务器
log "正在上传 $LOCAL_TAR_FILE 到 $REMOTE_HOST..."
scp $LOCAL_TAR_FILE $REMOTE_USER@$REMOTE_HOST:$REMOTE_DIR/
if [ $? -ne 0 ]; then
  log "文件上传失败，正在回退版本..."
  node -e "require('./scripts/version-manager').revertVersion('$CURRENT_VERSION')"
  rm -f $LOCAL_TAR_FILE
  exit 1
fi

# 远程执行命令
log "正在远程服务器上部署..."
ssh $REMOTE_USER@$REMOTE_HOST "
  cd $REMOTE_DIR || exit 1

  # 停止并删除旧容器
  docker stop $CONTAINER_NAME && docker rm $CONTAINER_NAME
  docker rmi $IMAGE_NAME

  # 加载新镜像并启动容器
  docker load -i $LOCAL_TAR_FILE
  docker run -it -d -v $VOLUME_MAPPING -p $PORT_MAPPING --restart=always --name $CONTAINER_NAME $IMAGE_NAME
"

if [ $? -ne 0 ]; then
  log "部署失败，正在回退版本..."
  node -e "require('./scripts/version-manager').revertVersion('$CURRENT_VERSION')"
  rm -f $LOCAL_TAR_FILE
  exit 1
fi

# 清理本地临时文件
log "正在清理本地临时文件..."
rm -f $LOCAL_TAR_FILE

log "部署成功完成！版本 $NEW_VERSION 已发布"