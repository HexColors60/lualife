-- Utility functions for AI scripts

local util = {}

-- Calculate distance between two points
function util.distance(x1, y1, x2, y2)
    local dx = x2 - x1
    local dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
end

-- Calculate Manhattan distance
function util.manhattan_distance(x1, y1, x2, y2)
    return math.abs(x2 - x1) + math.abs(y2 - y1)
end

-- Find the nearest entity from a list
function util.find_nearest(x, y, entities)
    local nearest = nil
    local min_dist = math.huge
    
    for _, entity in ipairs(entities) do
        local dist = util.distance(x, y, entity.x, entity.y)
        if dist < min_dist then
            min_dist = dist
            nearest = entity
        end
    end
    
    return nearest, min_dist
end

-- Check if a position is valid
function util.is_valid_position(x, y)
    return x >= 0 and x < 256 and y >= 0 and y < 256
end

-- Clamp a value between min and max
function util.clamp(value, min, max)
    return math.max(min, math.min(max, value))
end

-- Shuffle a table
function util.shuffle(tbl)
    for i = #tbl, 2, -1 do
        local j = math.random(i)
        tbl[i], tbl[j] = tbl[j], tbl[i]
    end
    return tbl
end

return util