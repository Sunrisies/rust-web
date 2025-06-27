DROP TABLE IF EXISTS `third_party_libraries`;
CREATE TABLE `third_party_libraries`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '唯一标识符，主键',
  `name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '第三方库的名称',
  `official_url` varchar(250) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '第三方库的官方网址',
  `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL COMMENT '第三方库的详细描述',
  `metadata` json NULL COMMENT '第三方库的额外元数据，以JSON格式存储',
  `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间',
  `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录更新时间',
  `category_id` int NULL DEFAULT NULL COMMENT '分类ID，关联categories表的id字段',
  PRIMARY KEY (`id`) USING BTREE,
  INDEX `FK_3340e8ff253aa6ef9cd80390618`(`category_id` ASC) USING BTREE COMMENT '外键索引，用于关联categories表',
  CONSTRAINT `FK_3340e8ff253aa6ef9cd80390618` FOREIGN KEY (`category_id`) REFERENCES `categories` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE = InnoDB AUTO_INCREMENT = 2 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic COMMENT='第三方库信息表，用于存储第三方库的详细信息及相关数据';