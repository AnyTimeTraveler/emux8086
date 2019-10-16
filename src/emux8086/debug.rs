use crate::emux8086::Computer;

pub fn u16_as_hex(data: u16) -> String {
    format!("{:01$X}", data, 4)
}

pub fn u8_as_hex(data: u8) -> String {
    format!("{:01$X}", data, 2)
}

pub fn print_registers(computer: &Computer) {
    println!(
        "Registers:\n\
        \tAX {}|{}\n\
        \tBX {}|{}\n\
        \tCX {}|{}\n\
        \tDX {}|{}\n",
        u8_as_hex(*computer.registers.ah), u8_as_hex(*computer.registers.al),
        u8_as_hex(*computer.registers.bh), u8_as_hex(*computer.registers.bl),
        u8_as_hex(*computer.registers.ch), u8_as_hex(*computer.registers.cl),
        u8_as_hex(*computer.registers.dh), u8_as_hex(*computer.registers.dl)
    )
}

pub fn new_print_registers(computer: &Computer) {
    println!(
        "Registers:\n\
        \tAX {:02x}|{:02x}\n\
        \tBX {:02x}|{:02x}\n\
        \tCX {:02x}|{:02x}\n\
        \tDX {:02x}|{:02x}\n",
        *computer.registers.ah, *computer.registers.al,
        *computer.registers.bh, *computer.registers.bl,
        *computer.registers.ch, *computer.registers.cl,
        *computer.registers.dh, *computer.registers.dl
    )
}