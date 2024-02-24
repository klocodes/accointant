use crate::errors::Error;
use crate::events::event::Event;
use crate::{log_error, log_trace};
use crate::errors::server::ServerErrors::InternalServerError;

pub struct EventResponder {
    event: Event,
    result: Result<Vec<Event>, Error>,
}

impl EventResponder {
    pub fn new(event: Event, result: Result<Vec<Event>, Error>) -> Self {
        Self {
            event,
            result,
        }
    }

    pub async fn handle(&self) -> Result<(), Error> {
        if let Err (e) = &self.result {
            log_error!("Error handling event {}: {:?}", self.event.name(), e.to_string());
            log_trace!("Error handling event {}: {:?}", self.event.name(), e.to_string());
            
            return Err(Error::Server(
                InternalServerError {
                    context: Some(e.to_string().into())
                }
            ));
        }
        
        Ok(())
    }
}