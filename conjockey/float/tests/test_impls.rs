use crate::float::Float;

#[test]
fn test_float_from_f64() {
    assert_eq!(Into::<Float>::into(2.718281828459045f64), Float(2.718281828459045f64));
}
