use super::GridStorage;
use noisy_float::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: R32,
    pub y: R32,
}

impl specs::Component for Position {
    type Storage = GridStorage;
}

impl Position {
    pub fn floor_usize(self) -> (usize, usize) {
        let (x, y) = (self.x.floor().raw() as isize, self.y.floor().raw() as isize);
        let x = x.max(0) as usize;
        let y = y.max(0) as usize;
        (x, y)
    }
}
