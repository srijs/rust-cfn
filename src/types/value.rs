use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

use super::Expr;

/// Value that a property can assume.
///
/// This can either be a literal value, or an invocation of an instrinsic function.
#[derive(Debug)]
pub struct Value<T>(ValueInner<T>);

impl<T> Value<T> {
    /// Create a new value.
    pub fn new(value: T) -> Value<T> {
        Value(ValueInner::Value(value))
    }

    /// Create a new value backed by a reference.
    pub fn reference<S: Into<String>>(id: S) -> Value<T> {
        Value(ValueInner::Ref(id.into()))
    }

    /// Create a new value backed by an expression.
    pub fn expression(expr: Expr) -> Value<T> {
        Value(ValueInner::Expr(expr))
    }

    /// If the value contains a literal value, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_value(&self) -> Option<&T> {
        if let ValueInner::Value(ref value) = self.0 {
            Some(value)
        } else {
            None
        }
    }

    /// If the value is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let ValueInner::Ref(ref ref_id) = self.0 {
            Some(ref_id)
        } else {
            None
        }
    }

    /// If the value is an expression, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_expression(&self) -> Option<&Expr> {
        if let ValueInner::Expr(ref expr) = self.0 {
            Some(expr)
        } else {
            None
        }
    }
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Value<T> {
        Value::new(value)
    }
}

#[derive(Debug)]
enum ValueInner<T> {
    Value(T),
    Ref(String),
    Expr(Expr)
}

#[derive(Serialize, Deserialize)]
struct SerdeRef<'a> {
    #[serde(rename = "Ref", borrow)]
    id: &'a str
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
        let proxy = match self.0 {
            ValueInner::Value(ref literal) => SerializeValueProxy::Value(literal),
            ValueInner::Ref(ref id) => SerializeValueProxy::Ref(SerdeRef { id }),
            ValueInner::Expr(ref expr) => SerializeValueProxy::Expr(expr)
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for Value<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            let inner = match proxy {
                DeserializeValueProxy::Value(t) => ValueInner::Value(t),
                DeserializeValueProxy::Ref(SerdeRef { id }) => ValueInner::Ref(id.to_owned()),
                DeserializeValueProxy::Expr(expr) => ValueInner::Expr(expr)
            };
            Value(inner)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Value, Expr};

    #[test]
    fn serialize_u32() {
        let value = Value::new(42u32);
        assert_eq!("\"42\"", ::serde_json::to_string(&value).unwrap());
    }

    #[test]
    fn serialize_u32_string() {
        let value = ::serde_json::from_str::<Value<u32>>("\"42\"").unwrap();
        assert_eq!(&42, value.as_value().unwrap());
    }

    #[test]
    fn serialize_u32_number() {
        let value = ::serde_json::from_str::<Value<u32>>("42").unwrap();
        assert_eq!(&42, value.as_value().unwrap());
    }

    #[test]
    fn serialize_ref() {
        let value = Value::<String>::reference("foo");
        assert_eq!("{\"Ref\":\"foo\"}", ::serde_json::to_string(&value).unwrap());
    }

    #[test]
    fn deserialize_ref() {
        let value = ::serde_json::from_str::<Value<String>>("{\"Ref\":\"foo\"}").unwrap();
        assert_eq!("foo", value.as_reference().unwrap());
    }

    #[test]
    fn serialize_fn_join() {
        let value = Value::<String>::expression(Expr::Join {
            delimiter: ":".to_owned(),
            values: vec![
                Value::new("a".to_owned()),
                Value::new("b".to_owned()),
                Value::new("c".to_owned())
            ]
        });
        assert_eq!("{\"Fn::Join\":[\":\",[\"a\",\"b\",\"c\"]]}",
            ::serde_json::to_string(&value).unwrap());
    }
}
