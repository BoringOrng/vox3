use bevy::prelude::*;
use vox3_world::position::components::VoxelPos;

use crate::position::{components::FloatingOrigin, resources::RenderOrigin};

#[expect(clippy::needless_pass_by_value)]
pub fn update_origin(
    focus: Single<&VoxelPos, (With<FloatingOrigin>, Changed<VoxelPos>)>,
    mut origin: ResMut<RenderOrigin>,
) {
    origin.set_if_neq(RenderOrigin(focus.chunk));
}
