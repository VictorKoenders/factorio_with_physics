mod font;

use self::font::Font;
use sdl2::{
    event::Event as SdlEvent,
    keyboard::{Keycode, TextInputUtil},
    render::WindowCanvas,
    EventPump,
};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum FontType {
    Roboto,
}

pub struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
    text_input: TextInputUtil,
    font: Font<FontType>,
    is_active: bool,
}

impl Default for Window {
    fn default() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let text_input = video_subsystem.text_input();

        let mut font = Font::new(canvas.texture_creator());
        font.load(FontType::Roboto, "resources/Roboto-Regular.ttf", 16)
            .unwrap();

        Self {
            canvas,
            event_pump,
            text_input,
            font,
            is_active: true,
        }
    }
}

impl Window {
    pub fn set_title(&mut self, title: String) {
        self.canvas.window_mut().set_title(&title).unwrap();
    }
    pub fn events(&mut self) -> Vec<Event> {
        let mut result = Vec::new();

        for event in self.event_pump.poll_iter() {
            if let Some(e) = Event::try_from_sdl_event(event) {
                result.push(e);
            }
        }

        result
    }

    pub fn is_open(&self) -> bool {
        self.is_active
    }

    pub fn close(&mut self) {
        self.is_active = false;
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /*
    pub fn ui_label_dimensions(&mut self, color: Color, text: &str) -> (u32, u32) {
        if text.is_empty() {
            (0, 19)
        } else {
            let texture = self.font.render(FontType::Roboto, 16, text, color);
            (texture.width(), texture.height())
        }
    }
    */

    pub fn ui_label(&mut self, color: Color, text: &str, x: i32, y: i32) {
        if text.is_empty() {
            return;
        }
        let texture = self.font.render(FontType::Roboto, 16, text, color);
        self.canvas
            .copy(
                &texture.texture,
                None,
                Some(sdl2::rect::Rect::new(
                    x,
                    y,
                    texture.width(),
                    texture.height(),
                )),
            )
            .unwrap();
    }

    pub fn rect(&mut self, color: Color, x: i32, y: i32, width: u32, height: u32) {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(Some(sdl2::rect::Rect::new(x, y, width, height)))
            .unwrap();
    }
    pub fn finish(&mut self) {
        self.canvas.present();
        self.font.cleanup();
    }

    pub fn stop_text_input(&self) {
        self.text_input.stop();
    }
    pub fn start_text_input(&self, rect: (i32, i32, u32, u32)) {
        self.text_input.start();
        self.text_input.set_rect(rect.into());
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.r, self.g, self.b)
    }
}

impl Color {
    pub fn blue() -> Color {
        Color { r: 0, g: 0, b: 255 }
    }
    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn dark_gray() -> Color {
        Color {
            r: 50,
            g: 50,
            b: 50,
        }
    }
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

pub enum Event {
    CloseRequested,
    Click { x: i32, y: i32 },
    Tab,
    Enter,
    Backspace,
    TextInput { text: String },
}

impl Event {
    pub(super) fn try_from_sdl_event(e: SdlEvent) -> Option<Self> {
        match e {
            SdlEvent::Quit { .. }
            | SdlEvent::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Some(Event::CloseRequested),
            SdlEvent::KeyDown {
                keycode: Some(Keycode::Tab),
                ..
            } => Some(Event::Tab),
            SdlEvent::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => Some(Event::Enter),
            SdlEvent::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => Some(Event::Backspace),
            SdlEvent::TextInput { text, .. } => Some(Event::TextInput { text }),
            SdlEvent::TextEditing {
                text,
                start,
                length,
                ..
            } => {
                println!("Text editing {:?} {} {}", text, start, length);
                None
            }
            SdlEvent::MouseButtonDown { x, y, .. } => Some(Event::Click { x, y }),
            _ => None,
        }
    }
}
