use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplorerEvent {
    Move(i32),
    Scroll(i32),
    Expand,
    Collapse,
    Quit,

    Sh { command: String, args: Vec<String> },

    Input { event: Box<ExplorerEvent> },
    Confirmation { event: Box<ExplorerEvent> },
}
