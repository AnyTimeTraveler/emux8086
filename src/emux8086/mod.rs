use crate::emux8086::memory::{Memory, Register};

pub mod utils;
pub mod instructions;
pub mod memory;

pub struct Computer {
    pub cpu: CPU,
    pub memory: Memory,
    pub io: Memory,
}

impl Computer {
    pub(crate) fn new() -> Self {
        Computer {
            cpu: CPU::new(),
            memory: [0u8;1_048_576] as Memory,
            io: [0u8;65_536] as Memory,
        }
    }
}

pub struct CPU {
    // general purpose registers
    pub ax: Register,
    pub bx: Register,
    pub cx: Register,
    pub dx: Register,

    // stack pointer
    pub sp: Register,
    // base pointer
    pub bp: Register,
    // source index
    pub si: Register,
    // destination index
    pub di: Register,

    // instruction pointer
    pub ip: Register,

    // alu flags
    /*
     0: unused
     1: unused
     2: unused
     3: unused
     4: Overflow
     5: Direction
     6: Interrupt
     7: Trace
     8: Sign
     9: Zero
    10: unused
    11: Auxiliary Carry (nibble carry)
    12: unused
    13: Parity
    14: unused
    15: Carry
    */
    pub flags: Register,

    // code segment
    pub cs: Register,
    // stack segment
    pub ss: Register,
    // data segment
    pub ds: Register,
    // extra segment
    pub es: Register,
}

impl CPU {
    fn new() -> Self {
        CPU {
            ax: Memory::new(2),
            bx: Memory::new(2),
            cx: Memory::new(2),
            dx: Memory::new(2),
            sp: Memory::new(2),
            bp: Memory::new(2),
            si: Memory::new(2),
            di: Memory::new(2),
            ip: Memory::new(2),
            flags: Memory::new(2),
            cs: Memory::new(2),
            ss: Memory::new(2),
            ds: Memory::new(2),
            es: Memory::new(2),
        }
    }
}

pub struct Instruction {
    pub code: u8,
    pub name: &'static str,
    pub args: [&'static str; 2],
    pub ip_change: u8,
    pub execute: fn(&mut Computer),
}

