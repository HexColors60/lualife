use mlua::Lua;

/// Debug API functions exposed to Lua (only in debug mode)
pub struct DebugApi;

impl DebugApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let debug: mlua::Table = globals.get("debug")?;

        // debug.inspect(value) -> string
        debug.set(
            "inspect",
            lua.create_function(|lua, value: mlua::Value| {
                let s = format!("{:?}", value);
                lua.create_string(&s)
            })?,
        )?;

        // debug.get_entity_count() -> number
        debug.set("get_entity_count", lua.create_function(|_, ()| Ok(0u64))?)?;

        Ok(())
    }
}
