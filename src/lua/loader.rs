use std::path::PathBuf;

use crate::config::AiConfig;
use crate::error::{GameError, GameResult};
use crate::factions::FactionId;
use crate::lua::LuaVm;

pub struct ScriptLoader;

impl ScriptLoader {
    pub fn load_faction_scripts(
        faction_id: FactionId,
        base_path: &str,
        config: &AiConfig,
    ) -> GameResult<LuaVm> {
        let vm = LuaVm::new(faction_id)?;

        let main_script_path = PathBuf::from(base_path).join(&config.main_script);

        if main_script_path.exists() {
            vm.load_file(&main_script_path)?;
            tracing::info!(
                "Loaded main script for faction {:?}: {:?}",
                faction_id,
                main_script_path
            );
        } else {
            tracing::warn!(
                "Main script not found for faction {:?}: {:?}",
                faction_id,
                main_script_path
            );
        }

        // Try to call on_init
        if let Err(e) = vm.call_init() {
            tracing::warn!("on_init failed for faction {:?}: {}", faction_id, e);
        }

        Ok(vm)
    }

    pub fn load_all_factions(
        faction_count: usize,
        config: &AiConfig,
    ) -> GameResult<std::collections::HashMap<FactionId, LuaVm>> {
        let mut vms = std::collections::HashMap::new();

        for i in 0..faction_count {
            let faction_id = FactionId(i as u16);
            let path = format!("ai/ai_{:02}", i);

            match Self::load_faction_scripts(faction_id, &path, config) {
                Ok(vm) => {
                    vms.insert(faction_id, vm);
                }
                Err(e) => {
                    tracing::error!("Failed to load scripts for faction {:?}: {}", faction_id, e);
                }
            }
        }

        Ok(vms)
    }
}