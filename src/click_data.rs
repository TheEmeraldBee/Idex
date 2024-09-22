use std::time::SystemTime;

use crate::config::Config;

pub struct ClickData {
    time: SystemTime,
    line: usize,
}

impl ClickData {
    pub fn new(line: usize) -> Self {
        Self {
            time: SystemTime::now(),
            line,
        }
    }

    pub fn is_double(&self, other: &ClickData, conf: &Config) -> bool {
        if self.line != other.line {
            return false;
        }
        other.time.duration_since(self.time).unwrap().as_millis() < conf.double_click_ms_delay
    }
}

impl Default for ClickData {
    fn default() -> Self {
        Self::new(10000)
    }
}
