use bevy::prelude::*;

use crate::mines::MineNode;

pub fn mine_regeneration_system(
    mut mines: Query<&mut MineNode>,
) {
    for mut mine in mines.iter_mut() {
        mine.regenerate();
    }
}