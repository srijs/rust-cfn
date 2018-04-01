use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

use super::Expr;

/// Value that a property can assume.
///
/// This can either be a literal value, or an invocation of an instrinsic function.
#[derive(Debug)]
pub enum Value<T> {
    /// Literal value.
    Value(T),
    /// Returns the value of the specified _parameter_ or _resource_. 
    Ref(String),
    /// Expression.
    Expr(Expr)
}

impl<T> Value<T> {
    /// If the value contains a literal value, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_value(&self) -> Option<&T> {
        if let &Value::Value(ref value) = self {
            Some(value)
        } else {
            None
        }
    }

    /// If the value is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let &Value::Ref(ref ref_id) = self {
            Some(ref_id)
        } else {
            None
        }
    }

    /// If the value is an expression, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_expression(&self) -> Option<&Expr> {
        if let &Value::Expr(ref expr) = self {
            Some(expr)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerdeRef<'a> {
    #[serde(rename = "Ref", borrow)]
    ref_id: &'a str
}

#[derive(Serialize)]
#[serde(untagged)]
enum SerializeValueProxy<'a, T: 'a + SerializeValue> {
    #[serde(serialize_with="SerializeValue::serialize_borrow")]
    Value(&'a T),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    #[serde(serialize_with="SerializeValue::serialize_borrow")]
    Expr(&'a Expr),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DeserializeValueProxy<'a, T: DeserializeValue> {
    #[serde(deserialize_with="DeserializeValue::deserialize")]
    Value(T),
    #[serde(borrow)]
    Ref(SerdeRef<'a>),
    #[serde(deserialize_with="DeserializeValue::deserialize")]
    Expr(Expr)
}

impl<T: SerializeValue> Serialize for Value<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            &Value::Value(ref literal) => SerializeValueProxy::Value(literal),
            &Value::Ref(ref ref_id) => SerializeValueProxy::Ref(SerdeRef { ref_id }),
            &Value::Expr(ref expr) => SerializeValueProxy::Expr(expr)
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for Value<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            match proxy {
                DeserializeValueProxy::Value(t) => Value::Value(t),
                DeserializeValueProxy::Ref(SerdeRef { ref_id }) => Value::Ref(ref_id.to_owned()),
                DeserializeValueProxy::Expr(expr) => Value::Expr(expr)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, Expr};

    #[test]
    fn serialize_ref() {
        let value = Value::Ref::<String>("foo".to_owned());
        assert_eq!("{\"Ref\":\"foo\"}", ::serde_json::to_string(&value).unwrap());
    }

    #[test]
    fn deserialize_ref() {
        let value = ::serde_json::from_str::<Value<String>>("{\"Ref\":\"foo\"}").unwrap();
        assert_eq!("foo", value.as_reference().unwrap());
    }

    #[test]
    fn serialize_fn_join() {
        let value = Value::Expr::<String>(Expr::Join {
            delimiter: ":".to_owned(),
            values: vec![
                Value::Value("a".to_owned()),
                Value::Value("b".to_owned()),
                Value::Value("c".to_owned())
            ]
        });
        assert_eq!("{\"Fn::Join\":[\":\",[\"a\",\"b\",\"c\"]]}",
            ::serde_json::to_string(&value).unwrap());
    }
}
