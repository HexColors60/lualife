-- Advanced AI script for faction ai_00
-- Demonstrates autonomous behavior with survival loop

-- Initialize memory if not exists
if not memory then
    memory = {
        initialized = false,
        tick_count = 0,
        last_log_tick = 0,
        targets = {}, -- unit_id -> target info
        base_pos = nil,
    }
end

-- Called once when the script is loaded
function on_init()
    log.info("AI 00 Advanced initialized!")
    memory.initialized = true
end

-- Called every tick
function on_tick()
    memory.tick_count = memory.tick_count + 1

    -- Get owned units
    local owned_units = units.list_owned()
    if not owned_units then
        return
    end

    -- Process each unit
    for _, unit_id in ipairs(owned_units) do
        local unit = units.get(unit_id)
        if unit then
            process_unit(unit_id, unit)
        end
    end

    -- Log status every 500 ticks
    if memory.tick_count - memory.last_log_tick >= 500 then
        memory.last_log_tick = memory.tick_count
        log.info(string.format("AI 00: Tick %d, Units: %d", memory.tick_count, #owned_units))
    end
end

-- Process a single unit
function process_unit(unit_id, unit)
    -- Skip if unit is busy
    if unit.action and unit.action ~= "idle" then
        return
    end

    -- Survival priority: check power level
    if unit.power and unit.power < 30 then
        -- Low power, find power mine urgently
        local mine = find_nearest_mine(unit, "power")
        if mine then
            units.mine(unit_id, mine.id)
            return
        end
    end

    -- Check body parts to determine role
    local has_mine = has_part(unit, "mine")
    local has_build = has_part(unit, "build")
    local has_work = has_part(unit, "work")

    -- Priority 1: Mine power if we can
    if has_mine then
        local mine = find_nearest_mine(unit, "power")
        if mine then
            units.mine(unit_id, mine.id)
            return
        end
    end

    -- Priority 2: Build if we have build capability
    if has_build then
        local site = find_construction_site(unit)
        if site then
            units.build(unit_id, site.id)
            return
        end
    end

    -- Priority 3: Mine other resources
    if has_mine then
        local resources = { "iron", "copper", "silicon", "crystal" }
        for _, res in ipairs(resources) do
            local mine = find_nearest_mine(unit, res)
            if mine then
                units.mine(unit_id, mine.id)
                return
            end
        end
    end

    -- Fallback: explore
    explore(unit_id, unit)
end

-- Find nearest mine of a specific type
function find_nearest_mine(unit, resource_type)
    local mines = world.get_mines()
    if not mines then
        return nil
    end

    local nearest = nil
    local min_dist = math.huge

    for _, mine in ipairs(mines) do
        if mine.resource_type == resource_type then
            local dist = distance(unit.x, unit.y, mine.x, mine.y)
            if dist < min_dist then
                min_dist = dist
                nearest = mine
            end
        end
    end

    return nearest
end

-- Find a construction site
function find_construction_site(unit)
    local sites = world.get_construction_sites()
    if not sites then
        return nil
    end

    local nearest = nil
    local min_dist = math.huge

    for _, site in ipairs(sites) do
        local dist = distance(unit.x, unit.y, site.x, site.y)
        if dist < min_dist then
            min_dist = dist
            nearest = site
        end
    end

    return nearest
end

-- Calculate distance between two points
function distance(x1, y1, x2, y2)
    local dx = x1 - x2
    local dy = y1 - y2
    return math.sqrt(dx * dx + dy * dy)
end

-- Check if a creep has a specific body part
function has_part(unit, part_type)
    if not unit.body then
        return false
    end
    for _, part in ipairs(unit.body) do
        if part == part_type then
            return true
        end
    end
    return false
end

-- Explore behavior
function explore(unit_id, unit)
    -- Move towards unexplored areas
    local dx = math.random(-2, 2)
    local dy = math.random(-2, 2)
    local target_x = math.max(0, math.min(255, unit.x + dx))
    local target_y = math.max(0, math.min(255, unit.y + dy))
    units.move_to(unit_id, target_x, target_y)
end

-- Called when a unit is spawned
function on_unit_spawned(unit_id)
    log.info("Unit spawned: " .. tostring(unit_id))
end

-- Called when a unit dies
function on_unit_died(unit_id)
    log.warn("Unit died: " .. tostring(unit_id))
    -- Clear from memory
    if memory.targets then
        memory.targets[unit_id] = nil
    end
end

-- Called when a building is completed
function on_building_completed(building_id, building_type)
    log.info("Building completed: " .. tostring(building_type))
end