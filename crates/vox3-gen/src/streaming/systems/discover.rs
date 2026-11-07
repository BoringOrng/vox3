use bevy::prelude::*;
use vox3_world::{
    chunk::resources::ChunkMap,
    position::components::{ChunkPos, VoxelPos},
};

use crate::streaming::{
    components::{LastPos, StreamingAnchor},
    resources::{
        StreamingConfig,
        stream_queue::{self, StreamQueue},
    },
};

type ChangedAnchors<'w, 's> = Query<
    'w,
    's,
    (&'static VoxelPos, &'static mut LastPos),
    (Changed<VoxelPos>, With<StreamingAnchor>),
>;

#[expect(clippy::needless_pass_by_value)]
pub fn discover_chunks(
    chunk_map: Res<ChunkMap>,
    stream_config: Res<StreamingConfig>,
    mut chunk_queue: ResMut<StreamQueue>,
    mut anchors: ChangedAnchors,
    all_anchors: Query<&VoxelPos, With<StreamingAnchor>>,
    mut commands: Commands,
) {
    if anchors.is_empty() {
        return;
    }

    let anchor_positions: Vec<_> = all_anchors.iter().map(|pos| pos.chunk).collect();
    let load_radius = stream_config.load_radius();
    let load_radius_sq = load_radius.pow(2);
    let unload_radius_sq = stream_config.unload_radius().pow(2);
    let mut moved_chunks = false;

    for (voxel_pos, mut last) in &mut anchors {
        let new_pos = voxel_pos.chunk;
        let old_pos = last.0.replace(new_pos);

        if old_pos == Some(new_pos) {
            continue;
        }

        moved_chunks = true;
        enqueue_box(
            new_pos,
            load_radius,
            load_radius_sq,
            &chunk_map,
            &mut chunk_queue,
        );
    }

    if !moved_chunks {
        return;
    }

    chunk_queue.retain(|pos| {
        anchor_positions
            .iter()
            .any(|anchor| anchor.distance_squared(*pos) <= load_radius_sq)
    });
    chunk_queue.reprioritize(&anchor_positions);

    for (pos, entity) in chunk_map.iter() {
        let keep = anchor_positions
            .iter()
            .any(|anchor| anchor.distance_squared(pos) <= unload_radius_sq);

        if !keep {
            commands.entity(entity).despawn();
        }
    }
}

fn enqueue_box(
    center: ChunkPos,
    radius: u32,
    radius_sq: u32,
    chunk_map: &ChunkMap,
    queue: &mut StreamQueue,
) {
    let radius = UVec3::splat(radius).as_ivec3();
    let min = center - ChunkPos::from_ivec3(radius);
    let max = center + ChunkPos::from_ivec3(radius);

    for x in min.x()..=max.x() {
        for y in min.y()..=max.y() {
            for z in min.z()..=max.z() {
                let pos = ChunkPos::new(x, y, z);
                if center.distance_squared(pos) <= radius_sq && !chunk_map.contains_key(pos) {
                    queue.push(stream_queue::Item::new(center, pos));
                }
            }
        }
    }
}
