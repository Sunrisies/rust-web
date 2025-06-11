// use futures::Future;
// use std::pin::Pin;
// use std::sync::{Arc, Mutex};
// use tokio::sync::broadcast;
// use tokio_stream::{Stream, StreamExt}; // 添加这个导入

// pub type SseEventStream = Pin<Box<dyn Stream<Item = String> + Send>>;

// pub struct SseNotifier {
//     tx: Arc<Mutex<broadcast::Sender<String>>>,
// }

// impl SseNotifier {
//     pub fn new() -> Self {
//         let (tx, _) = broadcast::channel(100);
//         Self {
//             tx: Arc::new(Mutex::new(tx)),
//         }
//     }

//     pub fn subscribe(&self) -> SseEventStream {
//         let rx = self.tx.lock().unwrap().subscribe();
//         Box::pin(
//             tokio_stream::wrappers::BroadcastStream::new(rx)
//                 .filter_map(|x| x.ok()) // 直接处理Result，不需要async
//                 .map(|x| format!("data: {}\n\n", x)),
//         )
//     }

//     pub fn notify(&self, message: String) {
//         if let Ok(tx) = self.tx.lock() {
//             let _ = tx.send(message);
//         }
//     }
// }

// sse_notifier.rs
// sse_notifier.rs
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct SseNotifier {
    user_channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl SseNotifier {
    pub fn new() -> Self {
        SseNotifier {
            user_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_channel(&self, user_id: &str) -> broadcast::Receiver<String> {
        let mut channels = self.user_channels.lock().unwrap();
        let tx = channels
            .entry(user_id.to_string())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone();
        tx.subscribe()
    }

    pub fn notify(&self, user_id: &str, message: impl Into<String>) {
        let channels = self.user_channels.lock().unwrap();
        if let Some(tx) = channels.get(user_id) {
            let _ = tx.send(message.into());
        }
    }
}
