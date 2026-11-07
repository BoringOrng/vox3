use std::collections::BinaryHeap;

use bevy::{platform::collections::HashSet, prelude::*};
use vox3_world::{chunk::resources::ChunkMap, position::components::ChunkPos};

#[derive(Resource, Default)]
pub struct StreamQueue {
    heap: BinaryHeap<Item>,
    queued: HashSet<ChunkPos>,
}

impl StreamQueue {
    pub fn push(&mut self, item: Item) -> bool {
        if self.queued.insert(item.pos()) {
            self.heap.push(item);

            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, pos: &ChunkPos) -> bool {
        let removed = self.queued.remove(pos);
        if removed {
            self.heap.retain(|item| item.pos() != *pos);
        }
        removed
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&ChunkPos) -> bool,
    {
        self.queued.retain(f);
        self.heap.retain(|item| self.queued.contains(&item.pos()));
    }

    pub fn reprioritize(&mut self, anchors: &[ChunkPos]) {
        if anchors.is_empty() {
            return;
        }

        self.heap = self
            .queued
            .iter()
            .map(|&pos| Item {
                distance_squared: anchors
                    .iter()
                    .map(|anchor| anchor.distance_squared(pos))
                    .min()
                    .unwrap_or(u32::MAX),
                pos,
            })
            .collect();
    }

    pub fn pop_batch(&mut self, n: usize, chunk_map: &ChunkMap) -> Vec<ChunkPos> {
        let mut out = Vec::with_capacity(n);

        while out.len() < n {
            let Some(item) = self.heap.pop() else {
                break;
            };

            let pos = item.pos();

            if !self.queued.remove(&pos) || chunk_map.contains_key(pos) {
                continue;
            }

            out.push(pos);
        }

        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Item {
    distance_squared: u32,
    pos: ChunkPos,
}

impl Item {
    #[must_use]
    pub fn new(anchor_pos: ChunkPos, chunk_pos: ChunkPos) -> Self {
        Self {
            distance_squared: anchor_pos.distance_squared(chunk_pos),
            pos: chunk_pos,
        }
    }

    #[must_use]
    pub const fn pos(self) -> ChunkPos {
        self.pos
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance_squared
            .cmp(&self.distance_squared)
            .then_with(|| {
                self.pos
                    .as_ivec3()
                    .to_array()
                    .cmp(&other.pos.as_ivec3().to_array())
            })
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
