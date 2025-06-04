use actix_web::error::JsonPayloadError;
use regex::Regex;

/// 解析JSON错误并返回用户友好的消息
pub fn parse_json_error(err: &JsonPayloadError) -> String {
    let error_str = err.to_string();

    // 提取缺失字段名
    let re = Regex::new(r"missing field `([^`]+)`").unwrap();
    if let Some(caps) = re.captures(&error_str) {
        if let Some(field) = caps.get(1) {
            return format!("缺少必填字段: {}", field.as_str());
        }
    }

    // 处理类型不匹配错误
    if error_str.contains("expected") && error_str.contains("found") {
        return "字段类型不匹配".to_string();
    }

    // 处理不完整请求体
    if error_str.contains("unexpected end of input") {
        return "请求体不完整".to_string();
    }

    // 默认错误信息
    "请求数据格式错误".to_string()
}
