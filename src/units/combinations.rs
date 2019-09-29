use super::si::*;
use noisy_float::prelude::Float;
use noisy_float::types::R32;

pub struct MeterSquared(pub R32);
macros::unit!(MeterSquared: "m²");
macros::conversion!(Meter * Meter => MeterSquared);

pub struct KgMeter(pub R32);
macros::unit!(KgMeter: "kg×m");
macros::conversion!(Kilogram * Meter => KgMeter);

pub struct KgMeterSquared(pub R32);
macros::unit!(KgMeterSquared: "kg×m²");
macros::conversion!(Kilogram * MeterSquared => KgMeterSquared);
macros::conversion!(MeterSquared * Kilogram => KgMeterSquared);

pub struct SecondsSquared(pub R32);
macros::unit!(SecondsSquared: "s²");
macros::conversion!(Seconds * Seconds => SecondsSquared);

pub struct Joule(pub R32);
macros::unit!(Joule: "J");
macros::conversion!(KgMeterSquared / SecondsSquared => Joule);

pub struct Watt(pub R32);
macros::unit!(Watt: "W");
macros::conversion!(Joule / Seconds => Watt);

pub struct Newton(pub R32);
macros::unit!(Newton: "N");
macros::conversion!(KgMeter / Seconds => Newton);

pub struct MeterPerSecondSquared(pub R32);
macros::unit!(MeterPerSecondSquared: "m/s²");
macros::conversion!(Meter / SecondsSquared => MeterPerSecondSquared);

// const_unchecked_new is only unsafe because it requires the caller to pass in a non-null and non-inf float value
// Because we pass a constant value of 9.82, this is actually safe
pub const GRAVITY: MeterPerSecondSquared =
    MeterPerSecondSquared(unsafe { R32::const_unchecked_new(9.82) });

pub struct HeatCapacity(pub R32);
macros::unit!(HeatCapacity: "J/K");
macros::conversion!(Joule / Kelvin => HeatCapacity);

pub struct SpecificHeatCapacity(pub R32);
macros::unit!(SpecificHeatCapacity: "J/(K Kg)");
macros::conversion!(HeatCapacity * Kilogram => SpecificHeatCapacity);

impl SpecificHeatCapacity {
    pub fn geometric_mean(self, other: SpecificHeatCapacity) -> SpecificHeatCapacity {
        SpecificHeatCapacity((self.0 * other.0).sqrt())
    }
}

pub struct ThermalConductivity(pub R32);
macros::unit!(ThermalConductivity: "W/K");
