#!/usr/bin/env lua
-- Standalone Evolution Simulation Runner
-- Run with: lua standalone.lua

local evolution = {
    creatures = {},
    food = {},
    generation = 1,
    tick = 0,
    stats = {
        total_born = 0,
        total_died = 0,
        avg_speed = 0,
        avg_size = 0,
        avg_sense = 0,
    },
    history = {},
    running = true,
}

local WORLD_WIDTH = 100
local WORLD_HEIGHT = 100
local INITIAL_CREATURES = 20
local INITIAL_FOOD = 50
local FOOD_SPAWN_RATE = 0.1
local MAX_FOOD = 100
local TICKS_PER_GENERATION = 500
local MUTATION_RATE = 0.2
local MUTATION_STRENGTH = 0.3

local DIET_HERBIVORE = 1
local DIET_CARNIVORE = 2
local DIET_OMNIVORE = 3

local function clamp(val, min, max)
    return math.max(min, math.min(max, val))
end

local function distance(x1, y1, x2, y2)
    local dx = x2 - x1
    local dy = y2 - y1
    return math.sqrt(dx * dx + dy * dy)
end

local function random_gene()
    return 0.3 + math.random() * 0.4
end

local function get_diet_name(diet)
    if diet == DIET_HERBIVORE then return "Herbivore"
    elseif diet == DIET_CARNIVORE then return "Carnivore"
    else return "Omnivore" end
end

local function create_creature(parent1, parent2)
    local creature = {
        id = evolution.stats.total_born + 1,
        x = parent1 and parent1.x or math.random() * WORLD_WIDTH,
        y = parent1 and parent1.y or math.random() * WORLD_HEIGHT,
        energy = parent1 and 50 or 100,
        age = 0,
        generation = parent1 and (evolution.generation + 1) or 1,
    }
    
    if parent1 then
        creature.genes = {
            size = parent1.genes.size,
            speed = parent1.genes.speed,
            sense = parent1.genes.sense,
            diet = parent1.genes.diet,
        }
        
        if parent2 then
            creature.genes.size = (parent1.genes.size + parent2.genes.size) / 2
            creature.genes.speed = (parent1.genes.speed + parent2.genes.speed) / 2
            creature.genes.sense = (parent1.genes.sense + parent2.genes.sense) / 2
        end
        
        if math.random() < MUTATION_RATE then
            local gene = math.random(3)
            local mutation = (math.random() - 0.5) * 2 * MUTATION_STRENGTH
            if gene == 1 then
                creature.genes.size = clamp(creature.genes.size + mutation, 0.1, 1.0)
            elseif gene == 2 then
                creature.genes.speed = clamp(creature.genes.speed + mutation, 0.1, 1.0)
            else
                creature.genes.sense = clamp(creature.genes.sense + mutation, 0.1, 1.0)
            end
        end
        
        if math.random() < MUTATION_RATE * 0.1 then
            creature.genes.diet = math.random(3)
        end
    else
        creature.genes = {
            size = random_gene(),
            speed = random_gene(),
            sense = random_gene(),
            diet = math.random(3),
        }
    end
    
    evolution.stats.total_born = evolution.stats.total_born + 1
    return creature
end

local function create_food()
    return {
        id = #evolution.food + 1 + math.random(10000),
        x = math.random() * WORLD_WIDTH,
        y = math.random() * WORLD_HEIGHT,
        energy = 20 + math.random() * 20,
        type = math.random(3),
    }
end

local function spawn_food()
    if #evolution.food < MAX_FOOD and math.random() < FOOD_SPAWN_RATE then
        table.insert(evolution.food, create_food())
    end
end

local function can_eat(creature, target_type)
    if creature.genes.diet == DIET_HERBIVORE then
        return target_type ~= 3
    elseif creature.genes.diet == DIET_CARNIVORE then
        return target_type == 3
    else
        return true
    end
end

local function find_nearest_food(creature)
    local nearest = nil
    local min_dist = math.huge
    
    for _, food in ipairs(evolution.food) do
        if can_eat(creature, food.type) then
            local d = distance(creature.x, creature.y, food.x, food.y)
            if d < min_dist then
                min_dist = d
                nearest = food
            end
        end
    end
    
    return nearest, min_dist
end

