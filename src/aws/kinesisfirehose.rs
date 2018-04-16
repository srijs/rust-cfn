//! Types for the `KinesisFirehose` service.

/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource type.
#[derive(Debug)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Debug)]
pub struct DeliveryStreamProperties {
    /// Property [`DeliveryStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-deliverystreamname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub delivery_stream_name: Option<::Value<String>>,
    /// Property [`DeliveryStreamType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-deliverystreamtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub delivery_stream_type: Option<::Value<String>>,
    /// Property [`ElasticsearchDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elasticsearch_destination_configuration: Option<::Value<self::delivery_stream::ElasticsearchDestinationConfiguration>>,
    /// Property [`ExtendedS3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extended_s3_destination_configuration: Option<::Value<self::delivery_stream::ExtendedS3DestinationConfiguration>>,
    /// Property [`KinesisStreamSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>>,
    /// Property [`RedshiftDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redshift_destination_configuration: Option<::Value<self::delivery_stream::RedshiftDestinationConfiguration>>,
    /// Property [`S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_destination_configuration: Option<::Value<self::delivery_stream::S3DestinationConfiguration>>,
}

impl ::serde::Serialize for DeliveryStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delivery_stream_name) = self.delivery_stream_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamName", delivery_stream_name)?;
        }
        if let Some(ref delivery_stream_type) = self.delivery_stream_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamType", delivery_stream_type)?;
        }
        if let Some(ref elasticsearch_destination_configuration) = self.elasticsearch_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchDestinationConfiguration", elasticsearch_destination_configuration)?;
        }
        if let Some(ref extended_s3_destination_configuration) = self.extended_s3_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedS3DestinationConfiguration", extended_s3_destination_configuration)?;
        }
        if let Some(ref kinesis_stream_source_configuration) = self.kinesis_stream_source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamSourceConfiguration", kinesis_stream_source_configuration)?;
        }
        if let Some(ref redshift_destination_configuration) = self.redshift_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftDestinationConfiguration", redshift_destination_configuration)?;
        }
        if let Some(ref s3_destination_configuration) = self.s3_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DestinationConfiguration", s3_destination_configuration)?;
        }
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
                let mut delivery_stream_name: Option<::Value<String>> = None;
                let mut delivery_stream_type: Option<::Value<String>> = None;
                let mut elasticsearch_destination_configuration: Option<::Value<self::delivery_stream::ElasticsearchDestinationConfiguration>> = None;
                let mut extended_s3_destination_configuration: Option<::Value<self::delivery_stream::ExtendedS3DestinationConfiguration>> = None;
                let mut kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>> = None;
                let mut redshift_destination_configuration: Option<::Value<self::delivery_stream::RedshiftDestinationConfiguration>> = None;
                let mut s3_destination_configuration: Option<::Value<self::delivery_stream::S3DestinationConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeliveryStreamName" => {
                            delivery_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryStreamType" => {
                            delivery_stream_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticsearchDestinationConfiguration" => {
                            elasticsearch_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtendedS3DestinationConfiguration" => {
                            extended_s3_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisStreamSourceConfiguration" => {
                            kinesis_stream_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedshiftDestinationConfiguration" => {
                            redshift_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3DestinationConfiguration" => {
                            s3_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for DeliveryStream {
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
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html#cfn-kinesisfirehose-deliverystream-bufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: ::Value<u32>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html#cfn-kinesisfirehose-deliverystream-bufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for BufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut interval_in_seconds: Option<::Value<u32>> = None;
                    let mut size_in_m_bs: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntervalInSeconds" => {
                                interval_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInMBs" => {
                                size_in_m_bs = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html#cfn-kinesisfirehose-deliverystream-cloudwatchloggingoptions-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html#cfn-kinesisfirehose-deliverystream-cloudwatchloggingoptions-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html#cfn-kinesisfirehose-deliverystream-cloudwatchloggingoptions-logstreamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_stream_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchLoggingOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_stream_name) = self.log_stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamName", log_stream_name)?;
            }
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
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogStreamName" => {
                                log_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`CopyOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html#cfn-kinesisfirehose-deliverystream-copycommand-copyoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_options: Option<::Value<String>>,
        /// Property [`DataTableColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html#cfn-kinesisfirehose-deliverystream-copycommand-datatablecolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_table_columns: Option<::Value<String>>,
        /// Property [`DataTableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html#cfn-kinesisfirehose-deliverystream-copycommand-datatablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for CopyCommand {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_options) = self.copy_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyOptions", copy_options)?;
            }
            if let Some(ref data_table_columns) = self.data_table_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTableColumns", data_table_columns)?;
            }
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
                    let mut copy_options: Option<::Value<String>> = None;
                    let mut data_table_columns: Option<::Value<String>> = None;
                    let mut data_table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyOptions" => {
                                copy_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTableColumns" => {
                                data_table_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTableName" => {
                                data_table_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html#cfn-kinesisfirehose-deliverystream-elasticsearchbufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: ::Value<u32>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html#cfn-kinesisfirehose-deliverystream-elasticsearchbufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ElasticsearchBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut interval_in_seconds: Option<::Value<u32>> = None;
                    let mut size_in_m_bs: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntervalInSeconds" => {
                                interval_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInMBs" => {
                                size_in_m_bs = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: ::Value<ElasticsearchBufferingHints>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`DomainARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-domainarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_arn: ::Value<String>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`IndexRotationPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-indexrotationperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_rotation_period: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: ::Value<ElasticsearchRetryOptions>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: ::Value<String>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-typename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticsearchDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainARN", &self.domain_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexRotationPeriod", &self.index_rotation_period)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
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
                    let mut buffering_hints: Option<::Value<ElasticsearchBufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut domain_arn: Option<::Value<String>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut index_rotation_period: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<ElasticsearchRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut type_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainARN" => {
                                domain_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexRotationPeriod" => {
                                index_rotation_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryOptions" => {
                                retry_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BackupMode" => {
                                s3_backup_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Configuration" => {
                                s3_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TypeName" => {
                                type_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html#cfn-kinesisfirehose-deliverystream-elasticsearchretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ElasticsearchRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut duration_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInSeconds" => {
                                duration_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`KMSEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html#cfn-kinesisfirehose-deliverystream-encryptionconfiguration-kmsencryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_encryption_config: Option<::Value<KMSEncryptionConfig>>,
        /// Property [`NoEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html#cfn-kinesisfirehose-deliverystream-encryptionconfiguration-noencryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_encryption_config: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_encryption_config) = self.kms_encryption_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSEncryptionConfig", kms_encryption_config)?;
            }
            if let Some(ref no_encryption_config) = self.no_encryption_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoEncryptionConfig", no_encryption_config)?;
            }
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
                    let mut kms_encryption_config: Option<::Value<KMSEncryptionConfig>> = None;
                    let mut no_encryption_config: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSEncryptionConfig" => {
                                kms_encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoEncryptionConfig" => {
                                no_encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`BucketARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`CompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-compressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_format: ::Value<String>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-s3backupconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_configuration: Option<::Value<S3DestinationConfiguration>>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExtendedS3DestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", &self.compression_format)?;
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_configuration) = self.s3_backup_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupConfiguration", s3_backup_configuration)?;
            }
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
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
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut buffering_hints: Option<::Value<BufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut compression_format: Option<::Value<String>> = None;
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompressionFormat" => {
                                compression_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BackupConfiguration" => {
                                s3_backup_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BackupMode" => {
                                s3_backup_mode = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`AWSKMSKeyARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kmsencryptionconfig.html#cfn-kinesisfirehose-deliverystream-kmsencryptionconfig-awskmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub awskms_key_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KMSEncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut awskms_key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AWSKMSKeyARN" => {
                                awskms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`KinesisStreamARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration-kinesisstreamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_stream_arn: ::Value<String>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut kinesis_stream_arn: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KinesisStreamARN" => {
                                kinesis_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html#cfn-kinesisfirehose-deliverystream-processingconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Processors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html#cfn-kinesisfirehose-deliverystream-processingconfiguration-processors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processors: ::ValueList<Processor>,
    }

    impl ::codec::SerializeValue for ProcessingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut processors: Option<::ValueList<Processor>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Processors" => {
                                processors = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html#cfn-kinesisfirehose-deliverystream-processor-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: ::ValueList<ProcessorParameter>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html#cfn-kinesisfirehose-deliverystream-processor-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Processor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut parameters: Option<::ValueList<ProcessorParameter>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html#cfn-kinesisfirehose-deliverystream-processorparameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html#cfn-kinesisfirehose-deliverystream-processorparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProcessorParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`ClusterJDBCURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-clusterjdbcurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_jdbcurl: ::Value<String>,
        /// Property [`CopyCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-copycommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_command: ::Value<CopyCommand>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterJDBCURL", &self.cluster_jdbcurl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyCommand", &self.copy_command)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
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
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut cluster_jdbcurl: Option<::Value<String>> = None;
                    let mut copy_command: Option<::Value<CopyCommand>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterJDBCURL" => {
                                cluster_jdbcurl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyCommand" => {
                                copy_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Configuration" => {
                                s3_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`BucketARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`CompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-compressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_format: ::Value<String>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3DestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", &self.buffering_hints)?;
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", &self.compression_format)?;
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
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
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut buffering_hints: Option<::Value<BufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut compression_format: Option<::Value<String>> = None;
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompressionFormat" => {
                                compression_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
