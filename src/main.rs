// Change program start with test_main when running tests
#![reexport_test_harness_main = "test_main"]
// Don't link the Rust standard library it's dependent on the os
#![no_std]
// Disable all Rust-level entry points
#![no_main]
// Override the testing framework runner with crate::test_runner
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]


use core::panic::PanicInfo;
#[allow(dead_code)]
use rust_os::println;

// Os Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Starting Rust_Os v:1.0.0");

    rust_os::kernel_init::run();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new

    #[cfg(test)]
    test_main();

    loop {}
}

// Test the test runner
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::panic_handler(info);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::panic_handler_test(info);
}