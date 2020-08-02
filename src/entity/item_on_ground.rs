use super::properties::{EntityAsset, Hitbox, Position};
use legion::{entity::Entity, world::World};

pub struct ItemOnGround {
    position_entity: Entity,
}

impl ItemOnGround {
    const HITBOX: Hitbox = Hitbox {
        left: 0.5,
        right: 0.5,
        up: 0.5,
        down: 0.5,
    };
    pub fn new(world: &mut World, position: Position) -> Self {
        let chunk = position.chunk();

        let position_entity = world.insert(
            (chunk,),
            Some((
                position,
                Self::HITBOX.clone(),
                EntityAsset::IronPlate.to_render(),
            )),
        )[0];
        Self { position_entity }
    }
}
