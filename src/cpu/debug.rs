use crate::cpu::Computer;

pub fn u16_as_hex(data: u16) -> String {
    format!("{:01$X}", data, 4)
}

pub fn u8_as_hex(data: u8) -> String {
    format!("{:01$X}", data, 2)
}

pub fn print_registers(computer: &Computer) {
    println!(
        "Registers:\n\
        \tAX {:02x}|{:02x}\n\
        \tBX {:02x}|{:02x}\n\
        \tCX {:02x}|{:02x}\n\
        \tDX {:02x}|{:02x}\n",
        computer.registers.read_u8(computer.registers.ah), computer.registers.read_u8(computer.registers.al),
        computer.registers.read_u8(computer.registers.bh), computer.registers.read_u8(computer.registers.bl),
        computer.registers.read_u8(computer.registers.ch), computer.registers.read_u8(computer.registers.cl),
        computer.registers.read_u8(computer.registers.dh), computer.registers.read_u8(computer.registers.dl)
    )
}