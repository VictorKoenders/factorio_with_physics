mod chunk;
mod coord;
mod resource;

use crate::entity::{
    hitbox::{CoordDiff, EntityHitbox},
    miner::Miner,
    Entities, EntityMap, EntityVec,
};
use noisy_float::types::R32;
use std::collections::HashMap;

pub use self::chunk::Chunk;
pub use self::coord::{ChunkCoord, WorldCoord};
pub use self::resource::Resource;

#[derive(Default)]
pub struct World {
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub entities: Entities,
    pub hitbox: EntityVec<EntityHitbox>,
    pub miners: EntityMap<Miner>,
    // TODO: Replace this by a better time struct
    pub time: R32, // time in seconds since the game started
}

impl World {
    pub fn update(&mut self, _ctx: &mut tetra::Context) {}
    pub fn draw(&mut self, ctx: &mut tetra::Context) {
        for chunk in self.chunks.values() {
            chunk.draw(ctx);
        }
    }

    // TODO: Make a wrapper for EntityVec<HitboxRange> and implement this method on there
    pub fn add_hitbox(&mut self, hitbox: EntityHitbox) {
        let entity = hitbox.entity.clone();
        for chunk_coord in hitbox.chunks() {
            if let Some(chunk) = self.chunks.get_mut(&chunk_coord) {
                chunk.add_entity(entity.clone());
            }
        }
        self.hitbox.set(entity, hitbox);
    }

    // TODO: Make a wrapper for EntityVec<HitboxRange> and implement this method on there
    #[allow(dead_code)] // will be used in the future
    pub fn update_hitbox(&mut self, hitbox: EntityHitbox) {
        let entity = hitbox.entity.clone();
        let previous_hitbox = self.hitbox.get(&entity).unwrap();
        for coord_diff in hitbox.chunks_diff_from(&previous_hitbox) {
            match coord_diff {
                CoordDiff::Added(coord) => {
                    if let Some(chunk) = self.chunks.get_mut(&coord) {
                        chunk.add_entity(entity.clone());
                    }
                }
                CoordDiff::Removed(coord) => {
                    if let Some(chunk) = self.chunks.get_mut(&coord) {
                        chunk.remove_entity(&entity);
                    }
                }
            }
        }
    }

    // TODO: Make a wrapper for EntityVec<HitboxRange> and implement this method on there
    #[allow(dead_code)] // will be used in the future
    pub fn remove_hitbox(&mut self, hitbox: EntityHitbox) {
        let entity = hitbox.entity.clone();
        for chunk_coord in hitbox.chunks() {
            if let Some(chunk) = self.chunks.get_mut(&chunk_coord) {
                chunk.remove_entity(&entity);
            }
        }
        self.hitbox.remove(&entity);
    }
}
