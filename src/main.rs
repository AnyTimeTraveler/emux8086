mod emux8086;

use emux8086::Computer;
use emux8086::instructions::INSTRUCTIONS;
use emux8086::utils::u8_as_hex;
use std::fs::File;
use std::io::Read;
use crate::emux8086::utils::print_registers;

fn main() {
    let mut file = File::open("asm/copy").expect("Error opening file!");

    let mut computer = Computer::new();

    let _read = file.read(&mut computer.memory).expect("Error reading file!");

    loop {
        let inst = &INSTRUCTIONS[computer.memory[computer.cpu.ip as usize] as usize];
        print!("{:5} {}", inst.name, u8_as_hex(inst.code));
        (inst.execute)(&mut computer);
        computer.cpu.ip += inst.ip_change as u16;
        println!();
        if inst.ip_change == 0 {
            println!("\n\n\n\n{} is not implemented!", inst.name);
            return;
        }
        print_registers(&computer);
    }
}


