use super::Entity;
use std::collections::HashMap;

/// Map of entity properties. This is more efficient when the property is not that common or takes up a lot of memory.
pub struct EntityMap<T> {
    inner: HashMap<Entity, T>,
}

impl<T> Default for EntityMap<T> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<T> EntityMap<T> {
    pub fn set(&mut self, entity: Entity, val: T) -> Option<T> {
        self.inner.insert(entity, val)
    }

    #[allow(dead_code)] // will be used in the future
    pub fn remove(&mut self, entity: &Entity) -> Option<T> {
        self.inner.remove(&entity)
    }

    #[allow(dead_code)] // will be used in the future
    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.inner.iter().map(|(i, val)| (i.clone(), val))
    }

    #[allow(dead_code)] // will be used in the future
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.inner.iter_mut().map(|(i, val)| (i.clone(), val))
    }
}
