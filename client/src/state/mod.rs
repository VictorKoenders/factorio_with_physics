mod layout;
mod text_input;

pub use self::{layout::Layout, text_input::TextInput};

use crate::{server::Server, window::Window};

pub struct State {
    pub layout: Box<dyn Layout>,
    pub player: Option<Player>,
}

pub struct Player {}

impl Default for State {
    fn default() -> Self {
        Self {
            layout: Box::new(self::layout::login::Login::default()),
            player: None,
        }
    }
}

impl State {
    pub fn render(&self, window: &mut Window, server: &Server) {
        self.layout.render(&self, window, server);
    }
}
