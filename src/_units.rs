use noisy_float::types::{n32, N32};

macro_rules! impl_convert {
    ($ty:ident from numerics) => {
        impl From<i32> for $ty {
            fn from(i: i32) -> Self {
                Self(n32(i as f32))
            }
        }
        impl From<f32> for $ty {
            fn from(i: f32) -> Self {
                Self(n32(i))
            }
        }
    };
    ($outer:ident from $inner:ident) => {
        impl From<$inner> for $outer {
            fn from(val: $inner) -> $outer {
                $outer(val.0)
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Joules(N32);
impl_convert!(Joules from numerics);

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    kelvin: N32,
}

impl std::ops::Add<Temperature> for Temperature {
    type Output = Temperature;
    fn add(self, other: Temperature) -> Temperature {
        Temperature {
            kelvin: self.kelvin + other.kelvin,
        }
    }
}

impl std::ops::Neg for Temperature {
    type Output = Temperature;
    fn neg(self) -> Temperature {
        Temperature {
            kelvin: -self.kelvin,
        }
    }
}

impl std::ops::AddAssign for Temperature {
    fn add_assign(&mut self, other: Temperature) {
        self.kelvin += other.kelvin;
    }
}

impl std::ops::Sub<Temperature> for Temperature {
    type Output = Temperature;
    fn sub(self, other: Temperature) -> Temperature {
        Temperature {
            kelvin: self.kelvin - other.kelvin,
        }
    }
}

impl std::ops::SubAssign for Temperature {
    fn sub_assign(&mut self, other: Temperature) {
        self.kelvin -= other.kelvin;
    }
}

impl Temperature {
    pub fn kelvin(val: N32) -> Temperature {
        Temperature { kelvin: val }
    }
    pub fn celsius(val: N32) -> Temperature {
        Temperature {
            kelvin: val + n32(273.15),
        }
    }
}

#[derive(Debug, Clone, Copy)]
// Heat capacity, in joules, in joules / K
pub struct HeatCapacity(N32);
impl_convert!(HeatCapacity from Joules);

impl std::ops::Mul<Joules> for HeatCapacity {
    type Output = Temperature;

    fn mul(self, other: Joules) -> Temperature {
        Temperature {
            kelvin: n32(1.0) / (self.0 * other.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
// Thermal conductivity in, in joules / (Area * s)
pub struct ThermalConductivity(N32);

impl ThermalConductivity {
    pub fn watts_per_meter(f: N32) -> Self {
        Self(f)
    }
    pub fn joules_per_sq_meter(f: N32) -> Self {
        Self(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    meters: N32,
}

impl Distance {
    pub fn tile_edge() -> Self {
        Self {
            meters: n32(0.00001),
        }
    }
}

impl std::ops::Mul<Distance> for Distance {
    type Output = Area;
    fn mul(self, other: Distance) -> Area {
        Area {
            sq_meters: self.meters * other.meters,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    sq_meters: N32,
}

impl Area {
    pub fn tile() -> Self {
        Self {
            sq_meters: n32(1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
// In joules per second
pub struct HeatFlow(N32);

impl HeatFlow {
    pub fn calculate(
        surface_area: Area,
        distance: Distance,
        from: (ThermalConductivity, Temperature),
        to: (ThermalConductivity, Temperature),
    ) -> Self {
        let conductivity_joules_per_sq_meter = if (from.0).0 > (to.0).0 {
            (from.0).0
        } else {
            (to.0).0
        };
        let from_temp = from.1;
        let to_temp = to.1;
        Self(
            -conductivity_joules_per_sq_meter
                * surface_area.sq_meters
                * ((from_temp.kelvin - to_temp.kelvin) / distance.meters),
        )
    }
}

impl std::ops::Mul<DeltaTime> for HeatFlow {
    type Output = Joules;
    fn mul(self, other: DeltaTime) -> Joules {
        Joules(self.0 * other.0)
    }
}

#[derive(Debug, Clone, Copy)]
// Delta time, in seconds
pub struct DeltaTime(N32);

impl DeltaTime {
    pub fn seconds(s: N32) -> Self {
        Self(s)
    }
}
