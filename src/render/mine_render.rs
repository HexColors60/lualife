use bevy::prelude::*;

use crate::mines::MineNode;

pub fn mine_render_system(
    mines: Query<&MineNode>,
) {
    for mine in mines.iter() {
        let _color = mine.resource_type().color();
        let fill_ratio = mine.fill_ratio();

        // Placeholder: would render mine sprite/icon
        tracing::trace!(
            "Mine {} at ({}, {}) type {:?} fill {:.0}%",
            mine.id,
            mine.position.x,
            mine.position.y,
            mine.resource_type(),
            fill_ratio * 100.0
        );
    }
}