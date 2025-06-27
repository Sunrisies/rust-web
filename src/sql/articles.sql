DROP TABLE IF EXISTS `article`;
CREATE TABLE `article`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '唯一标识符，主键',
  `title` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章标题',
  `content` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章内容',
  `cover` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章封面图片路径',
  `author` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '作者名称',
  `publish_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '文章发布时间',
  `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '文章更新时间',
  `views` int NOT NULL DEFAULT 0 COMMENT '文章浏览量',
  `is_top` tinyint NOT NULL DEFAULT 0 COMMENT '是否置顶：1表示置顶，0表示不置顶',
  `is_recommend` tinyint NOT NULL DEFAULT 0 COMMENT '是否推荐：1表示推荐，0表示不推荐',
  `is_delete` tinyint NOT NULL DEFAULT 0 COMMENT '是否删除：1表示已删除，0表示未删除',
  `is_publish` tinyint NOT NULL DEFAULT 0 COMMENT '是否发布：1表示已发布，0表示未发布',
  `is_hide` tinyint NOT NULL DEFAULT 1 COMMENT '是否隐藏：1表示隐藏，0表示不隐藏',
  `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章描述',
  `size` int NOT NULL DEFAULT 0 COMMENT '文章大小（字节）',
  `categoryId` int NULL DEFAULT NULL COMMENT '文章分类ID，关联categories表的id字段',
  `uuid` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '文章的UUID，用于唯一标识文章',
  PRIMARY KEY (`id`) USING BTREE,
  UNIQUE INDEX `IDX_36cdcdc76a24270d4ab6fb7986`(`uuid` ASC) USING BTREE COMMENT '唯一索引，用于快速查询文章的UUID',
  INDEX `FK_12824e4598ee46a0992d99ba553`(`categoryId` ASC) USING BTREE COMMENT '外键索引，用于关联categories表',
  CONSTRAINT `FK_12824e4598ee46a0992d99ba553` FOREIGN KEY (`categoryId`) REFERENCES `categories` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 46 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic COMMENT='文章信息表，用于存储文章的详细信息及相关状态';