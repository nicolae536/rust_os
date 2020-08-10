use super::interrupts::load_interrupt_descriptor_table;
use super::gdt::load_global_descriptor_table;
use crate::interrupts::load_programmable_interrupt_controller;

pub fn run() {
    load_global_descriptor_table();
    load_interrupt_descriptor_table();
    load_programmable_interrupt_controller();
    x86_64::instructions::interrupts::enable();
}