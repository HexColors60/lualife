-- Harvester role logic

local Harvester = {}

Harvester.role_name = "harvester"

function Harvester.new(unit_id)
    local self = {
        unit_id = unit_id,
        state = "idle",
        target_mine = nil,
        target_storage = nil,
    }
    setmetatable(self, { __index = Harvester })
    return self
end

function Harvester:tick()
    if self.state == "idle" then
        self:find_mine()
    elseif self.state == "mining" then
        self:do_mining()
    elseif self.state == "returning" then
        self:return_resources()
    end
end

function Harvester:find_mine()
    -- Find nearest power mine
    local mines = rooms.get_mines(0, 0) -- TODO: get current room
    if mines then
        for _, mine in ipairs(mines) do
            if mine.resource_type == "power" then
                self.target_mine = mine.id
                self.state = "mining"
                units.mine(self.unit_id, mine.id)
                return
            end
        end
    end
end

function Harvester:do_mining()
    -- Check if inventory is full
    local unit = units.get(self.unit_id)
    if unit and unit.carry >= unit.carry_capacity then
        self.state = "returning"
    end
end

function Harvester:return_resources()
    -- Find nearest storage
    -- TODO: implement storage finding
    self.state = "idle"
end

return Harvester