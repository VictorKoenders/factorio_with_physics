use crate::component::{Heat, Mass, MaterialColor};
use crate::grid_storage::Position;
use crate::units::{Kelvin, SpecificHeatCapacity, ThermalConductivity};
use noisy_float::types::r32;
use specs::prelude::*;

pub trait Material {
    fn specific_heat_capacity(&self) -> SpecificHeatCapacity;
    fn thermal_conductivity(&self) -> ThermalConductivity;
    fn material_color(&self) -> MaterialColor;

    fn build_entity(
        &self,
        builder: EntityBuilder,
        temperature: Kelvin,
        mass: Mass,
        position: Position,
    ) {
        builder
            .with(mass)
            .with(self.material_color())
            .with(Heat::from_material_specs(
                temperature,
                mass,
                self.specific_heat_capacity(),
                self.thermal_conductivity(),
            ))
            .with(position)
            .build();
    }
}

pub struct Steel;

impl Material for Steel {
    fn specific_heat_capacity(&self) -> SpecificHeatCapacity {
        SpecificHeatCapacity(r32(490.0))
    }
    fn thermal_conductivity(&self) -> ThermalConductivity {
        ThermalConductivity(r32(54.0))
    }
    fn material_color(&self) -> MaterialColor {
        MaterialColor::rgb(70, 130, 180)
    }
}

pub struct Water;

impl Material for Water {
    fn specific_heat_capacity(&self) -> SpecificHeatCapacity {
        SpecificHeatCapacity(r32(4179.0))
    }
    fn thermal_conductivity(&self) -> ThermalConductivity {
        ThermalConductivity(r32(0.609))
    }
    fn material_color(&self) -> MaterialColor {
        MaterialColor::rgb(235, 244, 250)
    }
}
