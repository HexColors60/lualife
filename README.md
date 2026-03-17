# bevy_screeps_lua

A Screeps-like autonomous simulation game where 32 AI factions run Lua scripts to control units, built with Bevy (Rust).

## Features

- **32 AI Factions**: Each faction runs its own Lua script to control units
- **Autonomous Simulation**: Units mine resources, build structures, and survive
- **Lua Scripting**: Full Lua API for AI development
- **Real-time Rendering**: Visual representation of the world with Bevy
- **Save/Load**: Quick save (F5) and quick load (F9) support

## World

- **Size**: 256x256 tiles
- **Rooms**: 32x32 room grid (8x8 tiles per room)
- **Resources**: 10 types (Power, Iron, Copper, Silicon, Crystal, Carbon, Stone, Sulfur, Water, Biomass)
- **Buildings**: 16 types (BaseCore, Spawn, Storage, PowerDepot, MineExtractor, Refinery, Workshop, Wall, Tower, Road, ScriptRelay, Scanner, RepairBay, Factory, Lab, Barracks)

## Controls

| Key | Action |
|-----|--------|
| WASD/Arrows | Pan camera |
| Scroll | Zoom in/out |
| Click | Select unit |
| ESC | Deselect |
| M | Toggle minimap |
| F3 | Toggle debug overlay |
| F4 | Toggle performance display |
| F5 | Quick save |
| F9 | Quick load |

## Building

```bash
cargo build
cargo run
```

## Project Structure

```
src/
├── app.rs           # Application setup
├── buildings/       # Building system
├── config/          # Configuration (RON format)
├── core/            # Core game state
├── creeps/          # Creep/unit system
├── debug/           # Debug tools
├── events/          # Event system
├── factions/        # Faction management
├── lua/             # Lua VM and API
├── mines/           # Mine system
├── render/          # Rendering systems
├── resources/       # Resource system
├── save/            # Save/load system
├── sim/             # Simulation phases
├── ui/              # UI systems
├── world/           # World data structures
└── worldgen/        # World generation
```

## Lua API

### Available Modules

- `log` - Logging functions (info, debug, warn, error)
- `units` - Unit control (list_owned, get, move_to, mine, attack, build)
- `world` - World queries (get_mines, get_enemies, get_construction_sites)
- `rooms` - Room information (get_mines, get_terrain)
- `debug` - Debug utilities

### Example Script

```lua
function on_tick()
    local units = units.list_owned()
    for _, unit_id in ipairs(units) do
        local unit = units.get(unit_id)
        if unit.action == "idle" then
            -- Find and mine power
            local mines = world.get_mines()
            for _, mine in ipairs(mines) do
                if mine.resource_type == "power" then
                    units.mine(unit_id, mine.id)
                    break
                end
            end
        end
    end
end
```

## Configuration

Configuration files are in RON format in the `config/` directory:

- `ai.ron` - AI settings
- `game.ron` - Game settings
- `ui.ron` - UI settings
- `worldgen.ron` - World generation settings

## License

MIT License