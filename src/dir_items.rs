use std::path::Path;

use crate::entry::Entry;

pub fn dir_items(path: &Path, depth: usize) -> anyhow::Result<Vec<Entry>> {
    let mut entries = vec![];
    for entry in path.read_dir()? {
        entries.push(Entry::new(entry?, depth))
    }
    Ok(entries)
}
