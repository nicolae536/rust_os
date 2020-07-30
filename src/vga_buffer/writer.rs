use core::fmt;

use volatile::Volatile;

use super::color;
use super::screen_character;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<screen_character::ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: color::ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(column_position: usize,
               color_code: color::ColorCode) -> Writer {
        return Writer {
            column_position,
            color_code,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(screen_character::ScreenCharacter::new(byte, color_code));
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let current_character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(current_character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row_index: usize) {
        let blank = screen_character::ScreenCharacter::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row_index][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}