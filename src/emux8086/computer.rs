use crate::emux8086::memory::Memory;
use crate::emux8086::registers::Registers;

pub struct Computer {
    pub registers: Registers,
    pub memory: Memory,
    pub io: [u8;65_536],
}

impl Computer {
    pub(crate) fn new() -> Self {
        Computer {
            registers: Registers::new(),
            memory: Memory::new(),
            io: [0u8;65_536],
        }
    }
}
