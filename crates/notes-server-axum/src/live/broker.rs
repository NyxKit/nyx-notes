use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
struct BrokerEntry {
    listeners: usize,
    version: u64,
}

#[derive(Debug, Clone, Default)]
pub struct LiveBroker {
    inner: Arc<Mutex<HashMap<String, BrokerEntry>>>,
}

impl LiveBroker {
    pub fn attach(&self, key: &str) -> usize {
        let mut inner = self.inner.lock().expect("live broker lock poisoned");
        let entry = inner.entry(key.to_string()).or_default();
        entry.listeners += 1;
        entry.listeners
    }

    pub fn release(&self, key: &str) -> usize {
        let mut inner = self.inner.lock().expect("live broker lock poisoned");
        let mut next = 0;

        if let Some(entry) = inner.get_mut(key) {
            if entry.listeners > 0 {
                entry.listeners -= 1;
            }
            next = entry.listeners;
            if entry.listeners == 0 {
                inner.remove(key);
            }
        }

        next
    }

    pub fn listeners(&self, key: &str) -> usize {
        self.inner
            .lock()
            .expect("live broker lock poisoned")
            .get(key)
            .map(|entry| entry.listeners)
            .unwrap_or(0)
    }

    pub fn publish(&self, key: &str) -> u64 {
        let mut inner = self.inner.lock().expect("live broker lock poisoned");
        let entry = inner.entry(key.to_string()).or_default();
        entry.version += 1;
        entry.version
    }

    pub fn version(&self, key: &str) -> u64 {
        self.inner
            .lock()
            .expect("live broker lock poisoned")
            .get(key)
            .map(|entry| entry.version)
            .unwrap_or(0)
    }
}
