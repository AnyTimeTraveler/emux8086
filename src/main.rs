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

    let mut data: Vec<u8> = Vec::new();

    let read = file.read_to_end(&mut data).expect("Error reading file!");

    computer.memory.write(&data.as_slice()[..read], 0x100);

    computer.cpu.ip.write_word(0x100);

    loop {
        let inst = &INSTRUCTIONS[computer.memory.read(computer.cpu.ip as usize,1) as usize];
        print!("{:5} {}", inst.name, u8_as_hex(inst.code));
        (inst.execute)(&mut computer);
        computer.cpu.ip += inst.ip_change;
        println!();
        if inst.ip_change == 0 {
            println!("\n\n\n\n{} is not implemented!", inst.name);
            return;
        }
        print_registers(&computer);
    }
}
