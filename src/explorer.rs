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

    pub fn selection_valid(&mut self, mut idx: usize) -> bool {
        idx += self.scroll;
        idx < self.entries.len()
    }

    pub fn set_selected(&mut self, mut idx: usize) {
        if !self.selection_valid(idx) {
            return;
        }
        idx += self.scroll;

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
        if self.selected < self.scroll {
            self.scroll = self.selected;
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

    pub fn find(&self, path: &Path) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .find(|x| x.1.path == path)
            .map(|x| x.0)
    }

    pub fn focused_path(&self) -> PathBuf {
        self.entries[self.selected].path.clone()
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        let selected_path = self.entries[self.selected].path.clone();
        let scroll = self.scroll;

        let entries = self.entries.clone();
        let expanded = entries
            .into_iter()
            .filter(|x| x.expanded)
            .map(|x| x.path)
            .collect::<Vec<PathBuf>>();

        *self = Self::new(&self.path, self.config)?;

        for item in &expanded {
            if let Some(idx) = self.find(item) {
                self.set_selected(idx);
                self.expand()?;
            }
        }

        match self.find(&selected_path) {
            Some(i) => {
                self.set_selected(i);
                self.scroll = scroll
            }
            None => self.set_selected(0),
        }

        Ok(())
    }
}

impl<'a> Render for Explorer<'a> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        let start_line = self.scroll;
        let max_lines = (start_line + buffer.size().y as usize - 7).min(self.entries.len());

        render!(buffer, loc => [ "./" ]);

        for i in start_line..max_lines {
            loc.y += 1;

            let entry = &self.entries[i];

            entry.render(loc, buffer, i == self.selected, self.config);
        }
        loc
    }
}
