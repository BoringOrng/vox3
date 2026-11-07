use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum StreamingSystems {
    Discover,
    Process,
}
