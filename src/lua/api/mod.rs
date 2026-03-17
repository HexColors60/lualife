mod build_api;
mod combat_api;
mod debug_api;
mod economy_api;
mod log_api;
mod room_api;
mod unit_api;
mod world_api;

pub use build_api::*;
pub use combat_api::*;
pub use debug_api::*;
pub use economy_api::*;
pub use log_api::*;
pub use room_api::*;
pub use unit_api::*;
pub use world_api::*;

use mlua::Lua;

use crate::lua::LuaVm;

pub struct LuaApi;

impl LuaApi {
    pub fn register_all(vm: &mut LuaVm) -> crate::error::GameResult<()> {
        let lua = vm.lua();

        // Create global tables for API namespaces
        let globals = lua.globals();

        // World API
        let world = lua.create_table()?;
        globals.set("world", world)?;

        // Units API
        let units = lua.create_table()?;
        globals.set("units", units)?;

        // Rooms API
        let rooms = lua.create_table()?;
        globals.set("rooms", rooms)?;

        // Economy API
        let economy = lua.create_table()?;
        globals.set("economy", economy)?;

        // Build API
        let build = lua.create_table()?;
        globals.set("build", build)?;

        // Combat API
        let combat = lua.create_table()?;
        globals.set("combat", combat)?;

        // Log API
        let log = lua.create_table()?;
        globals.set("log", log)?;

        // Debug API (only in debug mode)
        #[cfg(debug_assertions)]
        {
            let debug = lua.create_table()?;
            globals.set("debug", debug)?;
        }

        Ok(())
    }
}