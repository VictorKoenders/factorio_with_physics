use super::Chunk;
use crate::assets::Assets;
use graphics::DrawParams;
use legion::prelude::*;
use tetra::{graphics, math::Vec2, Context};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TileGridTag;

#[derive(Debug, Clone, PartialEq)]
pub struct TileGrid {
    pub tiles: Vec<Tile>,
}

impl TileGrid {
    pub fn new() -> Self {
        Self {
            tiles: vec![Tile {}; 32 * 32],
        }
    }

    pub fn add_to_world(self, world: &mut World, chunk: Chunk) {
        world.insert((chunk, TileGridTag), Some((self,)));
    }

    pub fn load_in_range(
        world: &World,
        min: Chunk,
        max: Chunk,
        mut cb: impl for<'a> FnMut(Chunk, &'a TileGrid),
    ) {
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let chunk = Chunk::new(x, y);
                let query =
                    <Read<TileGrid>>::query().filter(tag_value(&chunk) & tag::<TileGridTag>());
                for grid in query.iter(world) {
                    cb(chunk.clone(), &*grid);
                }
            }
        }
    }

    pub fn render(&self, assets: &Assets, ctx: &mut Context, chunk: Chunk) {
        let mut x = 0;
        let mut y = 0;
        const TILE_SIZE: Vec2<f32> = Vec2 { x: 32., y: 32. };
        const CHUNK_SIZE: Vec2<f32> = Vec2 {
            x: Chunk::CHUNK_SIZE_IN_PX,
            y: Chunk::CHUNK_SIZE_IN_PX,
        };

        for _t in &self.tiles {
            let position = (Vec2::new(x as f32, y as f32) * TILE_SIZE)
                + (Vec2::new(chunk.x as f32, chunk.y as f32) * CHUNK_SIZE);
            graphics::draw(
                ctx,
                &assets.grass,
                DrawParams::new()
                    .position(position)
                    .scale(Vec2::new(0.9, 0.9)),
            );
            x += 1;
            if x == 32 {
                y += 1;
                x = 0;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {}
