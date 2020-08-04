use super::Entity;
use bitvec::vec::BitVec;

/// Efficient counting on if an entity is set.
#[derive(Default)]
pub struct EntityBitVec {
    inner: BitVec,
}

impl EntityBitVec {
    pub fn set(&mut self, entity: Entity) {
        let idx = entity.0 as usize;
        let desired_len = idx + 1;
        if self.inner.len() < desired_len {
            self.inner.resize_with(desired_len, || false);
        }
        self.inner.set(idx, true);
    }

    #[allow(dead_code)] // will be used in the future
    pub fn is_set(&self, entity: Entity) -> bool {
        let idx = entity.0 as usize;
        self.inner.get(idx).copied().unwrap_or(false)
    }

    #[allow(dead_code)] // will be used in the future
    pub fn remove(&mut self, entity: &Entity) {
        let idx = entity.0 as usize;
        if self.inner.len() > idx {
            self.inner.set(idx, false);
            while self.inner.last() == Some(&false) {
                self.inner.pop();
            }
        }
    }
}
