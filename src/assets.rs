use tetra::graphics::Texture;
use tetra::Context;

pub struct Assets {
    pub grass: Texture,
    pub water: Texture,
    pub iron: Texture,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> Assets {
        Assets {
            grass: Texture::new(ctx, "assets/grass_tile.png").unwrap(),
            water: Texture::new(ctx, "assets/water_tile.png").unwrap(),
            iron: Texture::new(ctx, "assets/iron_plate.png").unwrap(),
        }
    }
}
