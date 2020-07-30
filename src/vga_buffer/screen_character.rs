use super::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenCharacter {
    ascii_character: u8,
    color_code: color::ColorCode,
}

impl ScreenCharacter {
    pub fn new(ascii_character: u8, color_code: color::ColorCode) -> ScreenCharacter {
        return ScreenCharacter {
            ascii_character,
            color_code
        }
    }
}