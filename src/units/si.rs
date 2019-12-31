use noisy_float::types::R32;

pub struct Seconds(pub R32);
macros::unit!(Seconds: "s");

pub struct Meter(pub R32);
macros::unit!(Meter: "m");

pub struct Kilogram(pub R32);
macros::unit!(Kilogram: "kg");

impl Kilogram {
    pub fn random() -> Self {
        use rand::{thread_rng, Rng};
        Self(R32::new(thread_rng().gen_range(0.0, 1000.0)))
    }
}

pub struct Ampere(pub R32);
macros::unit!(Ampere: "A");

pub struct Kelvin(pub R32);
macros::unit!(Kelvin: "K");

impl Kelvin {
    pub fn random() -> Self {
        use rand::{thread_rng, Rng};
        Self(R32::new(thread_rng().gen_range(0.0, 1000.0)))
    }
    pub fn one() -> Self {
        Self(R32::new(1.0))
    }
    pub fn min_value() -> Self {
        Self(R32::new(0.0))
    }
    pub fn max_value() -> Self {
        Self(R32::new(std::f32::MAX))
    }
}

pub struct Mole(pub R32);
macros::unit!(Mole: "mol");

pub struct Candela(pub R32);
macros::unit!(Candela: "cd");
