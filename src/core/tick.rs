use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Resource,
    Debug,
    Clone,
    Copy,
    Default,
    Deref,
    DerefMut,
    Serialize,
    Deserialize,
    Hash,
    Eq,
    PartialEq,
)]
pub struct TickNumber(pub u64);

impl TickNumber {
    pub fn new() -> Self {
        Self(0)
    }
}
