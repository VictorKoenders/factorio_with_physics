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

#[cfg(not(debug_assertions))]
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
#[cfg(debug_assertions)]
impl Default for Login {
    fn default() -> Self {
        Self {
            inputs: [
                TextInput::text("Username", (10, 10)).with_value("asd"),
                TextInput::password("Password", (10, 60)).with_value("dsa"),
            ],
        }
    }
}

impl Login {
    pub fn username(&self) -> &TextInput {
        &self.inputs[0]
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
    fn render(&self, _state: &State, window: &mut Window, server: &Server) {
        for input in self.inputs() {
            input.render(window);
        }

        window.ui_label(
            Color::white(),
            if server.status().is_connected() {
                "Online!"
            } else {
                "Connecting..."
            },
            290,
            105,
        );

        window.rect(Color::blue(), 90, 100, 100, 30);
        window.ui_label(Color::white(), "Log in", 100, 105);
    }

    fn click(&mut self, (x, y): (i32, i32), window: &Window, server: &mut Server) {
        super::layout_click_base(self, (x, y), window);
        if (90, 100, 100, 30).contains((x, y)) {
            // log in clicked
            self.blur_all(window);
            server.send(LoginMessage {
                username: self.username().clone_value(),
                password: self.password_mut().take_value(),
            });
        }
    }

    fn enter_pressed(&mut self, window: &Window, server: &mut Server) {
        self.blur_all(window);
        server.send(LoginMessage {
            username: self.username().clone_value(),
            password: self.password_mut().take_value(),
        });
    }
}

pub trait Contains {
    fn contains(self, point: (i32, i32)) -> bool;
}

impl Contains for (i32, i32, u32, u32) {
    fn contains(self, (x, y): (i32, i32)) -> bool {
        let (self_x, self_y, self_w, self_h) = self;
        self_x <= x && (self_x + self_w as i32) >= x && self_y <= y && (self_y + self_h as i32) >= y
    }
}
