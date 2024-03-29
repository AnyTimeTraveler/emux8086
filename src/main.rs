use std::fs::File;
use std::io::Read;

use emux8086::Computer;

use crate::emux8086::debug::print_registers;

mod emux8086;

fn main() {
    let mut file = File::open("asm/copy").expect("Error opening file!");

    let mut computer = Computer::new();

    let mut program_data = [0u8; 1024];

    let read = file.read(&mut program_data).expect("Error reading file!");

    println!("{} bytes loaded!", read);

    computer.load_program(&program_data);

    loop {
        computer.step();
        print_registers(&computer);
    }
}
