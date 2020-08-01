use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_FIST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_FIST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe {&STACK});
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}


struct GdtSelectors {
    code_segment: SegmentSelector,
    task_state_segment: SegmentSelector,
}

struct GlobalDescriptorTableRef {
    selectors: GdtSelectors,
    table: GlobalDescriptorTable,
}

lazy_static! {
    static ref GDT: GlobalDescriptorTableRef = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_segment = gdt.add_entry(Descriptor::kernel_code_segment());
        let task_state_segment = gdt.add_entry(Descriptor::tss_segment(&TSS));
        GlobalDescriptorTableRef {
            table: gdt,
            selectors: GdtSelectors {
                code_segment,
                task_state_segment,
            }
        }
    };
}

pub fn load_global_descriptor_table() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.table.load();
    unsafe {
        set_cs(GDT.selectors.code_segment);
        load_tss(GDT.selectors.task_state_segment)
    }
}