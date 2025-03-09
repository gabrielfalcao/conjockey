use crate::value::Value;
use ::tera::Value as Tera;

#[test]
fn test_tera_boolean_false() {
    assert_eq!(
        Tera::Bool(false),
        Into::<Tera>::into(Value::Boolean(false)),
    );
    assert_eq!(
        Value::Boolean(false),
        Into::<Value>::into(Tera::Bool(false)),
    );
}

#[test]
fn test_tera_boolean_true() {
    assert_eq!(
        Tera::Bool(true),
        Into::<Tera>::into(Value::Boolean(true)),
    );
    assert_eq!(
        Value::Boolean(true),
        Into::<Value>::into(Tera::Bool(true)),
    );
}
