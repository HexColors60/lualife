use std::collections::HashMap;
use std::path::PathBuf;

use crate::factions::FactionId;

pub struct HotReloadWatcher {
    watched_paths: HashMap<FactionId, Vec<PathBuf>>,
    last_modified: HashMap<PathBuf, std::time::SystemTime>,
}

impl Default for HotReloadWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl HotReloadWatcher {
    pub fn new() -> Self {
        Self {
            watched_paths: HashMap::new(),
            last_modified: HashMap::new(),
        }
    }

    pub fn watch_faction(&mut self, faction_id: FactionId, path: PathBuf) {
        self.watched_paths
            .entry(faction_id)
            .or_default()
            .push(path.clone());

        if let Ok(metadata) = std::fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                self.last_modified.insert(path, modified);
            }
        }
    }

    pub fn check_changes(&mut self) -> Vec<(FactionId, PathBuf)> {
        let mut changed = Vec::new();

        for (faction_id, paths) in &self.watched_paths {
            for path in paths {
                if let Ok(metadata) = std::fs::metadata(path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Some(&last) = self.last_modified.get(path) {
                            if modified > last {
                                changed.push((*faction_id, path.clone()));
                                self.last_modified.insert(path.clone(), modified);
                            }
                        }
                    }
                }
            }
        }

        changed
    }
}
