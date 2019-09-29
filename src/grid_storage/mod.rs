mod ext;
mod iterator;
mod position;

pub use self::ext::GridStorageExt;
pub use self::iterator::EntityPositionIterator;
pub use self::position::Position;

use noisy_float::prelude::Float;
use specs::hibitset::BitSetLike;
use specs::storage::{FlaggedStorage, UnprotectedStorage};
use specs::world::Index;
use specs::VecStorage;

/// Grid storage
///
/// This is a wrapper around a [VecStorage](specs::VecStorage), and is meant to be the storage system for an [Position].
///
/// In addition, this storage allows you to quickly look up all entities by position, as well as finding entities near other entities.
pub struct GridStorage {
    inner: FlaggedStorage<Position, VecStorage<Position>>,
    entities: Vec<Vec<Index>>,
    width: usize,
    height: usize,
}

impl specs::storage::TryDefault for GridStorage {
    fn try_default() -> Result<Self, String> {
        Err(String::from(
            "Could not manually create a grid storage; we don't have a size!",
        ))
    }
}

impl GridStorage {
    pub fn with_size(width: usize, height: usize) -> Self {
        GridStorage {
            inner: FlaggedStorage::default(),
            entities: vec![Vec::new(); width * height],
            width,
            height,
        }
    }

    /// Get the entity position by index. This is unsafe because the caller has to verify that the index is valid
    pub(self) unsafe fn get_by_index(&self, index: Index) -> Position {
        *self.inner.get(index)
    }

    pub(self) fn get_indices_on_tile(&self, position: Position) -> &[Index] {
        let index = self.position_to_index(position);

        // This is safe because self.position_to_index should always return a valid index.
        // In release mode this could cause UB, but we assume this has been properly tested in debug
        unsafe { self.entities.get_unchecked(index) }
    }

    pub(self) fn is_in_bounds(&self, position: Position) -> bool {
        !(position.x < 0.0
            || position.y < 0.0
            || position.x > (self.width - 1) as f32
            || position.y > (self.height - 1) as f32)
    }

    pub(self) fn position_to_index(&self, position: Position) -> usize {
        let (x, y) = (
            position.x.floor().raw() as isize,
            position.y.floor().raw() as isize,
        );
        let x = x.max(0).min((self.width as isize) - 1) as usize;
        let y = y.max(0).min((self.height as isize) - 1) as usize;

        let arr_index = y * self.width + x;
        if cfg!(debug_assertions) && self.entities.len() <= arr_index {
            panic!(
                "Tried to get an x/y that was not in range of the current grid (requested {}/{})",
                x, y
            );
        }

        arr_index
    }
}

impl UnprotectedStorage<Position> for GridStorage {
    unsafe fn clean<B>(&mut self, has: B)
    where
        B: BitSetLike,
    {
        self.inner.clean(has);
    }

    unsafe fn get(&self, id: specs::world::Index) -> &Position {
        let result = self.inner.get(id);
        result
    }

    unsafe fn get_mut(&mut self, id: specs::world::Index) -> &mut Position {
        self.inner.get_mut(id)
    }

    unsafe fn insert(&mut self, id: Index, v: Position) {
        self.inner.insert(id, v);

        let index = self.position_to_index(v);
        self.entities[index].push(id);
    }

    unsafe fn remove(&mut self, id: Index) -> Position {
        self.inner.remove(id)
    }
}
