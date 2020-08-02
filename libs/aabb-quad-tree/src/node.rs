use crate::{Bounds, AABB};
pub enum Node<T: AABB> {
    End([Option<T>; 4]),
    Nested([Option<Box<Node<T>>>; 4]),
}

impl<T: AABB> Default for Node<T> {
    fn default() -> Self {
        Self::End([None, None, None, None])
    }
}

impl<T: AABB> Node<T> {
    pub fn insert(&mut self, val: T, val_bounds: Bounds, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                if let Some(empty_index) = slice.iter().position(Option::is_none) {
                    slice[empty_index] = Some(val);
                    return;
                } else {
                    // If we insert 4 nodes with the exact same midpoint, the quad tree will turn into an infinite loop and overflow it's stack
                    // We'll check here if that happens and panic otherwise
                    assert!(
                        slice
                            .iter()
                            .all(|s| s.as_ref().unwrap().bounds().mid() != val_bounds.mid()),
                        "Tried inserting too many points with the same midpoint"
                    );
                    self.to_nested_with_new_value(val, val_bounds, node_bounds);
                    return;
                }
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection_for_bounds(&val_bounds);
                if slice[pos].is_none() {
                    slice[pos] = Some(Box::default());
                }
                slice[pos]
                    .as_mut()
                    .unwrap()
                    .insert(val, val_bounds, node_bounds);
            }
        }
    }
    pub fn remove(&mut self, val: &T, val_bounds: Bounds, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                for s in slice.iter_mut() {
                    if let Some(v) = s {
                        if v.is_eq(val) {
                            *s = None;
                            return;
                        }
                    }
                }
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection_for_bounds(&val_bounds);
                if let Some(sub) = &mut slice[pos] {
                    sub.remove(val, val_bounds, node_bounds);
                    if sub.is_empty() {
                        slice[pos] = None;
                        if slice.iter().all(Option::is_none) {
                            *self = Node::End([None, None, None, None]);
                        }
                    }
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Node::End(slice) => slice.iter().all(Option::is_none),
            Node::Nested(slice) => slice.iter().all(Option::is_none),
        }
    }

    fn assert_all_empty(&self) {
        match self {
            Node::End(_) => assert!(false),
            Node::Nested(slice) => assert!(slice.iter().all(|s| s.is_none())),
        }
    }

    fn to_nested_with_new_value(&mut self, val: T, val_bounds: Bounds, node_bounds: Bounds) {
        let values = std::mem::replace(self, Node::Nested([None, None, None, None]));
        self.assert_all_empty();
        let values = match values {
            Node::End(v) => v,
            Node::Nested(_) => {
                unreachable!("to_nested_with_new_value should only be called on a Node::End")
            }
        };
        // BlockedTODO: https://github.com/rust-lang/rust/issues/25725
        // Change this to .into_iter() once that's available

        for val in values.iter().cloned().filter_map(|v: Option<T>| v) {
            let val_bounds = val.bounds();
            self.insert(val, val_bounds, node_bounds.clone());
        }
        self.insert(val, val_bounds, node_bounds);
    }
}
