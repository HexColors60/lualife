-- Aggressive AI script for faction ai_01
-- Focuses on combat and expansion

if not memory then
    memory = {
        initialized = false,
        tick_count = 0,
        targets = {},
        enemies_seen = {},
    }
end

function on_init()
    log.info("AI 01 Aggressive initialized!")
    memory.initialized = true
end

function on_tick()
    memory.tick_count = memory.tick_count + 1

    local owned_units = units.list_owned()
    if not owned_units then
        return
    end

    for _, unit_id in ipairs(owned_units) do
        local unit = units.get(unit_id)
        if unit then
            process_unit(unit_id, unit)
        end
    end
end

function process_unit(unit_id, unit)
    if unit.action and unit.action ~= "idle" then
        return
    end

    -- Survival check
    if unit.power and unit.power < 20 then
        local mine = find_nearest_mine(unit, "power")
        if mine then
            units.mine(unit_id, mine.id)
            return
        end
    end

    -- Check for combat capability
    local has_fight = has_part(unit, "fight")
    if has_fight then
        -- Look for enemies
        local enemy = find_nearest_enemy(unit)
        if enemy then
            units.attack(unit_id, enemy.id)
            return
        end
    end

    -- Default: mine resources
    if has_part(unit, "mine") then
        local mine = find_nearest_mine(unit, "power")
        if mine then
            units.mine(unit_id, mine.id)
            return
        end
    end

    -- Patrol behavior
    patrol(unit_id, unit)
end

function find_nearest_mine(unit, resource_type)
    local mines = world.get_mines()
    if not mines then return nil end

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

function find_nearest_enemy(unit)
    local enemies = world.get_enemies()
    if not enemies then return nil end

    local nearest = nil
    local min_dist = math.huge

    for _, enemy in ipairs(enemies) do
        local dist = distance(unit.x, unit.y, enemy.x, enemy.y)
        if dist < min_dist and dist < 20 then
            min_dist = dist
            nearest = enemy
        end
    end

    return nearest
end

function distance(x1, y1, x2, y2)
    local dx = x1 - x2
    local dy = y1 - y2
    return math.sqrt(dx * dx + dy * dy)
end

function has_part(unit, part_type)
    if not unit.body then return false end
    for _, part in ipairs(unit.body) do
        if part == part_type then return true end
    end
    return false
end

function patrol(unit_id, unit)
    -- Move in a pattern
    local angle = (memory.tick_count * 0.1) % (2 * math.pi)
    local dx = math.cos(angle) * 3
    local dy = math.sin(angle) * 3
    local target_x = math.max(0, math.min(255, unit.x + math.floor(dx)))
    local target_y = math.max(0, math.min(255, unit.y + math.floor(dy)))
    units.move_to(unit_id, target_x, target_y)
end

function on_unit_spawned(unit_id)
    log.info("AI 01: Unit " .. tostring(unit_id) .. " ready for combat!")
end

function on_unit_died(unit_id)
    log.warn("AI 01: Unit " .. tostring(unit_id) .. " fell in battle!")
end