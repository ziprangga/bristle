pub mod status_channel;
pub mod status_event;

use status_channel::StatusChannel;
use status_event::StatusEvent;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;

pub trait StatusHandler: Send + Sync {
    fn handle_event(&self, event: StatusEvent);
}

#[derive(Clone)]
pub struct StatusEmitter {
    emitter: Arc<dyn StatusHandler>,
    event: StatusEvent,
}

impl StatusEmitter {
    pub fn new(emitter: Arc<dyn StatusHandler>) -> Self {
        Self {
            emitter,
            event: StatusEvent::new(),
        }
    }

    pub fn with_stage(&self, stage: impl Into<String>) -> Self {
        let mut new = self.clone();

        new.event = new.event.with_stage(stage);
        new
    }

    pub fn with_message(&self, message: impl Into<String>) -> Self {
        let mut new = self.clone();

        new.event = new.event.with_message(message);
        new
    }

    pub fn with_current(&self, current: usize) -> Self {
        let mut new = self.clone();

        new.event = new.event.with_current(current);
        new
    }

    pub fn with_total(&self, total: usize) -> Self {
        let mut new = self.clone();

        new.event = new.event.with_total(total);
        new
    }

    pub fn with_path(&self, path: PathBuf) -> Self {
        let mut new = self.clone();

        new.event = new.event.with_path(path);
        new
    }

    pub fn with_separator(&self, sep: impl Into<String>) -> Self {
        let mut new = self.clone();
        new.event = new.event.with_message(sep);
        new
    }

    pub fn emit(&self) {
        let event = self.event.clone();
        self.emitter.handle_event(event);
    }
}

pub fn setup_status_emitter(buffer: usize) -> (StatusEmitter, mpsc::Receiver<StatusEvent>) {
    let (tx, rx) = mpsc::channel::<StatusEvent>(buffer);
    let handler = Arc::new(StatusChannel::new(tx));
    let emitter = StatusEmitter::new(handler);

    (emitter, rx)
}
