use mlua::Lua;

use crate::resources::ResourceType;
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

        // economy.get_global_levels() -> table
        // Returns { power = { current, max, ratio }, iron = { ... }, ... }
        economy.set(
            "get_global_levels",
            lua.create_function(|lua, ()| {
                let result = lua.create_table()?;
                
                // This is a placeholder - actual implementation would access GlobalResourceLevels
                // For now, return mock data
                let resources = ["power", "iron", "copper", "silicon", "crystal", "carbon", "stone", "sulfur", "water", "biomass"];
                for resource in resources {
                    let info = lua.create_table()?;
                    info.set("current", 1000u64)?;
                    info.set("max", 10000u64)?;
                    info.set("ratio", 0.1f32)?;
                    result.set(resource, info)?;
                }
                
                Ok(result)
            })?,
        )?;

        // economy.get_scarcity_status(resource_type) -> string
        // Returns "critical", "scarce", "normal", or "abundant"
        economy.set(
            "get_scarcity_status",
            lua.create_function(|_, resource: String| {
                let _resource_type = parse_resource(&resource);
                // Placeholder - actual implementation would check GlobalResourceLevels
                Ok("normal".to_string())
            })?,
        )?;

        // economy.get_scarcity_thresholds() -> table
        // Returns { critical = 0.1, scarce = 0.3, abundant = 0.8 }
        economy.set(
            "get_scarcity_thresholds",
            lua.create_function(|lua, ()| {
                let result = lua.create_table()?;
                result.set("critical", 0.1f32)?;
                result.set("scarce", 0.3f32)?;
                result.set("abundant", 0.8f32)?;
                Ok(result)
            })?,
        )?;

        // economy.is_resource_critical(resource_type) -> boolean
        economy.set(
            "is_resource_critical",
            lua.create_function(|_, _resource: String| {
                // Placeholder - actual implementation would check GlobalResourceLevels
                Ok(false)
            })?,
        )?;

        // economy.is_resource_scarce(resource_type) -> boolean
        economy.set(
            "is_resource_scarce",
            lua.create_function(|_, _resource: String| {
                // Placeholder - actual implementation would check GlobalResourceLevels
                Ok(false)
            })?,
        )?;

        Ok(())
    }
}

fn parse_resource(s: &str) -> Option<ResourceType> {
    match s.to_lowercase().as_str() {
        "power" => Some(ResourceType::Power),
        "iron" => Some(ResourceType::Iron),
        "copper" => Some(ResourceType::Copper),
        "silicon" => Some(ResourceType::Silicon),
        "crystal" => Some(ResourceType::Crystal),
        "carbon" => Some(ResourceType::Carbon),
        "stone" => Some(ResourceType::Stone),
        "sulfur" => Some(ResourceType::Sulfur),
        "water" => Some(ResourceType::Water),
        "biomass" => Some(ResourceType::Biomass),
        _ => None,
    }
}