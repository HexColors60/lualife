use mlua::Lua;

pub struct LuaSandbox;

impl LuaSandbox {
    pub fn setup(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();

        // Remove dangerous standard library functions
        let nil = mlua::Value::Nil;

        // File I/O
        globals.set("dofile", nil.clone())?;
        globals.set("loadfile", nil.clone())?;

        // Dynamic code loading
        globals.set("load", nil.clone())?;
        globals.set("loadstring", nil.clone())?;

        // OS access (if present)
        if globals.contains_key("os")? {
            let os: mlua::Table = globals.get("os")?;
            os.set("execute", nil.clone())?;
            os.set("exit", nil.clone())?;
            os.set("remove", nil.clone())?;
            os.set("rename", nil.clone())?;
            os.set("getenv", nil.clone())?;
        }

        // Debug library restrictions
        if globals.contains_key("debug")? {
            let debug: mlua::Table = globals.get("debug")?;
            debug.set("getregistry", nil.clone())?;
            debug.set("getupvalue", nil.clone())?;
            debug.set("setupvalue", nil.clone())?;
        }

        Ok(())
    }
}