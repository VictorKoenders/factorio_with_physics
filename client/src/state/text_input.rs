use crate::window::{Color, Window};
use std::borrow::Cow;

pub struct TextInput {
    pub rect: (i32, i32, u32, u32),
    pub name: String,
    pub value: String,
    pub is_password: bool,
    pub has_focus: bool,
}

const INPUT_WIDTH: u32 = 200;
const INPUT_FIELD_OFFSET: i32 = 80;
const INPUT_HEIGHT: u32 = 20;
const POSITION_MARGIN: i32 = 4;
const SIZE_MARGIN: u32 = 8;

impl TextInput {
    pub fn text(name: impl Into<String>, (x, y): (i32, i32)) -> Self {
        Self {
            rect: (x, y, INPUT_WIDTH, INPUT_HEIGHT),
            name: name.into(),
            value: String::new(),
            is_password: false,
            has_focus: false,
        }
    }

    pub fn password(name: impl Into<String>, (x, y): (i32, i32)) -> Self {
        Self {
            rect: (x, y, INPUT_WIDTH, INPUT_HEIGHT),
            name: name.into(),
            value: String::new(),
            is_password: true,
            has_focus: false,
        }
    }

    pub fn is_clicked(&self, click_position: (i32, i32)) -> bool {
        let top_left = (self.rect.0, self.rect.1);
        let bottom_right = (
            self.rect.0 + self.rect.2 as i32,
            self.rect.1 + self.rect.3 as i32,
        );

        between((top_left, bottom_right), click_position)
    }

    pub fn clone_value(&self) -> String {
        self.value.clone()
    }

    pub fn take_value(&mut self) -> String {
        std::mem::replace(&mut self.value, String::new())
    }

    pub fn on_blur(&mut self, window: &Window) {
        self.has_focus = false;
        window.stop_text_input();
    }

    pub fn on_focus(&mut self, window: &Window) {
        self.has_focus = true;
        window.start_text_input(self.rect);
    }
    pub fn render(&self, window: &mut Window) {
        let value: Cow<str> = if self.is_password {
            self.value.chars().map(|_| '*').collect::<String>().into()
        } else {
            (self.value.as_str()).into()
        };

        window.ui_label(Color::white(), &self.name, self.rect.0, self.rect.1);
        if self.has_focus {
            window.rect(
                Color::dark_gray(),
                self.rect.0 + INPUT_FIELD_OFFSET - POSITION_MARGIN,
                self.rect.1 - POSITION_MARGIN,
                self.rect.2 - INPUT_FIELD_OFFSET as u32 + SIZE_MARGIN,
                self.rect.3 + SIZE_MARGIN,
            );
        }
        window.ui_label(
            Color::white(),
            &value,
            self.rect.0 + INPUT_FIELD_OFFSET,
            self.rect.1,
        );
    }

    pub fn backspace(&mut self) {
        self.value.pop();
    }
}

fn between(source: ((i32, i32), (i32, i32)), needle: (i32, i32)) -> bool {
    (source.0).0 <= needle.0
        && (source.1).0 >= needle.0
        && (source.0).1 <= needle.1
        && (source.1).1 >= needle.1
}
