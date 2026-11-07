use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum WorldgenSystems {
    Prune,
    Populate,
    Generate,
}
