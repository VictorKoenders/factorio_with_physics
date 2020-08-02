mod assets;
mod entity;

use self::entity::properties::{Chunk, Position};
use assets::Assets;
use entity::{
    properties::{EntityRender, TileGrid},
    ItemOnGround,
};
use graphics::{Camera, Color};
use legion::prelude::*;
use tetra::{
    graphics,
    input::{self, Key},
    Context, Event,
};

fn main() {
    if let Err(e) = tetra::ContextBuilder::new("Factorio with physics", 1280, 720)
        .build()
        .unwrap()
        .run(|ctx| Ok(GameState::new(ctx)))
    {
        eprintln!("Engine crashed {:?}", e);
    }
}

struct GameState {
    universe: Universe,
    world: World,
    assets: Assets,
    camera: Camera,
    draw_counts_in_millis: Vec<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();
        let _items: Vec<_> = (0..1_000_000)
            .map(|_| ItemOnGround::new(&mut world, Position::random()))
            .collect();

        (-10..10)
            .flat_map(|x| (-10..10).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let tilegrid = TileGrid::new();
                tilegrid.add_to_world(&mut world, Chunk::new(x, y));
            });

        let assets = Assets::load(ctx);
        let camera = Camera::with_window_size(ctx);

        Self {
            universe,
            world,
            assets,
            camera,
            draw_counts_in_millis: Vec::new(),
        }
    }
}
impl tetra::State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.camera.update();

        if input::is_key_down(ctx, Key::Escape) {
            tetra::window::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let start = std::time::Instant::now();
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        let mut count = 0;

        TileGrid::load_in_range(
            &self.world,
            Chunk::new(-1, -1),
            Chunk::new(1, 1),
            |chunk, tilegrid| {
                tilegrid.render(&self.assets, ctx, chunk);
                count += 1;
            },
        );

        EntityRender::get_in_range(
            &self.world,
            Chunk::new(-1, -1),
            Chunk::new(1, 1),
            |pos, render| {
                render.render(ctx, &self.assets, pos);
                count += 1;
            },
        );
        self.draw_counts_in_millis
            .push(start.elapsed().as_secs_f32() * 1000.);
        if self.draw_counts_in_millis.len() == 10 {
            let sum: f32 = self.draw_counts_in_millis.drain(..).sum();
            println!("Drew {} entities in {} ms", count, sum / 10.);
        }
        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
            self.camera.set_viewport_size(width as f32, height as f32);
            self.camera.update();
        }

        Ok(())
    }
}

/*

v2:
Set up 10000000 items in 4.659319793s
Found 246 that hit 0/0, took 34.660228ms
Found 246 that hit 0/0, took 16.640171ms
v2:
Set up 10000000 items in 18.668358243s
Found 3 that hit 0/0, took 23.024766ms
Found 3 that hit 0/0, took 215.291µs
Found 3 that hit 0/0, took 209.805µs
v3:
Set up 10000000 items in 10.270956338s
Found 2 that hit 0/0, took 27.73685ms
Found 2 that hit 0/0, took 232.333µs
Found 2 that hit 0/0, took 180.014µs
*/
