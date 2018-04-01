use serde::{Serializer, Deserializer};

#[macro_use] mod macros;
mod primitives;

pub trait SerializeValue {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>;

    fn serialize_borrow<S: Serializer>(self_ref: &&Self, s: S) -> Result<S::Ok, S::Error> {
        (*self_ref).serialize(s)
    }
}

pub trait DeserializeValue {
    fn deserialize<'de, D>(d: D) -> Result<Self, D::Error>
        where Self: Sized,
              D: Deserializer<'de>;
}

cfn_internal__str_codec_impls!(u32);
cfn_internal__str_codec_impls!(u64);
cfn_internal__str_codec_impls!(f64);

cfn_internal__inherit_codec_impls!(String);
cfn_internal__inherit_codec_impls!(::types::Tag);
cfn_internal__inherit_codec_impls!(::serde_json::Value);
