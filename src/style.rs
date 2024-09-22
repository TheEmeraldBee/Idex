use ascii_forge::{
    math::Vec2,
    render,
    window::{Buffer, Render},
};
use crossterm::style::{Color, Stylize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Style {
    pub icon: String,
    #[serde(default)]
    pub color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            icon: "".to_string(),
            color: None,
        }
    }
}

impl Render for Style {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        if self.icon.is_empty() {
            return loc;
        }

        if let Some(color) = self.color {
            render!(buffer, loc => [ self.icon.clone().with(color), " " ])
        } else {
            render!(buffer, loc => [ self.icon.clone(), " " ])
        }
    }
}
