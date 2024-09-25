use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::events::ExplorerEvent;

#[derive(Default, Debug)]
pub struct Input {
    text: String,
    active: bool,
    event: Option<ExplorerEvent>,
}

pub enum InputEvent {
    Cancel,
    Accept,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&mut self) -> String {
        std::mem::take(&mut self.text)
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn clear(&mut self) {
        self.text = String::new();
    }

    pub fn set_event(&mut self, event: ExplorerEvent) {
        self.event = Some(event)
    }

    pub fn take_event(&mut self) -> Option<ExplorerEvent> {
        self.event.take()
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn event(&mut self, event: KeyEvent) -> Option<InputEvent> {
        if !self.active {
            return None;
        }
        if !(event.kind == KeyEventKind::Press || event.kind == KeyEventKind::Repeat) {
            // Ignore if released.
            return None;
        }

        match event.code {
            KeyCode::Char(t) => self.text.push(t),
            KeyCode::Esc => return Some(InputEvent::Cancel),
            KeyCode::Enter => return Some(InputEvent::Accept),
            KeyCode::Backspace => {
                self.text.pop();
            }
            _ => {}
        }
        None
    }
}
