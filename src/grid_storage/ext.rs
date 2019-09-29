use super::{EntityPositionIterator, Position};
use specs::storage::{MaskedStorage, Storage};
use std::ops::Deref;

pub trait GridStorageExt {
    fn neighbours<'a>(&'a self, position: &Position) -> EntityPositionIterator<'a>;
}

impl<'b, D> GridStorageExt for Storage<'b, Position, D>
where
    D: Deref<Target = MaskedStorage<Position>>,
{
    fn neighbours<'a>(&'a self, position: &Position) -> EntityPositionIterator<'a> {
        let grid = self.unprotected_storage();
        let entities = self.fetched_entities();
        let offsets = &[(-1, 0), (0, -1), (1, 0), (0, 1)];

        EntityPositionIterator::new(position.floor_usize(), grid, entities, offsets)
    }
}
