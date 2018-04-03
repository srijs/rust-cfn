use std::collections::HashMap;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

/// Like `Value`, except it is used in place of maps of `Value`s in
/// templates.
#[derive(Debug)]
pub enum ValueMap<T> {
    /// Map of values.
    Values(HashMap<String, ::Value<T>>),
    /// Returns the value of the specified _parameter_ or _resource_. 
    Ref(String)
}

impl<T> ValueMap<T> {
    /// If the map contains values, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_values(&self) -> Option<&HashMap<String, ::Value<T>>> {
        if let &ValueMap::Values(ref values) = self {
            Some(values)
        } else {
            None
        }
    }

    /// If the map is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let &ValueMap::Ref(ref ref_id) = self {
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
enum SerializeValueMap<'a, T: 'a> {
    Values(&'a HashMap<String, ::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>)
}

#[derive(Deserialize)]
#[serde(untagged, bound = "T: DeserializeValue")]
enum DeserializeValueMap<'a, T> {
    Values(HashMap<String, ::Value<T>>),
    #[serde(borrow)]
    Ref(SerdeRef<'a>)
}

impl<T: SerializeValue> Serialize for ValueMap<T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let proxy = match self {
            &ValueMap::Values(ref values) => SerializeValueMap::Values(values),
            &ValueMap::Ref(ref ref_id) => SerializeValueMap::Ref(SerdeRef { ref_id })
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for ValueMap<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            match proxy {
                DeserializeValueMap::Values(t) => ValueMap::Values(t),
                DeserializeValueMap::Ref(SerdeRef { ref_id }) => ValueMap::Ref(ref_id.to_owned())
            }
        })
    }
}
