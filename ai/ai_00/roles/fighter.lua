-- Fighter role logic

local Fighter = {}

Fighter.role_name = "fighter"

function Fighter.new(unit_id)
    local self = {
        unit_id = unit_id,
        state = "patrol",
        target_enemy = nil,
        patrol_point = nil,
    }
    setmetatable(self, { __index = Fighter })
    return self
end

function Fighter:tick()
    if self.state == "patrol" then
        self:patrol()
    elseif self.state == "attack" then
        self:attack_enemy()
    end
end

function Fighter:patrol()
    -- Look for enemies
    local unit = units.get(self.unit_id)
    if unit then
        local enemies = combat.get_enemies_in_range(unit.x, unit.y, 5)
        if enemies and #enemies > 0 then
            self.target_enemy = enemies[1]
            self.state = "attack"
            return
        end
    end
    
    -- Move to patrol point
    if self.patrol_point then
        units.move_to(self.unit_id, self.patrol_point.x, self.patrol_point.y)
    end
end

function Fighter:attack_enemy()
    if self.target_enemy then
        units.attack(self.unit_id, self.target_enemy.id)
    else
        self.state = "patrol"
    end
end

return Fighter