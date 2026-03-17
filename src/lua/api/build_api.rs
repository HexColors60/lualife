use mlua::Lua;

/// Build API functions exposed to Lua
pub struct BuildApi;

impl BuildApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let build: mlua::Table = globals.get("build")?;

        // build.place_construction(unit_id, building_type, x, y) -> boolean
        build.set(
            "place_construction",
            lua.create_function(
                |_, (_unit_id, _building_type, _x, _y): (u32, String, i32, i32)| Ok(true),
            )?,
        )?;

        // build.get_construction_sites() -> table
        build.set(
            "get_construction_sites",
            lua.create_function(|_, ()| Ok(mlua::Value::Nil))?,
        )?;

        Ok(())
    }
}