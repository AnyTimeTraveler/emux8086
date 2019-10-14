mod emux8086;

use crate::emux8086::{INSTRUCTIONS, Computer, CPU};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("copy").expect("Error opening file!");

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

    let mut counter = 0usize;

    while counter < read {
        let inst = &INSTRUCTIONS[computer.memory[counter] as usize];
        counter += 1;
        print!("{:4}", inst.name);
        if !inst.args[0].is_empty() {
            print!(" {:3}", computer.memory[counter]);
            counter += 1;
        }
        if !inst.args[1].is_empty() {
            print!(" {:3}", computer.memory[counter]);
            counter += 1;
        }
        println!();
    }
}


