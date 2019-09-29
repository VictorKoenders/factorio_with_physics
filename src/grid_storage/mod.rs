mod ext;
mod iterator;
mod position;

pub use self::ext::GridStorageExt;
pub use self::iterator::EntityPositionIterator;
pub use self::position::Position;

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
            entities: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    /// Get the entity position by index. This is unsafe because the caller has to verify that the index is valid
    pub(self) unsafe fn get_by_index(&self, index: Index) -> Position {
        *self.inner.get(index)
    }

    pub(self) fn get_indices_by_position(&self, x: usize, y: usize) -> &[Index] {
        if x >= self.width || y >= self.height {
            return &[];
        }
        let arr_index = y * self.width + x;
        if cfg!(debug_assertions) && self.entities.len() <= arr_index {
            panic!("Tried to get an x/y that was not in range of the current grid");
        }

        // This is safe because of the cfg check above
        // In release mode this could cause UB, but we assume this has been properly tested in debug
        unsafe { self.entities.get_unchecked(arr_index) }
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
        self.inner.get(id)
    }

    unsafe fn get_mut(&mut self, id: specs::world::Index) -> &mut Position {
        self.inner.get_mut(id)
    }

    unsafe fn insert(&mut self, id: Index, v: Position) {
        self.inner.insert(id, v);
    }

    unsafe fn remove(&mut self, id: Index) -> Position {
        self.inner.remove(id)
    }
}
