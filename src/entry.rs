use std::fs::DirEntry;

use ascii_forge::prelude::*;

use crate::{config::Config, dir_items::dir_items};

pub enum EntryType {
    Dir,
    File,
}

pub struct Entry {
    pub depth: usize,
    pub entry: DirEntry,
    pub expanded: bool,
    pub entry_type: EntryType,
}

impl Entry {
    pub fn new(entry: DirEntry, depth: usize) -> Self {
        let data = entry.metadata().unwrap();
        Self {
            depth,
            entry,
            expanded: false,
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
                let entries = dir_items(self.entry.path().as_path(), self.depth + 1)?;
                Ok(Some(entries))
            }
            EntryType::File => Ok(None),
        }
    }

    pub fn render(&self, pos: Vec2, buffer: &mut Buffer, selected: bool, config: &Config) {
        let file_name = self.entry.file_name().into_string().unwrap();
        let suffix = self
            .entry
            .path()
            .extension()
            .map(|x| x.to_str().unwrap().to_string())
            .unwrap_or("".to_string());

        match self.entry_type {
            EntryType::Dir => {
                if selected {
                    render!(buffer, pos => [ "  ".repeat(self.depth - 1), " > ", config.folder, file_name.blue(), "/ <" ]);
                } else {
                    render!(buffer, pos => [ "  ".repeat(self.depth), config.folder, file_name.blue(), "/"]);
                }
            }
            EntryType::File => {
                let style = config.files.get(&suffix).cloned().unwrap_or_default();
                if selected {
                    render!(buffer, pos => [ "  ".repeat(self.depth - 1), " > ", style, file_name, " <" ]);
                } else {
                    render!(buffer, pos => [ "  ".repeat(self.depth), style, file_name ]);
                }
            }
        }
    }
}
