use mlua::Lua;

/// Combat API functions exposed to Lua
pub struct CombatApi;

impl CombatApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let combat: mlua::Table = globals.get("combat")?;

        // combat.get_enemies_in_range(x, y, range) -> table
        combat.set(
            "get_enemies_in_range",
            lua.create_function(|_, (_x, _y, _range): (i32, i32, f32)| Ok(mlua::Value::Nil))?,
        )?;

        // combat.get_friendly_in_range(x, y, range) -> table
        combat.set(
            "get_friendly_in_range",
            lua.create_function(|_, (_x, _y, _range): (i32, i32, f32)| Ok(mlua::Value::Nil))?,
        )?;

        Ok(())
    }
}
