use crate::events::event::Event;

pub trait EventListener {
    fn on_event<E>(&self, event: E)
        where
            E: Event;
}