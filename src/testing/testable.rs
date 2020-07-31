// Needed because otherwise the macro will not work
#[allow(dead_code)]
use crate::serial_print;
use crate::serial_println;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where T: Fn() {
    fn run(&self) {
        serial_print!("[Running] -> {} ... \t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}