local function find_nearest_prey(creature)
    if creature.genes.diet ~= DIET_CARNIVORE and creature.genes.diet ~= DIET_OMNIVORE then
        return nil, math.huge
    end
    
    local nearest = nil
    local min_dist = math.huge
    
    for _, other in ipairs(evolution.creatures) do
        if other.id ~= creature.id and other.genes.size < creature.genes.size * 0.8 then
            local d = distance(creature.x, creature.y, other.x, other.y)
            if d < min_dist then
                min_dist = d
                nearest = other
            end
        end
    end
    
    return nearest, min_dist
end

local function move_creature(creature, dx, dy)
    local speed = creature.genes.speed * 2
    local len = math.sqrt(dx * dx + dy * dy)
    if len > 0 then
        dx = dx / len * speed
        dy = dy / len * speed
    end
    
    creature.x = clamp(creature.x + dx, 0, WORLD_WIDTH)
    creature.y = clamp(creature.y + dy, 0, WORLD_HEIGHT)
    
    creature.energy = creature.energy - 0.1 * speed
end

local function update_creature(creature)
    creature.age = creature.age + 1
    creature.energy = creature.energy - 0.05 - creature.genes.size * 0.02
    
    if creature.energy <= 0 then
        return false
    end
    
    local sense_range = creature.genes.sense * 15 + 5
    
    local nearest_food, food_dist = find_nearest_food(creature)
    local nearest_prey, prey_dist = find_nearest_prey(creature)
    
    local target = nil
    local target_dist = math.huge
    
    if nearest_prey and prey_dist < sense_range and prey_dist < food_dist then
        target = nearest_prey
        target_dist = prey_dist
    elseif nearest_food and food_dist < sense_range then
        target = nearest_food
        target_dist = food_dist
    end
    
    if target then
        local dx = target.x - creature.x
        local dy = target.y - creature.y
        move_creature(creature, dx, dy)
        
        if target_dist < 1.5 then
            if target.energy then
                creature.energy = creature.energy + target.energy * 0.5
                for i, c in ipairs(evolution.creatures) do
                    if c.id == target.id then
                        table.remove(evolution.creatures, i)
                        evolution.stats.total_died = evolution.stats.total_died + 1
                        break
                    end
                end
            else
                creature.energy = creature.energy + target.energy
                for i, f in ipairs(evolution.food) do
                    if f.id == target.id then
                        table.remove(evolution.food, i)
                        break
                    end
                end
            end
        end
    else
        local dx = math.random(-1, 1)
        local dy = math.random(-1, 1)
        move_creature(creature, dx, dy)
    end
    
    if creature.energy > 150 and creature.age > 50 then
        creature.energy = creature.energy - 50
        local offspring = create_creature(creature)
        table.insert(evolution.creatures, offspring)
    end
    
    return true
end

local function calculate_stats()
    local total_speed = 0
    local total_size = 0
    local total_sense = 0
    local count = #evolution.creatures
    
    if count == 0 then return end
    
    for _, c in ipairs(evolution.creatures) do
        total_speed = total_speed + c.genes.speed
        total_size = total_size + c.genes.size
        total_sense = total_sense + c.genes.sense
    end
    
    evolution.stats.avg_speed = total_speed / count
    evolution.stats.avg_size = total_size / count
    evolution.stats.avg_sense = total_sense / count
end

local function end_generation()
    local gen_stats = {
        generation = evolution.generation,
        survivors = #evolution.creatures,
        total_born = evolution.stats.total_born,
        total_died = evolution.stats.total_died,
        avg_speed = evolution.stats.avg_speed,
        avg_size = evolution.stats.avg_size,
        avg_sense = evolution.stats.avg_sense,
    }
    table.insert(evolution.history, gen_stats)
    
    evolution.generation = evolution.generation + 1
    
    for _, c in ipairs(evolution.creatures) do
        c.energy = c.energy + 30
    end
    
    for i = 1, 10 do
        spawn_food()
    end
end

local function clear_screen()
    if package.config:sub(1,1) == '\\' then
        os.execute('cls')
    else
        os.execute('clear')
    end
end

