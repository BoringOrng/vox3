use bevy::prelude::*;
use vox3_world::position::components::VoxelPos;

#[derive(Debug, Component, Default)]
#[require(VoxelPos)]
pub struct SimEntity;
