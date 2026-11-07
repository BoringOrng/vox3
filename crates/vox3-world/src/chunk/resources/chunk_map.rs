use bevy::{platform::collections::HashMap, prelude::*};

use crate::position::components::ChunkPos;

#[derive(Default, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct ChunkMap(HashMap<ChunkPos, Entity>);

impl ChunkMap {
    pub fn insert(&mut self, pos: ChunkPos, ent: Entity) -> Option<Entity> {
        self.0.insert(pos, ent)
    }

    pub fn remove(&mut self, pos: ChunkPos) -> Option<Entity> {
        self.0.remove(&pos)
    }

    #[must_use]
    pub fn contains_key(&self, pos: ChunkPos) -> bool {
        self.0.contains_key(&pos)
    }

    #[must_use]
    pub fn get(&self, pos: ChunkPos) -> Option<Entity> {
        self.0.get(&pos).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ChunkPos, Entity)> {
        self.0.iter().map(|(&pos, &ent)| (pos, ent))
    }
}
