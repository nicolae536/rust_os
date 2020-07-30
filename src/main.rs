#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Provide a panic handler implementation since the current panic is based on the os
mod panic_handler;
mod vga_buffer;

// Provide memset, memcpy, memcmp implementation
extern crate rlibc;

// Os Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::WRITER.lock().write_string("Hello");
    println!("Hello World{}", "!");

    loop {}
}
