use crate::value::Value;


#[test]
fn test_value_boolean_false() {
    let false_value: Value = Into::<Value>::into(false);
    assert_eq!(false_value, Value::Boolean(false));
}

#[test]
fn test_value_boolean_true() {
    let true_value: Value = Into::<Value>::into(true);
    assert_eq!(true_value, Value::Boolean(true));
}
