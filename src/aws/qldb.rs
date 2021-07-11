//! Types for the `QLDB` service.

/// The [`AWS::QLDB::Ledger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html) resource type.
#[derive(Debug, Default)]
pub struct Ledger {
    properties: LedgerProperties
}

/// Properties for the `Ledger` resource.
#[derive(Debug, Default)]
pub struct LedgerProperties {
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html#cfn-qldb-ledger-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<::Value<bool>>,
    /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html#cfn-qldb-ledger-kmskey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html#cfn-qldb-ledger-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`PermissionsMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html#cfn-qldb-ledger-permissionsmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions_mode: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-ledger.html#cfn-qldb-ledger-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LedgerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref kms_key) = self.kms_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", kms_key)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionsMode", &self.permissions_mode)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LedgerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LedgerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LedgerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LedgerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deletion_protection: Option<::Value<bool>> = None;
                let mut kms_key: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permissions_mode: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKey" => {
                            kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionsMode" => {
                            permissions_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LedgerProperties {
                    deletion_protection: deletion_protection,
                    kms_key: kms_key,
                    name: name,
                    permissions_mode: permissions_mode.ok_or(::serde::de::Error::missing_field("PermissionsMode"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Ledger {
    type Properties = LedgerProperties;
    const TYPE: &'static str = "AWS::QLDB::Ledger";
    fn properties(&self) -> &LedgerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LedgerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Ledger {}

impl From<LedgerProperties> for Ledger {
    fn from(properties: LedgerProperties) -> Ledger {
        Ledger { properties }
    }
}

/// The [`AWS::QLDB::Stream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html) resource type.
#[derive(Debug, Default)]
pub struct Stream {
    properties: StreamProperties
}

/// Properties for the `Stream` resource.
#[derive(Debug, Default)]
pub struct StreamProperties {
    /// Property [`ExclusiveEndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-exclusiveendtime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub exclusive_end_time: Option<::Value<String>>,
    /// Property [`InclusiveStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-inclusivestarttime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inclusive_start_time: ::Value<String>,
    /// Property [`KinesisConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-kinesisconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kinesis_configuration: ::Value<self::stream::KinesisConfiguration>,
    /// Property [`LedgerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-ledgername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ledger_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-streamname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stream_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-qldb-stream.html#cfn-qldb-stream-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref exclusive_end_time) = self.exclusive_end_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusiveEndTime", exclusive_end_time)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusiveStartTime", &self.inclusive_start_time)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisConfiguration", &self.kinesis_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LedgerName", &self.ledger_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", &self.stream_name)?;
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
                let mut exclusive_end_time: Option<::Value<String>> = None;
                let mut inclusive_start_time: Option<::Value<String>> = None;
                let mut kinesis_configuration: Option<::Value<self::stream::KinesisConfiguration>> = None;
                let mut ledger_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stream_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExclusiveEndTime" => {
                            exclusive_end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InclusiveStartTime" => {
                            inclusive_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisConfiguration" => {
                            kinesis_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LedgerName" => {
                            ledger_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamName" => {
                            stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamProperties {
                    exclusive_end_time: exclusive_end_time,
                    inclusive_start_time: inclusive_start_time.ok_or(::serde::de::Error::missing_field("InclusiveStartTime"))?,
                    kinesis_configuration: kinesis_configuration.ok_or(::serde::de::Error::missing_field("KinesisConfiguration"))?,
                    ledger_name: ledger_name.ok_or(::serde::de::Error::missing_field("LedgerName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stream_name: stream_name.ok_or(::serde::de::Error::missing_field("StreamName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Stream {
    type Properties = StreamProperties;
    const TYPE: &'static str = "AWS::QLDB::Stream";
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

    /// The [`AWS::QLDB::Stream.KinesisConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qldb-stream-kinesisconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisConfiguration {
        /// Property [`AggregationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qldb-stream-kinesisconfiguration.html#cfn-qldb-stream-kinesisconfiguration-aggregationenabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub aggregation_enabled: Option<::Value<bool>>,
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-qldb-stream-kinesisconfiguration.html#cfn-qldb-stream-kinesisconfiguration-streamarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub stream_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KinesisConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregation_enabled) = self.aggregation_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationEnabled", aggregation_enabled)?;
            }
            if let Some(ref stream_arn) = self.stream_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", stream_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregation_enabled: Option<::Value<bool>> = None;
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregationEnabled" => {
                                aggregation_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisConfiguration {
                        aggregation_enabled: aggregation_enabled,
                        stream_arn: stream_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
