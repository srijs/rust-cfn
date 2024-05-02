//! Types for the `KinesisFirehose` service.

/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource type.
#[derive(Debug, Default)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Debug, Default)]
pub struct DeliveryStreamProperties {
    /// Property [`AmazonOpenSearchServerlessDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub amazon_open_search_serverless_destination_configuration: Option<::Value<self::delivery_stream::AmazonOpenSearchServerlessDestinationConfiguration>>,
    /// Property [`AmazonopensearchserviceDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub amazonopensearchservice_destination_configuration: Option<::Value<self::delivery_stream::AmazonopensearchserviceDestinationConfiguration>>,
    /// Property [`DeliveryStreamEncryptionConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_stream_encryption_configuration_input: Option<::Value<self::delivery_stream::DeliveryStreamEncryptionConfigurationInput>>,
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
    /// Property [`HttpEndpointDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_endpoint_destination_configuration: Option<::Value<self::delivery_stream::HttpEndpointDestinationConfiguration>>,
    /// Property [`KinesisStreamSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>>,
    /// Property [`MSKSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-msksourceconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub msk_source_configuration: Option<::Value<self::delivery_stream::MSKSourceConfiguration>>,
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
    /// Property [`SnowflakeDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snowflake_destination_configuration: Option<::Value<self::delivery_stream::SnowflakeDestinationConfiguration>>,
    /// Property [`SplunkDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub splunk_destination_configuration: Option<::Value<self::delivery_stream::SplunkDestinationConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html#cfn-kinesisfirehose-deliverystream-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DeliveryStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref amazon_open_search_serverless_destination_configuration) = self.amazon_open_search_serverless_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonOpenSearchServerlessDestinationConfiguration", amazon_open_search_serverless_destination_configuration)?;
        }
        if let Some(ref amazonopensearchservice_destination_configuration) = self.amazonopensearchservice_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonopensearchserviceDestinationConfiguration", amazonopensearchservice_destination_configuration)?;
        }
        if let Some(ref delivery_stream_encryption_configuration_input) = self.delivery_stream_encryption_configuration_input {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamEncryptionConfigurationInput", delivery_stream_encryption_configuration_input)?;
        }
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
        if let Some(ref http_endpoint_destination_configuration) = self.http_endpoint_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpEndpointDestinationConfiguration", http_endpoint_destination_configuration)?;
        }
        if let Some(ref kinesis_stream_source_configuration) = self.kinesis_stream_source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamSourceConfiguration", kinesis_stream_source_configuration)?;
        }
        if let Some(ref msk_source_configuration) = self.msk_source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MSKSourceConfiguration", msk_source_configuration)?;
        }
        if let Some(ref redshift_destination_configuration) = self.redshift_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftDestinationConfiguration", redshift_destination_configuration)?;
        }
        if let Some(ref s3_destination_configuration) = self.s3_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DestinationConfiguration", s3_destination_configuration)?;
        }
        if let Some(ref snowflake_destination_configuration) = self.snowflake_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnowflakeDestinationConfiguration", snowflake_destination_configuration)?;
        }
        if let Some(ref splunk_destination_configuration) = self.splunk_destination_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SplunkDestinationConfiguration", splunk_destination_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut amazon_open_search_serverless_destination_configuration: Option<::Value<self::delivery_stream::AmazonOpenSearchServerlessDestinationConfiguration>> = None;
                let mut amazonopensearchservice_destination_configuration: Option<::Value<self::delivery_stream::AmazonopensearchserviceDestinationConfiguration>> = None;
                let mut delivery_stream_encryption_configuration_input: Option<::Value<self::delivery_stream::DeliveryStreamEncryptionConfigurationInput>> = None;
                let mut delivery_stream_name: Option<::Value<String>> = None;
                let mut delivery_stream_type: Option<::Value<String>> = None;
                let mut elasticsearch_destination_configuration: Option<::Value<self::delivery_stream::ElasticsearchDestinationConfiguration>> = None;
                let mut extended_s3_destination_configuration: Option<::Value<self::delivery_stream::ExtendedS3DestinationConfiguration>> = None;
                let mut http_endpoint_destination_configuration: Option<::Value<self::delivery_stream::HttpEndpointDestinationConfiguration>> = None;
                let mut kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>> = None;
                let mut msk_source_configuration: Option<::Value<self::delivery_stream::MSKSourceConfiguration>> = None;
                let mut redshift_destination_configuration: Option<::Value<self::delivery_stream::RedshiftDestinationConfiguration>> = None;
                let mut s3_destination_configuration: Option<::Value<self::delivery_stream::S3DestinationConfiguration>> = None;
                let mut snowflake_destination_configuration: Option<::Value<self::delivery_stream::SnowflakeDestinationConfiguration>> = None;
                let mut splunk_destination_configuration: Option<::Value<self::delivery_stream::SplunkDestinationConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AmazonOpenSearchServerlessDestinationConfiguration" => {
                            amazon_open_search_serverless_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AmazonopensearchserviceDestinationConfiguration" => {
                            amazonopensearchservice_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryStreamEncryptionConfigurationInput" => {
                            delivery_stream_encryption_configuration_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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
                        "HttpEndpointDestinationConfiguration" => {
                            http_endpoint_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisStreamSourceConfiguration" => {
                            kinesis_stream_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MSKSourceConfiguration" => {
                            msk_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedshiftDestinationConfiguration" => {
                            redshift_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3DestinationConfiguration" => {
                            s3_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnowflakeDestinationConfiguration" => {
                            snowflake_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SplunkDestinationConfiguration" => {
                            splunk_destination_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeliveryStreamProperties {
                    amazon_open_search_serverless_destination_configuration: amazon_open_search_serverless_destination_configuration,
                    amazonopensearchservice_destination_configuration: amazonopensearchservice_destination_configuration,
                    delivery_stream_encryption_configuration_input: delivery_stream_encryption_configuration_input,
                    delivery_stream_name: delivery_stream_name,
                    delivery_stream_type: delivery_stream_type,
                    elasticsearch_destination_configuration: elasticsearch_destination_configuration,
                    extended_s3_destination_configuration: extended_s3_destination_configuration,
                    http_endpoint_destination_configuration: http_endpoint_destination_configuration,
                    kinesis_stream_source_configuration: kinesis_stream_source_configuration,
                    msk_source_configuration: msk_source_configuration,
                    redshift_destination_configuration: redshift_destination_configuration,
                    s3_destination_configuration: s3_destination_configuration,
                    snowflake_destination_configuration: snowflake_destination_configuration,
                    splunk_destination_configuration: splunk_destination_configuration,
                    tags: tags,
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

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonOpenSearchServerlessBufferingHints {
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: Option<::Value<u32>>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AmazonOpenSearchServerlessBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interval_in_seconds) = self.interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", interval_in_seconds)?;
            }
            if let Some(ref size_in_m_bs) = self.size_in_m_bs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", size_in_m_bs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonOpenSearchServerlessBufferingHints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonOpenSearchServerlessBufferingHints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonOpenSearchServerlessBufferingHints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonOpenSearchServerlessBufferingHints")
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

                    Ok(AmazonOpenSearchServerlessBufferingHints {
                        interval_in_seconds: interval_in_seconds,
                        size_in_m_bs: size_in_m_bs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonOpenSearchServerlessDestinationConfiguration {
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: Option<::Value<AmazonOpenSearchServerlessBufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`CollectionEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-collectionendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub collection_endpoint: Option<::Value<String>>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<AmazonOpenSearchServerlessRetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration-vpcconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_configuration: Option<::Value<VpcConfiguration>>,
    }

    impl ::codec::SerializeValue for AmazonOpenSearchServerlessDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref collection_endpoint) = self.collection_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionEndpoint", collection_endpoint)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonOpenSearchServerlessDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonOpenSearchServerlessDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonOpenSearchServerlessDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonOpenSearchServerlessDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffering_hints: Option<::Value<AmazonOpenSearchServerlessBufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut collection_endpoint: Option<::Value<String>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<AmazonOpenSearchServerlessRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut vpc_configuration: Option<::Value<VpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CollectionEndpoint" => {
                                collection_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmazonOpenSearchServerlessDestinationConfiguration {
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        collection_endpoint: collection_endpoint,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonOpenSearchServerlessRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessretryoptions.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserverlessretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AmazonOpenSearchServerlessRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonOpenSearchServerlessRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonOpenSearchServerlessRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonOpenSearchServerlessRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonOpenSearchServerlessRetryOptions")
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

                    Ok(AmazonOpenSearchServerlessRetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonopensearchserviceBufferingHints {
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: Option<::Value<u32>>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AmazonopensearchserviceBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interval_in_seconds) = self.interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", interval_in_seconds)?;
            }
            if let Some(ref size_in_m_bs) = self.size_in_m_bs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", size_in_m_bs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonopensearchserviceBufferingHints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonopensearchserviceBufferingHints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonopensearchserviceBufferingHints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonopensearchserviceBufferingHints")
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

                    Ok(AmazonopensearchserviceBufferingHints {
                        interval_in_seconds: interval_in_seconds,
                        size_in_m_bs: size_in_m_bs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonopensearchserviceDestinationConfiguration {
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: Option<::Value<AmazonopensearchserviceBufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`ClusterEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-clusterendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_endpoint: Option<::Value<String>>,
        /// Property [`DocumentIdOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-documentidoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_id_options: Option<::Value<DocumentIdOptions>>,
        /// Property [`DomainARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-domainarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_arn: Option<::Value<String>>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`IndexRotationPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-indexrotationperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_rotation_period: Option<::Value<String>>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<AmazonopensearchserviceRetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-typename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_name: Option<::Value<String>>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration-vpcconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_configuration: Option<::Value<VpcConfiguration>>,
    }

    impl ::codec::SerializeValue for AmazonopensearchserviceDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref cluster_endpoint) = self.cluster_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterEndpoint", cluster_endpoint)?;
            }
            if let Some(ref document_id_options) = self.document_id_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentIdOptions", document_id_options)?;
            }
            if let Some(ref domain_arn) = self.domain_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainARN", domain_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            if let Some(ref index_rotation_period) = self.index_rotation_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexRotationPeriod", index_rotation_period)?;
            }
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            if let Some(ref type_name) = self.type_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
            }
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonopensearchserviceDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonopensearchserviceDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonopensearchserviceDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonopensearchserviceDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffering_hints: Option<::Value<AmazonopensearchserviceBufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut cluster_endpoint: Option<::Value<String>> = None;
                    let mut document_id_options: Option<::Value<DocumentIdOptions>> = None;
                    let mut domain_arn: Option<::Value<String>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut index_rotation_period: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<AmazonopensearchserviceRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut type_name: Option<::Value<String>> = None;
                    let mut vpc_configuration: Option<::Value<VpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterEndpoint" => {
                                cluster_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentIdOptions" => {
                                document_id_options = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmazonopensearchserviceDestinationConfiguration {
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        cluster_endpoint: cluster_endpoint,
                        document_id_options: document_id_options,
                        domain_arn: domain_arn,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        index_rotation_period: index_rotation_period,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        type_name: type_name,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserviceretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonopensearchserviceRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserviceretryoptions.html#cfn-kinesisfirehose-deliverystream-amazonopensearchserviceretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AmazonopensearchserviceRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonopensearchserviceRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonopensearchserviceRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonopensearchserviceRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonopensearchserviceRetryOptions")
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

                    Ok(AmazonopensearchserviceRetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.AuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-authenticationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticationConfiguration {
        /// Property [`Connectivity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-authenticationconfiguration.html#cfn-kinesisfirehose-deliverystream-authenticationconfiguration-connectivity).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub connectivity: ::Value<String>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-authenticationconfiguration.html#cfn-kinesisfirehose-deliverystream-authenticationconfiguration-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Connectivity", &self.connectivity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connectivity: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Connectivity" => {
                                connectivity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticationConfiguration {
                        connectivity: connectivity.ok_or(::serde::de::Error::missing_field("Connectivity"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html) property type.
    #[derive(Debug, Default)]
    pub struct BufferingHints {
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html#cfn-kinesisfirehose-deliverystream-bufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: Option<::Value<u32>>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html#cfn-kinesisfirehose-deliverystream-bufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interval_in_seconds) = self.interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", interval_in_seconds)?;
            }
            if let Some(ref size_in_m_bs) = self.size_in_m_bs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", size_in_m_bs)?;
            }
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
                        interval_in_seconds: interval_in_seconds,
                        size_in_m_bs: size_in_m_bs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html) property type.
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
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

    /// The [`AWS::KinesisFirehose::DeliveryStream.DataFormatConversionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataFormatConversionConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html#cfn-kinesisfirehose-deliverystream-dataformatconversionconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`InputFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html#cfn-kinesisfirehose-deliverystream-dataformatconversionconfiguration-inputformatconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_format_configuration: Option<::Value<InputFormatConfiguration>>,
        /// Property [`OutputFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html#cfn-kinesisfirehose-deliverystream-dataformatconversionconfiguration-outputformatconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_format_configuration: Option<::Value<OutputFormatConfiguration>>,
        /// Property [`SchemaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html#cfn-kinesisfirehose-deliverystream-dataformatconversionconfiguration-schemaconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_configuration: Option<::Value<SchemaConfiguration>>,
    }

    impl ::codec::SerializeValue for DataFormatConversionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref input_format_configuration) = self.input_format_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputFormatConfiguration", input_format_configuration)?;
            }
            if let Some(ref output_format_configuration) = self.output_format_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFormatConfiguration", output_format_configuration)?;
            }
            if let Some(ref schema_configuration) = self.schema_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaConfiguration", schema_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataFormatConversionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataFormatConversionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataFormatConversionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataFormatConversionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut input_format_configuration: Option<::Value<InputFormatConfiguration>> = None;
                    let mut output_format_configuration: Option<::Value<OutputFormatConfiguration>> = None;
                    let mut schema_configuration: Option<::Value<SchemaConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputFormatConfiguration" => {
                                input_format_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputFormatConfiguration" => {
                                output_format_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaConfiguration" => {
                                schema_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataFormatConversionConfiguration {
                        enabled: enabled,
                        input_format_configuration: input_format_configuration,
                        output_format_configuration: output_format_configuration,
                        schema_configuration: schema_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.DeliveryStreamEncryptionConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct DeliveryStreamEncryptionConfigurationInput {
        /// Property [`KeyARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput.html#cfn-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput-keyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_arn: Option<::Value<String>>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput.html#cfn-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for DeliveryStreamEncryptionConfigurationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_arn) = self.key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyARN", key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", &self.key_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeliveryStreamEncryptionConfigurationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeliveryStreamEncryptionConfigurationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeliveryStreamEncryptionConfigurationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeliveryStreamEncryptionConfigurationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_arn: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyARN" => {
                                key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeliveryStreamEncryptionConfigurationInput {
                        key_arn: key_arn,
                        key_type: key_type.ok_or(::serde::de::Error::missing_field("KeyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.Deserializer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deserializer.html) property type.
    #[derive(Debug, Default)]
    pub struct Deserializer {
        /// Property [`HiveJsonSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deserializer.html#cfn-kinesisfirehose-deliverystream-deserializer-hivejsonserde).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hive_json_ser_de: Option<::Value<HiveJsonSerDe>>,
        /// Property [`OpenXJsonSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deserializer.html#cfn-kinesisfirehose-deliverystream-deserializer-openxjsonserde).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub open_x_json_ser_de: Option<::Value<OpenXJsonSerDe>>,
    }

    impl ::codec::SerializeValue for Deserializer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hive_json_ser_de) = self.hive_json_ser_de {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HiveJsonSerDe", hive_json_ser_de)?;
            }
            if let Some(ref open_x_json_ser_de) = self.open_x_json_ser_de {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenXJsonSerDe", open_x_json_ser_de)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Deserializer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Deserializer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Deserializer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Deserializer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hive_json_ser_de: Option<::Value<HiveJsonSerDe>> = None;
                    let mut open_x_json_ser_de: Option<::Value<OpenXJsonSerDe>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HiveJsonSerDe" => {
                                hive_json_ser_de = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OpenXJsonSerDe" => {
                                open_x_json_ser_de = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Deserializer {
                        hive_json_ser_de: hive_json_ser_de,
                        open_x_json_ser_de: open_x_json_ser_de,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.DocumentIdOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-documentidoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentIdOptions {
        /// Property [`DefaultDocumentIdFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-documentidoptions.html#cfn-kinesisfirehose-deliverystream-documentidoptions-defaultdocumentidformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_document_id_format: ::Value<String>,
    }

    impl ::codec::SerializeValue for DocumentIdOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultDocumentIdFormat", &self.default_document_id_format)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentIdOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentIdOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentIdOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentIdOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_document_id_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultDocumentIdFormat" => {
                                default_document_id_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentIdOptions {
                        default_document_id_format: default_document_id_format.ok_or(::serde::de::Error::missing_field("DefaultDocumentIdFormat"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.DynamicPartitioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamicPartitioningConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration.html#cfn-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration.html#cfn-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<RetryOptions>>,
    }

    impl ::codec::SerializeValue for DynamicPartitioningConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamicPartitioningConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamicPartitioningConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamicPartitioningConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamicPartitioningConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut retry_options: Option<::Value<RetryOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryOptions" => {
                                retry_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamicPartitioningConfiguration {
                        enabled: enabled,
                        retry_options: retry_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchBufferingHints {
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html#cfn-kinesisfirehose-deliverystream-elasticsearchbufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: Option<::Value<u32>>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html#cfn-kinesisfirehose-deliverystream-elasticsearchbufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ElasticsearchBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interval_in_seconds) = self.interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", interval_in_seconds)?;
            }
            if let Some(ref size_in_m_bs) = self.size_in_m_bs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", size_in_m_bs)?;
            }
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
                        interval_in_seconds: interval_in_seconds,
                        size_in_m_bs: size_in_m_bs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchDestinationConfiguration {
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: Option<::Value<ElasticsearchBufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`ClusterEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-clusterendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_endpoint: Option<::Value<String>>,
        /// Property [`DocumentIdOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-documentidoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_id_options: Option<::Value<DocumentIdOptions>>,
        /// Property [`DomainARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-domainarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_arn: Option<::Value<String>>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`IndexRotationPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-indexrotationperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_rotation_period: Option<::Value<String>>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<ElasticsearchRetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-typename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_name: Option<::Value<String>>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration-vpcconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_configuration: Option<::Value<VpcConfiguration>>,
    }

    impl ::codec::SerializeValue for ElasticsearchDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref cluster_endpoint) = self.cluster_endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterEndpoint", cluster_endpoint)?;
            }
            if let Some(ref document_id_options) = self.document_id_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentIdOptions", document_id_options)?;
            }
            if let Some(ref domain_arn) = self.domain_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainARN", domain_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            if let Some(ref index_rotation_period) = self.index_rotation_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexRotationPeriod", index_rotation_period)?;
            }
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            if let Some(ref type_name) = self.type_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", type_name)?;
            }
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
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
                    let mut cluster_endpoint: Option<::Value<String>> = None;
                    let mut document_id_options: Option<::Value<DocumentIdOptions>> = None;
                    let mut domain_arn: Option<::Value<String>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut index_rotation_period: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<ElasticsearchRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut type_name: Option<::Value<String>> = None;
                    let mut vpc_configuration: Option<::Value<VpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterEndpoint" => {
                                cluster_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentIdOptions" => {
                                document_id_options = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchDestinationConfiguration {
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        cluster_endpoint: cluster_endpoint,
                        document_id_options: document_id_options,
                        domain_arn: domain_arn,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        index_rotation_period: index_rotation_period,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        type_name: type_name,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html#cfn-kinesisfirehose-deliverystream-elasticsearchretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ElasticsearchRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
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
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
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
        pub buffering_hints: Option<::Value<BufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`CompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-compressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_format: Option<::Value<String>>,
        /// Property [`DataFormatConversionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-dataformatconversionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_format_conversion_configuration: Option<::Value<DataFormatConversionConfiguration>>,
        /// Property [`DynamicPartitioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-dynamicpartitioningconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamic_partitioning_configuration: Option<::Value<DynamicPartitioningConfiguration>>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`ErrorOutputPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-erroroutputprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_output_prefix: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-extendeds3destinationconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
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
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref compression_format) = self.compression_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", compression_format)?;
            }
            if let Some(ref data_format_conversion_configuration) = self.data_format_conversion_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFormatConversionConfiguration", data_format_conversion_configuration)?;
            }
            if let Some(ref dynamic_partitioning_configuration) = self.dynamic_partitioning_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamicPartitioningConfiguration", dynamic_partitioning_configuration)?;
            }
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            if let Some(ref error_output_prefix) = self.error_output_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorOutputPrefix", error_output_prefix)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
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
                    let mut data_format_conversion_configuration: Option<::Value<DataFormatConversionConfiguration>> = None;
                    let mut dynamic_partitioning_configuration: Option<::Value<DynamicPartitioningConfiguration>> = None;
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut error_output_prefix: Option<::Value<String>> = None;
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
                            "DataFormatConversionConfiguration" => {
                                data_format_conversion_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamicPartitioningConfiguration" => {
                                dynamic_partitioning_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorOutputPrefix" => {
                                error_output_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        compression_format: compression_format,
                        data_format_conversion_configuration: data_format_conversion_configuration,
                        dynamic_partitioning_configuration: dynamic_partitioning_configuration,
                        encryption_configuration: encryption_configuration,
                        error_output_prefix: error_output_prefix,
                        prefix: prefix,
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

    /// The [`AWS::KinesisFirehose::DeliveryStream.HiveJsonSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-hivejsonserde.html) property type.
    #[derive(Debug, Default)]
    pub struct HiveJsonSerDe {
        /// Property [`TimestampFormats`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-hivejsonserde.html#cfn-kinesisfirehose-deliverystream-hivejsonserde-timestampformats).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_formats: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HiveJsonSerDe {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref timestamp_formats) = self.timestamp_formats {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampFormats", timestamp_formats)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HiveJsonSerDe {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HiveJsonSerDe, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HiveJsonSerDe;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HiveJsonSerDe")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut timestamp_formats: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TimestampFormats" => {
                                timestamp_formats = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HiveJsonSerDe {
                        timestamp_formats: timestamp_formats,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.HttpEndpointCommonAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointcommonattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpEndpointCommonAttribute {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointcommonattribute.html#cfn-kinesisfirehose-deliverystream-httpendpointcommonattribute-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`AttributeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointcommonattribute.html#cfn-kinesisfirehose-deliverystream-httpendpointcommonattribute-attributevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpEndpointCommonAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeValue", &self.attribute_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpEndpointCommonAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpEndpointCommonAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpEndpointCommonAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpEndpointCommonAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut attribute_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeValue" => {
                                attribute_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpEndpointCommonAttribute {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        attribute_value: attribute_value.ok_or(::serde::de::Error::missing_field("AttributeValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.HttpEndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpEndpointConfiguration {
        /// Property [`AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointconfiguration-accesskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_key: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointconfiguration-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpEndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_key) = self.access_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessKey", access_key)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpEndpointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpEndpointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpEndpointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpEndpointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_key: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessKey" => {
                                access_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpEndpointConfiguration {
                        access_key: access_key,
                        name: name,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.HttpEndpointDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpEndpointDestinationConfiguration {
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: Option<::Value<BufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-endpointconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_configuration: ::Value<HttpEndpointConfiguration>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RequestConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-requestconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_configuration: Option<::Value<HttpEndpointRequestConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<RetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
    }

    impl ::codec::SerializeValue for HttpEndpointDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfiguration", &self.endpoint_configuration)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref request_configuration) = self.request_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestConfiguration", request_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", role_arn)?;
            }
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpEndpointDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpEndpointDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpEndpointDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpEndpointDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffering_hints: Option<::Value<BufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut endpoint_configuration: Option<::Value<HttpEndpointConfiguration>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut request_configuration: Option<::Value<HttpEndpointRequestConfiguration>> = None;
                    let mut retry_options: Option<::Value<RetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointConfiguration" => {
                                endpoint_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestConfiguration" => {
                                request_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            _ => {}
                        }
                    }

                    Ok(HttpEndpointDestinationConfiguration {
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        endpoint_configuration: endpoint_configuration.ok_or(::serde::de::Error::missing_field("EndpointConfiguration"))?,
                        processing_configuration: processing_configuration,
                        request_configuration: request_configuration,
                        retry_options: retry_options,
                        role_arn: role_arn,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.HttpEndpointRequestConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointrequestconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpEndpointRequestConfiguration {
        /// Property [`CommonAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointrequestconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointrequestconfiguration-commonattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub common_attributes: Option<::ValueList<HttpEndpointCommonAttribute>>,
        /// Property [`ContentEncoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointrequestconfiguration.html#cfn-kinesisfirehose-deliverystream-httpendpointrequestconfiguration-contentencoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_encoding: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpEndpointRequestConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref common_attributes) = self.common_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommonAttributes", common_attributes)?;
            }
            if let Some(ref content_encoding) = self.content_encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentEncoding", content_encoding)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpEndpointRequestConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpEndpointRequestConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpEndpointRequestConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpEndpointRequestConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut common_attributes: Option<::ValueList<HttpEndpointCommonAttribute>> = None;
                    let mut content_encoding: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CommonAttributes" => {
                                common_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentEncoding" => {
                                content_encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpEndpointRequestConfiguration {
                        common_attributes: common_attributes,
                        content_encoding: content_encoding,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.InputFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-inputformatconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InputFormatConfiguration {
        /// Property [`Deserializer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-inputformatconfiguration.html#cfn-kinesisfirehose-deliverystream-inputformatconfiguration-deserializer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deserializer: Option<::Value<Deserializer>>,
    }

    impl ::codec::SerializeValue for InputFormatConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref deserializer) = self.deserializer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Deserializer", deserializer)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputFormatConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputFormatConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputFormatConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputFormatConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut deserializer: Option<::Value<Deserializer>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Deserializer" => {
                                deserializer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputFormatConfiguration {
                        deserializer: deserializer,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.KMSEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kmsencryptionconfig.html) property type.
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
    pub struct KinesisStreamSourceConfiguration {
        /// Property [`KinesisStreamARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration-kinesisstreamarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kinesis_stream_arn: ::Value<String>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html#cfn-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
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

    /// The [`AWS::KinesisFirehose::DeliveryStream.MSKSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-msksourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MSKSourceConfiguration {
        /// Property [`AuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-msksourceconfiguration.html#cfn-kinesisfirehose-deliverystream-msksourceconfiguration-authenticationconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub authentication_configuration: ::Value<AuthenticationConfiguration>,
        /// Property [`MSKClusterARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-msksourceconfiguration.html#cfn-kinesisfirehose-deliverystream-msksourceconfiguration-mskclusterarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub msk_cluster_arn: ::Value<String>,
        /// Property [`TopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-msksourceconfiguration.html#cfn-kinesisfirehose-deliverystream-msksourceconfiguration-topicname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub topic_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for MSKSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationConfiguration", &self.authentication_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MSKClusterARN", &self.msk_cluster_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicName", &self.topic_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MSKSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MSKSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MSKSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MSKSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_configuration: Option<::Value<AuthenticationConfiguration>> = None;
                    let mut msk_cluster_arn: Option<::Value<String>> = None;
                    let mut topic_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationConfiguration" => {
                                authentication_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MSKClusterARN" => {
                                msk_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicName" => {
                                topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MSKSourceConfiguration {
                        authentication_configuration: authentication_configuration.ok_or(::serde::de::Error::missing_field("AuthenticationConfiguration"))?,
                        msk_cluster_arn: msk_cluster_arn.ok_or(::serde::de::Error::missing_field("MSKClusterARN"))?,
                        topic_name: topic_name.ok_or(::serde::de::Error::missing_field("TopicName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.OpenXJsonSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-openxjsonserde.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenXJsonSerDe {
        /// Property [`CaseInsensitive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-openxjsonserde.html#cfn-kinesisfirehose-deliverystream-openxjsonserde-caseinsensitive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub case_insensitive: Option<::Value<bool>>,
        /// Property [`ColumnToJsonKeyMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-openxjsonserde.html#cfn-kinesisfirehose-deliverystream-openxjsonserde-columntojsonkeymappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_to_json_key_mappings: Option<::ValueMap<String>>,
        /// Property [`ConvertDotsInJsonKeysToUnderscores`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-openxjsonserde.html#cfn-kinesisfirehose-deliverystream-openxjsonserde-convertdotsinjsonkeystounderscores).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub convert_dots_in_json_keys_to_underscores: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for OpenXJsonSerDe {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref case_insensitive) = self.case_insensitive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaseInsensitive", case_insensitive)?;
            }
            if let Some(ref column_to_json_key_mappings) = self.column_to_json_key_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnToJsonKeyMappings", column_to_json_key_mappings)?;
            }
            if let Some(ref convert_dots_in_json_keys_to_underscores) = self.convert_dots_in_json_keys_to_underscores {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConvertDotsInJsonKeysToUnderscores", convert_dots_in_json_keys_to_underscores)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenXJsonSerDe {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenXJsonSerDe, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenXJsonSerDe;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenXJsonSerDe")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut case_insensitive: Option<::Value<bool>> = None;
                    let mut column_to_json_key_mappings: Option<::ValueMap<String>> = None;
                    let mut convert_dots_in_json_keys_to_underscores: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaseInsensitive" => {
                                case_insensitive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnToJsonKeyMappings" => {
                                column_to_json_key_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConvertDotsInJsonKeysToUnderscores" => {
                                convert_dots_in_json_keys_to_underscores = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenXJsonSerDe {
                        case_insensitive: case_insensitive,
                        column_to_json_key_mappings: column_to_json_key_mappings,
                        convert_dots_in_json_keys_to_underscores: convert_dots_in_json_keys_to_underscores,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.OrcSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html) property type.
    #[derive(Debug, Default)]
    pub struct OrcSerDe {
        /// Property [`BlockSizeBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-blocksizebytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_size_bytes: Option<::Value<u32>>,
        /// Property [`BloomFilterColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-bloomfiltercolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bloom_filter_columns: Option<::ValueList<String>>,
        /// Property [`BloomFilterFalsePositiveProbability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-bloomfilterfalsepositiveprobability).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bloom_filter_false_positive_probability: Option<::Value<f64>>,
        /// Property [`Compression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-compression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression: Option<::Value<String>>,
        /// Property [`DictionaryKeyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-dictionarykeythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dictionary_key_threshold: Option<::Value<f64>>,
        /// Property [`EnablePadding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-enablepadding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_padding: Option<::Value<bool>>,
        /// Property [`FormatVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-formatversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format_version: Option<::Value<String>>,
        /// Property [`PaddingTolerance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-paddingtolerance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub padding_tolerance: Option<::Value<f64>>,
        /// Property [`RowIndexStride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-rowindexstride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub row_index_stride: Option<::Value<u32>>,
        /// Property [`StripeSizeBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html#cfn-kinesisfirehose-deliverystream-orcserde-stripesizebytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stripe_size_bytes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for OrcSerDe {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_size_bytes) = self.block_size_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockSizeBytes", block_size_bytes)?;
            }
            if let Some(ref bloom_filter_columns) = self.bloom_filter_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BloomFilterColumns", bloom_filter_columns)?;
            }
            if let Some(ref bloom_filter_false_positive_probability) = self.bloom_filter_false_positive_probability {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BloomFilterFalsePositiveProbability", bloom_filter_false_positive_probability)?;
            }
            if let Some(ref compression) = self.compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compression", compression)?;
            }
            if let Some(ref dictionary_key_threshold) = self.dictionary_key_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DictionaryKeyThreshold", dictionary_key_threshold)?;
            }
            if let Some(ref enable_padding) = self.enable_padding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePadding", enable_padding)?;
            }
            if let Some(ref format_version) = self.format_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormatVersion", format_version)?;
            }
            if let Some(ref padding_tolerance) = self.padding_tolerance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PaddingTolerance", padding_tolerance)?;
            }
            if let Some(ref row_index_stride) = self.row_index_stride {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowIndexStride", row_index_stride)?;
            }
            if let Some(ref stripe_size_bytes) = self.stripe_size_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StripeSizeBytes", stripe_size_bytes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrcSerDe {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrcSerDe, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrcSerDe;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrcSerDe")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_size_bytes: Option<::Value<u32>> = None;
                    let mut bloom_filter_columns: Option<::ValueList<String>> = None;
                    let mut bloom_filter_false_positive_probability: Option<::Value<f64>> = None;
                    let mut compression: Option<::Value<String>> = None;
                    let mut dictionary_key_threshold: Option<::Value<f64>> = None;
                    let mut enable_padding: Option<::Value<bool>> = None;
                    let mut format_version: Option<::Value<String>> = None;
                    let mut padding_tolerance: Option<::Value<f64>> = None;
                    let mut row_index_stride: Option<::Value<u32>> = None;
                    let mut stripe_size_bytes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockSizeBytes" => {
                                block_size_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BloomFilterColumns" => {
                                bloom_filter_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BloomFilterFalsePositiveProbability" => {
                                bloom_filter_false_positive_probability = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compression" => {
                                compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DictionaryKeyThreshold" => {
                                dictionary_key_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnablePadding" => {
                                enable_padding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FormatVersion" => {
                                format_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PaddingTolerance" => {
                                padding_tolerance = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RowIndexStride" => {
                                row_index_stride = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StripeSizeBytes" => {
                                stripe_size_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrcSerDe {
                        block_size_bytes: block_size_bytes,
                        bloom_filter_columns: bloom_filter_columns,
                        bloom_filter_false_positive_probability: bloom_filter_false_positive_probability,
                        compression: compression,
                        dictionary_key_threshold: dictionary_key_threshold,
                        enable_padding: enable_padding,
                        format_version: format_version,
                        padding_tolerance: padding_tolerance,
                        row_index_stride: row_index_stride,
                        stripe_size_bytes: stripe_size_bytes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.OutputFormatConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-outputformatconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputFormatConfiguration {
        /// Property [`Serializer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-outputformatconfiguration.html#cfn-kinesisfirehose-deliverystream-outputformatconfiguration-serializer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serializer: Option<::Value<Serializer>>,
    }

    impl ::codec::SerializeValue for OutputFormatConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref serializer) = self.serializer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Serializer", serializer)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputFormatConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputFormatConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputFormatConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputFormatConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut serializer: Option<::Value<Serializer>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Serializer" => {
                                serializer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputFormatConfiguration {
                        serializer: serializer,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ParquetSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html) property type.
    #[derive(Debug, Default)]
    pub struct ParquetSerDe {
        /// Property [`BlockSizeBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-blocksizebytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_size_bytes: Option<::Value<u32>>,
        /// Property [`Compression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-compression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression: Option<::Value<String>>,
        /// Property [`EnableDictionaryCompression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-enabledictionarycompression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_dictionary_compression: Option<::Value<bool>>,
        /// Property [`MaxPaddingBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-maxpaddingbytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_padding_bytes: Option<::Value<u32>>,
        /// Property [`PageSizeBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-pagesizebytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub page_size_bytes: Option<::Value<u32>>,
        /// Property [`WriterVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html#cfn-kinesisfirehose-deliverystream-parquetserde-writerversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub writer_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ParquetSerDe {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_size_bytes) = self.block_size_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockSizeBytes", block_size_bytes)?;
            }
            if let Some(ref compression) = self.compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compression", compression)?;
            }
            if let Some(ref enable_dictionary_compression) = self.enable_dictionary_compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDictionaryCompression", enable_dictionary_compression)?;
            }
            if let Some(ref max_padding_bytes) = self.max_padding_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxPaddingBytes", max_padding_bytes)?;
            }
            if let Some(ref page_size_bytes) = self.page_size_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PageSizeBytes", page_size_bytes)?;
            }
            if let Some(ref writer_version) = self.writer_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriterVersion", writer_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParquetSerDe {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParquetSerDe, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParquetSerDe;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParquetSerDe")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_size_bytes: Option<::Value<u32>> = None;
                    let mut compression: Option<::Value<String>> = None;
                    let mut enable_dictionary_compression: Option<::Value<bool>> = None;
                    let mut max_padding_bytes: Option<::Value<u32>> = None;
                    let mut page_size_bytes: Option<::Value<u32>> = None;
                    let mut writer_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockSizeBytes" => {
                                block_size_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compression" => {
                                compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableDictionaryCompression" => {
                                enable_dictionary_compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxPaddingBytes" => {
                                max_padding_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PageSizeBytes" => {
                                page_size_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriterVersion" => {
                                writer_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParquetSerDe {
                        block_size_bytes: block_size_bytes,
                        compression: compression,
                        enable_dictionary_compression: enable_dictionary_compression,
                        max_padding_bytes: max_padding_bytes,
                        page_size_bytes: page_size_bytes,
                        writer_version: writer_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProcessingConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html#cfn-kinesisfirehose-deliverystream-processingconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`Processors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html#cfn-kinesisfirehose-deliverystream-processingconfiguration-processors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processors: Option<::ValueList<Processor>>,
    }

    impl ::codec::SerializeValue for ProcessingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref processors) = self.processors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Processors", processors)?;
            }
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
                        enabled: enabled,
                        processors: processors,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.Processor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html) property type.
    #[derive(Debug, Default)]
    pub struct Processor {
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html#cfn-kinesisfirehose-deliverystream-processor-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueList<ProcessorParameter>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html#cfn-kinesisfirehose-deliverystream-processor-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Processor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Processor {
                        parameters: parameters,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessorParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html) property type.
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
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
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<RedshiftRetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-s3backupconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_configuration: Option<::Value<S3DestinationConfiguration>>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-redshiftdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
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
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_configuration) = self.s3_backup_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupConfiguration", s3_backup_configuration)?;
            }
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
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
                    let mut retry_options: Option<::Value<RedshiftRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
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
                            "RetryOptions" => {
                                retry_options = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        retry_options: retry_options,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_configuration: s3_backup_configuration,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.RedshiftRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftretryoptions.html#cfn-kinesisfirehose-deliverystream-redshiftretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RedshiftRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftRetryOptions")
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

                    Ok(RedshiftRetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-retryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct RetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-retryoptions.html#cfn-kinesisfirehose-deliverystream-retryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryOptions")
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

                    Ok(RetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html) property type.
    #[derive(Debug, Default)]
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
        pub buffering_hints: Option<::Value<BufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`CompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-compressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_format: Option<::Value<String>>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`ErrorOutputPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html#cfn-kinesisfirehose-deliverystream-s3destinationconfiguration-erroroutputprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_output_prefix: Option<::Value<String>>,
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
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref compression_format) = self.compression_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", compression_format)?;
            }
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            if let Some(ref error_output_prefix) = self.error_output_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorOutputPrefix", error_output_prefix)?;
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
                    let mut error_output_prefix: Option<::Value<String>> = None;
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
                            "ErrorOutputPrefix" => {
                                error_output_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        compression_format: compression_format,
                        encryption_configuration: encryption_configuration,
                        error_output_prefix: error_output_prefix,
                        prefix: prefix,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SchemaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaConfiguration {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: Option<::Value<String>>,
        /// Property [`VersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html#cfn-kinesisfirehose-deliverystream-schemaconfiguration-versionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", role_arn)?;
            }
            if let Some(ref table_name) = self.table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
            }
            if let Some(ref version_id) = self.version_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionId", version_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut version_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VersionId" => {
                                version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaConfiguration {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        region: region,
                        role_arn: role_arn,
                        table_name: table_name,
                        version_id: version_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.Serializer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-serializer.html) property type.
    #[derive(Debug, Default)]
    pub struct Serializer {
        /// Property [`OrcSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-serializer.html#cfn-kinesisfirehose-deliverystream-serializer-orcserde).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub orc_ser_de: Option<::Value<OrcSerDe>>,
        /// Property [`ParquetSerDe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-serializer.html#cfn-kinesisfirehose-deliverystream-serializer-parquetserde).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parquet_ser_de: Option<::Value<ParquetSerDe>>,
    }

    impl ::codec::SerializeValue for Serializer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref orc_ser_de) = self.orc_ser_de {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrcSerDe", orc_ser_de)?;
            }
            if let Some(ref parquet_ser_de) = self.parquet_ser_de {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParquetSerDe", parquet_ser_de)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Serializer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Serializer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Serializer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Serializer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut orc_ser_de: Option<::Value<OrcSerDe>> = None;
                    let mut parquet_ser_de: Option<::Value<ParquetSerDe>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OrcSerDe" => {
                                orc_ser_de = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParquetSerDe" => {
                                parquet_ser_de = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Serializer {
                        orc_ser_de: orc_ser_de,
                        parquet_ser_de: parquet_ser_de,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SnowflakeDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeDestinationConfiguration {
        /// Property [`AccountUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-accounturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_url: ::Value<String>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`ContentColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-contentcolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_column_name: Option<::Value<String>>,
        /// Property [`DataLoadingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-dataloadingoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_loading_option: Option<::Value<String>>,
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`KeyPassphrase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-keypassphrase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_passphrase: Option<::Value<String>>,
        /// Property [`MetaDataColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-metadatacolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub meta_data_column_name: Option<::Value<String>>,
        /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-privatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<SnowflakeRetryOptions>>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-schema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema: ::Value<String>,
        /// Property [`SnowflakeRoleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-snowflakeroleconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake_role_configuration: Option<::Value<SnowflakeRoleConfiguration>>,
        /// Property [`SnowflakeVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-snowflakevpcconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub snowflake_vpc_configuration: Option<::Value<SnowflakeVpcConfiguration>>,
        /// Property [`Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-table).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table: ::Value<String>,
        /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakedestinationconfiguration-user).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountUrl", &self.account_url)?;
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref content_column_name) = self.content_column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentColumnName", content_column_name)?;
            }
            if let Some(ref data_loading_option) = self.data_loading_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLoadingOption", data_loading_option)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            if let Some(ref key_passphrase) = self.key_passphrase {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPassphrase", key_passphrase)?;
            }
            if let Some(ref meta_data_column_name) = self.meta_data_column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetaDataColumnName", meta_data_column_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", &self.private_key)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
            if let Some(ref snowflake_role_configuration) = self.snowflake_role_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnowflakeRoleConfiguration", snowflake_role_configuration)?;
            }
            if let Some(ref snowflake_vpc_configuration) = self.snowflake_vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnowflakeVpcConfiguration", snowflake_vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Table", &self.table)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", &self.user)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_url: Option<::Value<String>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut content_column_name: Option<::Value<String>> = None;
                    let mut data_loading_option: Option<::Value<String>> = None;
                    let mut database: Option<::Value<String>> = None;
                    let mut key_passphrase: Option<::Value<String>> = None;
                    let mut meta_data_column_name: Option<::Value<String>> = None;
                    let mut private_key: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<SnowflakeRetryOptions>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;
                    let mut schema: Option<::Value<String>> = None;
                    let mut snowflake_role_configuration: Option<::Value<SnowflakeRoleConfiguration>> = None;
                    let mut snowflake_vpc_configuration: Option<::Value<SnowflakeVpcConfiguration>> = None;
                    let mut table: Option<::Value<String>> = None;
                    let mut user: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountUrl" => {
                                account_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentColumnName" => {
                                content_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataLoadingOption" => {
                                data_loading_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyPassphrase" => {
                                key_passphrase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetaDataColumnName" => {
                                meta_data_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKey" => {
                                private_key = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "Schema" => {
                                schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnowflakeRoleConfiguration" => {
                                snowflake_role_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnowflakeVpcConfiguration" => {
                                snowflake_vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Table" => {
                                table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeDestinationConfiguration {
                        account_url: account_url.ok_or(::serde::de::Error::missing_field("AccountUrl"))?,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        content_column_name: content_column_name,
                        data_loading_option: data_loading_option,
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        key_passphrase: key_passphrase,
                        meta_data_column_name: meta_data_column_name,
                        private_key: private_key.ok_or(::serde::de::Error::missing_field("PrivateKey"))?,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                        schema: schema.ok_or(::serde::de::Error::missing_field("Schema"))?,
                        snowflake_role_configuration: snowflake_role_configuration,
                        snowflake_vpc_configuration: snowflake_vpc_configuration,
                        table: table.ok_or(::serde::de::Error::missing_field("Table"))?,
                        user: user.ok_or(::serde::de::Error::missing_field("User"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SnowflakeRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeretryoptions.html#cfn-kinesisfirehose-deliverystream-snowflakeretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SnowflakeRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeRetryOptions")
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

                    Ok(SnowflakeRetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SnowflakeRoleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeroleconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeRoleConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeroleconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakeroleconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`SnowflakeRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeroleconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakeroleconfiguration-snowflakerole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake_role: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SnowflakeRoleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref snowflake_role) = self.snowflake_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnowflakeRole", snowflake_role)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeRoleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeRoleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeRoleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeRoleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut snowflake_role: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnowflakeRole" => {
                                snowflake_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeRoleConfiguration {
                        enabled: enabled,
                        snowflake_role: snowflake_role,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SnowflakeVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakevpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeVpcConfiguration {
        /// Property [`PrivateLinkVpceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakevpcconfiguration.html#cfn-kinesisfirehose-deliverystream-snowflakevpcconfiguration-privatelinkvpceid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub private_link_vpce_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateLinkVpceId", &self.private_link_vpce_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut private_link_vpce_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrivateLinkVpceId" => {
                                private_link_vpce_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeVpcConfiguration {
                        private_link_vpce_id: private_link_vpce_id.ok_or(::serde::de::Error::missing_field("PrivateLinkVpceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SplunkBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkbufferinghints.html) property type.
    #[derive(Debug, Default)]
    pub struct SplunkBufferingHints {
        /// Property [`IntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkbufferinghints.html#cfn-kinesisfirehose-deliverystream-splunkbufferinghints-intervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_in_seconds: Option<::Value<u32>>,
        /// Property [`SizeInMBs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkbufferinghints.html#cfn-kinesisfirehose-deliverystream-splunkbufferinghints-sizeinmbs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_m_bs: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SplunkBufferingHints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interval_in_seconds) = self.interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalInSeconds", interval_in_seconds)?;
            }
            if let Some(ref size_in_m_bs) = self.size_in_m_bs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMBs", size_in_m_bs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SplunkBufferingHints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SplunkBufferingHints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SplunkBufferingHints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SplunkBufferingHints")
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

                    Ok(SplunkBufferingHints {
                        interval_in_seconds: interval_in_seconds,
                        size_in_m_bs: size_in_m_bs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SplunkDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SplunkDestinationConfiguration {
        /// Property [`BufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-bufferinghints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffering_hints: Option<::Value<SplunkBufferingHints>>,
        /// Property [`CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-cloudwatchloggingoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property [`HECAcknowledgmentTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-hecacknowledgmenttimeoutinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hec_acknowledgment_timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`HECEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-hecendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hec_endpoint: ::Value<String>,
        /// Property [`HECEndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-hecendpointtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hec_endpoint_type: ::Value<String>,
        /// Property [`HECToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-hectoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hec_token: ::Value<String>,
        /// Property [`ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-processingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property [`RetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-retryoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_options: Option<::Value<SplunkRetryOptions>>,
        /// Property [`S3BackupMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-s3backupmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_backup_mode: Option<::Value<String>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html#cfn-kinesisfirehose-deliverystream-splunkdestinationconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
    }

    impl ::codec::SerializeValue for SplunkDestinationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffering_hints) = self.buffering_hints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferingHints", buffering_hints)?;
            }
            if let Some(ref cloud_watch_logging_options) = self.cloud_watch_logging_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOptions", cloud_watch_logging_options)?;
            }
            if let Some(ref hec_acknowledgment_timeout_in_seconds) = self.hec_acknowledgment_timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HECAcknowledgmentTimeoutInSeconds", hec_acknowledgment_timeout_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HECEndpoint", &self.hec_endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HECEndpointType", &self.hec_endpoint_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HECToken", &self.hec_token)?;
            if let Some(ref processing_configuration) = self.processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingConfiguration", processing_configuration)?;
            }
            if let Some(ref retry_options) = self.retry_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryOptions", retry_options)?;
            }
            if let Some(ref s3_backup_mode) = self.s3_backup_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BackupMode", s3_backup_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", &self.s3_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SplunkDestinationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SplunkDestinationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SplunkDestinationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SplunkDestinationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffering_hints: Option<::Value<SplunkBufferingHints>> = None;
                    let mut cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>> = None;
                    let mut hec_acknowledgment_timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut hec_endpoint: Option<::Value<String>> = None;
                    let mut hec_endpoint_type: Option<::Value<String>> = None;
                    let mut hec_token: Option<::Value<String>> = None;
                    let mut processing_configuration: Option<::Value<ProcessingConfiguration>> = None;
                    let mut retry_options: Option<::Value<SplunkRetryOptions>> = None;
                    let mut s3_backup_mode: Option<::Value<String>> = None;
                    let mut s3_configuration: Option<::Value<S3DestinationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferingHints" => {
                                buffering_hints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLoggingOptions" => {
                                cloud_watch_logging_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HECAcknowledgmentTimeoutInSeconds" => {
                                hec_acknowledgment_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HECEndpoint" => {
                                hec_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HECEndpointType" => {
                                hec_endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HECToken" => {
                                hec_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingConfiguration" => {
                                processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryOptions" => {
                                retry_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BackupMode" => {
                                s3_backup_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Configuration" => {
                                s3_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SplunkDestinationConfiguration {
                        buffering_hints: buffering_hints,
                        cloud_watch_logging_options: cloud_watch_logging_options,
                        hec_acknowledgment_timeout_in_seconds: hec_acknowledgment_timeout_in_seconds,
                        hec_endpoint: hec_endpoint.ok_or(::serde::de::Error::missing_field("HECEndpoint"))?,
                        hec_endpoint_type: hec_endpoint_type.ok_or(::serde::de::Error::missing_field("HECEndpointType"))?,
                        hec_token: hec_token.ok_or(::serde::de::Error::missing_field("HECToken"))?,
                        processing_configuration: processing_configuration,
                        retry_options: retry_options,
                        s3_backup_mode: s3_backup_mode,
                        s3_configuration: s3_configuration.ok_or(::serde::de::Error::missing_field("S3Configuration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.SplunkRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkretryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SplunkRetryOptions {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkretryoptions.html#cfn-kinesisfirehose-deliverystream-splunkretryoptions-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SplunkRetryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SplunkRetryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SplunkRetryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SplunkRetryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SplunkRetryOptions")
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

                    Ok(SplunkRetryOptions {
                        duration_in_seconds: duration_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisFirehose::DeliveryStream.VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-vpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfiguration {
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-vpcconfiguration.html#cfn-kinesisfirehose-deliverystream-vpcconfiguration-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-vpcconfiguration.html#cfn-kinesisfirehose-deliverystream-vpcconfiguration-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-vpcconfiguration.html#cfn-kinesisfirehose-deliverystream-vpcconfiguration-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfiguration {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
