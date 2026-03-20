//! Resource scarcity system that tracks global resource levels and affects gameplay.

use bevy::prelude::*;

use crate::mines::MineNode;
use crate::resources::ResourceType;

/// Plugin for resource scarcity systems.
pub struct ScarcityPlugin;

impl Plugin for ScarcityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalResourceLevels>()
            .init_resource::<ScarcityConfig>()
            .add_event::<ScarcityEvent>()
            .add_systems(Update, update_resource_levels);
    }
}

/// Tracks global resource levels across all mines.
#[derive(Resource, Debug, Clone, Default)]
pub struct GlobalResourceLevels {
    /// Total current amount of each resource type
    pub current: std::collections::HashMap<ResourceType, u64>,
    /// Total max amount of each resource type
    pub maximum: std::collections::HashMap<ResourceType, u64>,
    /// Number of mines per resource type
    pub mine_count: std::collections::HashMap<ResourceType, u32>,
    /// Number of exhausted mines per resource type
    pub exhausted_count: std::collections::HashMap<ResourceType, u32>,
}

impl GlobalResourceLevels {
    /// Get the fill ratio for a resource type (0.0 to 1.0).
    pub fn fill_ratio(&self, resource_type: ResourceType) -> f32 {
        let current = self.current.get(&resource_type).copied().unwrap_or(0);
        let maximum = self.maximum.get(&resource_type).copied().unwrap_or(1);
        if maximum == 0 {
            return 0.0;
        }
        current as f32 / maximum as f32
    }

    /// Check if a resource is scarce (below threshold).
    pub fn is_scarce(&self, resource_type: ResourceType, threshold: f32) -> bool {
        self.fill_ratio(resource_type) < threshold
    }

    /// Get the percentage of exhausted mines for a resource type.
    pub fn exhaustion_rate(&self, resource_type: ResourceType) -> f32 {
        let total = self.mine_count.get(&resource_type).copied().unwrap_or(0);
        if total == 0 {
            return 0.0;
        }
        let exhausted = self.exhausted_count.get(&resource_type).copied().unwrap_or(0);
        exhausted as f32 / total as f32
    }
}

/// Configuration for scarcity mechanics.
#[derive(Resource, Debug, Clone)]
pub struct ScarcityConfig {
    /// Threshold below which a resource is considered scarce (0.0 to 1.0)
    pub scarcity_threshold: f32,
    /// Threshold below which a resource is critical
    pub critical_threshold: f32,
    /// Threshold above which a resource is abundant
    pub abundant_threshold: f32,
    /// Multiplier applied to regeneration when scarce (0.0 to 1.0)
    pub scarcity_regen_multiplier: f32,
    /// Bonus multiplier when resource is abundant (> 80% fill)
    pub abundance_regen_bonus: f32,
}

impl Default for ScarcityConfig {
    fn default() -> Self {
        Self {
            scarcity_threshold: 0.3,
            critical_threshold: 0.1,
            abundant_threshold: 0.8,
            scarcity_regen_multiplier: 0.5,
            abundance_regen_bonus: 1.5,
        }
    }
}

/// Events related to resource scarcity.
#[derive(Event, Debug, Clone)]
pub enum ScarcityEvent {
    /// A resource has become scarce
    ResourceScarce { resource_type: ResourceType, fill_ratio: f32 },
    /// A resource has become critical
    ResourceCritical { resource_type: ResourceType, fill_ratio: f32 },
    /// A resource has recovered from scarcity
    ResourceRecovered { resource_type: ResourceType, fill_ratio: f32 },
    /// A mine has been exhausted
    MineExhausted { resource_type: ResourceType, mine_id: u32 },
    /// Multiple mines of the same type are exhausted
    MassExhaustion { resource_type: ResourceType, count: u32 },
}

/// Update global resource levels from mine states.
fn update_resource_levels(
    mines: Query<&MineNode>,
    mut levels: ResMut<GlobalResourceLevels>,
    mut events: EventWriter<ScarcityEvent>,
    config: Res<ScarcityConfig>,
    mut last_scarce: Local<std::collections::HashMap<ResourceType, bool>>,
    mut last_critical: Local<std::collections::HashMap<ResourceType, bool>>,
) {
    // Reset counts
    levels.current.clear();
    levels.maximum.clear();
    levels.mine_count.clear();
    levels.exhausted_count.clear();

    // Aggregate mine data
    for mine in mines.iter() {
        let resource = mine.resource_type();
        
        *levels.current.entry(resource).or_insert(0) += mine.current_amount as u64;
        *levels.maximum.entry(resource).or_insert(0) += mine.max_amount as u64;
        *levels.mine_count.entry(resource).or_insert(0) += 1;
        
        if mine.exhausted {
            *levels.exhausted_count.entry(resource).or_insert(0) += 1;
        }
    }

    // Check for scarcity events
    for resource in [
        ResourceType::Power,
        ResourceType::Iron,
        ResourceType::Copper,
        ResourceType::Silicon,
        ResourceType::Crystal,
        ResourceType::Carbon,
        ResourceType::Stone,
        ResourceType::Sulfur,
        ResourceType::Water,
        ResourceType::Biomass,
    ] {
        let fill_ratio = levels.fill_ratio(resource);
        let was_scarce = last_scarce.get(&resource).copied().unwrap_or(false);
        let was_critical = last_critical.get(&resource).copied().unwrap_or(false);
        
        let is_critical = fill_ratio < config.critical_threshold;
        let is_scarce = fill_ratio < config.scarcity_threshold;

        // Critical events
        if is_critical && !was_critical {
            events.send(ScarcityEvent::ResourceCritical {
                resource_type: resource,
                fill_ratio,
            });
        } else if !is_critical && was_critical {
            events.send(ScarcityEvent::ResourceRecovered {
                resource_type: resource,
                fill_ratio,
            });
        }
        // Scarcity events (only if not critical)
        else if is_scarce && !is_critical && !was_scarce {
            events.send(ScarcityEvent::ResourceScarce {
                resource_type: resource,
                fill_ratio,
            });
        } else if !is_scarce && was_scarce {
            events.send(ScarcityEvent::ResourceRecovered {
                resource_type: resource,
                fill_ratio,
            });
        }

        last_scarce.insert(resource, is_scarce);
        last_critical.insert(resource, is_critical);
    }
}

/// Get the regeneration rate multiplier based on global resource levels.
pub fn get_regen_multiplier(
    resource_type: ResourceType,
    levels: &GlobalResourceLevels,
    config: &ScarcityConfig,
) -> f32 {
    let fill_ratio = levels.fill_ratio(resource_type);
    
    if fill_ratio < config.critical_threshold {
        config.scarcity_regen_multiplier * 0.5 // Very slow recovery when critical
    } else if fill_ratio < config.scarcity_threshold {
        config.scarcity_regen_multiplier // Slow recovery when scarce
    } else if fill_ratio > 0.8 {
        config.abundance_regen_bonus // Faster regeneration when abundant
    } else {
        1.0 // Normal rate
    }
}