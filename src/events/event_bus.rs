use crate::events::event::Event;
use crate::events::event_listener::EventListener;

pub trait EventBus<EL>
    where
        EL: EventListener + Send + Sync + 'static
{
    fn register(&mut self, listener: EL);

    fn publish<E>(&self, event: E)
        where
            E: Event;
}