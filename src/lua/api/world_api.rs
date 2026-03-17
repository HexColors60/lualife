use mlua::Lua;

/// World API functions exposed to Lua
pub struct WorldApi;

impl WorldApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let world: mlua::Table = globals.get("world")?;

        // world.get_tick() -> number
        world.set(
            "get_tick",
            lua.create_function(|_, ()| Ok(0u64))?, // Placeholder
        )?;

        // world.get_time() -> number
        world.set(
            "get_time",
            lua.create_function(|_, ()| Ok(0u64))?,
        )?;

        Ok(())
    }
}