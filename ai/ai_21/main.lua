-- Nomad AI for faction ai_21
-- Constantly moves

if not memory then
    memory = {
        initialized = false,
        tick_count = 0,
        last_log_tick = 0,
        priority_resources = {"power", "biomass"},
        aggression = 0.3,
    }
end

function on_init()
    log.info("AI 21 Nomad initialized!")
    memory.initialized = true
end

function on_tick()
    memory.tick_count = memory.tick_count + 1
    
    local owned_units = units.list_owned()
    if not owned_units then return end
    
    for _, unit_id in ipairs(owned_units) do
        local unit = units.get(unit_id)
        if unit then process_unit(unit_id, unit) end
    end
    
    if memory.tick_count - memory.last_log_tick >= 500 then
        memory.last_log_tick = memory.tick_count
        log.info(string.format("AI 21 Nomad: Tick %%d, Units: %%d", memory.tick_count, #owned_units))
    end
end

function process_unit(unit_id, unit)
    if unit.action and unit.action ~= "idle" then return end
    
    -- Survival check
    if unit.power and unit.power < 25 then
        local mine = find_nearest_mine(unit, "power")
        if mine then units.mine(unit_id, mine.id) return end
    end
    
    -- Nomad behavior
    -- Nomad: wander
    if memory.tick_count % 30 == 0 then explore(unit_id, unit) return end
    
    -- Default: mine priority resources
    if has_part(unit, "mine") then
        for _, res in ipairs(memory.priority_resources) do
            local mine = find_nearest_mine(unit, res)
            if mine then units.mine(unit_id, mine.id) return end
        end
    end
    
    -- Fallback
    local mine = find_nearest_mine(unit, "power")
    if mine then units.mine(unit_id, mine.id) return end
    
    explore(unit_id, unit)
end

function find_nearest_mine(unit, resource_type)
    local mines = world.get_mines()
    if not mines then return nil end
    local nearest, min_dist = nil, math.huge
    for _, mine in ipairs(mines) do
        if mine.resource_type == resource_type then
            local dist = distance(unit.x, unit.y, mine.x, mine.y)
            if dist < min_dist then nearest, min_dist = mine, dist end
        end
    end
    return nearest
end

function find_nearest_enemy(unit)
    local enemies = world.get_enemies()
    if not enemies then return nil end
    local nearest, min_dist = nil, math.huge
    for _, enemy in ipairs(enemies) do
        local dist = distance(unit.x, unit.y, enemy.x, enemy.y)
        if dist < min_dist then nearest, min_dist = enemy, dist end
    end
    return nearest
end

function distance(x1, y1, x2, y2)
    return math.sqrt((x1-x2)^2 + (y1-y2)^2)
end

function has_part(unit, part_type)
    if not unit.body then return false end
    for _, part in ipairs(unit.body) do
        if part == part_type then return true end
    end
    return false
end

function explore(unit_id, unit)
    units.move_to(unit_id, 
        math.max(0, math.min(255, unit.x + math.random(-3, 3))),
        math.max(0, math.min(255, unit.y + math.random(-3, 3))))
end

function on_unit_spawned(unit_id)
    log.info("AI 21: Unit spawned: " .. tostring(unit_id))
end

function on_unit_died(unit_id)
    log.warn("AI 21: Unit died: " .. tostring(unit_id))
end
