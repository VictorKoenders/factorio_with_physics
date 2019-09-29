use crate::component::{Heat, Mass, StateChangeRequired};
use crate::{GridStorageExt, Position};
use noisy_float::prelude::Float;
use specs::prelude::*;

pub struct HeatSystem;

impl<'a> System<'a> for HeatSystem {
    type SystemData = (
        ReadStorage<'a, Mass>,
        WriteStorage<'a, Heat>,
        WriteStorage<'a, StateChangeRequired>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mass, mut heat, mut state_change_required, grid): Self::SystemData) {
        for (mass, heat, _state_change_required, position) in
            (&mass, &mut heat, &mut state_change_required, &grid).join()
        {
            if ((position.x + position.y) % 2.0 - 1.0).abs() < std::f32::EPSILON {
                // We only update the even tiles, the odd tiles will automatically get updated by their neighbours
                continue;
            }
            println!("Updating tile at {:?} with: ", position);
            println!(" - Mass {:?}", mass);
            println!(" - Heat {:?}", heat);
            println!(" - With neighbours:");

            for neighbour in grid.neighbours(position) {
                println!("   - {:?}", neighbour);
            }
        }
    }
}
