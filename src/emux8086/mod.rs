pub(crate) struct Computer {
    pub cpu: CPU,
    pub memory: [u8; 1_048_576],
    pub io: [u8; 65_536],
}

pub(crate) struct CPU {
    // general purpose registers
    pub ax: [u8; 2],
    pub bx: [u8; 2],
    pub cx: [u8; 2],
    pub dx: [u8; 2],

    // stack pointer
    pub sp: u16,
    // base pointer
    pub bp: u16,
    // source index
    pub si: u16,
    // destination index
    pub di: u16,

    // instruction pointer
    pub ip: u16,

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
    pub flags: [bool; 16],

    // code segment
    pub cs: u16,
    // stack segment
    pub ss: u16,
    // data segment
    pub ds: u16,
    // extra segment
    pub es: u16,
}

pub(crate) struct Instruction {
    pub code: u8,
    pub name: &'static str,
    pub args: u8,
    pub execute: InstructionAction,
}
type InstructionAction = fn(&mut Computer) -> &mut Computer;

fn add_register_or_memory_byte_addressing_mode_byte(computer: &mut Computer) -> &mut Computer {
    computer
}

pub(crate) static INSTRUCTIONS: &'static [Instruction] = &[
    Instruction {
        code: 0x00,
        name: "ADD",
        args: 2,
        execute: add_register_or_memory_byte_addressing_mode_byte,
    },
    Instruction {
        code: 0x00,
        name: "ADD",
        args: 2,
        execute: add_register_or_memory_byte_addressing_mode_byte,
    }
];
/*
    Instruction {
        code: 0x01,
        name: "ADD",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x02,
        name: "ADD",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x03,
        name: "ADD",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x04,
        name: "ADD",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x05,
        name: "ADD",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x06,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x07,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x08,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x09,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0A,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0B,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0C,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0D,
        name: "OR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0E,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x0F,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x10,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x11,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x12,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x13,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x14,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x15,
        name: "ADC",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x16,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x17,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x18,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x19,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1A,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1B,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1C,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1D,
        name: "SBB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1E,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x1F,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x20,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x21,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x22,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x23,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x24,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x25,
        name: "AND",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x26,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x27,
        name: "DAA",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x28,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x29,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2A,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2B,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2C,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2D,
        name: "SUB",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2E,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x2F,
        name: "DAS",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x30,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x31,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x32,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x33,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x34,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x35,
        name: "XOR",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x36,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x37,
        name: "AAA",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x38,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x39,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3A,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3B,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3C,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3D,
        name: "CMP",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3E,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x3F,
        name: "AAS",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x40,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x41,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x42,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x43,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x44,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x45,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x46,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x47,
        name: "INC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x48,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x49,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4A,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4B,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4C,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4D,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4E,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x4F,
        name: "DEC",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x50,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x51,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x52,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x53,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x54,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x55,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x56,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x57,
        name: "PUSH",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x58,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x59,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5A,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5B,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5C,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5D,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5E,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x5F,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x60,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x61,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x62,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x63,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x64,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x65,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x66,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x67,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x68,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x69,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6A,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6B,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6C,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6D,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6E,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x6F,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x70,
        name: "JO",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x71,
        name: "JNO",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x72,
        name: "JB",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x73,
        name: "JNB",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x74,
        name: "JZ",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x75,
        name: "JNZ",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x76,
        name: "JBE",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x77,
        name: "JA",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x78,
        name: "JS",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x79,
        name: "JNS",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7A,
        name: "JPE",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7B,
        name: "JPO",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7C,
        name: "JL",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7D,
        name: "JGE",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7E,
        name: "JLE",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x7F,
        name: "JG",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x80,
        name: "GRP1",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x81,
        name: "GRP1",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x82,
        name: "GRP1",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x83,
        name: "GRP1",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x84,
        name: "TEST",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x85,
        name: "TEST",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x86,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x87,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x88,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x89,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8A,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8B,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8C,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8D,
        name: "LEA",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8E,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x8F,
        name: "POP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x90,
        name: "NOP",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x91,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x92,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x93,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x94,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x95,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x96,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x97,
        name: "XCHG",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x98,
        name: "CBW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x99,
        name: "CWD",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9A,
        name: "CALL",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9B,
        name: "WAIT",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9C,
        name: "PUSHF",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9D,
        name: "POPF",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9E,
        name: "SAHF",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0x9F,
        name: "LAHF",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA0,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA1,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA2,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA3,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA4,
        name: "MOVSB",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA5,
        name: "MOVSW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA6,
        name: "CMPSB",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA7,
        name: "CMPSW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA8,
        name: "TEST",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xA9,
        name: "TEST",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAA,
        name: "STOSB",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAB,
        name: "STOSW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAC,
        name: "LODSB",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAD,
        name: "LODSW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAE,
        name: "SCASB",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xAF,
        name: "SCASW",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB0,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB1,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB2,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB3,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB4,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB5,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB6,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB7,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB8,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xB9,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBA,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBB,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBC,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBD,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBE,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xBF,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC0,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC1,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC2,
        name: "RET",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC3,
        name: "RET",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC4,
        name: "LES",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC5,
        name: "LDS",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC6,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC7,
        name: "MOV",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC8,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xC9,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCA,
        name: "RETF",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCB,
        name: "RETF",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCC,
        name: "INT",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCD,
        name: "INT",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCE,
        name: "INTO",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xCF,
        name: "IRET",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD0,
        name: "GRP2",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD1,
        name: "GRP2",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD2,
        name: "GRP2",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD3,
        name: "GRP2",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD4,
        name: "AAM",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD5,
        name: "AAD",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD6,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD7,
        name: "XLAT",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD8,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xD9,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDA,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDB,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDC,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDD,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDE,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xDF,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE0,
        name: "LOOPNZ",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE1,
        name: "LOOPZ",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE2,
        name: "LOOP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE3,
        name: "JCXZ",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE4,
        name: "IN",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE5,
        name: "IN",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE6,
        name: "OUT",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE7,
        name: "OUT",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE8,
        name: "CALL",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xE9,
        name: "JMP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xEA,
        name: "JMP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xEB,
        name: "JMP",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xEC,
        name: "IN",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xED,
        name: "IN",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xEE,
        name: "OUT",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xEF,
        name: "OUT",
        args: 2,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF0,
        name: "LOCK",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF1,
        name: "--",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF2,
        name: "REPNZ",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF3,
        name: "REPZ",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF4,
        name: "HLT",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF5,
        name: "CMC",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF6,
        name: "GRP3a",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF7,
        name: "GRP3b",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF8,
        name: "CLC",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xF9,
        name: "STC",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFA,
        name: "CLI",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFB,
        name: "STI",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFC,
        name: "CLD",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFD,
        name: "STD",
        args: 0,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFE,
        name: "GRP4",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
    Instruction {
        code: 0xFF,
        name: "GRP5",
        args: 1,
        func: Box::new(|computer: &mut Computer| {
            return computer;
        }),
    },
];

*/