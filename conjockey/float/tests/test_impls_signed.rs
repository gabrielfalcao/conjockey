use crate::float::Float;

#[test]
fn test_float_is_signed() {
    assert_eq!(Float(-9.0).is_signed(), true);
}

#[test]
fn test_float_as_signed() {
    assert_eq!(Float(-9.0).as_signed(), Some(-9.0f64));
}

#[test]
fn test_float_as_signed_f64() {
    assert_eq!(Float(-9.0).as_signed_f64(), Some(-9.0f64));
}

#[test]
fn test_float_to_signed() {
    assert_eq!(Float(-9.0).to_signed(), -9f64);
}

#[test]
fn test_float_to_signed_f64() {
    assert_eq!(Float(-9.0).to_signed_f64(), -9f64);
}
