use crate::value::Value;
use ::plist::Value as Plist;

#[test]
fn test_plist_boolean_false() {
    assert_eq!(
        Plist::Boolean(false),
        Into::<Plist>::into(Value::Boolean(false)),
    );
    assert_eq!(
        Value::Boolean(false),
        Into::<Value>::into(Plist::Boolean(false)),
    );
}

#[test]
fn test_plist_boolean_true() {
    assert_eq!(
        Plist::Boolean(true),
        Into::<Plist>::into(Value::Boolean(true)),
    );
    assert_eq!(
        Value::Boolean(true),
        Into::<Value>::into(Plist::Boolean(true)),
    );
}
