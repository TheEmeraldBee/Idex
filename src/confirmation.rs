use crossterm::event::{KeyCode, KeyEvent};

use crate::events::ExplorerEvent;

#[derive(Default)]
pub struct Confirmation {
    event: Option<ExplorerEvent>,
}

impl Confirmation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, event: ExplorerEvent) {
        self.event = Some(event)
    }

    pub fn active(&self) -> bool {
        self.event.is_some()
    }

    pub fn handle(&self, k: KeyEvent) -> bool {
        matches!(k.code, KeyCode::Char('y'))
    }

    pub fn take(&mut self) -> Option<ExplorerEvent> {
        self.event.take()
    }
}
