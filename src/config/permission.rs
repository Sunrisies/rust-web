use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct Permission: u64 {
        const NONE = 0;

        // 文章模块
        const READ_ARTICLE = 1 << 0;
        const WRITE_ARTICLE = 1 << 1;
        const READ_WRITE_ARTICLE = Self::READ_ARTICLE.bits() | Self::WRITE_ARTICLE.bits();

        // 评论模块
        const READ_COMMENT = 1 << 2;
        const WRITE_COMMENT = 1 << 3;
        const READ_WRITE_COMMENT = Self::READ_COMMENT.bits() | Self::WRITE_COMMENT.bits();

        // 用户模块
        const READ_USER = 1 << 4;
        const WRITE_USER = 1 << 5;
        const READ_WRITE_USER = Self::READ_USER.bits() | Self::WRITE_USER.bits();

        // 系统模块
        const READ_SYSTEM = 1 << 6;
        const WRITE_SYSTEM = 1 << 7;
        const READ_WRITE_SYSTEM = Self::READ_SYSTEM.bits() | Self::WRITE_SYSTEM.bits();

        // 文件模块
        const READ_FILE = 1 << 8;
        const WRITE_FILE = 1 << 9;
        const READ_WRITE_FILE = Self::READ_FILE.bits() | Self::WRITE_FILE.bits();

        // 标签模块
        const READ_TAG = 1 << 10;
        const WRITE_TAG = 1 << 11;
        const READ_WRITE_TAG = Self::READ_TAG.bits() | Self::WRITE_TAG.bits();

        // 分类模块
        const READ_CATEGORY = 1 << 12;
        const WRITE_CATEGORY = 1 << 13;
        const READ_WRITE_CATEGORY = Self::READ_CATEGORY.bits() | Self::WRITE_CATEGORY.bits();

        // 留言模块
        const READ_MESSAGE = 1 << 14;
        const WRITE_MESSAGE = 1 << 15;
        const READ_WRITE_MESSAGE = Self::READ_MESSAGE.bits() | Self::WRITE_MESSAGE.bits();

        // 所有权限
        const ALL = !0;
    }
}
