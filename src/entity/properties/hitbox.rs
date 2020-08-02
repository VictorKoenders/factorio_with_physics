use super::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Hitbox {
    pub left: f32,
    pub right: f32,
    pub up: f32,
    pub down: f32,
}

impl Hitbox {
    pub fn has_point(&self, position: &Position, target: &Position) -> bool {
        position.x - self.left <= target.x
            && position.x + self.right >= target.x
            && position.y - self.up <= target.y
            && position.y + self.down >= target.y
    }
}
