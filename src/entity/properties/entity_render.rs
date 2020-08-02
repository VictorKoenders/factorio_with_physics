use super::{Chunk, Position};
use crate::assets::Assets;
use graphics::{DrawParams, Texture};
use legion::prelude::*;
use tetra::{graphics, math::Vec2, Context};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityRender {
    pub asset: EntityAsset,
}

impl EntityRender {
    pub fn render(self, ctx: &mut Context, assets: &Assets, position: &Position) {
        let asset = self.asset.get(assets);
        let size = asset.size();
        let origin = Vec2::new(size.0 as f32, size.1 as f32) / 2.;
        graphics::draw(
            ctx,
            asset,
            DrawParams::new()
                .position(Vec2::new(position.x, position.y))
                .origin(origin),
        );
    }

    pub fn get_in_range(
        world: &World,
        min: Chunk,
        max: Chunk,
        mut cb: impl for<'a> FnMut(&'a Position, &'a EntityRender),
    ) {
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let chunk = Chunk::new(x, y);
                let query =
                    <(Read<Position>, Read<EntityRender>)>::query().filter(tag_value(&chunk));
                for (position, render) in query.iter(world) {
                    cb(&*position, &*render);
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityAsset {
    IronPlate,
}

impl EntityAsset {
    pub fn to_render(self) -> EntityRender {
        EntityRender { asset: self }
    }

    fn get<'a>(&self, assets: &'a Assets) -> &'a Texture {
        match self {
            EntityAsset::IronPlate => &assets.iron,
        }
    }
}
