use tokio::sync::broadcast;

// 全局事件广播器
#[derive(Clone)]
pub struct SseNotifier {
    tx: broadcast::Sender<String>,
}

impl SseNotifier {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        SseNotifier { tx }
    }

    // 创建新连接通道
    pub fn create_channel(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    // 发送事件给所有连接的客户端
    pub fn notify(&self, message: &str) {
        let _ = self.tx.send(message.to_string());
    }
}
