mod assets;
mod entity;
mod world;

use assets::Assets;
use entity::hitbox::HitboxRectangle;
use entity::miner::MinerConfig;
use graphics::{Camera, Color};
use tetra::{
    graphics,
    input::{self, Key},
    Context, Event,
};
use world::{Chunk, World};

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
    world: World,
    #[allow(dead_code)] // will be used in the future
    assets: Assets,
    camera: Camera,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let mut world = World::default();
        let assets = Assets::load(ctx);
        let camera = Camera::with_window_size(ctx);

        Chunk::generate(&mut world.chunks, ctx, &assets, -1, -1);
        Chunk::generate(&mut world.chunks, ctx, &assets, 0, -1);
        Chunk::generate(&mut world.chunks, ctx, &assets, -1, 0);
        Chunk::generate(&mut world.chunks, ctx, &assets, 0, 0);

        let miner_config = MinerConfig {
            mining_speed: 1,
            hitbox: HitboxRectangle {
                left: 0.5,
                up: 0.5,
                right: 1.5,
                down: 1.5,
            },
        };

        miner_config.spawn_entity(&mut world, (0.0, 0.0).into());

        Self {
            world,
            assets,
            camera,
        }
    }
}
impl tetra::State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.camera.update();
        self.world.update(ctx);

        if input::is_key_down(ctx, Key::Escape) {
            tetra::window::quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());
        self.world.draw(ctx);

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
