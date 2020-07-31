mod color;
mod screen_character;
mod writer;

use writer::*;
use lazy_static::lazy_static;

pub use color::*;

lazy_static! {
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(
        Writer::new(0, ColorCode::new(Color::Yellow, Color::Black))
    );
}