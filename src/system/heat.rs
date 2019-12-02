use crate::component::{DeltaTime, Heat, Mass, StateChangeRequired};
use crate::units::{Joule, Meter};
use crate::{GridStorageExt, Position};
use noisy_float::types::{r32, R32};
use specs::prelude::*;

const HEAT_EQUATION_DISTANCE: Meter = Meter(unsafe { R32::const_unchecked_new(0.0001) });

pub struct HeatSystem;

impl<'a> System<'a> for HeatSystem {
    type SystemData = (
        ReadStorage<'a, Mass>,
        WriteStorage<'a, Heat>,
        WriteStorage<'a, StateChangeRequired>,
        ReadStorage<'a, Position>,
        Entities<'a>,
        ReadExpect<'a, DeltaTime>,
    );

    fn run(
        &mut self,
        (
            mass_storage,
            mut heat_storage,
            _state_change_required_storage,
            grid_storage,
            entities,
            delta_time,
        ): Self::SystemData,
    ) {
        let mut joules_updates = JoulesList::with_size(heat_storage.count());
        for (mass, heat, position, entity) in
            (&mass_storage, &heat_storage, &grid_storage, &entities).join()
        {
            if ((position.x + position.y) % 2.0 - 1.0).raw().abs() < std::f32::EPSILON {
                // we only update the even tiles, the odd tiles will automatically be updated because they're neighbours of the even tiles
                continue;
            }
            let temp = heat.temperature(*mass);

            for (other_entity, _) in grid_storage.neighbours(*position) {
                if let (Some(other_mass), Some(other_heat)) = (
                    mass_storage.get(other_entity),
                    heat_storage.get(other_entity),
                ) {
                    let mean_conductivity =
                        heat.conductivity.geometric_mean(other_heat.conductivity);
                    let other_temp = other_heat.temperature(*other_mass);
                    let temp_diff = temp - other_temp;
                    let transfer_per_second =
                        mean_conductivity * temp_diff / HEAT_EQUATION_DISTANCE;
                    let transfer_this_tick = transfer_per_second * delta_time.as_si();

                    joules_updates.add(entity, -transfer_this_tick);
                    joules_updates.add(other_entity, transfer_this_tick);
                }
            }
        }

        joules_updates.apply(&mut heat_storage, &entities);
    }
}

struct JoulesList {
    entities: Vec<Joule>,
}

impl JoulesList {
    pub fn with_size(n: usize) -> Self {
        Self {
            entities: Vec::with_capacity(n),
        }
    }
    pub fn add(&mut self, entity: Entity, joules: Joule) {
        let index = entity.id() as usize;
        if self.entities.len() <= index {
            self.entities.reserve(index - self.entities.len());
            while self.entities.len() <= index {
                self.entities.push(Joule(r32(0.0)));
            }
        }
        self.entities[index] += joules;
    }

    pub fn apply(self, storage: &mut WriteStorage<Heat>, entities: &Entities) {
        for (entity, heat) in (entities, storage).join() {
            let joules = self.entities[entity.id() as usize];
            if joules.0 != 0.0 {
                heat.joules += joules;
            }
        }
    }
}
