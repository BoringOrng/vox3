use bevy::prelude::*;
use vox3_world::{
    chunk::components::Chunk,
    position::components::{ChunkPos, VoxelPos},
};

use crate::position::resources::RenderOrigin;

#[expect(clippy::needless_pass_by_value)]
pub fn sync_transforms(
    origin: Res<RenderOrigin>,
    mut transforms: Query<(Ref<VoxelPos>, &mut Transform)>,
) {
    let origin_moved = origin.is_changed();

    for (voxel, mut transform) in &mut transforms {
        if origin_moved || voxel.is_changed() {
            let old = transform.bypass_change_detection();
            let value = to_render(*voxel, **origin);

            if old.translation != value {
                old.translation = value;
                transform.set_changed();
            }
        }
    }
}

#[expect(clippy::needless_pass_by_value)]
pub fn sync_chunk_pos(
    origin: Res<RenderOrigin>,
    mut transforms: Query<(Ref<ChunkPos>, &mut Transform)>,
) {
    let origin_moved = origin.is_changed();

    for (chunk, mut transform) in &mut transforms {
        if origin_moved || chunk.is_changed() {
            let old = transform.bypass_change_detection();
            let value = to_render(VoxelPos::from_chunk(*chunk), **origin);

            if old.translation != value {
                old.translation = value;
                transform.set_changed();
            }
        }
    }
}

fn to_render(voxel: VoxelPos, origin: ChunkPos) -> Vec3 {
    (voxel.chunk - origin).as_vec3() * f32::from(Chunk::SIZE) + voxel.local
}
