use std::path::Path;

use mlua::Lua;

use crate::error::GameResult;
use crate::factions::FactionId;

pub struct LuaVm {
    lua: Lua,
    faction_id: FactionId,
    budget: usize,
}

impl LuaVm {
    pub fn new(faction_id: FactionId) -> GameResult<Self> {
        let lua = Lua::new();

        // Set up sandboxed environment
        {
            let globals = lua.globals();

            // Disable dangerous functions
            let nil = mlua::Value::Nil;
            globals.set("dofile", nil.clone())?;
            globals.set("loadfile", nil.clone())?;
            globals.set("load", nil)?;
        }

        Ok(Self {
            lua,
            faction_id,
            budget: 1_000_000,
        })
    }

    pub fn load_script(&self, script: &str) -> GameResult<()> {
        self.lua.load(script).exec()?;
        Ok(())
    }

    pub fn load_file(&self, path: &Path) -> GameResult<()> {
        let content = std::fs::read_to_string(path)?;
        self.load_script(&content)
    }

    pub fn call_hook(&self, hook_name: &str) -> GameResult<()> {
        let globals = self.lua.globals();

        let callback: Option<mlua::Function> = globals.get(hook_name)?;
        if let Some(callback) = callback {
            callback.call::<_, ()>(())?;
        }

        Ok(())
    }

    pub fn call_tick(&self) -> GameResult<()> {
        self.call_hook("on_tick")
    }

    pub fn call_init(&self) -> GameResult<()> {
        self.call_hook("on_init")
    }

    pub fn set_budget(&mut self, budget: usize) {
        self.budget = budget;
    }

    pub fn faction_id(&self) -> FactionId {
        self.faction_id
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    pub fn lua_mut(&mut self) -> &mut Lua {
        &mut self.lua
    }
}
