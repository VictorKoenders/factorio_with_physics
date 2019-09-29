#[macro_use]
pub mod macros;

pub mod component;
pub mod material;
pub mod system;
pub mod units;
pub mod utils;

mod grid_storage;

use crate::units::*;
use noisy_float::types::r32;

pub use crate::grid_storage::*;

fn main() {
    let a = Meter(r32(2.0));
    let b = Meter(r32(2.0));
    let squared: MeterSquared = a * b;
    println!("{:?} * {:?} = {:?}", a, b, squared);
    // 2 m * 2 m = 4 m²
}
