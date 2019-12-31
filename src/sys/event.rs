use crate::sys::{WindowState, WindowStateView};
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;
use specs::World;

pub enum Event {
    Quit,
    Input(UserInput),
}

impl Event {
    pub fn is_quit(&self) -> bool {
        match self {
            Event::Quit => true,
            _ => false,
        }
    }

    pub(super) fn from_sdl_event(event: SdlEvent) -> Option<Self> {
        match event {
            SdlEvent::Quit { .. }
            | SdlEvent::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Some(Event::Quit),
            SdlEvent::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => Some(Event::Input(UserInput::ViewMaterialMap)),
            SdlEvent::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => Some(Event::Input(UserInput::ViewHeatMap)),
            _ => None,
        }
    }
}

pub enum UserInput {
    ViewHeatMap,
    ViewMaterialMap,
}

impl UserInput {
    pub fn apply(self, window: &mut WindowState, _world: &mut World) {
        match self {
            UserInput::ViewHeatMap => window.view = WindowStateView::HeatMap,
            UserInput::ViewMaterialMap => window.view = WindowStateView::MaterialMap,
        }
    }
}
