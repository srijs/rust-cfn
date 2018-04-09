use std::iter::FromIterator;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use ::codec::{SerializeValue, DeserializeValue};

/// Like `Value`, except it is used in place of lists of `Value`s in
/// templates.
///
/// For example, if you have a parameter called `SubnetIds` of type
/// `List<AWS::EC2::Subnet::Id>` then, you can use `ValueList::reference("SubnetIds")`
/// to reference it.
#[derive(Debug)]
pub struct ValueList<T>(ValueListInner<T>);

impl<T> ValueList<T> {
    /// Create a new value list.
    pub fn new<I>(values: I) -> ValueList<T>
        where I: IntoIterator<Item = ::Value<T>>
    {
        ValueList(ValueListInner::Values(Vec::from_iter(values)))
    }

    /// Create a new value list backed by a reference.
    pub fn reference<S: Into<String>>(id: S) -> ValueList<T> {
        ValueList(ValueListInner::Ref(id.into()))
    }

    /// If the list contains values, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_values(&self) -> Option<&[::Value<T>]> {
        if let ValueListInner::Values(ref values) = self.0 {
            Some(values)
        } else {
            None
        }
    }

    /// If the list is a reference, return `Some`.
    ///
    /// Return `None` otherwise.
    pub fn as_reference(&self) -> Option<&str> {
        if let ValueListInner::Ref(ref ref_id) = self.0 {
            Some(ref_id)
        } else {
            None
        }
    }
}

impl<T> FromIterator<::Value<T>> for ValueList<T> {
    fn from_iter<I>(iter: I) -> ValueList<T>
        where I: IntoIterator<Item = ::Value<T>>
    {
        ValueList::new(iter)
    }
}

#[derive(Debug)]
enum ValueListInner<T> {
    Values(Vec<::Value<T>>),
    Ref(String)
}

#[derive(Serialize, Deserialize)]
struct SerdeRef<'a> {
    #[serde(rename = "Ref", borrow)]
    id: &'a str
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
        let proxy = match self.0 {
            ValueListInner::Values(ref values) => SerializeValueList::Values(values),
            ValueListInner::Ref(ref id) => SerializeValueList::Ref(SerdeRef { id })
        };
        Serialize::serialize(&proxy, s)
    }
}

impl<'de, T: DeserializeValue> Deserialize<'de> for ValueList<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(d).map(|proxy| {
            let inner = match proxy {
                DeserializeValueList::Values(t) =>
                    ValueListInner::Values(t),
                DeserializeValueList::Ref(SerdeRef { id }) =>
                    ValueListInner::Ref(id.to_owned())
            };
            ValueList(inner)
        })
    }
}
