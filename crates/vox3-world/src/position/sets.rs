use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PositionSystems {
    Origin,
    Sync,
    PostPropagate,
}
