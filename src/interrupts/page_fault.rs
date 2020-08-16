use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::hlt_loop;

pub fn handle(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: Page fault");
    println!("Accessed address: {:?}", Cr2::read());
    println!("Error code: {:?}", error_code);
    println!("{:#?}", stack_frame);

    hlt_loop();
}