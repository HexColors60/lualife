use mlua::Lua;

/// Room API functions exposed to Lua
pub struct RoomApi;

impl RoomApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let rooms: mlua::Table = globals.get("rooms")?;

        // rooms.get(x, y) -> table
        rooms.set(
            "get",
            lua.create_function(|_, (_x, _y): (u32, u32)| Ok(mlua::Value::Nil))?,
        )?;

        // rooms.get_mines(room_x, room_y) -> table
        rooms.set(
            "get_mines",
            lua.create_function(|_, (_x, _y): (u32, u32)| Ok(mlua::Value::Nil))?,
        )?;

        // rooms.get_buildings(room_x, room_y) -> table
        rooms.set(
            "get_buildings",
            lua.create_function(|_, (_x, _y): (u32, u32)| Ok(mlua::Value::Nil))?,
        )?;

        // rooms.get_units(room_x, room_y) -> table
        rooms.set(
            "get_units",
            lua.create_function(|_, (_x, _y): (u32, u32)| Ok(mlua::Value::Nil))?,
        )?;

        Ok(())
    }
}
