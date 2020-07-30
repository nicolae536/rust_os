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
mod panic_handler;
// Vga buffer implementation
mod vga_buffer;
// Qemu dependent instructions
mod qemu;

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

// Provide implementation for the test runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

// Test the test runner
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

