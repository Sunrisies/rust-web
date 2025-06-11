// use bitflags::bitflags;

// bitflags! {
//     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//     pub struct Permission: u64 {
//         const NONE = 0;

//         // 文章模块
//         const READ_ARTICLE = 1 << 0;
//         const WRITE_ARTICLE = 1 << 1;
//         const READ_WRITE_ARTICLE = Self::READ_ARTICLE.bits() | Self::WRITE_ARTICLE.bits();

//         // 评论模块
//         const READ_COMMENT = 1 << 2;
//         const WRITE_COMMENT = 1 << 3;
//         const READ_WRITE_COMMENT = Self::READ_COMMENT.bits() | Self::WRITE_COMMENT.bits();

//         // 用户模块
//         const READ_USER = 1 << 4;
//         const WRITE_USER = 1 << 5;
//         const READ_WRITE_USER = Self::READ_USER.bits() | Self::WRITE_USER.bits();

//         // 系统模块
//         const READ_SYSTEM = 1 << 6;
//         const WRITE_SYSTEM = 1 << 7;
//         const READ_WRITE_SYSTEM = Self::READ_SYSTEM.bits() | Self::WRITE_SYSTEM.bits();

//         // 文件模块
//         const READ_FILE = 1 << 8;
//         const WRITE_FILE = 1 << 9;
//         const READ_WRITE_FILE = Self::READ_FILE.bits() | Self::WRITE_FILE.bits();

//         // 标签模块
//         const READ_TAG = 1 << 10;
//         const WRITE_TAG = 1 << 11;
//         const READ_WRITE_TAG = Self::READ_TAG.bits() | Self::WRITE_TAG.bits();

//         // 分类模块
//         const READ_CATEGORY = 1 << 12;
//         const WRITE_CATEGORY = 1 << 13;
//         const READ_WRITE_CATEGORY = Self::READ_CATEGORY.bits() | Self::WRITE_CATEGORY.bits();

//         // 留言模块
//         const READ_MESSAGE = 1 << 14;
//         const WRITE_MESSAGE = 1 << 15;
//         const READ_WRITE_MESSAGE = Self::READ_MESSAGE.bits() | Self::WRITE_MESSAGE.bits();

//         // 所以的观看权限
//         const READ = Self::READ_ARTICLE.bits()
//             | Self::READ_COMMENT.bits()
//             | Self::READ_USER.bits()
//             | Self::READ_SYSTEM.bits()
//             | Self::READ_FILE.bits()
//             | Self::READ_TAG.bits()
//             | Self::READ_CATEGORY.bits()
//             | Self::READ_MESSAGE.bits();
//         // 所有权限
//         const ALL = !0;
//     }
// }

use bitflags::bitflags;
use lazy_static::lazy_static;

// 定义权限宏
#[macro_export]
macro_rules! define_permissions {
    ($($name:ident = $value:expr, $description:expr;)*) => {
        bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct Permission: u64 {
                const NONE = 0;
                $(
                    const $name = $value;
                )*
            }
        }

        // 自动生成权限列表
        pub static PERMISSION_LIST: &[(&str, &str)] = &[
            $(
                (stringify!($name), $description),
            )*
        ];

        lazy_static! {
            pub static ref PERMISSION_MAP: std::collections::HashMap<&'static str, Permission> = {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert(stringify!($name), Permission::$name);
                )*
                map
            };
        }
    };
}

// 定义权限
define_permissions! {
    READ_ARTICLE = 1 << 0, "读取文章权限";
    WRITE_ARTICLE = 1 << 1, "写入文章权限";
    READ_WRITE_ARTICLE = (1 << 0) | (1 << 1), "读写文章权限";
    READ_COMMENT = 1 << 2, "读取评论权限";
    WRITE_COMMENT = 1 << 3, "写入评论权限";
    READ_WRITE_COMMENT = (1 << 2) | (1 << 3), "读写评论权限";
    READ_USER = 1 << 4, "读取用户权限";
    WRITE_USER = 1 << 5, "写入用户权限";
    READ_WRITE_USER = (1 << 4) | (1 << 5), "读写用户权限";
    READ_SYSTEM = 1 << 6, "读取系统权限";
    WRITE_SYSTEM = 1 << 7, "写入系统权限";
    READ_WRITE_SYSTEM = (1 << 6) | (1 << 7), "读写系统权限";
    READ_FILE = 1 << 8, "读取文件权限";
    WRITE_FILE = 1 << 9, "写入文件权限";
    READ_WRITE_FILE = (1 << 8) | (1 << 9), "读写文件权限";
    READ_TAG = 1 << 10, "读取标签权限";
    WRITE_TAG = 1 << 11, "写入标签权限";
    READ_WRITE_TAG = (1 << 10) | (1 << 11), "读写标签权限";
    READ_CATEGORY = 1 << 12, "读取分类权限";
    WRITE_CATEGORY = 1 << 13, "写入分类权限";
    READ_WRITE_CATEGORY = (1 << 12) | (1 << 13), "读写分类权限";
    READ_MESSAGE = 1 << 14, "读取消息权限";
    WRITE_MESSAGE = 1 << 15, "写入消息权限";
    READ_WRITE_MESSAGE = (1 << 14) | (1 << 15), "读写消息权限";
    READ = (1 << 0) | (1 << 2) | (1 << 4) | (1 << 6) | (1 << 8) | (1 << 10) | (1 << 12) | (1 << 14), "所有读取权限";
    ALL = !0, "所有权限";
}
