use crate::value::Value;


#[test]
fn test_value_null() {
    let null: Value = Into::<Value>::into(None::<Value>);
    assert_eq!(null, Value::Null);
}
