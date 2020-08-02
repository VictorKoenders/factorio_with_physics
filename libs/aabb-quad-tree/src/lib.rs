mod bounds;
mod node;

pub use self::bounds::Bounds;
use self::node::Node;

pub struct AABBQuadTree<T: AABB> {
    root: Node<T>,
    bounds: Bounds,
}
impl<T: AABB> AABBQuadTree<T> {
    pub fn new(bounds: Bounds) -> Self {
        Self {
            root: Node::default(),
            bounds,
        }
    }
    pub fn insert(&mut self, t: T) {
        let val_bounds = t.bounds();
        let node_bounds = self.bounds.clone();
        if val_bounds.right > node_bounds.right
            || val_bounds.left < node_bounds.left
            || val_bounds.top < node_bounds.top
            || val_bounds.bottom > node_bounds.bottom
        {
            unimplemented!("Resizing not implemented yet");
        }
        self.root.insert(t, val_bounds, node_bounds);
    }
    pub fn remove(&mut self, t: &T) {
        let val_bounds = t.bounds();
        let node_bounds = self.bounds.clone();
        self.root.remove(t, val_bounds, node_bounds);
    }

    pub fn dbg_rectangles(&self) -> impl IntoIterator<Item = Bounds> {
        let mut rects = Vec::new();
        get_bounds_recursive(&self.root, self.bounds.clone(), &mut rects);
        rects
    }
}

fn get_bounds_recursive<T: AABB>(node: &Node<T>, bounds: Bounds, rects: &mut Vec<Bounds>) {
    const MARGIN: f32 = 2.0;
    rects.push(Bounds {
        left: bounds.left + MARGIN,
        right: bounds.right - MARGIN,
        top: bounds.top + MARGIN,
        bottom: bounds.bottom - MARGIN,
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

// BlockedTODO: https://github.com/rust-lang/rust/issues/25725
// Remove `Clone` bounds once this issue is resolved
pub trait AABB: Clone {
    fn bounds(&self) -> Bounds;
    fn is_eq(&self, other: &Self) -> bool;
}
