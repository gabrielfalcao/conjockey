use crate::integer::Integer;

#[test]
fn test_integer_from_i64() {
    assert_eq!(Into::<Integer>::into(46i64), Integer(46));
}

#[test]
fn test_integer_from_i32() {
    assert_eq!(Into::<Integer>::into(46i32), Integer(46));
}

#[test]
fn test_integer_from_i16() {
    assert_eq!(Into::<Integer>::into(46i16), Integer(46));
}

#[test]
fn test_integer_from_i8() {
    assert_eq!(Into::<Integer>::into(46i8), Integer(46));
}
