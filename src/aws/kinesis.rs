//! Types for the `Kinesis` service.

/// The [`AWS::Kinesis::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html) resource type.
#[derive(Debug)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Debug)]
pub struct StreamProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `RetentionPeriodHours`.
    pub retention_period_hours: Option<::Value<u32>>,
    /// Property `ShardCount`.
    pub shard_count: ::Value<u32>,
    /// Property `StreamEncryption`.
    pub stream_encryption: Option<::Value<self::stream::StreamEncryption>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriodHours", &self.retention_period_hours)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShardCount", &self.shard_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamEncryption", &self.stream_encryption)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;
                let mut retention_period_hours = None;
                let mut shard_count = None;
                let mut stream_encryption = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RetentionPeriodHours" => {
                            retention_period_hours = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ShardCount" => {
                            shard_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StreamEncryption" => {
                            stream_encryption = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(StreamProperties {
                    name: name,
                    retention_period_hours: retention_period_hours,
                    shard_count: shard_count.ok_or(::serde::de::Error::missing_field("ShardCount"))?,
                    stream_encryption: stream_encryption,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Stream {
    type Properties = StreamProperties;
    const TYPE: &'static str = "AWS::Kinesis::Stream";
    fn properties(&self) -> &StreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stream {}

impl From<StreamProperties> for Stream {
    fn from(properties: StreamProperties) -> Stream {
        Stream { properties }
    }
}

pub mod stream {
    //! Property types for the `Stream` resource.

    /// The [`AWS::Kinesis::Stream.StreamEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streamencryption.html) property type.
    #[derive(Debug)]
    pub struct StreamEncryption {
        /// Property `EncryptionType`.
        pub encryption_type: ::Value<String>,
        /// Property `KeyId`.
        pub key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for StreamEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyId", &self.key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type = None;
                    let mut key_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KeyId" => {
                                key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamEncryption {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                        key_id: key_id.ok_or(::serde::de::Error::missing_field("KeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
