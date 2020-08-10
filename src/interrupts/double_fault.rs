use x86_64::structures::idt::InterruptStackFrame;

pub fn handle(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    // #[cfg(test)]
    // serial_println!("TEST: Exception caught: {:#?} \n{:#?}", error_code, stack_frame);
    // TODO ask about this
    // println!();
    panic!("EXCEPTION: DOUBLE_FAULT CODE: {:#?} \n{:#?}", error_code, stack_frame);
}