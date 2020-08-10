use x86_64::structures::idt::InterruptStackFrame;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

use super::hw_interrupts;

const IO_PORT_ADDR: u16 = 0x60;

pub fn handle(_stack_frame: &mut InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(IO_PORT_ADDR);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        hw_interrupts::PICS.lock()
            .notify_end_of_interrupt(hw_interrupts::Index::Keyboard.as_u8());
    }
}