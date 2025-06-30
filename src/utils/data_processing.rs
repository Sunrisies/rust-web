use log::*;
use serde_json::Value;
use std::collections::HashSet;

pub fn deep_filter_data<T>(data: Vec<T>, exclude: Vec<&str>) -> Vec<Value>
where
    T: Into<Value> + TryFrom<Value>,
{
    data.into_iter()
        .map(|item| {
            // 将输入类型转换为 Value
            let value: Value = item.into();
            // 过滤处理
            filter_value(value, exclude.clone())
        })
        .collect()
}

pub fn filter_value(value: Value, exclude: Vec<&str>) -> Value {
    let sensitive_fields: HashSet<String> = exclude.into_iter().map(String::from).collect();
    match value {
        Value::Object(mut map) => {
            // 过滤当前层
            for field in &sensitive_fields {
                error!("exclude field: {}", field);
                map.remove(field);
            }

            // 递归处理嵌套结构
            // for (_, v) in map.iter_mut() {
            //     *v = filter_value(v.clone(), exclude);
            // }
            error!("map: {:?}", map);
            Value::Object(map)
        }
        Value::Array(arr) => Value::Array(
            arr.into_iter()
                .map(|v| filter_value(v, sensitive_fields.iter().map(|s| s.as_str()).collect()))
                .collect(),
        ),
        _ => value,
    }
}
