use crate::integer::Integer;
use ::plist::Integer as PlistInteger;

#[test]
fn test_integer_into_plist_integer() {
    assert_eq!(PlistInteger(901), Into::<PlistInteger>::into(Integer(901)));
}

#[test]
fn test_integer_from_plist_integer() {
    assert_eq!(Integer(901), Into::<Integer>::into(901 as PlistInteger));
}
