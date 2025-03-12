use crate::integer::Integer;
use crate::value::Value;
use tera::Value as TeraValue;
use tera::Number as TeraNumber;

#[test]
fn test_tera_integer() {
    assert_eq!(
        TeraValue::Number(TeraNumber::from(501)),
        Into::<TeraValue>::into(Value::Integer(Integer(501))),
    );
    assert_eq!(
        Value::Integer(Integer(501)),
        Into::<Value>::into(TeraValue::Number(TeraNumber::from(501))),
    );
}
