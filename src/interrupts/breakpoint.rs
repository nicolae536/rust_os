use x86_64::structures::idt::InterruptStackFrame;

pub fn handle(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
