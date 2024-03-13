use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone)]
pub struct EventChannel<T>
    where T: Clone
{
    sender: Arc<Mutex<mpsc::Sender<T>>>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

impl<T: Clone> EventChannel<T> {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(size);

        Self {
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub async fn send(&self, message: T) -> Result<(), mpsc::error::SendError<T>> {
        self.sender.lock().await.send(message).await
    }

    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.lock().await.recv().await
    }
}
