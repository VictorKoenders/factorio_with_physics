#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color(sdl2::pixels::Color);

impl Color {
    pub fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self(sdl2::pixels::Color::RGB(r, g, b))
    }

    pub(super) fn into_sdl(self) -> sdl2::pixels::Color {
        self.0
    }
}
