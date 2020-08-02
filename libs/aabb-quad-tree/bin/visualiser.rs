use aabb_quad_tree::*;
use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Mesh},
    input::mouse::MouseButton,
    Context, ContextBuilder, GameResult,
};
use graphics::{Color, DrawMode, FillOptions, StrokeOptions};
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    // Make a Context and an EventLoop.
    let window_size = (1000., 1000.);
    let (mut ctx, mut event_loop) = ContextBuilder::new("Quadtree visualiser", "Trangar")
        .window_mode(WindowMode::default().dimensions(window_size.0, window_size.1))
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx, window_size);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    pub id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
}

impl Point {
    pub fn mesh(&self, ctx: &mut Context) -> Mesh {
        let rect = graphics::Rect {
            x: self.position.0 - self.size.0,
            y: self.position.1 - self.size.1,
            w: self.size.0 * 2.,
            h: self.size.1 * 2.,
        };
        Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            rect,
            graphics::WHITE,
        )
        .unwrap()
    }
}
impl AABB for Point {
    fn bounds(&self) -> Bounds {
        Bounds {
            left: self.position.0 - self.size.0,
            top: self.position.1 - self.size.1,
            right: self.position.0 + self.size.0,
            bottom: self.position.1 + self.size.1,
        }
    }
    fn is_eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct MyGame {
    quadtree: AABBQuadTree<Point>,
    rectangles: Vec<Mesh>,
    points: Vec<(Mesh, Point)>,
}

impl MyGame {
    pub fn new(ctx: &mut Context, window_size: (f32, f32)) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut game = MyGame {
            quadtree: AABBQuadTree::new(Bounds {
                left: 0.0,
                top: 0.0,
                right: window_size.0,
                bottom: window_size.1,
            }),
            rectangles: Vec::new(),
            points: Vec::new(),
        };
        game.update_rectangles(ctx);
        game
    }

    fn update_rectangles(&mut self, ctx: &mut Context) {
        self.rectangles.clear();
        for b in self.quadtree.dbg_rectangles() {
            let rect = graphics::Rect {
                x: b.left,
                y: b.top,
                w: b.right - b.left,
                h: b.bottom - b.top,
            };
            let mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::Stroke(StrokeOptions::DEFAULT),
                rect,
                Color::from_rgb(0, 50, 180),
            )
            .unwrap();
            self.rectangles.push(mesh);
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        for (index, (_mesh, point)) in self.points.iter().enumerate() {
            if cursor_is_near(ctx, point) {
                self.quadtree.remove(&point);
                self.points.remove(index);
                self.update_rectangles(ctx);
                return;
            }
        }
        static POINT_ID: AtomicU64 = AtomicU64::new(1);

        let point = Point {
            id: POINT_ID.fetch_add(1, Ordering::Relaxed),
            position: (_x, _y),
            size: (5.0, 5.0),
        };
        let mesh = point.mesh(ctx);
        self.points.push((mesh, point.clone()));
        self.quadtree.insert(point);
        self.update_rectangles(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        for rectangle in &self.rectangles {
            graphics::draw(ctx, rectangle, graphics::DrawParam::default())?;
        }

        for (mesh, point) in &self.points {
            let mut params = graphics::DrawParam::default();
            params.color = if cursor_is_near(ctx, &point) {
                Color::from_rgb(255, 0, 0)
            } else {
                Color::from_rgb(0, 0, 255)
            };
            graphics::draw(ctx, mesh, params)?;
        }

        graphics::present(ctx)
    }
}

fn cursor_is_near(ctx: &mut Context, point: &Point) -> bool {
    let position = ggez::input::mouse::position(ctx);
    let dx = position.x - point.position.0;
    let dy = position.y - point.position.1;

    dx.abs() < point.size.0 * 2. && dy.abs() < point.size.1 * 2.
}
