mod color;
mod screen_character;
mod writer;

use writer::*;
use core::fmt;
use lazy_static::lazy_static;

pub use color::*;

lazy_static! {
    pub static ref WRITER: spin::Mutex<Writer> = spin::Mutex::new(
        Writer::new(0, ColorCode::new(Color::Yellow, Color::Black))
    );
}

// Provide global
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}