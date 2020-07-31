// Change program start with test_main when running tests
#![reexport_test_harness_main = "test_main"]
// Don't link the Rust standard library it's dependent on the os
#![no_std]
#![cfg_attr(test, no_main)]
// Override the testing framework runner with crate::test_runner
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// Provide a panic handler implementation since the current panic is based on the os
// <editor-fold desc="TODO Find a convenient way to define macros in a separated file"
use core::fmt;

// <editor-fold desc="print!, println! => Macros">
#[doc(hidden)]
pub fn _print_to_vga_buffer(args: fmt::Arguments) {
    use core::fmt::Write;
    crate::vga_buffer::WRITER.lock().write_fmt(args).unwrap();
}

// Provide global
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print_to_vga_buffer(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
// </editor-fold>

// <editor-fold desc="serial_print!, serial_println! => Macros">
#[doc(hidden)]
pub fn _print_to_serial_port(args: core::fmt::Arguments) {
    use core::fmt::Write;
    crate::serial_port_com::HOST.lock().write_fmt(args).expect("Printing to host serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::_print_to_serial_port(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
// </editor-fold>

// <editor-fold desc="Implement panic handler dependent on above macros">
// We need to implement a new panic handler because the rust standard panic
// handler is dependent on the os
use core::panic::PanicInfo;

// Panic handler
#[allow(dead_code)]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler in test mode
#[allow(dead_code)]
pub fn panic_handler_test(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    crate::qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
// </editor-fold>
// </editor-fold>

// Provide testing framework
mod testing;
// Vga buffer implementation
pub mod vga_buffer;
// Qemu dependent instructions
pub mod qemu;
// Serial Port communication
pub mod serial_port_com;
// Provide memset, memcpy, memcmp implementation since the os usually provides thos
extern crate rlibc;

use testing::Testable;
// Provide implementation for the test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

// Entry point for test
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_handler_test(info);
}