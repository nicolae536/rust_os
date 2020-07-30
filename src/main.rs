#![reexport_test_harness_main = "test_main"]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// Provide a panic handler implementation since the current panic is based on the os
mod panic_handler;
mod vga_buffer;
mod qemu;

// Provide memset, memcpy, memcmp implementation
extern crate rlibc;


// Os Entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::WRITER.lock().write_string("Hello");
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu::exit(qemu::ExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

