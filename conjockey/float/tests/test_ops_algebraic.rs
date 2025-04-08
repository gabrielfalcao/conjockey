use crate::float::Float;

#[test]
fn test_float_add() {
    assert_eq!(Float(2.0) + Float(2.0), Float(4.0));
}

#[test]
fn test_float_sub() {
    assert_eq!(Float(9.0) - Float(2.0), Float(7.0));
}

#[test]
fn test_float_add_assign() {
    let mut value = Float(2.0);
    value += Float(2.0);
    assert_eq!(value, Float(4.0));
}

#[test]
fn test_float_sub_assign() {
    let mut value = Float(9.0);
    value -= Float(2.0);
    assert_eq!(value, Float(7.0));
}

#[test]
fn test_float_mul() {
    assert_eq!(Float(3.0) * Float(5.0), Float(15.0));
}

#[test]
fn test_float_mul_assign() {
    let mut value = Float(3.0);
    value *= Float(5.0);
    assert_eq!(value, Float(15.0));
}

#[test]
fn test_float_div() {
    assert_eq!(Float(10.0) / Float(2.0), Float(5.0));
}

#[test]
fn test_float_div_assign() {
    let mut value = Float(10.0);
    value /= Float(2.0);
    assert_eq!(value, Float(5.0));
}

#[test]
fn test_float_rem() {
    assert_eq!(Float(11.0) % Float(2.0), Float(1.0));
}

#[test]
fn test_float_neg() {
    assert_eq!(-Float(-1.0), Float(1.0));
}


#[test]
fn test_float_rem_assign() {
    let mut value = Float(11.0);
    value %= Float(2.0);
    assert_eq!(value, Float(1.0));
}

#[test]
fn test_float_ord() {
    let two = Float(2.0);
    let three = Float(3.0);
    let mut twotri = vec![three, two];
    twotri.sort();
    assert_eq!(twotri, vec![two, three]);
}
