/// Standard Lua hook names
pub const HOOK_ON_INIT: &str = "on_init";
pub const HOOK_ON_TICK: &str = "on_tick";
pub const HOOK_ON_ROOM_SEEN: &str = "on_room_seen";
pub const HOOK_ON_UNIT_SPAWNED: &str = "on_unit_spawned";
pub const HOOK_ON_UNIT_DIED: &str = "on_unit_died";

/// All available hooks
pub const ALL_HOOKS: &[&str] = &[
    HOOK_ON_INIT,
    HOOK_ON_TICK,
    HOOK_ON_ROOM_SEEN,
    HOOK_ON_UNIT_SPAWNED,
    HOOK_ON_UNIT_DIED,
];

/// Hook parameter types
#[derive(Debug, Clone)]
pub enum HookParam {
    None,
    UnitId(u32),
    RoomCoord { x: u32, y: u32 },
}
