use crate::units::{SpecificHeatCapacity, ThermalConductivity};
use noisy_float::types::r32;

pub trait Material {
    fn specific_heat_capacity() -> SpecificHeatCapacity;
    fn thermal_conductivity() -> ThermalConductivity;
}

pub struct Steel;

impl Material for Steel {
    fn specific_heat_capacity() -> SpecificHeatCapacity {
        SpecificHeatCapacity(r32(490.0))
    }
    fn thermal_conductivity() -> ThermalConductivity {
        ThermalConductivity(r32(54.0))
    }
}

pub struct Water;

impl Material for Water {
    fn specific_heat_capacity() -> SpecificHeatCapacity {
        SpecificHeatCapacity(r32(4179.0))
    }
    fn thermal_conductivity() -> ThermalConductivity {
        ThermalConductivity(r32(0.609))
    }
}
