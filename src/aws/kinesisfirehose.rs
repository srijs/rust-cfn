//! Types for the `KinesisFirehose` service.

/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource type.
#[derive(Debug)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Debug)]
pub struct DeliveryStreamProperties {
    /// Property `DeliveryStreamName`.
    pub delivery_stream_name: Option<::Value<String>>,
    /// Property `DeliveryStreamType`.
    pub delivery_stream_type: Option<::Value<String>>,
    /// Property `ElasticsearchDestinationConfiguration`.
    pub elasticsearch_destination_configuration: Option<::Value<self::delivery_stream::ElasticsearchDestinationConfiguration>>,
    /// Property `ExtendedS3DestinationConfiguration`.
    pub extended_s3_destination_configuration: Option<::Value<self::delivery_stream::ExtendedS3DestinationConfiguration>>,
    /// Property `KinesisStreamSourceConfiguration`.
    pub kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>>,
    /// Property `RedshiftDestinationConfiguration`.
    pub redshift_destination_configuration: Option<::Value<self::delivery_stream::RedshiftDestinationConfiguration>>,
    /// Property `S3DestinationConfiguration`.
    pub s3_destination_configuration: Option<::Value<self::delivery_stream::S3DestinationConfiguration>>,
}

impl ::serde::Serialize for DeliveryStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamName", &self.delivery_stream_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamType", &self.delivery_stream_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchDestinationConfiguration", &self.elasticsearch_destination_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedS3DestinationConfiguration", &self.extended_s3_destination_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamSourceConfiguration", &self.kinesis_stream_source_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftDestinationConfiguration", &self.redshift_destination_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DestinationConfiguration", &self.s3_destination_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeliveryStreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeliveryStreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeliveryStreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeliveryStreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut delivery_stream_name = None;
                let mut delivery_stream_type = None;
                let mut elasticsearch_destination_configuration = None;
                let mut extended_s3_destination_configuration = None;
                let mut kinesis_stream_source_configuration = None;
                let mut redshift_destination_configuration = None;
                let mut s3_destination_configuration = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeliveryStreamName" => {
                            delivery_stream_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeliveryStreamType" => {
                            delivery_stream_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ElasticsearchDestinationConfiguration" => {
                            elasticsearch_destination_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ExtendedS3DestinationConfiguration" => {
                            extended_s3_destination_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KinesisStreamSourceConfiguration" => {
                            kinesis_stream_source_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RedshiftDestinationConfiguration" => {
                            redshift_destination_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3DestinationConfiguration" => {
                            s3_destination_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DeliveryStreamProperties {
                    delivery_stream_name: delivery_stream_name,
                    delivery_stream_type: delivery_stream_type,
                    elasticsearch_destination_configuration: elasticsearch_destination_configuration,
                    extended_s3_destination_configuration: extended_s3_destination_configuration,
                    kinesis_stream_source_configuration: kinesis_stream_source_configuration,
                    redshift_destination_configuration: redshift_destination_configuration,
                    s3_destination_configuration: s3_destination_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DeliveryStream {
    type Properties = DeliveryStreamProperties;
    const TYPE: &'static str = "AWS::KinesisFirehose::DeliveryStream";
    fn properties(&self) -> &DeliveryStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeliveryStreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeliveryStream {}

impl From<DeliveryStreamProperties> for DeliveryStream {
    fn from(properties: DeliveryStreamProperties) -> DeliveryStream {
        DeliveryStream { properties }
    }
}

pub mod delivery_stream {
    //! Property types for the `DeliveryStream` resource.

    /// The [`AWS::KinesisFirehose::DeliveryStream.BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html) property type.
    #[derive(Debug)]
    pub struct BufferingHints {
        /// Property `IntervalInSeconds`.
        pub interval_in_seconds: ::Value<u32>,
        /// Property `SizeInMBs`.
        pub size_in_m_bs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for BufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", &self.interval_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", &self.size_in_m_bs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BufferingHints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BufferingHints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BufferingHints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BufferingHints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interval_in_seconds = None;
                    let mut size_in_m_bs = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntervalInSeconds" => {
                                interval_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SizeInMBs" => {
                                size_in_m_bs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BufferingHints {
                        interval_in_seconds: interval_in_seconds.ok_or(::serde::de::Error::missing_field("IntervalInSeconds"))?,
                        size_in_m_bs: size_in_m_bs.ok_or(::serde::de::Error::missing_field("SizeInMBs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html) property type.
    #[derive(Debug)]
    pub struct CloudWatchLoggingOptions {
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `LogGroupName`.
        pub log_group_name: Option<::Value<String>>,
        /// Property `LogStreamName`.
        pub log_stream_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchLoggingOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamName", &self.log_stream_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLoggingOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLoggingOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLoggingOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLoggingOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled = None;
                    let mut log_group_name = None;
                    let mut log_stream_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LogGroupName" => {
                                log_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LogStreamName" => {
                                log_stream_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLoggingOptions {
                        enabled: enabled,
                        log_group_name: log_group_name,
                        log_stream_name: log_stream_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.CopyCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html) property type.
    #[derive(Debug)]
    pub struct CopyCommand {
        /// Property `CopyOptions`.
        pub copy_options: Option<::Value<String>>,
        /// Property `DataTableColumns`.
        pub data_table_columns: Option<::Value<String>>,
        /// Property `DataTableName`.
        pub data_table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for CopyCommand {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyOptions", &self.copy_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTableColumns", &self.data_table_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTableName", &self.data_table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CopyCommand {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CopyCommand, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CopyCommand;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CopyCommand")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_options = None;
                    let mut data_table_columns = None;
                    let mut data_table_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyOptions" => {
                                copy_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DataTableColumns" => {
                                data_table_columns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DataTableName" => {
                                data_table_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CopyCommand {
                        copy_options: copy_options,
                        data_table_columns: data_table_columns,
                        data_table_name: data_table_name.ok_or(::serde::de::Error::missing_field("DataTableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html) property type.
    #[derive(Debug)]
    pub struct ElasticsearchBufferingHints {
        /// Property `IntervalInSeconds`.
        pub interval_in_seconds: ::Value<u32>,
        /// Property `SizeInMBs`.
        pub size_in_m_bs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ElasticsearchBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", &self.interval_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", &self.size_in_m_bs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchBufferingHints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchBufferingHints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchBufferingHints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchBufferingHints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interval_in_seconds = None;
                    let mut size_in_m_bs = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntervalInSeconds" => {
                                interval_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SizeInMBs" => {
                                size_in_m_bs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchBufferingHints {
                        interval_in_seconds: interval_in_seconds.ok_or(::serde::de::Error::missing_field("IntervalInSeconds"))?,
                        size_in_m_bs: size_in_m_bs.ok_or(::serde::de::Error::missing_field("SizeInMBs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ElasticsearchDestinationConfiguration {
        /// Property `BufferingHints`.
        pub buffering_hints: ::Value<ElasticsearchBufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `DomainARN`.
        pub domain_arn: ::Value<String>,
        /// Property `IndexName`.
        pub index_name: ::Value<String>,
        /// Property `IndexRotationPeriod`.
        pub index_rotation_period: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RetryOptions`.
        pub retry_options: ::Value<ElasticsearchRetryOptions>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
        /// Property `S3BackupMode`.
        pub s3_backup_mode: ::Value<String>,
        /// Property `S3Configuration`.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property `TypeName`.
        pub type_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticsearchDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", &self.cloud_watch_logging_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainARN", &self.domain_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexRotationPeriod", &self.index_rotation_period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", &self.processing_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", &self.retry_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", &self.s3_backup_mode)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", &self.type_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffering_hints = None;
                    let mut cloud_watch_logging_options = None;
                    let mut domain_arn = None;
                    let mut index_name = None;
                    let mut index_rotation_period = None;
                    let mut processing_configuration = None;
                    let mut retry_options = None;
                    let mut role_arn = None;
                    let mut s3_backup_mode = None;
                    let mut s3_configuration = None;
                    let mut type_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DomainARN" => {
                                domain_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IndexName" => {
                                index_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IndexRotationPeriod" => {
                                index_rotation_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RetryOptions" => {
                                retry_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3BackupMode" => {
                                s3_backup_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Configuration" => {
                                s3_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TypeName" => {
                                type_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchDestinationConfiguration {
                        buffering_hints: buffering_hints.ok_or(::serde::de::Error::missing_field("BufferingHints"))?,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        domain_arn: domain_arn.ok_or(::serde::de::Error::missing_field("DomainARN"))?,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        index_rotation_period: index_rotation_period.ok_or(::serde::de::Error::missing_field("IndexRotationPeriod"))?,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options.ok_or(::serde::de::Error::missing_field("RetryOptions"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_mode: s3_backup_mode.ok_or(::serde::de::Error::missing_field("S3BackupMode"))?,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        type_name: type_name.ok_or(::serde::de::Error::missing_field("TypeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html) property type.
    #[derive(Debug)]
    pub struct ElasticsearchRetryOptions {
        /// Property `DurationInSeconds`.
        pub duration_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ElasticsearchRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", &self.duration_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchRetryOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_in_seconds = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInSeconds" => {
                                duration_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchRetryOptions {
                        duration_in_seconds: duration_in_seconds.ok_or(::serde::de::Error::missing_field("DurationInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EncryptionConfiguration {
        /// Property `KMSEncryptionConfig`.
        pub kms_encryption_config: Option<::Value<KMSEncryptionConfig>>,
        /// Property `NoEncryptionConfig`.
        pub no_encryption_config: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSEncryptionConfig", &self.kms_encryption_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoEncryptionConfig", &self.no_encryption_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_encryption_config = None;
                    let mut no_encryption_config = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSEncryptionConfig" => {
                                kms_encryption_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NoEncryptionConfig" => {
                                no_encryption_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        kms_encryption_config: kms_encryption_config,
                        no_encryption_config: no_encryption_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ExtendedS3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ExtendedS3DestinationConfiguration {
        /// Property `BucketARN`.
        pub bucket_arn: ::Value<String>,
        /// Property `BufferingHints`.
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `CompressionFormat`.
        pub compression_format: ::Value<String>,
        /// Property `EncryptionConfiguration`.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property `Prefix`.
        pub prefix: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
        /// Property `S3BackupConfiguration`.
        pub s3_backup_configuration: Option<::Value<S3DestinationConfiguration>>,
        /// Property `S3BackupMode`.
        pub s3_backup_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExtendedS3DestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", &self.cloud_watch_logging_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", &self.compression_format)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", &self.encryption_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", &self.processing_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupConfiguration", &self.s3_backup_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", &self.s3_backup_mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExtendedS3DestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExtendedS3DestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExtendedS3DestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExtendedS3DestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn = None;
                    let mut buffering_hints = None;
                    let mut cloud_watch_logging_options = None;
                    let mut compression_format = None;
                    let mut encryption_configuration = None;
                    let mut prefix = None;
                    let mut processing_configuration = None;
                    let mut role_arn = None;
                    let mut s3_backup_configuration = None;
                    let mut s3_backup_mode = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BufferingHints" => {
                                buffering_hints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CompressionFormat" => {
                                compression_format = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Prefix" => {
                                prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3BackupConfiguration" => {
                                s3_backup_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3BackupMode" => {
                                s3_backup_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ExtendedS3DestinationConfiguration {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                        buffering_hints: buffering_hints.ok_or(::serde::de::Error::missing_field("BufferingHints"))?,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        compression_format: compression_format.ok_or(::serde::de::Error::missing_field("CompressionFormat"))?,
                        encryption_configuration: encryption_configuration,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                        processing_configuration: processing_configuration,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_configuration: s3_backup_configuration,
                        s3_backup_mode: s3_backup_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.KMSEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kmsencryptionconfig.html) property type.
    #[derive(Debug)]
    pub struct KMSEncryptionConfig {
        /// Property `AWSKMSKeyARN`.
        pub awskms_key_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KMSEncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSKMSKeyARN", &self.awskms_key_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KMSEncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KMSEncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KMSEncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KMSEncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut awskms_key_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AWSKMSKeyARN" => {
                                awskms_key_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KMSEncryptionConfig {
                        awskms_key_arn: awskms_key_arn.ok_or(::serde::de::Error::missing_field("AWSKMSKeyARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.KinesisStreamSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html) property type.
    #[derive(Debug)]
    pub struct KinesisStreamSourceConfiguration {
        /// Property `KinesisStreamARN`.
        pub kinesis_stream_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamARN", &self.kinesis_stream_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kinesis_stream_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KinesisStreamARN" => {
                                kinesis_stream_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamSourceConfiguration {
                        kinesis_stream_arn: kinesis_stream_arn.ok_or(::serde::de::Error::missing_field("KinesisStreamARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ProcessingConfiguration {
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `Processors`.
        pub processors: ::ValueList<Processor>,
    }

    impl ::codec::SerializeValue for ProcessingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Processors", &self.processors)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProcessingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProcessingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProcessingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProcessingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled = None;
                    let mut processors = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Processors" => {
                                processors = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ProcessingConfiguration {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        processors: processors.ok_or(::serde::de::Error::missing_field("Processors"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.Processor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html) property type.
    #[derive(Debug)]
    pub struct Processor {
        /// Property `Parameters`.
        pub parameters: ::ValueList<ProcessorParameter>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Processor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Processor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Processor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Processor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Processor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameters = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Processor {
                        parameters: parameters.ok_or(::serde::de::Error::missing_field("Parameters"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessorParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html) property type.
    #[derive(Debug)]
    pub struct ProcessorParameter {
        /// Property `ParameterName`.
        pub parameter_name: ::Value<String>,
        /// Property `ParameterValue`.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProcessorParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProcessorParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProcessorParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProcessorParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProcessorParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name = None;
                    let mut parameter_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ParameterValue" => {
                                parameter_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ProcessorParameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.RedshiftDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html) property type.
    #[derive(Debug)]
    pub struct RedshiftDestinationConfiguration {
        /// Property `CloudWatchLoggingOptions`.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `ClusterJDBCURL`.
        pub cluster_jdbcurl: ::Value<String>,
        /// Property `CopyCommand`.
        pub copy_command: ::Value<CopyCommand>,
        /// Property `Password`.
        pub password: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
        /// Property `S3Configuration`.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property `Username`.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", &self.cloud_watch_logging_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterJDBCURL", &self.cluster_jdbcurl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyCommand", &self.copy_command)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", &self.processing_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logging_options = None;
                    let mut cluster_jdbcurl = None;
                    let mut copy_command = None;
                    let mut password = None;
                    let mut processing_configuration = None;
                    let mut role_arn = None;
                    let mut s3_configuration = None;
                    let mut username = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ClusterJDBCURL" => {
                                cluster_jdbcurl = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CopyCommand" => {
                                copy_command = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Password" => {
                                password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Configuration" => {
                                s3_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Username" => {
                                username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftDestinationConfiguration {
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        cluster_jdbcurl: cluster_jdbcurl.ok_or(::serde::de::Error::missing_field("ClusterJDBCURL"))?,
                        copy_command: copy_command.ok_or(::serde::de::Error::missing_field("CopyCommand"))?,
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        processing_configuration: processing_configuration,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html) property type.
    #[derive(Debug)]
    pub struct S3DestinationConfiguration {
        /// Property `BucketARN`.
        pub bucket_arn: ::Value<String>,
        /// Property `BufferingHints`.
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `CompressionFormat`.
        pub compression_format: ::Value<String>,
        /// Property `EncryptionConfiguration`.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property `Prefix`.
        pub prefix: Option<::Value<String>>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3DestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", &self.cloud_watch_logging_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", &self.compression_format)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", &self.encryption_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3DestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3DestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3DestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3DestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn = None;
                    let mut buffering_hints = None;
                    let mut cloud_watch_logging_options = None;
                    let mut compression_format = None;
                    let mut encryption_configuration = None;
                    let mut prefix = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BufferingHints" => {
                                buffering_hints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CompressionFormat" => {
                                compression_format = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Prefix" => {
                                prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3DestinationConfiguration {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                        buffering_hints: buffering_hints.ok_or(::serde::de::Error::missing_field("BufferingHints"))?,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        compression_format: compression_format.ok_or(::serde::de::Error::missing_field("CompressionFormat"))?,
                        encryption_configuration: encryption_configuration,
                        prefix: prefix,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
