pub mod login;

use super::TextInput;
use crate::{server::Server, state::State, window::Window};

pub trait Layout {
    fn inputs(&self) -> &[TextInput];
    fn inputs_mut(&mut self) -> &mut [TextInput];
    fn render(&self, state: &State, window: &mut Window, server: &Server);

    fn blur_all(&mut self, window: &Window) {
        for input in self.inputs_mut() {
            if input.has_focus {
                input.on_blur(window);
            }
        }
    }

    fn enter_pressed(&mut self, _window: &Window, _server: &mut Server) {}

    fn click(&mut self, (x, y): (i32, i32), window: &Window) {
        let clicked_input = self.inputs().iter().position(|i| i.is_clicked((x, y)));
        if let Some(clicked_input) = clicked_input {
            self.blur_all(window);
            self.inputs_mut()[clicked_input].on_focus(window);
        }
    }

    fn text_input(&mut self, text: String) {
        for input in self.inputs_mut() {
            if input.has_focus {
                input.value += &text;
                break;
            }
        }
    }

    fn backspace_pressed(&mut self) {
        for input in self.inputs_mut() {
            if input.has_focus {
                input.backspace();
            }
        }
    }

    fn tab_pressed(&mut self, window: &Window) {
        let inputs = self.inputs_mut();
        let (current_index, next_index) = if let Some(p) = inputs.iter().position(|i| i.has_focus) {
            (Some(p), (p + 1) % inputs.len())
        } else {
            (None, 0)
        };

        if let Some(current_index) = current_index {
            inputs[current_index].on_blur(window);
        }
        inputs[next_index].on_focus(window);
    }
}
