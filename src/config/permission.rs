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
