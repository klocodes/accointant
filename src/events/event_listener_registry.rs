use std::sync::Arc;
use tokio::sync::Mutex;
use crate::di::service_container::ServiceContainer;
use crate::errors::Error;
use crate::events::event_listener::EventListener;
use crate::features::categories::infrastructure::event_listeners::category_creation_requested_listener::CategoryCreationRequestedListener;

pub struct EventListenerRegistry {
    service_container: Arc<ServiceContainer>,
    listeners: Arc<Mutex<Vec<Box<dyn EventListener>>>>,
}

impl EventListenerRegistry {
    pub fn new(service_container: Arc<ServiceContainer>) -> Self {
        Self {
            service_container,
            listeners: Arc::new(
                Mutex::new(
                    Vec::new()
                )
            ),
        }
    }

    pub async fn register_listeners(&mut self) -> Result<(), Error> {
        let mut guard = self.listeners.lock().await;

        guard.push(
            Box::new(CategoryCreationRequestedListener::new()),
        );

        Ok(())
    }

    pub fn listeners(&self) -> Arc<Mutex<Vec<Box<dyn EventListener>>>> {
        self.listeners.clone()
    }
}
