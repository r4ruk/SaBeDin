use async_trait::async_trait;
use tokio::sync::broadcast;
use crate::core::contracts::errors::GeneralServerError;

#[async_trait]
pub trait PushMessengerProvider: Send + Sync {
    async fn push_message(&self, message: &str) -> Result<(), GeneralServerError>;
}
struct PushMessenger {
    broadcast_sender: broadcast::Sender<String>
}

#[allow(unused)]
impl PushMessenger{
    fn new() -> Self {
        let (broadcast_sender, _) = broadcast::channel(10);
        Self { broadcast_sender }
    }
}

#[async_trait]
impl PushMessengerProvider for PushMessenger {
    async fn push_message(&self, message: &str) -> Result<(), GeneralServerError> {
        let _ = self.broadcast_sender.send(message.to_string())?;

        return Ok(())
    }
}