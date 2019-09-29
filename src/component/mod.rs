mod heat;

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
