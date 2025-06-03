#!/bin/bash

echo "开始构建最小化的发布版本..."

# 清理之前的构建
echo "清理之前的构建..."
cargo clean

# 执行 release 构建
echo "执行 release 构建..."
cargo build --release

# 获取可执行文件路径
BINARY_PATH="target/release/mysql_user_crud"

if [ ! -f "$BINARY_PATH" ]; then
    echo "错误：构建失败，未找到可执行文件"
    exit 1
fi

# 显示原始大小
ORIGINAL_SIZE=$(stat -f %z "$BINARY_PATH")
echo "原始文件大小: $(($ORIGINAL_SIZE/1024/1024))MB (${ORIGINAL_SIZE}字节)"

# 检查是否安装了 upx
if command -v upx >/dev/null 2>&1; then
    echo "使用 UPX 压缩..."
    upx --best --lzma "$BINARY_PATH"
    
    # 显示压缩后的大小
    COMPRESSED_SIZE=$(stat -f %z "$BINARY_PATH")
    echo "压缩后文件大小: $(($COMPRESSED_SIZE/1024/1024))MB (${COMPRESSED_SIZE}字节)"
    echo "压缩率: $((($ORIGINAL_SIZE-$COMPRESSED_SIZE)*100/$ORIGINAL_SIZE))%"
else
    echo "未找到 UPX。如果要进一步压缩，请安装 UPX:"
    echo "  macOS: brew install upx"
    echo "  Ubuntu/Debian: sudo apt-get install upx"
    echo "  CentOS/RHEL: sudo yum install upx"
fi

echo -e "\n构建完成！"
echo "可执行文件位置: $BINARY_PATH"
echo -e "\n部署说明:"
echo "1. 确保目标系统已安装 MySQL"
echo "2. 创建 .env 文件配置数据库连接:"
echo "   DATABASE_URL=mysql://username:password@localhost:3306/database_name"
echo "3. 运行可执行文件: ./$BINARY_PATH"