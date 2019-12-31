use crate::material::Material;
use crate::units::{Joule, Kelvin, Kilogram, SpecificHeatCapacity, ThermalConductivity};
use specs::{Component, VecStorage};

#[derive(Debug, Copy, Clone)]
pub struct Heat {
    pub capacity: SpecificHeatCapacity,
    pub conductivity: ThermalConductivity,
    pub joules: Joule,
}

impl Component for Heat {
    type Storage = VecStorage<Self>;
}

impl Heat {
    pub fn from_material_specs(
        temp: Kelvin,
        mass: Kilogram,
        specific_heat_capacity: SpecificHeatCapacity,
        thermal_conductivity: ThermalConductivity,
    ) -> Self {
        let joules: Joule = specific_heat_capacity * mass * temp;
        Heat {
            capacity: specific_heat_capacity,
            conductivity: thermal_conductivity,
            joules,
        }
    }

    pub fn from_material<M: Material>(mat: M, temp: Kelvin, mass: Kilogram) -> Self {
        let capacity = mat.specific_heat_capacity();
        let conductivity = mat.thermal_conductivity();
        Self::from_material_specs(temp, mass, capacity, conductivity)
    }

    pub fn temperature(self, mass: Kilogram) -> Kelvin {
        self.joules / (self.capacity * mass)
    }
}
