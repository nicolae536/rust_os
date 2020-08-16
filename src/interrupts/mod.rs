use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use super::gdt;

mod breakpoint;
mod hw_interrupts;
mod double_fault;
mod timer;
mod keyboard;
mod page_fault;

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_FIST_INDEX);
        }
        idt[hw_interrupts::Index::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[hw_interrupts::Index::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt
    };
}


pub fn load_interrupt_descriptor_table() {
    // Load our Interrupt Descriptor table so the cpu can use it
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

pub fn load_programmable_interrupt_controller() {
    unsafe {
        hw_interrupts::PICS.lock().initialize()
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame
) {
    breakpoint::handle(stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    double_fault::handle(stack_frame, error_code);
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame
)
{
    timer::handle(_stack_frame);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame
)
{
    keyboard::handle(_stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    page_fault::handle(stack_frame, error_code);
}