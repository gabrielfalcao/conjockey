use crate::value::Value;
use tera::Value as Tera;

#[test]
fn test_tera_null() {
    assert_eq!(
        Tera::Null,
        Into::<Tera>::into(Value::Null),
    );
    assert_eq!(
        Value::Null,
        Into::<Value>::into(Tera::Null),
    );
}
