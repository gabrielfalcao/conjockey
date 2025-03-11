use crate::integer::Integer;

#[test]
fn test_integer_add() {
    assert_eq!(Integer(2) + Integer(2), Integer(4));
}

#[test]
fn test_integer_sub() {
    assert_eq!(Integer(9) - Integer(2), Integer(7));
}

#[test]
fn test_integer_add_assign() {
    let mut value = Integer(2);
    value += Integer(2);
    assert_eq!(value, Integer(4));
}

#[test]
fn test_integer_sub_assign() {
    let mut value = Integer(9);
    value -= Integer(2);
    assert_eq!(value, Integer(7));
}

#[test]
fn test_integer_mul() {
    assert_eq!(Integer(3) * Integer(5), Integer(15));
}

#[test]
fn test_integer_mul_assign() {
    let mut value = Integer(3);
    value *= Integer(5);
    assert_eq!(value, Integer(15));
}

#[test]
fn test_integer_div() {
    assert_eq!(Integer(10) / Integer(2), Integer(5));
}

#[test]
fn test_integer_div_assign() {
    let mut value = Integer(10);
    value /= Integer(2);
    assert_eq!(value, Integer(5));
}

#[test]
fn test_integer_rem() {
    assert_eq!(Integer(11) % Integer(2), Integer(1));
}

#[test]
fn test_integer_neg() {
    assert_eq!(-Integer(-1), Integer(1));
}

#[test]
fn test_integer_not() {
    assert_eq!(!Integer(6), Integer(-7));
}

#[test]
fn test_integer_rem_assign() {
    let mut value = Integer(11);
    value %= Integer(2);
    assert_eq!(value, Integer(1));
}

#[test]
fn test_integer_ord() {
    let two = Integer(2);
    let three = Integer(3);
    let mut twotri = vec![three, two];
    twotri.sort();
    assert_eq!(twotri, vec![two, three]);
}
