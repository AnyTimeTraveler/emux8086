use crate::emux8086::alu::add;
use crate::emux8086::utils::write_word;


fn expect_add_u8(operand_a: u8, operand_b: u8, result: u8) {
    let src = [operand_a as u8; 1];
    let mut dst = [operand_b as u8; 1];
    add(&src, &mut dst);
    assert_eq!(dst, [result; 1]);
}

fn expect_add_u16(operand_a: u16, operand_b: u16, result: u16) {
    let mut src = [0u8; 2];
    let mut dst = [0u8; 2];

    write_word(&mut src, operand_a);
    write_word(&mut dst, operand_b);

    add(&src, &mut dst);
    let mut result_bytes = [0u8; 2];
    write_word(&mut result_bytes, result);
    assert_eq!(dst, result_bytes);
}

#[test]
fn test_add_positive_u8_with_positive_u8() {
    expect_add_u8(10u8, 10u8, 20u8);
}

#[test]
fn test_add_positive_u8_with_negative_u8() {
    expect_add_u8(-10i8 as u8, 200u8, 190u8);
}

#[test]
fn test_add_negative_u8_with_negative_u8() {
    expect_add_u8(-10i8 as u8, -10i8 as u8, -20i8 as u8);
}

#[test]
fn test_add_positive_u16_with_positive_u16() {
    expect_add_u16(10u16, 10u16, 20u16);
}

#[test]
fn test_add_positive_u16_with_negative_u16() {
    expect_add_u16(-10i16 as u16, 200u16, 190u16);
}

#[test]
fn test_add_negative_u16_with_negative_u16() {
    expect_add_u16(-10i16 as u16, -10i16 as u16, -20i16 as u16);
}
