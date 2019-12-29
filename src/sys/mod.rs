mod color;
mod event;

pub use self::color::Color;
pub use self::event::Event;

use crate::component::MaterialColor;
use crate::grid_storage::Position;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Instant;

pub struct Window {
    canvas: sdl2::render::WindowCanvas,
    viewport: sdl2::rect::Rect,
    event_pump: sdl2::EventPump,
    clear_color_index: u8,
    clear_color_direction: i8,
    frame_start: Instant,
}

impl Window {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Factorio with physics", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let clear_color = sdl2::pixels::Color::RGB(0, 0, 0);
        canvas.set_draw_color(clear_color);
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();
        Window {
            canvas,
            viewport: sdl2::rect::Rect::new(0, 0, 800, 600),
            event_pump,
            clear_color_index: 0,
            clear_color_direction: 1,
            frame_start: Instant::now(),
        }
    }

    pub fn update<F>(&mut self, mut cb: F)
    where
        F: FnMut(Event),
    {
        for event in self.event_pump.poll_iter() {
            match event {
                SdlEvent::Quit { .. }
                | SdlEvent::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    cb(Event::Quit);
                }
                _ => {}
            }
        }
    }

    pub fn clear(&mut self) {
        let i = self.clear_color_index;
        let clear_color = sdl2::pixels::Color::RGB(i, 64, 255 - i);

        const ANIM_SPEED: u8 = 3;

        self.clear_color_index = if self.clear_color_direction > 0 {
            if let Some(i) = self.clear_color_index.checked_add(ANIM_SPEED) {
                i
            } else {
                self.clear_color_direction = -1;
                self.clear_color_index
            }
        } else if let Some(i) = self.clear_color_index.checked_sub(ANIM_SPEED) {
            i
        } else {
            self.clear_color_direction = 1;
            self.clear_color_index
        };
        self.frame_start = Instant::now();

        self.canvas.set_draw_color(clear_color);
        self.canvas.clear();
    }

    pub fn draw_material(&mut self, material_color: MaterialColor, position: Position) {
        self.canvas.set_draw_color(material_color.0.into_sdl());
        self.canvas
            .fill_rect(Rect::new(
                (position.x.raw() * 64.) as i32,
                (position.y.raw() * 64.) as i32,
                64,
                64,
            ))
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
        let diff = self.frame_start.elapsed();
        const TARGET_FRAME_TIME: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
        if diff < TARGET_FRAME_TIME {
            std::thread::sleep(TARGET_FRAME_TIME - diff);
        }
    }
}
