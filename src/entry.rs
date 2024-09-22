use std::{cmp::Ordering, fs::DirEntry, path::PathBuf};

use ascii_forge::prelude::*;

use crate::{config::Config, dir_items::dir_items};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryType {
    Dir = 1,
    File = 2,
}

#[derive(Clone)]
pub struct Entry {
    pub depth: usize,
    pub expanded: bool,

    // Entry Data
    pub path: PathBuf,
    pub file_name: String,
    pub suffix: String,

    pub entry_type: EntryType,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.entry_type.cmp(&other.entry_type);
        match ord {
            Ordering::Equal => self.file_name.cmp(&other.file_name),
            _ => ord,
        }
    }
}

impl Entry {
    pub fn new(entry: DirEntry, depth: usize) -> Self {
        let data = entry.metadata().unwrap();
        let path = entry.path().canonicalize().unwrap();
        Self {
            depth,
            expanded: false,
            suffix: path
                .extension()
                .map(|x| x.to_str().unwrap().to_string())
                .unwrap_or("".to_string()),
            path,
            file_name: entry.file_name().into_string().unwrap(),
            entry_type: match data.is_dir() {
                true => EntryType::Dir,
                false => EntryType::File,
            },
        }
    }

    pub fn expand(&mut self) -> anyhow::Result<Option<Vec<Entry>>> {
        if self.expanded {
            return Ok(None);
        }
        self.expanded = true;
        match self.entry_type {
            EntryType::Dir => {
                let entries = dir_items(&self.path, self.depth + 1)?;
                Ok(Some(entries))
            }
            EntryType::File => Ok(None),
        }
    }

    pub fn render(&self, pos: Vec2, buffer: &mut Buffer, selected: bool, config: &Config) {
        match self.entry_type {
            EntryType::Dir => {
                let style = config
                    .styles
                    .get(&self.file_name.to_lowercase())
                    .cloned()
                    .unwrap_or(config.folder.clone());

                if selected {
                    render!(buffer, pos => [ config.tab.text.repeat(self.depth - 1).with(config.tab.color), " > ", style, self.file_name.clone().blue(), "/ <" ]);
                } else {
                    render!(buffer, pos => [ config.tab.text.repeat(self.depth).with(config.tab.color), style, self.file_name.clone().blue(), "/"]);
                }
            }
            EntryType::File => {
                let style = match config.styles.get(&self.file_name.to_lowercase()).cloned() {
                    Some(t) => t,
                    None => config
                        .styles
                        .get(&self.suffix.to_lowercase())
                        .cloned()
                        .unwrap_or_default(),
                };

                if selected {
                    render!(buffer, pos => [ config.tab.text.repeat(self.depth - 1).with(config.tab.color), " > ", style, self.file_name, " <" ]);
                } else {
                    render!(buffer, pos => [ config.tab.text.repeat(self.depth).with(config.tab.color), style, self.file_name ]);
                }
            }
        }
    }
}
