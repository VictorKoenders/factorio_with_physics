use crate::units::{SpecificHeatCapacity, ThermalConductivity};
use noisy_float::types::r32;

pub trait Material {
    fn specific_heat_capacity() -> SpecificHeatCapacity;
    fn thermal_conductivity() -> ThermalConductivity;
}

pub struct Steel;

impl Material for Steel {
    fn specific_heat_capacity() -> SpecificHeatCapacity {
        SpecificHeatCapacity(r32(502.416))
    }
    fn thermal_conductivity() -> ThermalConductivity {
        ThermalConductivity(r32(0.0))
    }
}
