// Change program start with test_main when running tests
#![reexport_test_harness_main = "test_main"]
// Don't link the Rust standard library it's dependent on the os
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// Override the testing framework runner with crate::test_runner
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// Provide a panic handler implementation since the current panic is based on the os
mod macros;
// Provide testing framework
mod testing;
// Vga buffer implementation
mod vga_buffer;
// Qemu dependent instructions
mod qemu;
// Serial Port communication
mod serial_port_com;
// Provide memset, memcpy, memcmp implementation since the os usually provides thos
extern crate rlibc;


// Os Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
use testing::Testable;
// Provide implementation for the test runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}

// Test the test runner
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

