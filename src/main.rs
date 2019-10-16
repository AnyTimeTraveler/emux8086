mod emux8086;

use emux8086::computer::Computer;
use emux8086::instructions::INSTRUCTIONS;
use emux8086::utils::u8_as_hex;
use std::fs::File;
use std::io::Read;
use crate::emux8086::utils::{print_registers, write_word, as_u16};

fn main() {
    let mut file = File::open("asm/copy").expect("Error opening file!");

    let mut computer = Computer::new();

    let read = file.read(computer.memory.program_memory).expect("Error reading file!");

    println!("{} bytes loaded!", read);

    write_word(computer.registers.ip, 0, 0x100);

    loop {
        let inst = &INSTRUCTIONS[computer.memory.program_memory[as_u16(computer.registers.ip, 0) as usize] as usize];
        print!("{:5} {}", inst.name, u8_as_hex(inst.code));
        (inst.execute)(&mut computer);
        // TODO: Implement byte-wise addition
//        computer.registers.ip += inst.ip_change;
        println!();
        if inst.ip_change == 0 {
            println!("\n\n\n\n{} is not implemented!", inst.name);
            return;
        }
        print_registers(&computer);
    }
}
