use mlua::Lua;

/// Economy API functions exposed to Lua
pub struct EconomyApi;

impl EconomyApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let economy: mlua::Table = globals.get("economy")?;

        // economy.get_stockpile() -> table
        economy.set(
            "get_stockpile",
            lua.create_function(|_, ()| Ok(mlua::Value::Nil))?,
        )?;

        // economy.get_resource(resource_type) -> number
        economy.set(
            "get_resource",
            lua.create_function(|_, _resource: String| Ok(0u32))?,
        )?;

        // economy.get_capacity(resource_type) -> number
        economy.set(
            "get_capacity",
            lua.create_function(|_, _resource: String| Ok(u32::MAX))?,
        )?;

        Ok(())
    }
}