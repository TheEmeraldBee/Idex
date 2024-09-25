use std::fmt::Display;

use ascii_forge::{
    math::Vec2,
    render,
    window::{Buffer, Render},
};
use crossterm::style::{Color, StyledContent, Stylize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Style {
    pub icon: String,
    #[serde(default, rename = "color")]
    pub icon_color: Option<Color>,
    #[serde(default)]
    pub text_color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            icon: "".to_string(),
            icon_color: None,
            text_color: None,
        }
    }
}

impl Render for Style {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        if self.icon.is_empty() {
            return loc;
        }

        if let Some(color) = self.icon_color {
            render!(buffer, loc => [ self.icon.clone().with(color), " " ])
        } else {
            render!(buffer, loc => [ self.icon, " " ])
        }
    }
}

impl Style {
    pub fn style<D: Display + Stylize<Styled = StyledContent<D>>>(
        &self,
        text: D,
    ) -> StyledContent<D> {
        let mut styled = text.stylize();

        if let Some(color) = self.text_color {
            styled = styled.with(color)
        }

        styled
    }
}
