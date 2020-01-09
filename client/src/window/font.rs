use crate::window::Color;
use sdl2::{
    render::{Texture, TextureCreator, TextureQuery},
    ttf::{Font as SdlFont, Sdl2TtfContext},
    video::WindowContext,
};
use std::{collections::HashMap, hash::Hash, path::Path};

pub struct Font<T: Hash + Eq> {
    texture_creator: TextureCreator<WindowContext>,
    context: Box<Sdl2TtfContext>,
    fonts: HashMap<T, HashMap<u16, SdlFont<'static, 'static>>>,
    text_cache: HashMap<(T, u16, String, Color), FontTexture>,
}

impl<T: Hash + Eq> Font<T> {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            context: Box::new(sdl2::ttf::init().unwrap()),
            fonts: HashMap::new(),
            text_cache: HashMap::new(),
        }
    }

    pub fn load<'a>(
        &'a mut self,
        name: T,
        path: impl AsRef<Path>,
        point_size: u16,
    ) -> Result<(), ()> {
        let font_path = self.fonts.entry(name).or_insert_with(HashMap::new);

        if font_path.contains_key(&point_size) {
            panic!("Font is already loaded");
        }

        let font: SdlFont<'a, 'static> = self.context.load_font(path, point_size).map_err(|e| {
            eprintln!("{:?}", e);
        })?;
        let font: SdlFont<'static, 'static> = unsafe { std::mem::transmute(font) };

        font_path.insert(point_size, font);
        Ok(())
    }

    pub fn render(&mut self, font: T, pixel_size: u16, text: &str, color: Color) -> &FontTexture
    where
        T: Clone,
    {
        let fonts = &self.fonts;
        let texture_creator = &self.texture_creator;
        let text_cache = &mut self.text_cache;

        // TODO: only use .clone() and .to_owned() when the entry does not exist
        // We'll probably need to switch to a wrapper struct for the key and implementing Hash on that
        // We can also look into this experimental feature to reduce allocations:
        // https://doc.rust-lang.org/std/collections/hash_map/enum.RawEntryMut.html
        text_cache
            .entry((font.clone(), pixel_size, text.to_owned(), color))
            .or_insert_with(|| {
                let font = fonts.get(&font).and_then(|f| f.get(&pixel_size)).unwrap();

                let texture = texture_creator
                    .create_texture_from_surface(font.render(&text).solid(color).unwrap())
                    .unwrap();

                FontTexture {
                    stats: texture.query(),
                    texture: unsafe { std::mem::transmute(texture) },
                    last_used: std::time::Instant::now(),
                }
            })
    }

    pub fn cleanup(&mut self) {
        self.text_cache
            .retain(|_, f| f.last_used.elapsed().as_secs() == 0);
    }
}

pub struct FontTexture {
    stats: TextureQuery,
    pub(super) texture: Texture<'static>,
    last_used: std::time::Instant,
}

impl FontTexture {
    pub fn width(&self) -> u32 {
        self.stats.width
    }
    pub fn height(&self) -> u32 {
        self.stats.height
    }
}
