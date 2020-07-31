use super::interrupts::load_interrupt_descriptor_table;

pub fn run() {
    load_interrupt_descriptor_table();
}