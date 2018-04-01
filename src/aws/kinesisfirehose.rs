//! Types for the `KinesisFirehose` service.

/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource type.
#[derive(Debug)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryStreamProperties {
    /// Property `DeliveryStreamName`.
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_stream_name: Option<::Value<String>>,
    /// Property `DeliveryStreamType`.
    #[serde(rename = "DeliveryStreamType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<::Value<String>>,
    /// Property `ElasticsearchDestinationConfiguration`.
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_configuration: Option<::Value<self::delivery_stream::ElasticsearchDestinationConfiguration>>,
    /// Property `ExtendedS3DestinationConfiguration`.
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_configuration: Option<::Value<self::delivery_stream::ExtendedS3DestinationConfiguration>>,
    /// Property `KinesisStreamSourceConfiguration`.
    #[serde(rename = "KinesisStreamSourceConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_configuration: Option<::Value<self::delivery_stream::KinesisStreamSourceConfiguration>>,
    /// Property `RedshiftDestinationConfiguration`.
    #[serde(rename = "RedshiftDestinationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redshift_destination_configuration: Option<::Value<self::delivery_stream::RedshiftDestinationConfiguration>>,
    /// Property `S3DestinationConfiguration`.
    #[serde(rename = "S3DestinationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3_destination_configuration: Option<::Value<self::delivery_stream::S3DestinationConfiguration>>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BufferingHints {
        /// Property `IntervalInSeconds`.
        #[serde(rename = "IntervalInSeconds")]
        pub interval_in_seconds: ::Value<u32>,
        /// Property `SizeInMBs`.
        #[serde(rename = "SizeInMBs")]
        pub size_in_m_bs: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(BufferingHints);

    /// The [`AWS::KinesisFirehose::DeliveryStream.CloudWatchLoggingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudWatchLoggingOptions {
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<::Value<bool>>,
        /// Property `LogGroupName`.
        #[serde(rename = "LogGroupName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log_group_name: Option<::Value<String>>,
        /// Property `LogStreamName`.
        #[serde(rename = "LogStreamName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log_stream_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CloudWatchLoggingOptions);

    /// The [`AWS::KinesisFirehose::DeliveryStream.CopyCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CopyCommand {
        /// Property `CopyOptions`.
        #[serde(rename = "CopyOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub copy_options: Option<::Value<String>>,
        /// Property `DataTableColumns`.
        #[serde(rename = "DataTableColumns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data_table_columns: Option<::Value<String>>,
        /// Property `DataTableName`.
        #[serde(rename = "DataTableName")]
        pub data_table_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CopyCommand);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchBufferingHints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchBufferingHints {
        /// Property `IntervalInSeconds`.
        #[serde(rename = "IntervalInSeconds")]
        pub interval_in_seconds: ::Value<u32>,
        /// Property `SizeInMBs`.
        #[serde(rename = "SizeInMBs")]
        pub size_in_m_bs: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ElasticsearchBufferingHints);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchDestinationConfiguration {
        /// Property `BufferingHints`.
        #[serde(rename = "BufferingHints")]
        pub buffering_hints: ::Value<ElasticsearchBufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        #[serde(rename = "CloudWatchLoggingOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `DomainARN`.
        #[serde(rename = "DomainARN")]
        pub domain_arn: ::Value<String>,
        /// Property `IndexName`.
        #[serde(rename = "IndexName")]
        pub index_name: ::Value<String>,
        /// Property `IndexRotationPeriod`.
        #[serde(rename = "IndexRotationPeriod")]
        pub index_rotation_period: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        #[serde(rename = "ProcessingConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RetryOptions`.
        #[serde(rename = "RetryOptions")]
        pub retry_options: ::Value<ElasticsearchRetryOptions>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
        /// Property `S3BackupMode`.
        #[serde(rename = "S3BackupMode")]
        pub s3_backup_mode: ::Value<String>,
        /// Property `S3Configuration`.
        #[serde(rename = "S3Configuration")]
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property `TypeName`.
        #[serde(rename = "TypeName")]
        pub type_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ElasticsearchDestinationConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ElasticsearchRetryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchRetryOptions {
        /// Property `DurationInSeconds`.
        #[serde(rename = "DurationInSeconds")]
        pub duration_in_seconds: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ElasticsearchRetryOptions);

    /// The [`AWS::KinesisFirehose::DeliveryStream.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EncryptionConfiguration {
        /// Property `KMSEncryptionConfig`.
        #[serde(rename = "KMSEncryptionConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kms_encryption_config: Option<::Value<KMSEncryptionConfig>>,
        /// Property `NoEncryptionConfig`.
        #[serde(rename = "NoEncryptionConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub no_encryption_config: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(EncryptionConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ExtendedS3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtendedS3DestinationConfiguration {
        /// Property `BucketARN`.
        #[serde(rename = "BucketARN")]
        pub bucket_arn: ::Value<String>,
        /// Property `BufferingHints`.
        #[serde(rename = "BufferingHints")]
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        #[serde(rename = "CloudWatchLoggingOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `CompressionFormat`.
        #[serde(rename = "CompressionFormat")]
        pub compression_format: ::Value<String>,
        /// Property `EncryptionConfiguration`.
        #[serde(rename = "EncryptionConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        pub prefix: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        #[serde(rename = "ProcessingConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
        /// Property `S3BackupConfiguration`.
        #[serde(rename = "S3BackupConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_backup_configuration: Option<::Value<S3DestinationConfiguration>>,
        /// Property `S3BackupMode`.
        #[serde(rename = "S3BackupMode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_backup_mode: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ExtendedS3DestinationConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.KMSEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kmsencryptionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KMSEncryptionConfig {
        /// Property `AWSKMSKeyARN`.
        #[serde(rename = "AWSKMSKeyARN")]
        pub awskms_key_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KMSEncryptionConfig);

    /// The [`AWS::KinesisFirehose::DeliveryStream.KinesisStreamSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisStreamSourceConfiguration {
        /// Property `KinesisStreamARN`.
        #[serde(rename = "KinesisStreamARN")]
        pub kinesis_stream_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisStreamSourceConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProcessingConfiguration {
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `Processors`.
        #[serde(rename = "Processors")]
        pub processors: ::ValueList<Processor>,
    }

    cfn_internal__inherit_codec_impls!(ProcessingConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.Processor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Processor {
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        pub parameters: ::ValueList<ProcessorParameter>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Processor);

    /// The [`AWS::KinesisFirehose::DeliveryStream.ProcessorParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProcessorParameter {
        /// Property `ParameterName`.
        #[serde(rename = "ParameterName")]
        pub parameter_name: ::Value<String>,
        /// Property `ParameterValue`.
        #[serde(rename = "ParameterValue")]
        pub parameter_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ProcessorParameter);

    /// The [`AWS::KinesisFirehose::DeliveryStream.RedshiftDestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RedshiftDestinationConfiguration {
        /// Property `CloudWatchLoggingOptions`.
        #[serde(rename = "CloudWatchLoggingOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `ClusterJDBCURL`.
        #[serde(rename = "ClusterJDBCURL")]
        pub cluster_jdbcurl: ::Value<String>,
        /// Property `CopyCommand`.
        #[serde(rename = "CopyCommand")]
        pub copy_command: ::Value<CopyCommand>,
        /// Property `Password`.
        #[serde(rename = "Password")]
        pub password: ::Value<String>,
        /// Property `ProcessingConfiguration`.
        #[serde(rename = "ProcessingConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub processing_configuration: Option<::Value<ProcessingConfiguration>>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
        /// Property `S3Configuration`.
        #[serde(rename = "S3Configuration")]
        pub s3_configuration: ::Value<S3DestinationConfiguration>,
        /// Property `Username`.
        #[serde(rename = "Username")]
        pub username: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RedshiftDestinationConfiguration);

    /// The [`AWS::KinesisFirehose::DeliveryStream.S3DestinationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3DestinationConfiguration {
        /// Property `BucketARN`.
        #[serde(rename = "BucketARN")]
        pub bucket_arn: ::Value<String>,
        /// Property `BufferingHints`.
        #[serde(rename = "BufferingHints")]
        pub buffering_hints: ::Value<BufferingHints>,
        /// Property `CloudWatchLoggingOptions`.
        #[serde(rename = "CloudWatchLoggingOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_watch_logging_options: Option<::Value<CloudWatchLoggingOptions>>,
        /// Property `CompressionFormat`.
        #[serde(rename = "CompressionFormat")]
        pub compression_format: ::Value<String>,
        /// Property `EncryptionConfiguration`.
        #[serde(rename = "EncryptionConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(S3DestinationConfiguration);
}
