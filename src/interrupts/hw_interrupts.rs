use pic8259_simple::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Index {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl Index {
    pub fn as_u8(self) -> u8 {
        return self as u8;
    }

    pub fn as_usize(self) -> usize {
        return usize::from(self.as_u8());
    }
}