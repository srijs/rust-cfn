use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

/// Value that a property can assume.
///
/// This can either be a literal value, or an invocation of an instrinsic function.
#[derive(Debug)]
pub enum Value<T> {
    /// Literal value.
    Value(T),
    /// Returns the value of the specified _parameter_ or _resource_. 
    Ref(String)
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
    Ref(SerdeRef<'a>)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DeserializeValueProxy<'a, T: DeserializeValue> {
    #[serde(deserialize_with="DeserializeValue::deserialize")]
    Value(T),
    #[serde(borrow)]
    Ref(SerdeRef<'a>)
}

impl<T: SerializeValue> Serialize for Value<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            &Value::Value(ref literal) => SerializeValueProxy::Value(literal),
            &Value::Ref(ref ref_id) => SerializeValueProxy::Ref(SerdeRef { ref_id })
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for Value<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            match proxy {
                DeserializeValueProxy::Value(t) => Value::Value(t),
                DeserializeValueProxy::Ref(SerdeRef { ref_id }) => Value::Ref(ref_id.to_owned())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

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
}
