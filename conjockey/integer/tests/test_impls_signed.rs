use crate::integer::Integer;
#[test]
fn test_integer_is_signed() {
    assert_eq!(Integer(5).is_signed(), false);
    assert_eq!(Integer(-5).is_signed(), true)
}


#[test]
fn test_integer_as_signed() {
    assert_eq!(Integer(5).as_signed(), None);
    assert_eq!(Integer(-5).as_signed(), Some(-5i64))
}
