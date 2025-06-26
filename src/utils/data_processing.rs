use serde_json::Value;
use std::collections::HashSet;

pub fn deep_filter_data<T, E>(data: Vec<T>, exclude: E) -> Vec<Value>
where
    T: Into<Value> + TryFrom<Value>,
    E: Into<HashSet<String>>,
{
    let exclude_fields = exclude.into();

    data.into_iter()
        .map(|item| {
            // 将输入类型转换为 Value
            let value: Value = item.into();
            // 过滤处理
            filter_value(value, &exclude_fields)
        })
        .collect()
}

fn filter_value(value: Value, exclude: &HashSet<String>) -> Value {
    match value {
        Value::Object(mut map) => {
            // 过滤当前层
            for field in exclude {
                log::error!("exclude field: {}", field);
                map.remove(field);
            }

            // 递归处理嵌套结构
            // for (_, v) in map.iter_mut() {
            //     *v = filter_value(v.clone(), exclude);
            // }
            log::error!("map: {:?}", map);
            Value::Object(map)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(|v| filter_value(v, exclude)).collect())
        }
        _ => value,
    }
}
