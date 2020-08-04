use super::Entity;
use crate::world::{ChunkCoord, WorldCoord};

#[derive(Clone, Debug)]
pub enum HitboxRange {
    Rectangle(HitboxRectangle),
    Circle(HitboxCircle),
}

impl HitboxRange {
    fn min(&self, pos: WorldCoord) -> WorldCoord {
        match self {
            HitboxRange::Rectangle(rect) => pos - rect.top_left(),
            HitboxRange::Circle(circle) => pos - WorldCoord::new(circle.radius, circle.radius),
        }
    }
    fn max(&self, pos: WorldCoord) -> WorldCoord {
        match self {
            HitboxRange::Rectangle(rect) => pos + rect.bottom_right(),
            HitboxRange::Circle(circle) => pos + WorldCoord::new(circle.radius, circle.radius),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitboxRectangle {
    pub left: f32,
    pub up: f32,
    pub right: f32,
    pub down: f32,
}

impl HitboxRectangle {
    fn top_left(&self) -> (f32, f32) {
        (self.left, self.up)
    }
    fn bottom_right(&self) -> (f32, f32) {
        (self.right, self.down)
    }
}

impl Into<HitboxRange> for HitboxRectangle {
    fn into(self) -> HitboxRange {
        HitboxRange::Rectangle(self)
    }
}

#[derive(Clone, Debug)]
pub struct HitboxCircle {
    pub radius: f32,
}

impl Into<HitboxRange> for HitboxCircle {
    fn into(self) -> HitboxRange {
        HitboxRange::Circle(self)
    }
}

#[derive(Clone, Debug)]
pub struct EntityHitbox {
    pub entity: Entity,
    pub hitbox: HitboxRange,
    pub position: WorldCoord,
}

impl EntityHitbox {
    pub fn new(entity: Entity, hitbox: HitboxRange, position: WorldCoord) -> Self {
        Self {
            entity,
            hitbox,
            position,
        }
    }

    pub fn chunks(&self) -> impl Iterator<Item = ChunkCoord> {
        let min: ChunkCoord = self.hitbox.min(self.position).into();
        let max: ChunkCoord = self.hitbox.max(self.position).into();

        (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ChunkCoord { x, y }))
    }
    pub fn chunks_diff_from(&self, old: &EntityHitbox) -> impl Iterator<Item = CoordDiff> {
        let self_min: ChunkCoord = self.hitbox.min(self.position).into();
        let self_max: ChunkCoord = self.hitbox.max(self.position).into();

        let old_min: ChunkCoord = old.hitbox.min(old.position).into();
        let old_max: ChunkCoord = old.hitbox.max(old.position).into();

        let abs_min = ChunkCoord {
            x: self_min.x.min(old_min.x),
            y: self_min.y.min(old_min.y),
        };

        let abs_max = ChunkCoord {
            x: self_max.x.max(old_max.x),
            y: self_max.y.max(old_max.y),
        };

        let mut result = Vec::new();
        for x in abs_min.x..=abs_max.x {
            for y in abs_min.y..=abs_max.y {
                let self_contains_chunk =
                    self_min.x <= x && self_max.x >= x && self_min.y >= y && self_max.y <= y;
                let old_contains_chunk =
                    old_min.x <= x && old_max.x >= x && old_min.y >= y && old_max.y <= y;

                if self_contains_chunk && !old_contains_chunk {
                    result.push(CoordDiff::Added(ChunkCoord { x, y }));
                } else if !self_contains_chunk && old_contains_chunk {
                    result.push(CoordDiff::Removed(ChunkCoord { x, y }));
                }
            }
        }
        result.into_iter()
    }
}

#[derive(Clone)]
pub enum CoordDiff {
    Added(ChunkCoord),
    Removed(ChunkCoord),
}
