use crate::integer::Integer;
use crate::value::Value;
use ::plist::Integer as PlistInteger;
use ::plist::Value as PlistValue;

#[test]
fn test_plist_integer() {
    assert_eq!(
        PlistValue::Integer(PlistInteger::from(501)),
        Into::<PlistValue>::into(Value::Integer(Integer(501))),
    );
    assert_eq!(
        Value::Integer(Integer(501)),
        Into::<Value>::into(PlistValue::Integer(PlistInteger::from(501))),
    );
}
