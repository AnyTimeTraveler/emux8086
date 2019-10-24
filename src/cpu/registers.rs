use crate::cpu::utils::read_word;

pub struct Registers {
    // general purpose registers
    pub ax: usize,
    pub al: usize,
    pub ah: usize,

    pub bx: usize,
    pub bl: usize,
    pub bh: usize,

    pub cx: usize,
    pub cl: usize,
    pub ch: usize,

    pub dx: usize,
    pub dl: usize,
    pub dh: usize,

    // stack pointer
    pub sp: usize,
    // base pointer
    pub bp: usize,
    // source index
    pub si: usize,
    // destination index
    pub di: usize,

    // instruction pointer
    pub ip: usize,

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
    pub flags: usize,

    // code segment
    pub cs: usize,
    // stack segment
    pub ss: usize,
    // data segment
    pub ds: usize,
    // extra segment
    pub es: usize,

    pub raw: [u8; 28],
}

impl Registers {
    pub(crate) fn new() -> Self {
        Registers {
            raw: [0u8; 28],

            ax: 0,
            al: 0,
            ah: 1,

            cx: 2,
            cl: 2,
            ch: 3,

            dx: 4,
            dl: 4,
            dh: 5,

            bx: 6,
            bl: 6,
            bh: 7,

            sp: 8,
            bp: 10,
            si: 12,
            di: 14,

            ip: 16,
            flags: 18,

            cs: 20,
            ss: 22,
            ds: 24,
            es: 26,
        }
    }

    pub fn point_to_byte(&self, pos: usize) -> &[u8] {
        &self.raw[pos..pos + 1]
    }
    pub fn mut_point_to_byte(&mut self, pos: usize) -> &mut [u8] {
        &mut self.raw[pos..pos + 1]
    }

    pub fn point_to_word(&self, pos: usize) -> &[u8] {
        &self.raw[pos..pos + 2]
    }
    pub fn mut_point_to_word(&mut self, pos: usize) -> &mut [u8] {
        &mut self.raw[pos..pos + 2]
    }

    pub fn read_u16(&self, register: usize) -> u16 {
        read_word(&self.raw[register..register + 2])
    }
    pub fn read_u8(&self, register: usize) -> u8 {
        self.raw[register]
    }
}
