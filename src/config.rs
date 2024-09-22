use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, str::FromStr};

use crokey::KeyCombination;
use serde::{Deserialize, Serialize};

use crate::{events::ExplorerEvent, style::Style};

pub fn double_click_ms_delay_default() -> u128 {
    500
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "double_click_ms_delay_default")]
    #[serde(rename = "double_click_delay")]
    pub double_click_ms_delay: u128,

    pub bindings: HashMap<KeyCombination, ExplorerEvent>,

    #[serde(default)]
    pub double_click: Option<ExplorerEvent>,

    pub folder: Style,
    #[serde(rename = "style")]
    pub styles: HashMap<String, Style>,
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
}
