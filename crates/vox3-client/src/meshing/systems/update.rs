use bevy::{platform::collections::HashSet, prelude::*};
use vox3_gen::worldgen::components::Generated;
use vox3_world::{
    chunk::{
        components::{Chunk, ChunkData},
        resources::ChunkMap,
    },
    coords::ChunkLocalPos,
    position::components::ChunkPos,
};

use crate::{
    material::resources::{VoxelMaterial, VoxelMaterialHandle},
    meshing::mesh::build_chunk_mesh,
    position::resources::RenderOrigin,
};

const NEIGHBOR_OFFSETS: [IVec3; 6] = [
    IVec3::X,
    IVec3::NEG_X,
    IVec3::Y,
    IVec3::NEG_Y,
    IVec3::Z,
    IVec3::NEG_Z,
];

type Chunks<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Ref<'static, ChunkPos>,
        Ref<'static, ChunkData>,
        Option<Ref<'static, Generated>>,
        Option<&'static Mesh3d>,
        Option<&'static Transform>,
    ),
    With<Chunk>,
>;

#[derive(Resource, Default)]
pub struct MeshingState {
    loaded: HashSet<ChunkPos>,
    ready: HashSet<ChunkPos>,
    pending: HashSet<ChunkPos>,
}

#[expect(clippy::needless_pass_by_value)]
pub fn update_chunk_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_material: Res<VoxelMaterialHandle>,
    render_origin: Res<RenderOrigin>,
    chunk_map: Res<ChunkMap>,
    chunks: Chunks,
    mut state: ResMut<MeshingState>,
) {
    let mut loaded: HashSet<ChunkPos> = HashSet::default();
    let mut ready: HashSet<ChunkPos> = HashSet::default();
    let mut dirty: HashSet<ChunkPos> = HashSet::default();

    for (_, chunk_pos, data, chunk_ready, _, _) in &chunks {
        let pos = *chunk_pos;
        loaded.insert(pos);

        if chunk_ready.is_some() {
            ready.insert(pos);
        }

        if chunk_pos.is_changed()
            || data.is_changed()
            || chunk_ready.as_ref().is_some_and(DetectChanges::is_changed)
        {
            dirty.insert(pos);
        }
    }

    dirty.extend(loaded.symmetric_difference(&state.loaded).copied());
    dirty.extend(ready.symmetric_difference(&state.ready).copied());

    for pos in dirty {
        mark_neighborhood(pos, &mut state.pending);
    }

    state.loaded = loaded;
    state.ready = ready;

    let pending: Vec<_> = state.pending.iter().copied().collect();

    for pos in pending {
        let Some(entity) = chunk_map.get(pos) else {
            state.pending.remove(&pos);
            continue;
        };

        let Ok((_, chunk_pos, data, chunk_ready, mesh_3d, transform)) = chunks.get(entity) else {
            state.pending.remove(&pos);
            continue;
        };

        if chunk_ready.is_none() || !neighbors_ready(pos, &state.ready) {
            if mesh_3d.is_some() {
                commands
                    .entity(entity)
                    .remove::<Mesh3d>()
                    .remove::<MeshMaterial3d<VoxelMaterial>>();
            }
            continue;
        }

        let mesh_data = build_chunk_mesh(&data, |neighbor| {
            is_solid_at(pos, neighbor, &chunk_map, &chunks)
        });

        if mesh_data.is_empty() {
            if mesh_3d.is_some() {
                commands
                    .entity(entity)
                    .remove::<Mesh3d>()
                    .remove::<MeshMaterial3d<VoxelMaterial>>();
            }
        } else {
            let mesh = meshes.add(mesh_data.into_mesh());
            let material = (**voxel_material).clone();
            let mut entity_commands = commands.entity(entity);
            entity_commands.insert((Mesh3d(mesh), MeshMaterial3d(material)));

            if transform.is_none() {
                let translation = (chunk_pos.as_ivec3() - (**render_origin).as_ivec3()).as_vec3()
                    * f32::from(vox3_world::chunk::components::Chunk::SIZE);
                entity_commands.insert(Transform::from_translation(translation));
            }
        }

        state.pending.remove(&pos);
    }
}

fn mark_neighborhood(pos: ChunkPos, pending: &mut HashSet<ChunkPos>) {
    pending.insert(pos);
    for offset in NEIGHBOR_OFFSETS {
        pending.insert(pos + ChunkPos::from_ivec3(offset));
    }
}

fn neighbors_ready(pos: ChunkPos, ready: &HashSet<ChunkPos>) -> bool {
    NEIGHBOR_OFFSETS
        .iter()
        .all(|&offset| ready.contains(&(pos + ChunkPos::from_ivec3(offset))))
}

fn is_solid_at(chunk_pos: ChunkPos, local: IVec3, chunk_map: &ChunkMap, chunks: &Chunks) -> bool {
    let size = i32::from(vox3_world::chunk::components::Chunk::SIZE);
    let offset = IVec3::new(
        local.x.div_euclid(size),
        local.y.div_euclid(size),
        local.z.div_euclid(size),
    );
    let local = IVec3::new(
        local.x.rem_euclid(size),
        local.y.rem_euclid(size),
        local.z.rem_euclid(size),
    );
    let neighbor_pos = chunk_pos + ChunkPos::from_ivec3(offset);
    let Some(neighbor_entity) = chunk_map.get(neighbor_pos) else {
        return false;
    };
    let Ok((_, _, neighbor_data, Some(_), _, _)) = chunks.get(neighbor_entity) else {
        return false;
    };

    let [x, y, z] = local.as_uvec3().as_u8vec3().to_array();
    neighbor_data.get(ChunkLocalPos::new(x, y, z)) == Some(true)
}
