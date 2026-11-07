use bevy::prelude::*;
use vox3_world::position::components::ChunkPos;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct RenderOrigin(pub(in crate::position) ChunkPos);
