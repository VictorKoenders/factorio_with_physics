mod color;
mod event;

pub use self::color::Color;
pub use self::event::Event;
use crate::units::si::Kelvin;

use crate::component::MaterialColor;
use crate::grid_storage::Position;
use sdl2::rect::Rect;
use std::time::Instant;

const FLASHY_BACKGROUND_ENABLED: bool = cfg!(debug_assertions);

pub struct Window {
    canvas: sdl2::render::WindowCanvas,
    event_pump: sdl2::EventPump,
    clear_color_index: u8,
    clear_color_direction: i8,
    frame_start: Instant,
    state: WindowState,
    last_frame_times: Vec<u128>,
}

pub struct WindowState {
    pub viewport: sdl2::rect::Rect,
    pub view: WindowStateView,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            viewport: sdl2::rect::Rect::new(0, 0, 800, 600),
            view: WindowStateView::HeatMap,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WindowStateView {
    HeatMap,
    MaterialMap,
}

impl Default for Window {
    fn default() -> Self {
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
            event_pump,
            clear_color_index: 0,
            clear_color_direction: 1,
            frame_start: Instant::now(),
            state: Default::default(),
            last_frame_times: Vec::with_capacity(10),
        }
    }
}

impl Window {
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    pub fn update<F>(&mut self, mut cb: F)
    where
        F: FnMut(&mut WindowState, Event),
    {
        for event in self.event_pump.poll_iter() {
            if let Some(event) = Event::from_sdl_event(event) {
                cb(&mut self.state, event);
            }
        }
    }

    pub fn clear(&mut self) {
        let i = self.clear_color_index;
        let clear_color = sdl2::pixels::Color::RGB(i, 64, 255 - i);

        if FLASHY_BACKGROUND_ENABLED {
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
        }

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

    pub fn draw_temperature(
        &mut self,
        temperature: Kelvin,
        position: Position,
        min_temperature: Kelvin,
        max_temperature: Kelvin,
    ) {
        let range = max_temperature - min_temperature;
        let percent = ((temperature - min_temperature) / range).raw();

        let r = (percent * 255.0) as u8;
        let g = 0;
        let b = ((1.0 - percent) * 255.0) as u8;

        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
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
        self.last_frame_times.push(diff.as_nanos());

        if self.last_frame_times.len() == self.last_frame_times.capacity() {
            let avg_update_time = self.last_frame_times.iter().sum::<u128>()
                / self.last_frame_times.len() as u128
                / 1000;
            self.canvas
                .window_mut()
                .set_title(&format!("Factorio with physics ({}μs)", avg_update_time,))
                .unwrap();
            self.last_frame_times.clear();
        }

        const TARGET_FRAME_TIME: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
        if diff < TARGET_FRAME_TIME {
            std::thread::sleep(TARGET_FRAME_TIME - diff);
        }
    }
}
