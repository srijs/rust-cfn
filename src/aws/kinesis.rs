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
    pub shard_count: Option<::Value<u32>>,
    /// Property [`StreamEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-streamencryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_encryption: Option<::Value<self::stream::StreamEncryption>>,
    /// Property [`StreamModeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html#cfn-kinesis-stream-streammodedetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_mode_details: Option<::Value<self::stream::StreamModeDetails>>,
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
        if let Some(ref shard_count) = self.shard_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShardCount", shard_count)?;
        }
        if let Some(ref stream_encryption) = self.stream_encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamEncryption", stream_encryption)?;
        }
        if let Some(ref stream_mode_details) = self.stream_mode_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamModeDetails", stream_mode_details)?;
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
                let mut stream_mode_details: Option<::Value<self::stream::StreamModeDetails>> = None;
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
                        "StreamModeDetails" => {
                            stream_mode_details = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    shard_count: shard_count,
                    stream_encryption: stream_encryption,
                    stream_mode_details: stream_mode_details,
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

/// The [`AWS::Kinesis::StreamConsumer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-streamconsumer.html) resource type.
#[derive(Debug, Default)]
pub struct StreamConsumer {
    properties: StreamConsumerProperties
}

/// Properties for the `StreamConsumer` resource.
#[derive(Debug, Default)]
pub struct StreamConsumerProperties {
    /// Property [`ConsumerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-streamconsumer.html#cfn-kinesis-streamconsumer-consumername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub consumer_name: ::Value<String>,
    /// Property [`StreamARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-streamconsumer.html#cfn-kinesis-streamconsumer-streamarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stream_arn: ::Value<String>,
}

impl ::serde::Serialize for StreamConsumerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerName", &self.consumer_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamARN", &self.stream_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamConsumerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamConsumerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamConsumerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamConsumerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut consumer_name: Option<::Value<String>> = None;
                let mut stream_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConsumerName" => {
                            consumer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamARN" => {
                            stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamConsumerProperties {
                    consumer_name: consumer_name.ok_or(::serde::de::Error::missing_field("ConsumerName"))?,
                    stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamARN"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StreamConsumer {
    type Properties = StreamConsumerProperties;
    const TYPE: &'static str = "AWS::Kinesis::StreamConsumer";
    fn properties(&self) -> &StreamConsumerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamConsumerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StreamConsumer {}

impl From<StreamConsumerProperties> for StreamConsumer {
    fn from(properties: StreamConsumerProperties) -> StreamConsumer {
        StreamConsumer { properties }
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

    /// The [`AWS::Kinesis::Stream.StreamModeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streammodedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamModeDetails {
        /// Property [`StreamMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streammodedetails.html#cfn-kinesis-stream-streammodedetails-streammode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for StreamModeDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamMode", &self.stream_mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamModeDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamModeDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamModeDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamModeDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamMode" => {
                                stream_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamModeDetails {
                        stream_mode: stream_mode.ok_or(::serde::de::Error::missing_field("StreamMode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
