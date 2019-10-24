#![allow(dead_code)]
use crate::emux8086::debug::{u16_as_hex, u8_as_hex};
use crate::emux8086::instructions::INSTRUCTIONS;
use crate::emux8086::registers::Registers;
use crate::emux8086::utils::write_word;

mod utils;
mod instructions;
mod registers;
mod mod_byte;
mod instruction_implementations;
mod alu;
pub mod debug;
#[cfg(test)]
mod tests;

pub struct Computer {
    pub registers: Registers,
    pub memory: Memory,
    pub io: IO,
}

type Memory = [u8; 1_048_576];
type IO = [u8; 65_536];


impl Computer {
    pub fn new() -> Self {
        Computer {
            registers: Registers::new(),
            memory: [0u8; 1_048_576],
            io: [0u8; 65_536],
        }
    }

    pub fn load_program(&mut self, data: &[u8]) {
        self.memory[0x100..0x100 + data.len()].copy_from_slice(data);
        write_word(self.registers.mut_point_to_word(self.registers.ip), 0x100);
    }

    pub fn step(&mut self) {
        let ip = self.registers.read_u16(self.registers.ip);
        let inst = &INSTRUCTIONS[self.memory[ip as usize] as usize];
        println!("IP: 0x{}", u16_as_hex(ip));
        print!("0x{:3}: {} {} {} | {:5} {:5}", u8_as_hex(inst.code), inst.name, inst.args[0],inst.args[1],u8_as_hex(self.memory[ip as usize + 1]), u8_as_hex(self.memory[ip as usize + 2]));

        (inst.execute)(&mut self.io, &mut self.memory, &mut self.registers);

        write_word(self.registers.mut_point_to_word(self.registers.ip), ip + inst.ip_change as u16);
        println!();
        if inst.ip_change == 0 {
            panic!("\n\n\n\
            ==========================\
            \n\n\
            0x{}  {} is not implemented!\
            \n\n\
            ==========================\
            \n\n\n\n", u8_as_hex(inst.code), inst.name);
        }
    }
}
