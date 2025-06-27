use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt;

// 通用反序列化函数（直接内联在结构体中使用）
pub fn deserialize_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: EnumDeserialize + fmt::Debug,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(|_| {
        let valid = T::valid_values();
        let err_msg = format!("{} 不在{} 范围里面", s, valid.join("或"),);
        de::Error::custom(err_msg)
    })
}

// 定义枚举需要实现的trait
pub trait EnumDeserialize {
    fn from_str(s: &str) -> Result<Self, ()>
    where
        Self: Sized;
    fn valid_values() -> Vec<&'static str>;
}
