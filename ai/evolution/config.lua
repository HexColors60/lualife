-- Evolution Simulation Configuration

return {
    name = "Evolution",
    version = "1.0.0",
    author = "lualife",
    
    world = {
        width = 100,
        height = 100,
    },
    
    simulation = {
        initial_creatures = 20,
        initial_food = 50,
        food_spawn_rate = 0.1,
        max_food = 100,
        ticks_per_generation = 500,
    },
    
    genetics = {
        mutation_rate = 0.2,
        mutation_strength = 0.3,
        reproduction_energy = 150,
        reproduction_cost = 50,
        min_reproduction_age = 50,
    },
    
    energy = {
        movement_cost_base = 0.1,
        size_cost = 0.02,
        metabolism_base = 0.05,
        food_energy_min = 20,
        food_energy_max = 40,
        initial_energy = 100,
        offspring_energy = 50,
    },
    
    traits = {
        speed_multiplier = 2,
        sense_base = 5,
        sense_multiplier = 15,
        catch_distance = 1.5,
    },
    
    diet = {
        herbivore = 1,
        carnivore = 2,
        omnivore = 3,
    },
}