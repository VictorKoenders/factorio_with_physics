use aabb_quad_tree::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
struct Point {
    id: usize,
    bounds: Bounds,
}

impl AABB for Point {
    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }

    fn is_eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn batch_insert(c: &mut Criterion) {
    let bounds = Bounds {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    let mut nodes = Vec::new();
    for id in 0..1_000 {
        nodes.push(Point {
            id,
            bounds: random_bounds(&bounds),
        });
    }

    c.bench_function("batch_insert", |b| {
        b.iter(|| {
            let mut tree = AABBQuadTree::new(bounds);
            for node in nodes.iter().cloned() {
                tree.insert(black_box(node));
            }
        })
    });
}

fn batch_insert_and_remove(c: &mut Criterion) {
    let bounds = Bounds {
        left: 0.0,
        top: 0.0,
        right: 100.0,
        bottom: 100.0,
    };
    let mut nodes = Vec::new();
    for id in 0..1_000 {
        nodes.push(Point {
            id,
            bounds: random_bounds(&bounds),
        });
    }

    c.bench_function("batch_insert_and_remove", |b| {
        b.iter(|| {
            let mut tree = AABBQuadTree::new(bounds);
            for node in nodes.iter().cloned() {
                tree.insert(black_box(node));
            }
            for node in nodes.iter().cloned() {
                tree.remove(black_box(&node));
            }
        })
    });
}

criterion_group!(benches, batch_insert, batch_insert_and_remove);
criterion_main!(benches);

fn random_bounds(bounds: &Bounds) -> Bounds {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut hor_points = (
        rng.gen_range(bounds.left, bounds.right),
        rng.gen_range(bounds.left, bounds.right),
    );
    if hor_points.0 > hor_points.1 {
        std::mem::swap(&mut hor_points.0, &mut hor_points.1);
    }
    let mut vert_points = (
        rng.gen_range(bounds.top, bounds.bottom),
        rng.gen_range(bounds.top, bounds.bottom),
    );
    if vert_points.0 > vert_points.1 {
        std::mem::swap(&mut vert_points.0, &mut vert_points.1);
    }

    Bounds {
        top: vert_points.0,
        left: hor_points.0,
        right: hor_points.1,
        bottom: vert_points.1,
    }
}
