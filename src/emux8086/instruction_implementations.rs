use crate::emux8086::Computer;
use crate::emux8086::mod_byte::{DataDirection, get_rm_index, InstructionWidth, read_reg};
use crate::emux8086::alu::add;

pub fn mov(src: &[u8], src_pos: u16, dst: &mut [u8], dst_pos: u16, count: u8) {
    for i in 0..count as u16 {
        dst[(dst_pos + i) as usize] = src[(src_pos + i) as usize];
    }
}


pub fn add_based_on_modrm_byte(computer: &mut Computer, width: InstructionWidth, direction: DataDirection) {
    let ip = computer.registers.read_u16(computer.registers.ip) as usize;
    let rm_index = get_rm_index(&computer.memory, &computer.registers) as usize;
    let reg_index = read_reg(computer.memory[ip + 1], &computer.registers, width);
    match direction {
        DataDirection::MemToReg => add(&computer.registers.raw[reg_index..], &mut computer.memory[rm_index..]),
        DataDirection::RegToMem => add(&computer.memory[rm_index..], &mut computer.registers.raw[reg_index..]),
    }
}
