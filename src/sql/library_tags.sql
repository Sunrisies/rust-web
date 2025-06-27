-- ----------------------------
-- Table structure for library_tags
-- ----------------------------
DROP TABLE IF EXISTS `library_tags`;
CREATE TABLE `library_tags`  (
  `library_id` int NOT NULL COMMENT '第三方库ID，关联third_party_libraries表的id字段',
  `tag_id` int NOT NULL COMMENT '标签ID，关联tags表的id字段',
  PRIMARY KEY (`library_id`, `tag_id`) USING BTREE ,
  INDEX `IDX_7909346747d002ee2406fb2aa8`(`library_id` ASC) USING BTREE ,
  INDEX `IDX_d055ad00a9516587fdf20c2fed`(`tag_id` ASC) USING BTREE ,
  CONSTRAINT `FK_7909346747d002ee2406fb2aa8e` FOREIGN KEY (`library_id`) REFERENCES `third_party_libraries` (`id`) ON DELETE CASCADE ON UPDATE CASCADE ,
  CONSTRAINT `FK_d055ad00a9516587fdf20c2fed1` FOREIGN KEY (`tag_id`) REFERENCES `tags` (`id`) ON DELETE CASCADE ON UPDATE CASCADE 
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic COMMENT='第三方库标签关联表，用于存储第三方库和标签之间的多对多关系';