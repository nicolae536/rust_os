use super::interrupts::load_interrupt_descriptor_table;
use super::gdt::load_global_descriptor_table;

pub fn run() {
    load_global_descriptor_table();
    load_interrupt_descriptor_table();
}