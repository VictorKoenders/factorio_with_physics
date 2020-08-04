use noisy_float::types::{r32, R32};
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl From<WorldCoord> for ChunkCoord {
    fn from(coord: WorldCoord) -> ChunkCoord {
        ChunkCoord {
            x: coord.x.raw() as i32,
            y: coord.y.raw() as i32,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorldCoord {
    pub x: R32,
    pub y: R32,
}

impl WorldCoord {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: r32(x),
            y: r32(y),
        }
    }
}

impl From<(f32, f32)> for WorldCoord {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl ops::Sub for WorldCoord {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Add for WorldCoord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<(f32, f32)> for WorldCoord {
    type Output = Self;
    fn sub(self, other: (f32, f32)) -> Self {
        Self {
            x: self.x - other.0,
            y: self.y - other.1,
        }
    }
}

impl ops::Add<(f32, f32)> for WorldCoord {
    type Output = Self;
    fn add(self, other: (f32, f32)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}
