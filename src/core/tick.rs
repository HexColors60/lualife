use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, Deref, DerefMut)]
pub struct TickNumber(pub u64);

impl TickNumber {
    pub fn new() -> Self {
        Self(0)
    }
}