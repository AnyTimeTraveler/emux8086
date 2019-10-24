use crate::emux8086::Memory;
use crate::emux8086::mod_byte::{DataDirection, get_rm_index, InstructionWidth, read_reg};
use crate::emux8086::alu::add;
use crate::emux8086::registers::Registers;

pub fn mov(src: &[u8], src_pos: u16, dst: &mut [u8], dst_pos: u16, count: u8) {
    for i in 0..count as u16 {
        dst[(dst_pos + i) as usize] = src[(src_pos + i) as usize];
    }
}


pub fn add_based_on_modrm_byte(memory: &mut Memory,registers: &mut Registers, width: InstructionWidth, direction: DataDirection) {
    let ip = registers.read_u16(registers.ip) as usize;
    let rm_index = get_rm_index(&*memory, &registers) as usize;
    let reg_index = read_reg(memory[ip + 1], &registers, &width);
    let second_reg = read_reg(memory[ip + 1], &registers, &width);
    let mut src_register = [0u8; 2];
    src_register.copy_from_slice(&registers.raw[reg_index..reg_index + 2]);
    match direction {
        DataDirection::MemToReg => add(&src_register, &mut memory[rm_index..]),
        DataDirection::RegToMem => add(&memory[rm_index..], &mut registers.raw[reg_index..]),
        DataDirection::RegToReg => add(&src_register, &mut registers.raw[second_reg..]),
    }
}

pub fn jmp(registers: &mut Registers, change: usize) {

}