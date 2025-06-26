CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY COMMENT '主键ID，自增',
    uuid CHAR(36) NOT NULL COMMENT '全局唯一标识符（字符串格式）',
    user_name VARCHAR(255) NOT NULL COMMENT '用户名',
    pass_word VARCHAR(255) NOT NULL COMMENT '密码',
    email VARCHAR(255) COMMENT '电子邮箱',
    image VARCHAR(255) COMMENT '头像图片路径',
    phone VARCHAR(20) COMMENT '手机号码',
    role VARCHAR(50) COMMENT '用户角色',
    permissions TEXT COMMENT '用户权限',
    binding VARCHAR(255) COMMENT '绑定信息',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    UNIQUE KEY unique_uuid (uuid),
    UNIQUE KEY unique_email (email),
    UNIQUE KEY unique_user_name (user_name),
    UNIQUE KEY unique_phone (phone)
) COMMENT='用户信息表';