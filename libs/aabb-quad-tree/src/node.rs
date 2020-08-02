use crate::{bounds::Coord, Bounds, AABB};
use smallvec::SmallVec;

const NODE_COUNT: usize = 16;

pub enum Node<T: AABB> {
    End(SmallVec<[NodeInner<T>; NODE_COUNT]>),
    Nested([Box<Node<T>>; 4]),
}

pub struct NodeInner<T: AABB> {
    pub id: T::ID,
    pub coord: Coord,
}

impl<T: AABB> Default for Node<T> {
    fn default() -> Self {
        Self::End(SmallVec::default())
    }
}

impl<T: AABB> Node<T> {
    pub fn insert(&mut self, node: NodeInner<T>, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                if slice.len() < slice.inline_size() {
                    slice.push(node);
                    debug_assert!(!slice.spilled());
                    return;
                } else {
                    // If we insert `NODE_COUNT` nodes with the exact same midpoint, the quad tree will turn into an infinite loop and overflow it's stack
                    // We'll check here if that happens and panic otherwise
                    assert!(
                        slice.iter().all(|s| s.coord != node.coord),
                        "Tried inserting too many points with the same midpoint"
                    );
                    self.to_nested_with_new_value(node, node_bounds);
                    return;
                }
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection(node.coord);
                slice[pos].insert(node, node_bounds);
            }
        }
    }
    pub fn remove(&mut self, id: &T::ID, coord: Coord, node_bounds: Bounds) {
        match self {
            Node::End(slice) => {
                slice.retain(|node| &node.id != id);
            }
            Node::Nested(slice) => {
                let (node_bounds, pos) = node_bounds.get_subsection(coord);
                let sub = &mut slice[pos];
                sub.remove(id, coord, node_bounds);

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

    fn to_nested_with_new_value(&mut self, node: NodeInner<T>, node_bounds: Bounds) {
        let values = std::mem::replace(self, Node::Nested(Default::default()));
        let values = match values {
            Node::End(v) => v,
            Node::Nested(_) => {
                unreachable!("to_nested_with_new_value should only be called on a Node::End")
            }
        };
        // BlockedTODO: https://github.com/rust-lang/rust/issues/25725
        // Change this to .into_iter() once that's available

        for node in values {
            self.insert(node, node_bounds.clone());
        }
        self.insert(node, node_bounds);
    }
}
