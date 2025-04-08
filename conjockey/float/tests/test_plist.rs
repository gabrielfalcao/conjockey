use crate::float::Float;
use ::plist::Real as PlistFloat;

#[test]
fn test_float_into_plist_float() {
    assert_eq!(PlistFloat(2.718281828459045.0), Into::<PlistFloat>::into(Float(2.718281828459045.0)));
}

#[test]
fn test_float_from_plist_float() {
    assert_eq!(Float(2.718281828459045.0), Into::<Float>::into(2.718281828459045 as PlistFloat));
}
