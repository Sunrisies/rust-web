-- ----------------------------
-- Table structure for article_tags
-- ----------------------------
DROP TABLE IF EXISTS `article_tags`;
CREATE TABLE `article_tags`  (
  `articleId` int NOT NULL COMMENT '文章ID，关联article表的id字段',
  `tagsId` int NOT NULL COMMENT '标签ID，关联tags表的id字段',
  PRIMARY KEY (`articleId`, `tagsId`) USING BTREE ,
  INDEX `IDX_acbc7f775fb5e3fe2627477b5f`(`articleId` ASC) USING BTREE ,
  INDEX `IDX_ab2929ab48ecedb624e30b2649`(`tagsId` ASC) USING BTREE ,
  CONSTRAINT `FK_ab2929ab48ecedb624e30b26490` FOREIGN KEY (`tagsId`) REFERENCES `tags` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT ,
  CONSTRAINT `FK_acbc7f775fb5e3fe2627477b5f7` FOREIGN KEY (`articleId`) REFERENCES `article` (`id`) ON DELETE CASCADE ON UPDATE CASCADE 
) ENGINE = InnoDB CHARACTER SET = utf8mb4  ROW_FORMAT = Dynamic COMMENT='文章标签关联表，用于存储文章和标签之间的多对多关系';