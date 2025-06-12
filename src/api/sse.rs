use crate::utils::sse::SseNotifier;
use actix_web::{web, Responder};
use actix_web_lab::sse::{self, Event, Sse};
use futures_util::stream::StreamExt;
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::BroadcastStream;

pub async fn sse_stream(notifier: web::Data<SseNotifier>) -> impl Responder {
    // 订阅全局事件通道
    let rx = notifier.create_channel();

    // 创建兼容的事件流
    let sse_stream = BroadcastStream::new(rx)
        .filter_map(|msg| async move {
            match msg {
                Ok(data) => Some(sse::Event::Data(sse::Data::new(data))),
                Err(_) => None,
            }
        })
        // 将错误转换为 Infallible
        .map(|event| Ok::<Event, Infallible>(event));

    // 创建SSE响应
    Sse::from_stream(sse_stream).with_keep_alive(Duration::from_secs(5))
}
