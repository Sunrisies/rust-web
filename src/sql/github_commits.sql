DROP TABLE IF EXISTS `github_commits`;
CREATE TABLE `github_commits`  (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '主键ID，用于唯一标识每条提交记录',
  `sha` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交的SHA值，用于唯一标识一次提交',
  `node_id` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '节点ID，GitHub API中用于标识提交的ID',
  `author_name` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交作者的名称',
  `author_email` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交作者的邮箱',
  `commit_date` datetime NOT NULL COMMENT '提交的时间',
  `message` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交的消息，描述本次提交的内容或修改',
  `tree_sha` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '树对象的SHA值，指向该提交对应的文件树',
  `url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交的URL地址，用于访问该提交的详细信息',
  `html_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '提交的HTML页面URL，通常用于在浏览器中查看提交',
  `comments_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '评论的URL地址，用于访问该提交的评论',
  `comment_count` int NOT NULL DEFAULT 0 COMMENT '评论数量，表示该提交收到的评论数目',
  `parent_sha` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '父提交的SHA值，表示该提交基于哪个提交进行的修改',
  `parent_url` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '父提交的URL地址，用于访问父提交的详细信息',
  `verified` tinyint NOT NULL DEFAULT 1 COMMENT '验证状态，表示该提交是否经过验证',
  `verify_reason` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL DEFAULT NULL COMMENT '验证原因，说明验证状态的具体原因',
  `created_at` datetime(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) COMMENT '记录创建时间，表示该提交记录在数据库中创建的时间',
  `repository` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '仓库名称，表示该提交所属的代码仓库',
  `branch` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '分支名称，表示该提交所在的分支',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB AUTO_INCREMENT = 178 CHARACTER SET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci ROW_FORMAT = Dynamic COMMENT='GitHub提交信息表，用于存储GitHub仓库的提交记录及相关信息';