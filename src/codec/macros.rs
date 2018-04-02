#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__inherit_serialize_impl {
    ( $t:ty ) => {
        impl ::codec::SerializeValue for $t {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                ::serde::Serialize::serialize(self, s)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__inherit_deserialize_impl {
    ( $t:ty ) => {
        impl ::codec::DeserializeValue for $t {
            fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<$t, D::Error> {
                ::serde::Deserialize::deserialize(d)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__inherit_codec_impls {
    ( $t:ty ) => {
        cfn_internal__inherit_serialize_impl!($t);
        cfn_internal__inherit_deserialize_impl!($t);
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__num_serialize_impl {
    ( $t:ty ) => {
        impl ::codec::SerializeValue for $t {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                ::serde::Serializer::collect_str(s, self)
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__num_deserialize_impl {
    ( $t:ty ) => {
        impl ::codec::DeserializeValue for $t {
            fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<$t, D::Error> {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Value<'a> {
                    Number($t),
                    String(&'a str)
                }
                match ::serde::Deserialize::deserialize(d)? {
                    Value::Number(number) => Ok(number),
                    Value::String(string) => {
                        string.parse().map_err(|_err| {
                            ::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(string),
                                &stringify!($t))
                        })
                    }
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! cfn_internal__num_codec_impls {
    ( $t:ty ) => {
        cfn_internal__num_serialize_impl!($t);
        cfn_internal__num_deserialize_impl!($t);
    }
}
