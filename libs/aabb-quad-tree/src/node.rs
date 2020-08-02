use crate::{Bounds, AABB};
use smallvec::SmallVec;

pub enum Node<T: AABB> {
    End(SmallVec<[T; 4]>),
    Nested([Box<Node<T>>; 4]),
}

impl<T: AABB> Default for Node<T> {
    fn default() -> Self {
        Self::End(SmallVec::default())
    }
}

impl<T: AABB> Node<T> {
    pub fn insert(&mut self, val: T, val_bounds: Bounds, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                if slice.len() < slice.inline_size() {
                    slice.push(val);
                    return;
                } else {
                    // If we insert 4 nodes with the exact same midpoint, the quad tree will turn into an infinite loop and overflow it's stack
                    // We'll check here if that happens and panic otherwise
                    assert!(
                        slice.iter().all(|s| s.bounds().mid() != val_bounds.mid()),
                        "Tried inserting too many points with the same midpoint"
                    );
                    self.to_nested_with_new_value(val, val_bounds, node_bounds);
                    return;
                }
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection_for_bounds(&val_bounds);
                slice[pos].insert(val, val_bounds, node_bounds);
            }
        }
    }
    pub fn remove(&mut self, val: &T, val_bounds: Bounds, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                slice.retain(|s| !s.is_eq(val));
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection_for_bounds(&val_bounds);
                let sub = &mut slice[pos];
                sub.remove(val, val_bounds, node_bounds);

                if slice.iter().all(|s| s.is_empty()) {
                    *self = Node::End(SmallVec::new());
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Node::End(slice) => slice.is_empty(),
            Node::Nested(slice) => slice.iter().all(|s| s.is_empty()),
        }
    }

    fn to_nested_with_new_value(&mut self, val: T, val_bounds: Bounds, node_bounds: Bounds) {
        let values = std::mem::replace(self, Node::Nested(Default::default()));
        let values = match values {
            Node::End(v) => v,
            Node::Nested(_) => {
                unreachable!("to_nested_with_new_value should only be called on a Node::End")
            }
        };
        // BlockedTODO: https://github.com/rust-lang/rust/issues/25725
        // Change this to .into_iter() once that's available

        for val in values {
            let val_bounds = val.bounds();
            self.insert(val, val_bounds, node_bounds.clone());
        }
        self.insert(val, val_bounds, node_bounds);
    }
}
