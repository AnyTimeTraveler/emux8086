pub struct Registers {
    // general purpose registers
    pub ax: &'static mut [u8],
    pub al: &'static mut u8,
    pub ah: &'static mut u8,

    pub bx: &'static mut [u8],
    pub bl: &'static mut u8,
    pub bh: &'static mut u8,

    pub cx: &'static mut [u8],
    pub cl: &'static mut u8,
    pub ch: &'static mut u8,

    pub dx: &'static mut [u8],
    pub dl: &'static mut u8,
    pub dh: &'static mut u8,

    // stack pointer
    pub sp: &'static mut [u8],
    // base pointer
    pub bp: &'static mut [u8],
    // source index
    pub si: &'static mut [u8],
    // destination index
    pub di: &'static mut [u8],

    // instruction pointer
    pub ip: &'static mut [u8],

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
    pub flags: &'static mut [u8],

    // code segment
    pub cs: &'static mut [u8],
    // stack segment
    pub ss: &'static mut [u8],
    // data segment
    pub ds: &'static mut [u8],
    // extra segment
    pub es: &'static mut [u8],

    pub raw: [u8;26],
}

impl Registers {
    pub(crate) fn new() -> Self {
        let mut registers = [0u8;26];
        Registers {
            raw: registers,

            ax:&mut registers[0..2],
            al:&mut registers[0],
            ah:&mut registers[1],

            cx:&mut registers[2..4],
            cl:&mut registers[2],
            ch:&mut registers[3],

            dx:&mut registers[4..6],
            dl:&mut registers[4],
            dh:&mut registers[5],

            bx:&mut registers[6..8],
            bl:&mut registers[6],
            bh:&mut registers[7],

            sp:&mut registers[8..10],
            bp:&mut registers[10..12],
            si:&mut registers[12..14],
            di:&mut registers[14..16],

            ip:&mut registers[14..16],
            flags:&mut registers[16..18],

            cs:&mut registers[18..20],
            ss:&mut registers[20..22],
            ds:&mut registers[22..24],
            es:&mut registers[24..26],
        }
    }
}
