use crate::integer::Integer;

#[test]
fn test_integer_is_unsigned() {
    assert_eq!(Integer(1).is_unsigned(), true);
    assert_eq!(Integer(-8).is_unsigned(), false);
}

#[test]
fn test_integer_as_unsigned() {
    assert_eq!(Integer(1).as_unsigned(), Some(1u64));
    assert_eq!(Integer(-5).as_unsigned(), None)
}

#[test]
fn test_integer_as_unsigned_u64() {
    assert_eq!(Integer(1).as_unsigned_u64(), Some(1u64));
    assert_eq!(Integer(-5).as_unsigned_u64(), None)
}

#[test]
fn test_integer_as_unsigned_u32() {
    assert_eq!(Integer(1).as_unsigned_u32(), Some(1u32));
    assert_eq!(Integer(-5).as_unsigned_u32(), None)
}

#[test]
fn test_integer_as_unsigned_u16() {
    assert_eq!(Integer(1).as_unsigned_u16(), Some(1u16));
    assert_eq!(Integer(-5).as_unsigned_u16(), None)
}

#[test]
fn test_integer_as_unsigned_u8() {
    assert_eq!(Integer(1).as_unsigned_u8(), Some(1u8));
    assert_eq!(Integer(-5).as_unsigned_u8(), None)
}

#[test]
fn test_integer_to_unsigned() {
    assert_eq!(Integer(1).to_unsigned(), 1u64);
}

#[test]
fn test_integer_to_unsigned_u64() {
    assert_eq!(Integer(1).to_unsigned_u64(), 1u64);
}

#[test]
fn test_integer_to_unsigned_u32() {
    assert_eq!(Integer(1).to_unsigned_u32(), 1u32);
}

#[test]
fn test_integer_to_unsigned_u16() {
    assert_eq!(Integer(1).to_unsigned_u16(), 1u16);
}

#[test]
fn test_integer_to_unsigned_u8() {
    assert_eq!(Integer(1).to_unsigned_u8(), 1u8);
}

#[test]
#[should_panic]
fn test_integer_to_unsigned_panics() {
    Integer(-5).to_unsigned();
}

#[test]
#[should_panic]
fn test_integer_to_unsigned_u64_panics() {
    Integer(-5).to_unsigned_u64();
}

#[test]
#[should_panic]
fn test_integer_to_unsigned_u32_panics() {
    Integer(-5).to_unsigned_u32();
}

#[test]
#[should_panic]
fn test_integer_to_unsigned_u16_panics() {
    Integer(-5).to_unsigned_u16();
}

#[test]
#[should_panic]
fn test_integer_to_unsigned_u8_panics() {
    Integer(-5).to_unsigned_u8();
}
