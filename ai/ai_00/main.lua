-- Advanced AI script for faction ai_00
-- Demonstrates autonomous behavior with survival loop and scarcity awareness

-- Initialize memory if not exists
if not memory then
    memory = {
        initialized = false,
        tick_count = 0,
        last_log_tick = 0,
        targets = {}, -- unit_id -> target info
        base_pos = nil,
        scarcity_check_tick = 0,
        priority_resources = nil,
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

    -- Update scarcity-aware priorities periodically
    if memory.tick_count - memory.scarcity_check_tick >= 100 then
        memory.scarcity_check_tick = memory.tick_count
        update_resource_priorities()
    end

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

-- Update resource priorities based on scarcity
function update_resource_priorities()
    -- Check scarcity status for each resource
    local resources = { "power", "iron", "copper", "silicon", "crystal", "carbon" }
    local priorities = {}
    
    for _, res in ipairs(resources) do
        local status = economy.get_scarcity_status(res)
        local priority = 1 -- normal priority
        
        if status == "critical" then
            priority = 3 -- highest priority
        elseif status == "scarce" then
            priority = 2 -- high priority
        elseif status == "abundant" then
            priority = 0 -- low priority, we have plenty
        end
        
        priorities[res] = priority
    end
    
    -- Sort resources by priority
    memory.priority_resources = {}
    for _, res in ipairs(resources) do
        table.insert(memory.priority_resources, { resource = res, priority = priorities[res] })
    end
    table.sort(memory.priority_resources, function(a, b) return a.priority > b.priority end)
    
    -- Log critical resources
    for _, item in ipairs(memory.priority_resources) do
        if item.priority >= 2 then
            log.info(string.format("AI 00: Resource %s is scarce (priority %d)", item.resource, item.priority))
        end
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

    -- Priority 1: Mine based on scarcity-aware priorities
    if has_mine then
        -- Always check power first for survival
        if economy.is_resource_critical("power") or economy.is_resource_scarce("power") then
            local mine = find_nearest_mine(unit, "power")
            if mine then
                units.mine(unit_id, mine.id)
                return
            end
        end
        
        -- Mine based on priority list
        if memory.priority_resources then
            for _, item in ipairs(memory.priority_resources) do
                local mine = find_nearest_mine(unit, item.resource)
                if mine then
                    units.mine(unit_id, mine.id)
                    return
                end
            end
        else
            -- Fallback: mine power
            local mine = find_nearest_mine(unit, "power")
            if mine then
                units.mine(unit_id, mine.id)
                return
            end
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

-- Helper: check if unit has a specific body part
function has_part(unit, part_name)
    if not unit.body_parts then
        return false
    end
    for _, part in ipairs(unit.body_parts) do
        if part == part_name then
            return true
        end
    end
    return false
end

-- Helper: calculate distance between two points
function distance(x1, y1, x2, y2)
    local dx = x2 - x1
    local dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
end

-- Explore behavior
function explore(unit_id, unit)
    -- Random exploration
    local dx = math.random(-5, 5)
    local dy = math.random(-5, 5)
    local target_x = unit.x + dx
    local target_y = unit.y + dy
    
    -- Clamp to world bounds
    target_x = math.max(0, math.min(255, target_x))
    target_y = math.max(0, math.min(255, target_y))
    
    units.move_to(unit_id, target_x, target_y)
end