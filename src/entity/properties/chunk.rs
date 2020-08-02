use super::Position;
use legion::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
}

impl Chunk {
    pub const CHUNK_SIZE_IN_PX: f32 = 32. * 32.;

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn query_in_range<'a>(
        world: &'a World,
        position: &Position,
        range: f32,
        mut cb: impl FnMut(Entity),
    ) {
        let min_chunk = Position {
            x: position.x - range,
            y: position.y - range,
        };
        let max_chunk = Position {
            x: position.x + range,
            y: position.y + range,
        };

        let range_squared = range.sqrt();

        Self::query_between(world, min_chunk, max_chunk, |pos, e| {
            if pos.distance_to_squared(&position) <= range_squared {
                cb(e);
            }
        });
    }

    pub fn query_between<'a>(
        world: &'a World,
        min: Position,
        max: Position,
        mut cb: impl for<'b> FnMut(&'b Position, Entity),
    ) {
        let min = min.chunk();
        let max = max.chunk();
        let mut chunk_count = 0;
        let mut entity_count = 0;
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let chunk = Chunk { x, y };
                let query = <Read<Position>>::query().filter(tag_value(&chunk));
                for (entity, position) in query.iter_entities_immutable(world) {
                    cb(position.as_ref(), entity);
                    entity_count += 1;
                }
                chunk_count += 1;
            }
        }
        println!(
            "Checked {} chunks and {} entities",
            chunk_count, entity_count
        );
    }
}
