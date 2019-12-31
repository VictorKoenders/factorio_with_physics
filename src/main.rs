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
use crate::grid_storage::*;
use crate::material::{Material, Steel, Water};
use crate::sys::{Event, WindowStateView};
use crate::units::Kelvin;
use noisy_float::types::r32;
use specs::prelude::*;

fn main() {
    let mut world = World::empty();
    world.insert(specs::world::EntitiesRes::default());

    let world_dimensions = (10, 10);

    world.register_with_storage::<_, Position>(|| {
        GridStorage::with_size(world_dimensions.0, world_dimensions.1)
    });
    world.register::<MaterialColor>();
    world.register::<Mass>();
    world.register::<Heat>();
    world.register::<StateChangeRequired>();

    for x in 0..world_dimensions.0 {
        for y in 0..world_dimensions.1 {
            let material: Box<dyn Material> = if rand::random::<bool>() {
                Box::new(Steel)
            } else {
                Box::new(Water)
            };
            material.build_entity(
                world.create_entity(),
                Kelvin::random(),
                Mass::random(),
                Position {
                    x: r32(x as f32),
                    y: r32(y as f32),
                },
            );
        }
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(crate::system::HeatSystem, "heat system", &[])
        .build();

    let mut window = crate::sys::Window::default();
    let mut running = true;

    let mut last_update_time = std::time::Instant::now();

    while running {
        window.update(|window, e| match e {
            Event::Quit => running = false,
            Event::Input(user_input) => user_input.apply(window, &mut world),
        });

        world.insert(DeltaTime::from_elapsed(&mut last_update_time));
        dispatcher.dispatch(&world);

        window.clear();

        match window.state().view {
            WindowStateView::HeatMap => world.exec(
                |(heat, mass, position): (
                    ReadStorage<Heat>,
                    ReadStorage<Mass>,
                    ReadStorage<Position>,
                )| {
                    let mut min = Kelvin::max_value();
                    let mut max = Kelvin::min_value();
                    for (heat, mass) in (&heat, &mass).join() {
                        let temp = heat.temperature(*mass);
                        if temp > max {
                            max = temp;
                        }
                        if temp < min {
                            min = temp;
                        }
                    }
                    min *= 0.9;
                    max *= 1.1;
                    for (heat, mass, position) in (&heat, &mass, &position).join() {
                        let temp = heat.temperature(*mass);
                        window.draw_temperature(temp, *position, min, max);
                    }
                },
            ),
            WindowStateView::MaterialMap => {
                world.exec(
                    |(material_color, position): (
                        ReadStorage<MaterialColor>,
                        ReadStorage<Position>,
                    )| {
                        for (color, position) in (&material_color, &position).join() {
                            window.draw_material(*color, *position);
                        }
                    },
                );
            }
        }
        window.present();
    }
}
