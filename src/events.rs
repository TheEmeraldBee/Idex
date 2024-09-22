use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExplorerEvent {
    Move(i32),
    Scroll(i32),
    Expand,
    Collapse,
    Quit,

    Sh { command: String, args: Vec<String> },
}
