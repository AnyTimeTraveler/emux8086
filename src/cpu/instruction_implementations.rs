use crate::cpu::Memory;
use crate::cpu::registers::Registers;
use crate::cpu::mod_byte::{InstructionWidth, DataDirection, get_rm_index, read_reg};
use crate::cpu::alu::add;
use crate::cpu::utils::{write_word, fill_msb};

pub fn mov(src: &[u8], src_pos: u16, dst: &mut [u8], dst_pos: u16, count: u8) {
    for i in 0..count as u16 {
        dst[(dst_pos + i) as usize] = src[(src_pos + i) as usize];
    }
}


pub fn add_based_on_modrm_byte(memory: &mut Memory, registers: &mut Registers, width: InstructionWidth, direction: DataDirection) {
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
    let mut change_bytes = [0u8; 2];
    write_word(&mut change_bytes, change as u16);
    fill_msb(&mut change_bytes, 1);
//    println!("\nIP: {}\nChange: {}", u16_as_hex(read_word(registers.point_to_word(registers.ip))), read_word(&change_bytes) as i16);
    add(&change_bytes, registers.mut_point_to_word(registers.ip));
//    println!("New IP: {}", u16_as_hex(read_word(registers.point_to_word(registers.ip))));
}