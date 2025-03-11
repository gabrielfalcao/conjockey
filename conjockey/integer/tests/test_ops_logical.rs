use crate::integer::Integer;

#[test]
fn test_integer_and() {
    assert_eq!(Integer(0b101) & Integer(0b101), Integer(0b101));
}

#[test]
fn test_integer_or() {
    assert_eq!(Integer(0) | Integer(1), Integer(1));
    assert_eq!(Integer(1) | Integer(0), Integer(1));
}

#[test]
fn test_integer_and_assign() {
    let mut value = Integer(0b101);
    value &= Integer(0b101);
    assert_eq!(value, Integer(0b101));
}

#[test]
fn test_integer_or_assign() {
    let mut value = Integer(0);
    value |= Integer(1);
    assert_eq!(value, Integer(1));
}

#[test]
fn test_integer_shr() {
    assert_eq!(Integer(0b10100000) >> Integer(5), Integer(0b101));
}

#[test]
fn test_integer_shr_assign() {
    let mut value = Integer(0b10100000);
    value >>= Integer(5);
    assert_eq!(value, Integer(0b101));
}

#[test]
fn test_integer_shl() {
    assert_eq!(Integer(0b1001) << Integer(2), Integer(0b100100));
}

#[test]
fn test_integer_shl_assign() {
    let mut value = Integer(0b1001);
    value <<= Integer(2);
    assert_eq!(value, Integer(0b100100));
}

#[test]
fn test_integer_xor() {
    assert_eq!(Integer(0b100) ^ Integer(0b001), Integer(0b101));
}

#[test]
fn test_integer_xor_assign() {
    let mut value = Integer(0b100);
    value ^= Integer(0b001);
    assert_eq!(value, Integer(0b101));
}
