mod utils;
mod instructions;
mod memory;
mod registers;
mod mod_byte;
mod instruction_implementations;
pub mod debug;

use crate::emux8086::memory::Memory;
use crate::emux8086::registers::Registers;
use crate::emux8086::instructions::INSTRUCTIONS;
use crate::emux8086::utils::as_u16;
use crate::emux8086::debug::u8_as_hex;

pub struct Computer<'a> {
    pub registers: Registers<'a>,
    pub memory: Memory<'a>,
    pub io: [u8;65_536],
}

impl <'a>Computer<'a> {

    pub fn new() -> Self {
        Computer {
            registers: Registers::new(),
            memory: Memory::new(),
            io: [0u8;65_536],
        }
    }

    pub fn load_program(&mut self,data: &[u8]) {
        self.memory.program_memory.copy_from_slice(data);
    }

    pub fn step(&mut self) {
        let inst = &INSTRUCTIONS[self.memory.program_memory[as_u16(self.registers.ip, 0) as usize] as usize];
        print!("{:5} {}", inst.name, u8_as_hex(inst.code));
        (inst.execute)(self);
        // TODO: Implement byte-wise addition
        self.registers.ip[0] += inst.ip_change;
        println!();
        if inst.ip_change == 0 {
            println!("\n\n\n\n{} is not implemented!", inst.name);
            return;
        }
    }
}
