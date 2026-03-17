-- Configuration for AI 00

return {
    name = "AI_00",
    version = "1.0.0",
    author = "lualife",
    
    -- Spawn configuration
    initial_creeps = {
        { role = "harvester", count = 3 },
        { role = "builder", count = 1 },
    },
    
    -- Behavior priorities
    priorities = {
        power_mining = 1.0,
        building = 0.5,
        expansion = 0.3,
        combat = 0.2,
    },
    
    -- Thresholds
    min_power_reserve = 500,
    min_harvesters = 2,
    max_creeps = 50,
}