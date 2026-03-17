use mlua::Lua;

/// Log API functions exposed to Lua
pub struct LogApi;

impl LogApi {
    pub fn register(lua: &Lua) -> mlua::Result<()> {
        let globals = lua.globals();
        let log: mlua::Table = globals.get("log")?;

        // log.info(...)
        log.set(
            "info",
            lua.create_function(|_, args: mlua::MultiValue| {
                let msg: String = args
                    .iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                tracing::info!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        // log.warn(...)
        log.set(
            "warn",
            lua.create_function(|_, args: mlua::MultiValue| {
                let msg: String = args
                    .iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                tracing::warn!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        // log.error(...)
        log.set(
            "error",
            lua.create_function(|_, args: mlua::MultiValue| {
                let msg: String = args
                    .iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                tracing::error!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        // log.debug(...)
        log.set(
            "debug",
            lua.create_function(|_, args: mlua::MultiValue| {
                let msg: String = args
                    .iter()
                    .map(|v| format!("{:?}", v))
                    .collect::<Vec<_>>()
                    .join(" ");
                tracing::debug!("[Lua] {}", msg);
                Ok(())
            })?,
        )?;

        Ok(())
    }
}
