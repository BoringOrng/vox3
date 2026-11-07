use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    chunk::{components::ChunkData, resources::ChunkMap},
    position::components::ChunkPos,
};

#[derive(Component, Reflect, Debug, Serialize, Deserialize)]
#[component(on_add, on_remove)]
#[require(ChunkPos, ChunkData)]
pub struct Chunk;

impl Chunk {
    pub const SIZE: u8 = 32;
    pub const VOLUME: u16 = (Self::SIZE as u16).pow(3);

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let pos = *world.get::<ChunkPos>(ctx.entity).unwrap();
        let mut chunk_map = world.resource_mut::<ChunkMap>();

        chunk_map.insert(pos, ctx.entity);
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        let pos = *world.get::<ChunkPos>(ctx.entity).unwrap();
        let mut chunk_map = world.resource_mut::<ChunkMap>();

        chunk_map.remove(pos);
    }
}
