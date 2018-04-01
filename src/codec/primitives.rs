use serde::{Serializer, Deserializer, Deserialize};
use serde::de::Unexpected;

use super::{SerializeValue, DeserializeValue};

impl SerializeValue for bool {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match *self {
            true => s.serialize_str("true"),
            false => s.serialize_str("false")
        }
    }
}

impl DeserializeValue for bool {
    fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
        match Deserialize::deserialize(d)? {
            "True" | "true" => Ok(true),
            "False" | "false" => Ok(true),
            string => {
                Err(::serde::de::Error::invalid_value(Unexpected::Str(string),
                    &"a boolean identifier"))
            }
        }
    }
}
