use super::ChunkCoord;
use crate::assets::Assets;
use crate::entity::{Entity, EntityBitVec};
use bitvec::vec::BitVec;
use std::collections::HashMap;
use tetra::{graphics, Context};

pub const CHUNK_TILE_WIDTH: usize = 16;
pub const CHUNK_TILE_HEIGHT: usize = 16;
pub const CHUNK_TILE_COUNT: usize = CHUNK_TILE_WIDTH * CHUNK_TILE_HEIGHT;

const TILE_SIZE_IN_PX: i32 = 32;
const CHUNK_WIDTH_IN_PX: f32 = (CHUNK_TILE_WIDTH * TILE_SIZE_IN_PX as usize) as f32;
const CHUNK_HEIGHT_IN_PX: f32 = (CHUNK_TILE_HEIGHT * TILE_SIZE_IN_PX as usize) as f32;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub entity_hitbox: EntityBitVec,
    pub tiles: ChunkTileset,
}

impl Chunk {
    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entity_hitbox.remove(&entity);
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entity_hitbox.set(entity);
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.tiles.canvas,
            graphics::DrawParams::new().position(
                (
                    self.x as f32 * CHUNK_WIDTH_IN_PX,
                    self.y as f32 * CHUNK_HEIGHT_IN_PX,
                )
                    .into(),
            ),
        );
    }

    pub fn generate(
        chunks: &mut HashMap<ChunkCoord, Chunk>,
        ctx: &mut Context,
        assets: &Assets,
        x: i32,
        y: i32,
    ) {
        let chunk = Chunk {
            x,
            y,
            entity_hitbox: EntityBitVec::default(),
            tiles: ChunkTileset::random(ctx, assets),
        };
        chunks.insert(ChunkCoord { x, y }, chunk);
    }
}

pub struct ChunkTileset {
    #[allow(dead_code)] // will be used in the future
    tiles: Vec<Tile>,
    #[allow(dead_code)] // will be used in the future
    hitbox: BitVec,
    canvas: graphics::Canvas,
}

enum Tile {
    Grass,
    Water,
}

impl ChunkTileset {
    pub fn random(ctx: &mut Context, assets: &Assets) -> Self {
        let tiles: Vec<_> = (0..CHUNK_TILE_COUNT)
            .map(|_| {
                if rand::random() {
                    Tile::Grass
                } else {
                    Tile::Water
                }
            })
            .collect();

        let hitbox = tiles
            .iter()
            .map(|t| match t {
                Tile::Grass => true,
                _ => false,
            })
            .collect();

        let canvas = graphics::Canvas::new(
            ctx,
            CHUNK_TILE_WIDTH as i32 * TILE_SIZE_IN_PX,
            CHUNK_TILE_HEIGHT as i32 * TILE_SIZE_IN_PX,
        )
        .unwrap();
        graphics::set_canvas(ctx, &canvas);
        for (index, tile) in tiles.iter().enumerate() {
            let x = (index % CHUNK_TILE_WIDTH) as i32 * TILE_SIZE_IN_PX;
            let y = (index / CHUNK_TILE_WIDTH) as i32 * TILE_SIZE_IN_PX;
            let texture = match tile {
                Tile::Grass => &assets.grass,
                Tile::Water => &assets.water,
            };
            graphics::draw(
                ctx,
                texture,
                graphics::DrawParams::new().position((x as f32, y as f32).into()),
            );
        }
        graphics::reset_canvas(ctx);

        Self {
            tiles,
            hitbox,
            canvas,
        }
    }
}
