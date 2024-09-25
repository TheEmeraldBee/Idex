use crate::{events::ExplorerEvent, input::Input};

pub enum AppState {
    Exploring,
    Input(Input, ExplorerEvent),
}
