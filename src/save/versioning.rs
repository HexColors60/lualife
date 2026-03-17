/// Current save format version
pub const SAVE_VERSION: u32 = 1;

/// Minimum compatible save version
pub const MIN_COMPATIBLE_VERSION: u32 = 1;

pub fn is_compatible(version: u32) -> bool {
    version >= MIN_COMPATIBLE_VERSION && version <= SAVE_VERSION
}

pub fn migrate(snapshot: &mut crate::save::GameSnapshot) -> bool {
    if snapshot.version == SAVE_VERSION {
        return true;
    }

    if !is_compatible(snapshot.version) {
        tracing::error!(
            "Incompatible save version: {} (current: {}, min: {})",
            snapshot.version,
            SAVE_VERSION,
            MIN_COMPATIBLE_VERSION
        );
        return false;
    }

    // Future migrations would go here
    // if snapshot.version < 2 { migrate_to_v2(snapshot); }

    snapshot.version = SAVE_VERSION;
    true
}
