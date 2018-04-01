use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

/// Like `Value`, except it is used in place of lists of `Value`s in
/// templates.
///
/// For example, if you have a parameter called `SubnetIds` of type
/// `List<AWS::EC2::Subnet::Id>` then, you can use `ValueList::Ref("SubnetIds".to_owned())`
/// to reference it.
#[derive(Debug)]
pub enum ValueList<T> {
    /// List of values.
    Values(Vec<::Value<T>>),
    /// Returns the value of the specified _parameter_ or _resource_. 
    Ref(String)
}

impl<T> ValueList<T> {
    /// If the list contains values, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_values(&self) -> Option<&[::Value<T>]> {
        if let &ValueList::Values(ref values) = self {
            Some(values)
        } else {
            None
        }
    }

    /// If the list is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let &ValueList::Ref(ref ref_id) = self {
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
#[serde(untagged, bound = "T: SerializeValue")]
enum SerializeValueList<'a, T: 'a> {
    Values(&'a Vec<::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>)
}

#[derive(Deserialize)]
#[serde(untagged, bound = "T: DeserializeValue")]
enum DeserializeValueList<'a, T> {
    Values(Vec<::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>)
}

impl<T: SerializeValue> Serialize for ValueList<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            &ValueList::Values(ref values) => SerializeValueList::Values(values),
            &ValueList::Ref(ref ref_id) => SerializeValueList::Ref(SerdeRef { ref_id })
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for ValueList<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            match proxy {
                DeserializeValueList::Values(t) => ValueList::Values(t),
                DeserializeValueList::Ref(SerdeRef { ref_id }) => ValueList::Ref(ref_id.to_owned())
            }
        })
    }
}
