//! Types for the `Kinesis` service.

/// The [`AWS::Kinesis::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html) resource type.
#[derive(Debug, Default)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Debug, Default)]
pub struct StreamProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RetentionPeriodHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-retentionperiodhours).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_period_hours: Option<::Value<u32>>,
    /// Property [`ShardCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-shardcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub shard_count: ::Value<u32>,
    /// Property [`StreamEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-streamencryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_encryption: Option<::Value<self::stream::StreamEncryption>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref retention_period_hours) = self.retention_period_hours {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriodHours", retention_period_hours)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShardCount", &self.shard_count)?;
        if let Some(ref stream_encryption) = self.stream_encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamEncryption", stream_encryption)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut name: Option<::Value<String>> = None;
                let mut retention_period_hours: Option<::Value<u32>> = None;
                let mut shard_count: Option<::Value<u32>> = None;
                let mut stream_encryption: Option<::Value<self::stream::StreamEncryption>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionPeriodHours" => {
                            retention_period_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ShardCount" => {
                            shard_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamEncryption" => {
                            stream_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Stream {
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
    #[derive(Debug, Default)]
    pub struct StreamEncryption {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streamencryption.html#cfn-kinesis-stream-streamencryption-encryptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_type: ::Value<String>,
        /// Property [`KeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streamencryption.html#cfn-kinesis-stream-streamencryption-keyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyId" => {
                                key_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
