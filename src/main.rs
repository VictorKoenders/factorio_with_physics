#[macro_use]
pub mod macros;

pub mod component;
pub mod material;
pub mod sys;
pub mod system;
pub mod units;
pub mod utils;

mod grid_storage;

use crate::component::{DeltaTime, Heat, Mass, MaterialColor, StateChangeRequired};
use crate::material::{Steel, Water};
use crate::units::Kelvin;
use noisy_float::types::r32;
use specs::prelude::*;

pub use crate::grid_storage::*;

fn main() {
    let mut world = World::empty();
    world.insert(specs::world::EntitiesRes::default());

    world.register_with_storage::<_, Position>(|| GridStorage::with_size(2, 2));
    world.register::<MaterialColor>();
    world.register::<Mass>();
    world.register::<Heat>();
    world.register::<StateChangeRequired>();

    let mass = Mass(r32(1000.0));
    let temp = Kelvin(r32(700.0));
    world
        .create_entity()
        .with(mass)
        .with(MaterialColor::steel())
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
        .with(MaterialColor::water())
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

    let mut window = crate::sys::Window::new();
    let mut running = true;

    let mut last_update_time = std::time::Instant::now();

    while running {
        window.update(|e| match e {
            crate::sys::Event::Quit => running = false,
        });

        world.insert(DeltaTime::from_elapsed(&mut last_update_time));
        dispatcher.dispatch(&world);

        window.clear();
        world.exec(
            |(material_color, position): (ReadStorage<MaterialColor>, ReadStorage<Position>)| {
                for (color, position) in (&material_color, &position).join() {
                    window.draw_material(*color, *position);
                }
            },
        );
        window.present();
    }
}

fn print_world(world: &World) {
    let (heat_storage, mass_storage): (ReadStorage<Heat>, ReadStorage<Mass>) = world.system_data();
    for (heat, mass) in (&heat_storage, &mass_storage).join() {
        println!("{:?}\t({:?})", heat.temperature(*mass), heat);
    }
}