local function render()
    local grid_w = 60
    local grid_h = 20
    local grid = {}
    
    for y = 1, grid_h do
        grid[y] = {}
        for x = 1, grid_w do
            grid[y][x] = ' '
        end
    end
    
    for _, food in ipairs(evolution.food) do
        local gx = math.floor(food.x / WORLD_WIDTH * (grid_w - 1)) + 1
        local gy = math.floor(food.y / WORLD_HEIGHT * (grid_h - 1)) + 1
        gx = clamp(gx, 1, grid_w)
        gy = clamp(gy, 1, grid_h)
        grid[gy][gx] = '.'
    end
    
    for _, creature in ipairs(evolution.creatures) do
        local gx = math.floor(creature.x / WORLD_WIDTH * (grid_w - 1)) + 1
        local gy = math.floor(creature.y / WORLD_HEIGHT * (grid_h - 1)) + 1
        gx = clamp(gx, 1, grid_w)
        gy = clamp(gy, 1, grid_h)
        
        local char = 'O'
        if creature.genes.diet == DIET_HERBIVORE then
            char = 'H'
        elseif creature.genes.diet == DIET_CARNIVORE then
            char = 'C'
        else
            char = 'M'
        end
        
        if creature.energy > 100 then
            char = string.lower(char)
        end
        
        grid[gy][gx] = char
    end
    
    clear_screen()
    
    print("=" .. string.rep("=", grid_w - 2) .. "=")
    print("| EVOLUTION SIMULATION - Gen " .. evolution.generation .. string.rep(" ", grid_w - 28 - #tostring(evolution.generation)) .. "|")
    print("=" .. string.rep("=", grid_w - 2) .. "=")
    
    for y = 1, grid_h do
        io.write("|")
        for x = 1, grid_w do
            io.write(grid[y][x])
        end
        print("|")
    end
    
    print("=" .. string.rep("=", grid_w - 2) .. "=")
    
    calculate_stats()
    local h, c, o = 0, 0, 0
    for _, cr in ipairs(evolution.creatures) do
        if cr.genes.diet == DIET_HERBIVORE then h = h + 1
        elseif cr.genes.diet == DIET_CARNIVORE then c = c + 1
        else o = o + 1 end
    end
    
    print(string.format("Pop: %d (H:%d C:%d M:%d) | Food: %d | Tick: %d",
        #evolution.creatures, h, c, o, #evolution.food, evolution.tick))
    print(string.format("Avg: Speed=%.2f Size=%.2f Sense=%.2f",
        evolution.stats.avg_speed, evolution.stats.avg_size, evolution.stats.avg_sense))
    print("Legend: H=Herbivore C=Carnivore M=Omnivore .=Food | Lowercase=HighEnergy")
    print("Press Ctrl+C to stop")
end

local function init()
    print("=== EVOLUTION SIMULATION ===")
    print(string.format("World: %dx%d | Creatures: %d", WORLD_WIDTH, WORLD_HEIGHT, INITIAL_CREATURES))
    
    math.randomseed(os.time())
    
    for i = 1, INITIAL_CREATURES do
        table.insert(evolution.creatures, create_creature())
    end
    
    for i = 1, INITIAL_FOOD do
        table.insert(evolution.food, create_food())
    end
end

local function tick()
    evolution.tick = evolution.tick + 1
    
    for i = #evolution.creatures, 1, -1 do
        if not update_creature(evolution.creatures[i]) then
            table.remove(evolution.creatures, i)
            evolution.stats.total_died = evolution.stats.total_died + 1
        end
    end
    
    spawn_food()
    
    if evolution.tick % TICKS_PER_GENERATION == 0 then
        calculate_stats()
        print(string.format("\n[Gen %d End] Survivors: %d | Avg: Spd=%.2f Siz=%.2f Sen=%.2f",
            evolution.generation, #evolution.creatures,
            evolution.stats.avg_speed, evolution.stats.avg_size, evolution.stats.avg_sense))
        end_generation()
    end
    
    if #evolution.creatures == 0 then
        print("\nEXTINCTION! All creatures died.")
        for i = 1, 5 do
            table.insert(evolution.creatures, create_creature())
        end
        evolution.generation = evolution.generation + 1
        print("Restarted with 5 new creatures.")
    end
end

local function main()
    init()
    
    while evolution.running do
        tick()
        
        if evolution.tick % 10 == 0 then
            render()
        end
        
        local start = os.clock()
        while os.clock() - start < 0.05 do end
    end
end

main()