use std::path::Path;

use crate::error::GameResult;
use crate::save::GameSnapshot;

pub struct LoadGame;

impl LoadGame {
    pub fn load(path: &Path) -> GameResult<GameSnapshot> {
        let content = std::fs::read_to_string(path)?;
        let snapshot: GameSnapshot = ron::from_str(&content).map_err(|e| crate::error::GameError::Serialization(e.into()))?;
        tracing::info!("Game loaded from {:?}", path);
        Ok(snapshot)
    }

    pub fn load_from_file(filename: &str) -> GameResult<GameSnapshot> {
        let path = Path::new("saves").join(filename);
        Self::load(&path)
    }

    pub fn list_saves() -> GameResult<Vec<String>> {
        let mut saves = Vec::new();

        if let Ok(entries) = std::fs::read_dir("saves") {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".ron") {
                            saves.push(name.to_string());
                        }
                    }
                }
            }
        }

        saves.sort();
        saves.reverse(); // Most recent first
        Ok(saves)
    }
}