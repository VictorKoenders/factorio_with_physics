use super::{
    hitbox::{EntityHitbox, HitboxRectangle},
    Entity,
};
use crate::world::{Resource, World, WorldCoord};
use noisy_float::types::R32;

#[derive(Clone, Debug)]
pub struct MinerConfig {
    pub mining_speed: usize,
    pub hitbox: HitboxRectangle,
}

impl MinerConfig {
    pub fn spawn_entity(&self, world: &mut World, at: WorldCoord) {
        let entity = world.entities.create();
        let miner = Miner {
            entity: entity.clone(),
            next_mine_time: world.time + self.interval(),
            next_resource_index: 0,
            resources: vec![Resource::Iron],
            config: self.clone(),
        };
        let hitbox = EntityHitbox::new(entity.clone(), self.hitbox.clone().into(), at);

        world.miners.set(entity, miner);
        world.add_hitbox(hitbox);
    }

    fn interval(&self) -> f32 {
        1000. / self.mining_speed as f32
    }
}

#[derive(Clone, Debug)]
pub struct Miner {
    pub entity: Entity,
    pub next_mine_time: R32,
    pub next_resource_index: usize,
    pub resources: Vec<Resource>,
    pub config: MinerConfig,
}

impl Miner {}
