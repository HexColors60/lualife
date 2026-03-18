use mlua::Lua;

/// Unit API functions exposed to Lua
pub struct UnitApi;

impl UnitApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let units: mlua::Table = globals.get("units")?;

        // units.list_owned() -> table
        units.set(
            "list_owned",
            lua.create_function(|_, ()| Ok(mlua::Value::Nil))?,
        )?;

        // units.get(id) -> table
        units.set(
            "get",
            lua.create_function(|_, _id: u32| Ok(mlua::Value::Nil))?,
        )?;

        // units.move_to(id, x, y) -> boolean
        units.set(
            "move_to",
            lua.create_function(|_, (_id, _x, _y): (u32, i32, i32)| Ok(true))?,
        )?;

        // units.mine(id, mine_id) -> boolean
        units.set(
            "mine",
            lua.create_function(|_, (_id, _mine_id): (u32, u32)| Ok(true))?,
        )?;

        // units.attack(id, target_id) -> boolean
        units.set(
            "attack",
            lua.create_function(|_, (_id, _target_id): (u32, u32)| Ok(true))?,
        )?;

        // units.transfer(id, target_id, resource, amount) -> boolean
        units.set(
            "transfer",
            lua.create_function(
                |_, (_id, _target, _resource, _amount): (u32, u32, String, u32)| Ok(true),
            )?,
        )?;

        // units.pack_all(id, target_id) -> boolean
        units.set(
            "pack_all",
            lua.create_function(|_, (_id, _target_id): (u32, u32)| Ok(true))?,
        )?;

        Ok(())
    }
}
