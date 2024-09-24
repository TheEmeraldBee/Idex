use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, str::FromStr};

use crokey::KeyCombination;
use crossterm::style::Color;
use globset::Glob;
use serde::{Deserialize, Serialize};

use crate::{events::ExplorerEvent, style::Style};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TabConfig {
    pub text: String,
    pub color: Color,
}

impl Default for TabConfig {
    fn default() -> Self {
        Self {
            text: "  ".to_string(),
            color: Color::White,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub tab: TabConfig,

    #[serde(rename = "double_click_delay")]
    pub double_click_ms_delay: u128,

    pub bindings: HashMap<KeyCombination, ExplorerEvent>,

    pub double_click: Option<ExplorerEvent>,

    pub folder: Style,
    #[serde(rename = "style")]
    #[serde(with = "tuple_vec_map")]
    pub styles: Vec<(Glob, Style)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tab: TabConfig::default(),
            double_click_ms_delay: 500,
            bindings: HashMap::new(),
            double_click: None,
            folder: Style::default(),
            styles: vec![],
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let mut home_path = dirs::home_dir().unwrap();
        home_path.push(".config");

        let conf_home = std::env::var("XDG_CONFIG_HOME")
            .map(|x| PathBuf::from_str(&x).unwrap())
            .unwrap_or(home_path);
        let conf_path = conf_home.join("idex/conf.toml");

        let mut config_text = String::new();
        match File::open(conf_path) {
            Ok(mut t) => {
                t.read_to_string(&mut config_text)?;
            }
            Err(_) => config_text = include_str!("../default_config/conf.toml").to_string(),
        }

        Ok(toml::from_str(&config_text)?)
    }

    pub fn find_match(&self, name: &str) -> Option<Style> {
        for (glob, style) in &self.styles {
            if glob.compile_matcher().is_match(name) {
                return Some(style.clone());
            }
        }
        None
    }
}
