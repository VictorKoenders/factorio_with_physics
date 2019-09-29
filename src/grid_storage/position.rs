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
