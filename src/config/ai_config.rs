use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::*;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// AI folder path pattern
    pub folder_pattern: String,

    /// Main script filename
    pub main_script: String,

    /// Config script filename
    pub config_script: String,

    /// Roles folder name
    pub roles_folder: String,

    /// Libs folder name
    pub libs_folder: String,

    /// Enable hot reload
    pub hot_reload: bool,

    /// Script timeout in milliseconds
    pub script_timeout_ms: u64,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            folder_pattern: AI_FOLDER_PATTERN.to_string(),
            main_script: "main.lua".to_string(),
            config_script: "config.lua".to_string(),
            roles_folder: "roles".to_string(),
            libs_folder: "libs".to_string(),
            hot_reload: true,
            script_timeout_ms: 100,
        }
    }
}