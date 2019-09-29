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
    pub fn from_material<M: Material>(temp: Kelvin, mass: Kilogram) -> Self {
        let capacity = M::specific_heat_capacity();
        let conductivity = M::thermal_conductivity();
        let joules: Joule = capacity * mass * temp;
        Heat {
            capacity,
            conductivity,
            joules,
        }
    }

    pub fn temperature(self, mass: Kilogram) -> Kelvin {
        self.joules / (self.capacity * mass)
    }
}
