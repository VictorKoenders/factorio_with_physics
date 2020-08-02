use super::Chunk;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn chunk(&self) -> Chunk {
        Chunk {
            x: (self.x / Chunk::CHUNK_SIZE_IN_PX).floor() as i32,
            y: (self.y / Chunk::CHUNK_SIZE_IN_PX).floor() as i32,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-10000., 10000.);
        let y = rng.gen_range(-10000., 10000.);

        Self { x, y }
    }

    pub fn distance_to_squared(&self, other: &Self) -> f32 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)
    }
}

#[test]
fn test_chunk_rounding() {
    assert_eq!(Chunk::new(0, 0), Position::new(0.0, 0.0).chunk());
    assert_eq!(Chunk::new(0, 0), Position::new(10.0, 10.0).chunk());
    assert_eq!(Chunk::new(3, 3), Position::new(100.0, 100.0).chunk());
    assert_eq!(Chunk::new(-1, -1), Position::new(-0.1, -0.1).chunk());
}
