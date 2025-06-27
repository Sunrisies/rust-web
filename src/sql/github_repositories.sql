DROP TABLE IF EXISTS `github_repositories`;
CREATE TABLE `github_repositories`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '唯一标识符，主键',
  `owner` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '仓库所有者',
  `repository` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '仓库名称',
  `branch` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '分支名称',
  `enabled` tinyint NOT NULL DEFAULT 1 COMMENT '是否启用，1表示启用，0表示禁用',
  `created_at` datetime(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '记录创建时间',
  `last_sync_at` datetime NULL DEFAULT NULL COMMENT '最后同步时间',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 4 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic COMMENT='GitHub仓库信息表，用于存储GitHub仓库的基础信息';