use noisy_float::types::R32;

pub struct Seconds(pub R32);
macros::unit!(Seconds: "s");

pub struct Meter(pub R32);
macros::unit!(Meter: "m");

pub struct Kilogram(pub R32);
macros::unit!(Kilogram: "kg");

pub struct Ampere(pub R32);
macros::unit!(Ampere: "A");

pub struct Kelvin(pub R32);
macros::unit!(Kelvin: "K");

pub struct Mole(pub R32);
macros::unit!(Mole: "mol");

pub struct Candela(pub R32);
macros::unit!(Candela: "cd");
