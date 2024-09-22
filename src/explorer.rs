use std::path::{Path, PathBuf};

use ascii_forge::prelude::*;

use crate::{
    config::Config,
    dir_items::dir_items,
    entry::{Entry, EntryType},
    events::ExplorerEvent,
};

pub struct Explorer<'a> {
    path: String,
    entries: Vec<Entry>,
    selected: usize,
    config: &'a Config,
    scroll: usize,
}

impl<'a> Explorer<'a> {
    pub fn new(path: &str, config: &'a Config) -> anyhow::Result<Self> {
        Ok(Self {
            path: path.to_string(),
            entries: dir_items(Path::new(path), 1)?,
            selected: 0,
            config,
            scroll: 0,
        })
    }

    pub fn set_selected(&mut self, idx: usize) {
        let idx = self.scroll + idx;
        if idx >= self.entries.len() {
            return;
        }

        self.selected = idx;
    }

    pub fn handle_event(&mut self, event: ExplorerEvent) -> anyhow::Result<()> {
        match event {
            ExplorerEvent::Scroll(d) => {
                if d < 0 {
                    self.scroll_up(d.unsigned_abs() as usize);
                } else {
                    self.scroll_down(d.unsigned_abs() as usize);
                }
            }
            ExplorerEvent::Move(d) => {
                if d < 0 {
                    self.back(d.unsigned_abs() as usize);
                } else {
                    self.advance(d.unsigned_abs() as usize);
                }
            }
            ExplorerEvent::Expand => self.expand()?,
            ExplorerEvent::Collapse => self.collapse(),
            _ => {
                unimplemented!("event {event:?} should not be handled by explorer")
            }
        }
        Ok(())
    }

    pub fn scroll_up(&mut self, dist: usize) {
        if self.scroll < dist {
            self.scroll = 0;
        } else {
            self.scroll -= dist;
        }
    }

    pub fn scroll_down(&mut self, dist: usize) {
        if self.scroll + dist >= self.entries.len() {
            self.scroll = self.entries.len() - 1;
        } else {
            self.scroll += dist;
        }
    }

    pub fn back(&mut self, dist: usize) {
        if self.selected < dist {
            self.selected = 0;
        } else {
            self.selected -= dist;
        }
    }

    pub fn advance(&mut self, dist: usize) {
        self.selected += dist;
        self.selected = self.selected.min(self.entries.len() - 1);
    }

    pub fn expand(&mut self) -> anyhow::Result<()> {
        let new_entries = self.entries[self.selected].expand()?;

        if let Some(new_entries) = new_entries {
            let mut v = self.entries.split_off(self.selected + 1);
            self.entries.extend(new_entries);
            self.entries.append(&mut v);
        }

        Ok(())
    }

    pub fn collapse(&mut self) {
        if !self.entries[self.selected].expanded {
            return;
        }
        self.entries[self.selected].expanded = false;

        let depth = self.entries[self.selected].depth;

        while self.entries[self.selected + 1].depth > depth {
            self.entries.remove(self.selected + 1);
            if self.entries.len() <= self.selected + 1 {
                break;
            }
        }
    }

    pub fn toggle(&mut self) -> anyhow::Result<()> {
        match self.entries[self.selected].expanded {
            true => self.collapse(),
            false => self.expand()?,
        }

        Ok(())
    }

    pub fn is_file(&self) -> bool {
        match self.entries[self.selected].entry_type {
            EntryType::Dir => false,
            EntryType::File => true,
        }
    }

    pub fn focused_path(&self) -> PathBuf {
        self.entries[self.selected].entry.path()
    }

    pub fn refresh(self) -> anyhow::Result<Self> {
        let mut new = Explorer::new(&self.path, self.config)?;
        for (i, entry) in self.entries.into_iter().enumerate() {
            if entry.expanded {
                new.set_selected(i);
                new.expand()?;
            }
        }
        new.selected = self.selected;
        Ok(new)
    }
}

impl<'a> Render for Explorer<'a> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        let start_line = self.scroll;
        let max_lines = (start_line + buffer.size().y as usize - 6).min(self.entries.len());

        render!(buffer, loc => [ "./" ]);

        for i in start_line..max_lines {
            loc.y += 1;

            let entry = &self.entries[i];

            entry.render(loc, buffer, i == self.selected, self.config);
        }
        loc
    }
}
