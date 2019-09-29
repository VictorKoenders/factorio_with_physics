mod heat;

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
