use std::path::Path;

use crate::error::GameResult;
use crate::save::GameSnapshot;

pub struct SaveGame;

impl SaveGame {
    pub fn save(snapshot: &GameSnapshot, path: &Path) -> GameResult<()> {
        let content = ron::ser::to_string_pretty(snapshot, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, content)?;
        tracing::info!("Game saved to {:?}", path);
        Ok(())
    }

    pub fn save_to_file(snapshot: &GameSnapshot, filename: &str) -> GameResult<()> {
        let path = Path::new("saves").join(filename);
        std::fs::create_dir_all("saves")?;
        Self::save(snapshot, &path)
    }

    pub fn quick_save(snapshot: &GameSnapshot) -> GameResult<()> {
        let filename = format!("quicksave_{}.ron", snapshot.tick);
        Self::save_to_file(snapshot, &filename)
    }
}
