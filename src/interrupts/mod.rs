use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use super::gdt;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_FIST_INDEX);
        }
        idt
    };
}


pub fn load_interrupt_descriptor_table() {
    // Load our Interrupt Descriptor table so the cpu can use it
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    // #[cfg(test)]
    // serial_println!("TEST: Exception caught: {:#?} \n{:#?}", error_code, stack_frame);
    // TODO ask about this
    // println!();
    panic!("EXCEPTION: DOUBLE_FAULT CODE: {:#?} \n{:#?}", error_code, stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}