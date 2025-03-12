use crate::value::Value;
use plist::Value as Plist;

#[test]
fn test_plist_string() {
    assert_eq!(
        Plist::String(String::from("String")),
        Into::<Plist>::into(Value::String(String::from("String"))),
    );
    assert_eq!(
        Value::String(String::from("String")),
        Into::<Value>::into(Plist::String(String::from("String"))),
    );
}
