mod heat;

use crate::sys::Color;
use noisy_float::types::{r32, R32};

pub use self::heat::*;
pub use crate::units::Kilogram as Mass;

impl specs::Component for Mass {
    type Storage = specs::DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct StateChangeRequired;

impl specs::Component for StateChangeRequired {
    type Storage = specs::NullStorage<Self>;
}

#[derive(Debug, Clone, Copy)]
// Delta time, in seconds
pub struct DeltaTime(R32);

impl DeltaTime {
    pub fn from_elapsed(instant: &mut std::time::Instant) -> DeltaTime {
        let result = Self::seconds(R32::new(instant.elapsed().as_secs_f32()));
        *instant = std::time::Instant::now();
        result
    }
    pub fn seconds(s: R32) -> Self {
        Self(s)
    }

    pub fn tick() -> Self {
        Self(r32(1.0 / 10.0))
    }

    pub fn as_si(self) -> crate::units::Seconds {
        crate::units::Seconds(self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialColor(pub Color);

impl MaterialColor {
    pub fn steel() -> MaterialColor {
        MaterialColor(Color::RGB(70, 130, 180))
    }
    pub fn water() -> MaterialColor {
        MaterialColor(Color::RGB(235, 244, 250))
    }
}

impl specs::Component for MaterialColor {
    type Storage = specs::VecStorage<Self>;
}
