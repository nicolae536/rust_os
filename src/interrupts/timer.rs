use x86_64::structures::idt::InterruptStackFrame;

use super::hw_interrupts;

pub fn handle(_stack_frame: &mut InterruptStackFrame) {
    print!(".");
    unsafe {
        hw_interrupts::PICS.lock()
            .notify_end_of_interrupt(hw_interrupts::Index::Timer.as_u8());
    }
}