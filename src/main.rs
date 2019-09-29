#[macro_use]
pub mod macros;

pub mod component;
pub mod material;
pub mod system;
pub mod units;
pub mod utils;

mod grid_storage;

use crate::component::{DeltaTime, Heat, Mass, StateChangeRequired};
use crate::material::{Steel, Water};
use crate::units::Kelvin;
use noisy_float::types::r32;
use specs::prelude::*;

pub use crate::grid_storage::*;

fn main() {
    let mut world = World::empty();
    world.insert(specs::world::EntitiesRes::default());

    world.register_with_storage::<_, Position>(|| GridStorage::with_size(2, 2));
    world.register::<Mass>();
    world.register::<Heat>();
    world.register::<StateChangeRequired>();

    let mass = Mass(r32(1000.0));
    let temp = Kelvin(r32(700.0));
    world
        .create_entity()
        .with(mass)
        .with(Heat::from_material::<Steel>(temp, mass))
        .with(Position {
            x: r32(0.0),
            y: r32(0.0),
        })
        .build();

    let mass = Mass(r32(1000.0));
    let temp = Kelvin(r32(300.0));
    world
        .create_entity()
        .with(mass)
        .with(Heat::from_material::<Water>(temp, mass))
        .with(Position {
            x: r32(1.0),
            y: r32(0.0),
        })
        .build();
    print_world(&world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(crate::system::HeatSystem, "heat system", &[])
        .build();

    for _ in 0..10 {
        println!("Tick!");
        world.insert(DeltaTime::tick());
        dispatcher.dispatch(&world);
        print_world(&world);
    }
}

fn print_world(world: &World) {
    let (heat_storage, mass_storage): (ReadStorage<Heat>, ReadStorage<Mass>) = world.system_data();
    for (heat, mass) in (&heat_storage, &mass_storage).join() {
        println!("{:?}\t({:?})", heat.temperature(*mass), heat);
    }
}
