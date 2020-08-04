use bitvec::vec::BitVec;

mod entity_bitvec;
mod entity_map;
mod entity_vec;

pub mod hitbox;
pub mod miner;

pub use self::entity_bitvec::EntityBitVec;
pub use self::entity_map::EntityMap;
pub use self::entity_vec::EntityVec;

// TODO: Make this a reference-counted entity and automatically reclaim it when
// the last entry is reclaimed?
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Entity(u64);

#[derive(Default)]
pub struct Entities {
    entities: BitVec,
    reclaimed_ids: BitVec,
    next_id: u64,
}

impl Entities {
    pub fn create(&mut self) -> Entity {
        let entity_id = if !self.reclaimed_ids.is_empty() {
            // We have a reclaimed ID we can use instead of generating a new one
            let last_idx = self.reclaimed_ids.len() - 1;
            debug_assert!(self.reclaimed_ids[last_idx]); // last entry must always be `true`
            self.reclaimed_ids.pop();
            trim_end(&mut self.reclaimed_ids); // make sure to trim trailing `false`'s

            last_idx as u64
        } else {
            // no IDs to reclaim, grab the next_id, which should always be valid
            let entity_id = self.next_id;
            self.next_id += 1;
            entity_id
        };

        resize_and_set(&mut self.entities, entity_id as usize);
        Entity(entity_id)
    }

    #[allow(dead_code)] // will be used in the future
    pub fn reclaim(&mut self, entity: Entity) {
        let idx = entity.0 as usize;
        self.entities.set(idx, false);

        // make sure the entities list has no trailing `false` entries
        // TODO: is this actually needed? Can't we just keep `self.entities` at the max length?
        trim_end(&mut self.entities);

        resize_and_set(&mut self.reclaimed_ids, idx); // make sure the ID is properly reclaimed
    }
}

/// Resize the `vec` so it can set index `idx`, and set `idx` to `true`
fn resize_and_set(vec: &mut BitVec, idx: usize) {
    let desired_len = idx + 1; // len needs to be 1 past the index we're setting
    if vec.len() < desired_len {
        vec.resize_with(desired_len, || false);
    }
    vec.set(idx, true);
}

/// Remove all trailing `false` entries from the given bitvec
fn trim_end(vec: &mut BitVec) {
    while vec.last() == Some(&false) {
        vec.pop();
    }
}
