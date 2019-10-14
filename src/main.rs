use std::fs::File;
use std::io::Read;

use crate::emux8086::{Computer, CPU, INSTRUCTIONS};

mod emux8086;

fn main() {
    let mut file = File::open("asm/copy").expect("Error opening file!");

    let mut computer = Computer {
        cpu: CPU {
            ax: [0u8; 2],
            bx: [0u8; 2],
            cx: [0u8; 2],
            dx: [0u8; 2],
            sp: 0,
            bp: 0,
            si: 0,
            di: 0,
            ip: 0,
            flags: [false; 16],
            cs: 0,
            ss: 0,
            ds: 0,
            es: 0,
        },
        memory: [0u8; 1_048_576],
        io: [0u8; 65_536],
    };

    let read = file.read(&mut computer.memory).expect("Error reading file!");

    let mut c = &mut computer;
    while c.cpu.ip < read as u16 {
        let instruction = &INSTRUCTIONS[c.memory[c.cpu.ip as usize] as usize];
        c = (instruction.execute)(c);


    }
}


