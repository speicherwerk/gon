use serde_json::Value as JsonValue;

use crate::Value;

impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        match value {
            Value::None => JsonValue::Null,
            Value::Bool(b) => JsonValue::Bool(b),
            // FIXME
            Value::Num(_) => {
                if let Some(n) = value.as_i128() {
                    JsonValue::Number(serde_json::Number::from_i128(n).unwrap())
                } else {
                    JsonValue::Number(
                        serde_json::Number::from_f64(value.as_f64().unwrap()).unwrap(),
                    )
                }
            }
            Value::Str(s) => JsonValue::String(s),
            Value::List(xs) => JsonValue::Array(xs.into_iter().map(Value::into).collect()),
            Value::Obj(obj) => {
                JsonValue::Object(obj.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        match value {
            JsonValue::Null => Value::None,
            JsonValue::Bool(b) => Value::Bool(b),
            // FIXME
            JsonValue::Number(n) => Value::Num(n.to_string()),
            JsonValue::String(s) => Value::Str(s),
            JsonValue::Array(xs) => Value::List(xs.into_iter().map(JsonValue::into).collect()),
            JsonValue::Object(obj) => {
                Value::Obj(obj.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}
