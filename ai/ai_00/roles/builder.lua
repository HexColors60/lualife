-- Builder role logic

local Builder = {}

Builder.role_name = "builder"

function Builder.new(unit_id)
    local self = {
        unit_id = unit_id,
        state = "idle",
        target_site = nil,
    }
    setmetatable(self, { __index = Builder })
    return self
end

function Builder:tick()
    if self.state == "idle" then
        self:find_construction_site()
    elseif self.state == "building" then
        self:do_building()
    end
end

function Builder:find_construction_site()
    local sites = build.get_construction_sites()
    if sites and #sites > 0 then
        self.target_site = sites[1]
        self.state = "building"
        build.place_construction(self.unit_id, self.target_site.type, self.target_site.x, self.target_site.y)
    end
end

function Builder:do_building()
    -- Check if construction is complete
    if self.target_site then
        -- Continue building
    else
        self.state = "idle"
    end
end

return Builder