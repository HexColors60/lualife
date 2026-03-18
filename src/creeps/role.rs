use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CreepRole {
    #[default]
    Idle,
    Harvester,
    Builder,
    Fighter,
    Hauler,
    Scout,
    Upgrader,
    Repairer,
}

impl CreepRole {
    pub fn name(&self) -> &'static str {
        match self {
            CreepRole::Idle => "idle",
            CreepRole::Harvester => "harvester",
            CreepRole::Builder => "builder",
            CreepRole::Fighter => "fighter",
            CreepRole::Hauler => "hauler",
            CreepRole::Scout => "scout",
            CreepRole::Upgrader => "upgrader",
            CreepRole::Repairer => "repairer",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "idle" => Some(CreepRole::Idle),
            "harvester" => Some(CreepRole::Harvester),
            "builder" => Some(CreepRole::Builder),
            "fighter" => Some(CreepRole::Fighter),
            "hauler" => Some(CreepRole::Hauler),
            "scout" => Some(CreepRole::Scout),
            "upgrader" => Some(CreepRole::Upgrader),
            "repairer" => Some(CreepRole::Repairer),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creep_role_default() {
        let role = CreepRole::default();
        assert_eq!(role, CreepRole::Idle);
    }

    #[test]
    fn test_creep_role_name() {
        assert_eq!(CreepRole::Idle.name(), "idle");
        assert_eq!(CreepRole::Harvester.name(), "harvester");
        assert_eq!(CreepRole::Builder.name(), "builder");
        assert_eq!(CreepRole::Fighter.name(), "fighter");
        assert_eq!(CreepRole::Hauler.name(), "hauler");
        assert_eq!(CreepRole::Scout.name(), "scout");
        assert_eq!(CreepRole::Upgrader.name(), "upgrader");
        assert_eq!(CreepRole::Repairer.name(), "repairer");
    }

    #[test]
    fn test_creep_role_from_name() {
        assert_eq!(CreepRole::from_name("idle"), Some(CreepRole::Idle));
        assert_eq!(CreepRole::from_name("harvester"), Some(CreepRole::Harvester));
        assert_eq!(CreepRole::from_name("builder"), Some(CreepRole::Builder));
        assert_eq!(CreepRole::from_name("fighter"), Some(CreepRole::Fighter));
        assert_eq!(CreepRole::from_name("hauler"), Some(CreepRole::Hauler));
        assert_eq!(CreepRole::from_name("scout"), Some(CreepRole::Scout));
        assert_eq!(CreepRole::from_name("upgrader"), Some(CreepRole::Upgrader));
        assert_eq!(CreepRole::from_name("repairer"), Some(CreepRole::Repairer));
        assert_eq!(CreepRole::from_name("unknown"), None);
    }

    #[test]
    fn test_creep_role_equality() {
        assert_eq!(CreepRole::Harvester, CreepRole::Harvester);
        assert_ne!(CreepRole::Harvester, CreepRole::Builder);
    }

    #[test]
    fn test_creep_role_roundtrip() {
        for role in [
            CreepRole::Idle,
            CreepRole::Harvester,
            CreepRole::Builder,
            CreepRole::Fighter,
            CreepRole::Hauler,
            CreepRole::Scout,
            CreepRole::Upgrader,
            CreepRole::Repairer,
        ] {
            let name = role.name();
            let parsed = CreepRole::from_name(name);
            assert_eq!(parsed, Some(role));
        }
    }

    #[test]
    fn test_creep_role_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(CreepRole::Harvester);
        set.insert(CreepRole::Builder);
        set.insert(CreepRole::Harvester);
        assert_eq!(set.len(), 2);
    }
}
