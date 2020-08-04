use super::Entity;

/// Vector of properties associated with an entity.
/// This is more efficient than [EntityMap] when a lot of entities have this
// property and if the property does not take a lot of memory.
pub struct EntityVec<T> {
    inner: Vec<Option<T>>,
}

impl<T> Default for EntityVec<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T> EntityVec<T> {
    pub fn set(&mut self, entity: Entity, val: T) -> Option<T> {
        let idx = entity.0 as usize;
        let desired_len = idx + 1;
        if self.inner.len() < desired_len {
            self.inner.resize_with(desired_len, || None);
        }
        self.inner[idx].replace(val)
    }

    #[allow(dead_code)] // will be used in the future
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        let idx = entity.0 as usize;
        self.inner.get(idx).and_then(|t| t.as_ref())
    }

    #[allow(dead_code)] // will be used in the future
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        let idx = entity.0 as usize;
        self.inner.get_mut(idx).and_then(|t| t.as_mut())
    }

    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        let idx = entity.0 as usize;
        if let Some(prop_opt) = self.inner.get_mut(idx) {
            let val = prop_opt.take();
            while let Some(entry) = self.inner.last() {
                if entry.is_none() {
                    self.inner.pop();
                }
            }
            val
        } else {
            None
        }
    }

    #[allow(dead_code)] // will be used in the future
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.inner.iter().enumerate().filter_map(|(i, val)| {
            if let Some(val) = val {
                Some((Entity(i as u64), val))
            } else {
                None
            }
        })
    }

    #[allow(dead_code)] // will be used in the future
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.inner.iter_mut().enumerate().filter_map(|(i, val)| {
            if let Some(val) = val {
                Some((Entity(i as u64), val))
            } else {
                None
            }
        })
    }
}
