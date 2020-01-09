use super::{Layout, TextInput};
use crate::{
    server::Server,
    state::State,
    window::{Color, Window},
};
use shared::to_server::Login as LoginMessage;

pub struct Login {
    pub inputs: [TextInput; 2],
}

impl Default for Login {
    fn default() -> Self {
        Self {
            inputs: [
                TextInput::text("Username", (10, 10)),
                TextInput::password("Password", (10, 60)),
            ],
        }
    }
}

impl Login {
    pub fn username(&self) -> &TextInput {
        &self.inputs[0]
    }
    pub fn password(&self) -> &TextInput {
        &self.inputs[1]
    }
    pub fn password_mut(&mut self) -> &mut TextInput {
        &mut self.inputs[1]
    }
}

impl Layout for Login {
    fn inputs(&self) -> &[TextInput] {
        &self.inputs
    }
    fn inputs_mut(&mut self) -> &mut [TextInput] {
        &mut self.inputs
    }
    fn render(&self, _state: &State, window: &mut Window, _server: &Server) {
        for input in self.inputs() {
            input.render(window);
        }

        window.rect(Color::blue(), 90, 100, 100, 30);
        window.ui_label(Color::white(), "Log in", 100, 110);
    }

    fn enter_pressed(&mut self, window: &Window, server: &mut Server) {
        self.blur_all(window);
        server.send(LoginMessage {
            username: self.username().clone_value(),
            password: self.password_mut().take_value(),
        });
    }
}
