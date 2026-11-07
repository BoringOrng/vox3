use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use vox3_world::{
    chunk::components::{Chunk, ChunkData},
    coords::ChunkLocalPos,
};

use crate::material::resources::VoxelMaterial;

const FACES: [Face; 6] = [
    Face {
        direction: IVec3::X,
        normal: [1.0, 0.0, 0.0],
        corners: [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
    },
    Face {
        direction: IVec3::NEG_X,
        normal: [-1.0, 0.0, 0.0],
        corners: [
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0],
        ],
    },
    Face {
        direction: IVec3::Y,
        normal: [0.0, 1.0, 0.0],
        corners: [
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
        ],
    },
    Face {
        direction: IVec3::NEG_Y,
        normal: [0.0, -1.0, 0.0],
        corners: [
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
        ],
    },
    Face {
        direction: IVec3::Z,
        normal: [0.0, 0.0, 1.0],
        corners: [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
    },
    Face {
        direction: IVec3::NEG_Z,
        normal: [0.0, 0.0, -1.0],
        corners: [
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
        ],
    },
];

#[derive(Clone, Copy)]
struct Face {
    direction: IVec3,
    normal: [f32; 3],
    corners: [[f32; 3]; 4],
}

#[derive(Default)]
pub(super) struct MeshData {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    layers: Vec<u32>,
    indices: Vec<u32>,
}

impl MeshData {
    pub(super) const fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    fn push_face(&mut self, voxel: Vec3, face: Face) {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "a chunk has a bounded number of exposed faces"
        )]
        let first_vertex = self.positions.len() as u32;

        for corner in face.corners {
            let corner = Vec3::from_array(corner);
            self.positions.push((voxel + corner).to_array());
            self.normals.push(face.normal);
            self.layers.push(0);
        }

        self.indices.extend_from_slice(&[
            first_vertex,
            first_vertex + 1,
            first_vertex + 2,
            first_vertex,
            first_vertex + 2,
            first_vertex + 3,
        ]);
    }

    pub(super) fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(VoxelMaterial::ATTRIBUTE_LAYER, self.layers);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }
}

pub(super) fn build_chunk_mesh(
    data: &ChunkData,
    mut neighbor_is_solid: impl FnMut(IVec3) -> bool,
) -> MeshData {
    let mut mesh = MeshData::default();
    let size = i32::from(Chunk::SIZE);

    for (local_pos, is_solid) in data.iter() {
        if !is_solid {
            continue;
        }

        let local = IVec3::new(
            i32::from(local_pos.x()),
            i32::from(local_pos.y()),
            i32::from(local_pos.z()),
        );
        let voxel = local.as_vec3();

        for face in FACES {
            let neighbor = local + face.direction;
            let in_chunk =
                neighbor.cmpge(IVec3::ZERO).all() && neighbor.cmplt(IVec3::splat(size)).all();

            let occluded = if in_chunk {
                let [x, y, z] = neighbor.as_uvec3().as_u8vec3().to_array();
                let pos = ChunkLocalPos::new(x, y, z);
                data.get(pos) == Some(true)
            } else {
                neighbor_is_solid(neighbor)
            };

            if !occluded {
                mesh.push_face(voxel, face);
            }
        }
    }

    mesh
}
