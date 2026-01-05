use iced::{Element, Task};
use status::status_event::StatusEvent;

#[derive(Debug, Clone)]
pub enum StatusMessage {
    Event(StatusEvent),
}

#[derive(Clone)]
pub struct Status {
    pub message: Option<String>,
    pub event: Option<StatusEvent>,
    pub show_percentage: bool,
}

impl Status {
    fn update_status_message(&mut self) {
        if let Some(ref event) = self.event {
            let mut status = format!("{}", event);

            if let (Some(c), Some(t)) = (event.current, event.total)
                && self.show_percentage
                && t > 0
            {
                let pct = format!(" ({:.1}%)", ((c.min(t)) as f32 / t as f32) * 100.0);
                status.push_str(&pct);
            }

            self.message = Some(status);
        } else {
            self.message = None;
        }
    }

    pub fn reset(&mut self) {
        self.message = None;
        self.event = None;
    }

    pub fn update(&mut self, msg: StatusMessage) -> Task<StatusMessage> {
        match msg {
            StatusMessage::Event(event) => {
                self.event = Some(event);
                self.update_status_message();
                Task::none()
            }
        }
    }

    pub fn view<F>(&self, widget_fn: F) -> Element<'_, StatusMessage>
    where
        F: Fn(&Option<String>) -> Element<'_, StatusMessage>,
    {
        widget_fn(&self.message)
    }
}
