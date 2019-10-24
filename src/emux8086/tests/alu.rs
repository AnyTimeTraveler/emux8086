use crate::emux8086::alu::add;

fn expect_add_u8(operand_a: u8, operand_b: u8, result: u8){
    let src = [operand_a as u8; 1];
    let mut dst = [operand_b as u8; 1];
    add(&src, &mut dst);
    assert_eq!(dst, [result; 1]);
}

#[test]
fn test_add_positive_u8_with_positive_u8() {
    expect_add_u8(10u8,10u8,20u8);
}

#[test]
fn test_add_positive_u8_with_negative_u8() {
    expect_add_u8(-10i8 as u8,200u8,190u8);
}

#[test]
fn test_add_negative_u8_with_negative_u8() {
    expect_add_u8(-10i8 as u8,-10i8 as u8,-20i8 as u8);
}
