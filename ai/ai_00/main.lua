-- Main AI script for faction ai_00
-- This is a sample AI that demonstrates basic behavior

-- Initialize memory if not exists
if not memory then
    memory = {
        initialized = false,
        tick_count = 0,
        harvesters = {},
        builders = {},
    }
end

-- Called once when the script is loaded
function on_init()
    log.info("AI 00 initialized!")
    memory.initialized = true
end

-- Called every tick
function on_tick()
    memory.tick_count = memory.tick_count + 1
    
    -- Log every 100 ticks
    if memory.tick_count % 100 == 0 then
        log.info("Tick " .. memory.tick_count)
    end
    
    -- Get owned units
    local units = units.list_owned()
    if units then
        for _, unit_id in ipairs(units) do
            local unit = units.get(unit_id)
            if unit then
                -- Simple behavior: if unit is idle, assign a task
                if unit.action == "idle" then
                    assign_task(unit_id, unit)
                end
            end
        end
    end
end

-- Assign a task to an idle unit
function assign_task(unit_id, unit)
    -- Check if unit has mining capability
    if unit.body and has_part(unit.body, "mine") then
        -- Find nearest power mine
        local mines = rooms.get_mines(unit.room_x, unit.room_y)
        if mines then
            for _, mine in ipairs(mines) do
                if mine.resource_type == "power" then
                    units.mine(unit_id, mine.id)
                    log.debug("Unit " .. unit_id .. " assigned to mine power")
                    return
                end
            end
        end
    end
    
    -- Default: move randomly
    local dx = math.random(-1, 1)
    local dy = math.random(-1, 1)
    units.move_to(unit_id, unit.x + dx, unit.y + dy)
end

-- Check if a creep body has a specific part
function has_part(body, part_type)
    for _, part in ipairs(body) do
        if part == part_type then
            return true
        end
    end
    return false
end

-- Called when a unit is spawned
function on_unit_spawned(unit_id)
    log.info("Unit spawned: " .. unit_id)
end

-- Called when a unit dies
function on_unit_died(unit_id)
    log.warn("Unit died: " .. unit_id)
end