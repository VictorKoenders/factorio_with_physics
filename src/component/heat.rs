use crate::units::{Joule, SpecificHeatCapacity, ThermalConductivity};
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Heat {
    pub capacity: SpecificHeatCapacity,
    pub conductivity: ThermalConductivity,
    pub joules: Joule,
}

impl Component for Heat {
    type Storage = VecStorage<Self>;
}
