use super::{GridStorage, Position};
use specs::world::{EntitiesRes, Entity, Index};

pub struct EntityPositionIterator<'a> {
    starting_position: (usize, usize),
    offsets: &'static [(isize, isize)],
    grid: &'a GridStorage,
    entities: &'a EntitiesRes,
    current_tile_remaining: &'a [Index],
}

impl<'a> EntityPositionIterator<'a> {
    pub fn new(
        starting_position: (usize, usize),
        grid: &'a GridStorage,
        entities: &'a EntitiesRes,
        offsets: &'static [(isize, isize)],
    ) -> Self {
        Self {
            starting_position,
            offsets,
            grid,
            entities,
            current_tile_remaining: &[],
        }
    }

    fn lookup_index(&self, index: Index) -> Option<(Entity, Position)> {
        let entity = self.entities.entity(index);
        if cfg!(debug_assertions) && !self.entities.is_alive(entity) {
            eprintln!("EntityPositionIterator::lookup_index found an entity that was not alive!");
            return None;
        }

        // This _should_ be safe, because we checked that the index is a valid entity
        let position = unsafe { self.grid.get_by_index(index) };

        Some((entity, position))
    }
}

impl<'a> Iterator for EntityPositionIterator<'a> {
    type Item = (Entity, Position);

    fn next(&mut self) -> Option<(Entity, Position)> {
        'outer: loop {
            while !self.current_tile_remaining.is_empty() {
                let index = self.current_tile_remaining[0];
                self.current_tile_remaining = &self.current_tile_remaining[1..];
                if let Some(result) = self.lookup_index(index) {
                    return Some(result);
                }
            }

            while !self.offsets.is_empty() {
                let (x, y) = self.offsets[0];
                self.offsets = &self.offsets[1..];

                let x = (x + self.starting_position.0 as isize).max(0) as usize;
                let y = (y + self.starting_position.0 as isize).max(0) as usize;

                let indices = self.grid.get_indices_by_position(x, y);
                if !indices.is_empty() {
                    self.current_tile_remaining = indices;
                    continue 'outer;
                }
            }

            return None;
        }
    }
}
