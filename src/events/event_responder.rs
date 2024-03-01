use crate::events::event::Event;
use crate::{log_error, log_trace};
use crate::events::error::EventError;

#[derive(Debug, Clone)]
pub struct EventResponder {
    event: Event,
    result: Result<Vec<Event>, EventError>,
}

impl EventResponder {
    pub fn new(event: Event, result: Result<Vec<Event>, EventError>) -> Self {
        Self {
            event,
            result,
        }
    }

    pub async fn handle(&self) -> Result<(), EventError> {
        if let Err (e) = &self.result {
            log_error!("Error handling event {}: {:?}", self.event.name(), e.to_string());
            log_trace!("Error handling event {}: {:?}", self.event.name(), e.to_string());
            
            return Err(e.clone());
        }
        
        Ok(())
    }
}