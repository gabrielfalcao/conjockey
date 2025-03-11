use crate::integer::Integer;

#[test]
fn test_integer_is_signed() {
    assert_eq!(Integer(-9).is_signed(), true);
}

#[test]
fn test_integer_as_signed() {
    assert_eq!(Integer(-9).as_signed(), Some(-9i64));
}

#[test]
fn test_integer_as_signed_i64() {
    assert_eq!(Integer(-9).as_signed_i64(), Some(-9i64));
}

#[test]
fn test_integer_as_signed_i32() {
    assert_eq!(Integer(-9).as_signed_i32(), Some(-9i32));
}

#[test]
fn test_integer_as_signed_i16() {
    assert_eq!(Integer(-9).as_signed_i16(), Some(-9i16));
}

#[test]
fn test_integer_as_signed_i8() {
    assert_eq!(Integer(-9).as_signed_i8(), Some(-9i8));
}

#[test]
fn test_integer_to_signed() {
    assert_eq!(Integer(-9).to_signed(), -9i64);
}

#[test]
fn test_integer_to_signed_i64() {
    assert_eq!(Integer(-9).to_signed_i64(), -9i64);
}

#[test]
fn test_integer_to_signed_i32() {
    assert_eq!(Integer(-9).to_signed_i32(), -9i32);
}

#[test]
fn test_integer_to_signed_i16() {
    assert_eq!(Integer(-9).to_signed_i16(), -9i16);
}

#[test]
fn test_integer_to_signed_i8() {
    assert_eq!(Integer(-9).to_signed_i8(), -9i8);
}

#[test]
#[should_panic]
fn test_integer_to_signed_i32_panics() {
    Integer(-9223372036854775808).to_signed_i32();
}

#[test]
#[should_panic]
fn test_integer_to_signed_i16_panics() {
    Integer(-2147483648).to_signed_i16();
}

#[test]
#[should_panic]
fn test_integer_to_signed_i8_panics() {
    Integer(-32768).to_signed_i8();
}
