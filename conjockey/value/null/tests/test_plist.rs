use crate::value::Value;
use plist::Value as Plist;

#[test]
fn test_plist_null() {
    assert_eq!(
        Plist::Boolean(false),
        Into::<Plist>::into(Value::Null),
    );
    assert_eq!(
        Value::Null,
        Into::<Value>::into(Plist::Boolean(false)),
    );
}
