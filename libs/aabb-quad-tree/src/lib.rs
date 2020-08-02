mod bounds;
mod node;

pub use self::bounds::{Bounds, Coord};
use self::node::Node;
use node::NodeInner;
use std::{collections::HashMap, hash::Hash};

pub struct AABBQuadTree<T: AABB> {
    root: Node<T>,
    bounds: Bounds,
    entries: HashMap<T::ID, T>,
}
impl<T: AABB> AABBQuadTree<T> {
    pub fn new(bounds: Bounds) -> Self {
        Self {
            root: Node::default(),
            bounds,
            entries: HashMap::new(),
        }
    }
    pub fn insert(&mut self, t: T) {
        let val_bounds = t.bounds();
        let node_bounds = self.bounds.clone();
        if val_bounds.bottom_right.x > node_bounds.bottom_right.x
            || val_bounds.top_left.x < node_bounds.top_left.x
            || val_bounds.bottom_right.y > node_bounds.bottom_right.y
            || val_bounds.top_left.y < node_bounds.top_left.y
        {
            unimplemented!("Resizing not implemented yet");
        }
        let id = t.id();
        self.entries.insert(id.clone(), t);

        for coord in val_bounds.corners().iter().cloned() {
            self.root.insert(
                NodeInner {
                    id: id.clone(),
                    coord,
                },
                node_bounds,
            );
        }
    }

    pub fn remove(&mut self, t: &T) {
        let id = t.id();
        if self.entries.remove(&id).is_some() {
            let node_bounds = self.bounds.clone();
            for corner in t.bounds().corners().iter() {
                self.root.remove(&id, *corner, node_bounds);
            }
        }
    }

    pub fn dbg_rectangles(&self) -> impl IntoIterator<Item = Bounds> {
        let mut rects = Vec::new();
        get_bounds_recursive(&self.root, self.bounds.clone(), &mut rects);
        rects
    }

    pub fn dbg_entity_count(&self) -> usize {
        self.entries.len()
    }

    pub fn dbg_point_count(&self) -> usize {
        get_point_count_recursive(&self.root)
    }
}

fn get_bounds_recursive<T: AABB>(node: &Node<T>, bounds: Bounds, rects: &mut Vec<Bounds>) {
    const MARGIN: f32 = 2.0;
    rects.push(Bounds {
        top_left: bounds.top_left + Coord::broadcast(MARGIN),
        bottom_right: bounds.bottom_right - Coord::broadcast(MARGIN),
    });
    match node {
        Node::Nested(children) => {
            for (i, child) in children.iter().enumerate() {
                let bounds = bounds.section_by_index(i);
                get_bounds_recursive(child, bounds, rects);
            }
        }
        Node::End(_) => {}
    }
}

fn get_point_count_recursive<T: AABB>(node: &Node<T>) -> usize {
    match node {
        Node::Nested(children) => children.iter().map(|n| get_point_count_recursive(n)).sum(),
        Node::End(children) => children.len(),
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/25725
// Remove `Clone` bounds once this issue is resolved
pub trait AABB: Clone {
    type ID: Eq + Clone + Hash;

    fn bounds(&self) -> Bounds;
    fn id(&self) -> Self::ID;
}
