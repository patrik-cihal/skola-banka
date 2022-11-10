use std::collections::HashMap;
use std::fmt::Debug;

use super::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum SubscribeEvent {
    Load,
    Save,
    CreateAccount,
}

pub type Subscriber = fn(String);

#[derive(Default, Debug)]
pub struct Publisher {
    events: HashMap<SubscribeEvent, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: SubscribeEvent, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }
    pub fn unsubscribe(&mut self, event_type: SubscribeEvent, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }
    pub fn notify(&self, event_type: SubscribeEvent, message: String) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(message.clone());
        }
    }
}

pub trait Observable {
    fn events(&mut self) -> &mut Publisher;
}
