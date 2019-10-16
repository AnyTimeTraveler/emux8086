pub struct Registers<'a> {
    // general purpose registers
    pub ax: &'a mut [u8],
    pub al: &'a mut u8,
    pub ah: &'a mut u8,

    pub bx: &'a mut [u8],
    pub bl: &'a mut u8,
    pub bh: &'a mut u8,

    pub cx: &'a mut [u8],
    pub cl: &'a mut u8,
    pub ch: &'a mut u8,

    pub dx: &'a mut [u8],
    pub dl: &'a mut u8,
    pub dh: &'a mut u8,

    // stack pointer
    pub sp: &'a mut [u8],
    // base pointer
    pub bp: &'a mut [u8],
    // source index
    pub si: &'a mut [u8],
    // destination index
    pub di: &'a mut [u8],

    // instruction pointer
    pub ip: &'a mut [u8],

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
    pub flags: &'a mut [u8],

    // code segment
    pub cs: &'a mut [u8],
    // stack segment
    pub ss: &'a mut [u8],
    // data segment
    pub ds: &'a mut [u8],
    // extra segment
    pub es: &'a mut [u8],

    pub raw: &'a mut [u8; 26],
}

impl <'a>Registers<'a> {
    pub(crate) fn new() -> Self {
        let mut registers = [0u8; 26];
        Registers {
            raw: &mut registers,

            ax: &mut registers[0..2],
            al: &mut registers[0],
            ah: &mut registers[1],

            cx: &mut registers[2..4],
            cl: &mut registers[2],
            ch: &mut registers[3],

            dx: &mut registers[4..6],
            dl: &mut registers[4],
            dh: &mut registers[5],

            bx: &mut registers[6..8],
            bl: &mut registers[6],
            bh: &mut registers[7],

            sp: &mut registers[8..10],
            bp: &mut registers[10..12],
            si: &mut registers[12..14],
            di: &mut registers[14..16],

            ip: &mut registers[14..16],
            flags: &mut registers[16..18],

            cs: &mut registers[18..20],
            ss: &mut registers[20..22],
            ds: &mut registers[22..24],
            es: &mut registers[24..26],
        }
    }
}
