use crate::emux8086::mod_byte::AddressingMode::{TwoByteSignedDisplacement, IndirectAddressingMode, OneByteSignedDisplacement, RegisterAddressingMode};
use crate::emux8086::mod_byte::InstructionWidth::{EightBits, SixteenBits};
use crate::emux8086::registers::Registers;
use crate::emux8086::utils::read_word;

#[derive(PartialEq)]
pub enum AddressingMode {
    IndirectAddressingMode,
    OneByteSignedDisplacement,
    TwoByteSignedDisplacement,
    RegisterAddressingMode,
}

#[derive(PartialEq)]
pub enum InstructionWidth {
    EightBits,
    SixteenBits,
}

#[derive(PartialEq)]
pub enum DataDirection {
    RegToMem,
    MemToReg,
    RegToReg,
}

pub fn read_mode(byte: u8) -> AddressingMode {
    match byte & 0b11000000 {
        0b00000000 => IndirectAddressingMode,
        0b01000000 => OneByteSignedDisplacement,
        0b10000000 => TwoByteSignedDisplacement,
        0b11000000 => RegisterAddressingMode,
        _ => panic!("Logic error!")
    }
}

pub fn read_reg(byte: u8, registers: &Registers, width: &InstructionWidth) -> usize {
    let reg_bits = byte & 0b00111000;
    if *width == EightBits {
        match reg_bits {
            0b00000000 => registers.al,
            0b00001000 => registers.cl,
            0b00010000 => registers.dl,
            0b00011000 => registers.bl,
            0b00100000 => registers.ah,
            0b00101000 => registers.ch,
            0b00110000 => registers.dh,
            0b00111000 => registers.bh,
            _ => panic!("Logic error!")
        }
    } else if *width == SixteenBits {
        match reg_bits {
            0b00000000 => registers.ax,
            0b00001000 => registers.cx,
            0b00010000 => registers.dx,
            0b00011000 => registers.bx,
            0b00100000 => registers.sp,
            0b00101000 => registers.bp,
            0b00110000 => registers.si,
            0b00111000 => registers.di,
            _ => panic!("Logic error!")
        }
    } else {
        panic!("Logic error!")
    }
}

pub fn get_rm_index(memory: &[u8], registers: &Registers) -> u16 {
    let ip = registers.read_u16(registers.ip);
    let byte = registers.read_u8((ip + 1) as usize);
    let disp8 = memory[ip as usize + 2] as u16;
    let disp16 = read_word(&memory[ip as usize + 2..]);

    match byte & 0b11000111 {
        0b00000000 => registers.read_u16(registers.bx) + registers.read_u16(registers.si), // [ BX + SI ]
        0b01000000 => registers.read_u16(registers.bx) + registers.read_u16(registers.si) + disp8, // [ BX + SI + disp8 ]
        0b10000000 => registers.read_u16(registers.bx) + registers.read_u16(registers.si) + disp16, // [ BP + SI + disp16 ]

        0b00000001 => registers.read_u16(registers.bx) + registers.read_u16(registers.di), // [ BX + DI ]
        0b01000001 => registers.read_u16(registers.bx) + registers.read_u16(registers.di) + disp8, // [ BX + DI + disp8 ]
        0b10000001 => registers.read_u16(registers.bx) + registers.read_u16(registers.di) + disp16, // [ BX + DI + disp16 ]

        0b00000010 => registers.read_u16(registers.bp) + registers.read_u16(registers.si), // [ BP + SI ]
        0b01000010 => registers.read_u16(registers.bp) + registers.read_u16(registers.si) + disp8, // [ BP + SI + disp8 ]
        0b10000010 => registers.read_u16(registers.bp) + registers.read_u16(registers.si) + disp16, // [ BP + SI + disp16 ]

        0b00000011 => registers.read_u16(registers.bp) + registers.read_u16(registers.di), // [ BP + DI ]
        0b01000011 => registers.read_u16(registers.bp) + registers.read_u16(registers.di) + disp8, // [ BP + DI + disp8 ]
        0b10000011 => registers.read_u16(registers.bp) + registers.read_u16(registers.di) + disp16, // [ BP + DI + disp16 ]

        0b00000100 => registers.read_u16(registers.si), // [ SI ]
        0b01000100 => registers.read_u16(registers.si) + disp8, // [ SI  +  disp8 ]
        0b10000100 => registers.read_u16(registers.si) + disp16, // [ SI  +  disp16 ]

        0b00000101 => registers.read_u16(registers.di), // [ DI ]
        0b01000101 => registers.read_u16(registers.di) + disp8, // [ DI + disp8 ]
        0b10000101 => registers.read_u16(registers.di) + disp16, // [ DI + disp16 ]

        0b00000110 => disp16, // [ disp16 ]
        0b01000110 => registers.read_u16(registers.bp) + disp8, // [ BP + disp8 ]
        0b10000110 => registers.read_u16(registers.bp) + disp16, // [ BP + disp16 ]

        0b00000111 => registers.read_u16(registers.bx), // [BX]
        0b01000111 => registers.read_u16(registers.bx) + disp8, // [ BX + disp8 ]
        0b10000111 => registers.read_u16(registers.bx) + disp16, // [ BX + disp16 ]
        _ => panic!("Logic error!")
    }
}