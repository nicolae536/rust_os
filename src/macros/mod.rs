use core::fmt;

// <editor-fold desc="print!, println! => Macros">
#[doc(hidden)]
pub fn _print_to_vga_buffer(args: fmt::Arguments) {
    use core::fmt::Write;
    use crate::vga_buffer;
    vga_buffer::WRITER.lock().write_fmt(args).unwrap();
}

// Provide global
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print_to_vga_buffer(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
// </editor-fold>

// <editor-fold desc="serial_print!, serial_println! => Macros">
#[doc(hidden)]
pub fn _print_to_serial_port(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use crate::serial_port_com;
    serial_port_com::HOST.lock().write_fmt(args).expect("Printing to host serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::macros::_print_to_serial_port(format_args!($($arg)*));
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
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    use crate::qemu;
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}
// </editor-fold>
