use salsa::DebugWithDb;
use std::sync::{Arc, Mutex};

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
    logs: Arc<Mutex<Vec<String>>>,
}

impl salsa::Database for Database {
    fn salsa_event(&self, event: salsa::Event) {
        // Log interesting events, if logging is enabled don't log boring events
        if let salsa::EventKind::WillExecute { .. } = event.kind {
            self.logs
                .lock()
                .unwrap()
                .push(format!("Event: {:?}", event.debug(self)));
        }
    }
}
