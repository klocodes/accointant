use crate::events::event::Event;
use crate::events::event_listener::EventListener;
use crate::mq::manager::MqManager;

pub struct MqEventBus<B, EL>
    where
        B: MqManager,
        EL: EventListener + Send + Sync + 'static
{
    broker: B,
    listeners: Vec<EL>,
}

impl<B, EL> MqEventBus<B, EL>
    where
        B: MqManager,
        EL: EventListener + Send + Sync + 'static
{
    pub fn new(broker: B, listeners: Vec<EL>) -> Self {
        Self {
            broker,
            listeners,
        }
    }
}

impl<B, EL> MqEventBus<B, EL>
    where
        B: MqManager,
        EL: EventListener + Send + Sync + 'static
{
    pub fn register(&mut self, listener: EL) {
        self.listeners.push(listener);
    }

    pub fn publish<E>(&self, event: E)
        where
            E: Event,
    {
        for listener in &self.listeners {
            listener.on_event(event.clone());
        }


    }
}