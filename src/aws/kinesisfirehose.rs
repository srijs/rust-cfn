/// The [`AWS::KinesisFirehose::DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryStream {
    properties: DeliveryStreamProperties
}

/// Properties for the `DeliveryStream` resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryStreamProperties {
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[serde(rename="DeliveryStreamType")]
    pub delivery_stream_type: String,
    #[serde(rename="ElasticsearchDestinationConfiguration")]
    pub elasticsearch_destination_configuration: self::delivery_stream::ElasticsearchDestinationConfiguration,
    #[serde(rename="ExtendedS3DestinationConfiguration")]
    pub extended_s3_destination_configuration: self::delivery_stream::ExtendedS3DestinationConfiguration,
    #[serde(rename="KinesisStreamSourceConfiguration")]
    pub kinesis_stream_source_configuration: self::delivery_stream::KinesisStreamSourceConfiguration,
    #[serde(rename="RedshiftDestinationConfiguration")]
    pub redshift_destination_configuration: self::delivery_stream::RedshiftDestinationConfiguration,
    #[serde(rename="S3DestinationConfiguration")]
    pub s3_destination_configuration: self::delivery_stream::S3DestinationConfiguration,
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

impl From<DeliveryStreamProperties> for DeliveryStream {
    fn from(properties: DeliveryStreamProperties) -> DeliveryStream {
        DeliveryStream { properties }
    }
}

pub mod delivery_stream {
    #[derive(Serialize, Deserialize)]
    pub struct BufferingHints {
        #[serde(rename="IntervalInSeconds")]
        pub interval_in_seconds: u32,
        #[serde(rename="SizeInMBs")]
        pub size_in_m_bs: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CloudWatchLoggingOptions {
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="LogGroupName")]
        pub log_group_name: String,
        #[serde(rename="LogStreamName")]
        pub log_stream_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CopyCommand {
        #[serde(rename="CopyOptions")]
        pub copy_options: String,
        #[serde(rename="DataTableColumns")]
        pub data_table_columns: String,
        #[serde(rename="DataTableName")]
        pub data_table_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticsearchBufferingHints {
        #[serde(rename="IntervalInSeconds")]
        pub interval_in_seconds: u32,
        #[serde(rename="SizeInMBs")]
        pub size_in_m_bs: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticsearchDestinationConfiguration {
        #[serde(rename="BufferingHints")]
        pub buffering_hints: ElasticsearchBufferingHints,
        #[serde(rename="CloudWatchLoggingOptions")]
        pub cloud_watch_logging_options: CloudWatchLoggingOptions,
        #[serde(rename="DomainARN")]
        pub domain_arn: String,
        #[serde(rename="IndexName")]
        pub index_name: String,
        #[serde(rename="IndexRotationPeriod")]
        pub index_rotation_period: String,
        #[serde(rename="ProcessingConfiguration")]
        pub processing_configuration: ProcessingConfiguration,
        #[serde(rename="RetryOptions")]
        pub retry_options: ElasticsearchRetryOptions,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
        #[serde(rename="S3BackupMode")]
        pub s3_backup_mode: String,
        #[serde(rename="S3Configuration")]
        pub s3_configuration: S3DestinationConfiguration,
        #[serde(rename="TypeName")]
        pub type_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticsearchRetryOptions {
        #[serde(rename="DurationInSeconds")]
        pub duration_in_seconds: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EncryptionConfiguration {
        #[serde(rename="KMSEncryptionConfig")]
        pub kms_encryption_config: KMSEncryptionConfig,
        #[serde(rename="NoEncryptionConfig")]
        pub no_encryption_config: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExtendedS3DestinationConfiguration {
        #[serde(rename="BucketARN")]
        pub bucket_arn: String,
        #[serde(rename="BufferingHints")]
        pub buffering_hints: BufferingHints,
        #[serde(rename="CloudWatchLoggingOptions")]
        pub cloud_watch_logging_options: CloudWatchLoggingOptions,
        #[serde(rename="CompressionFormat")]
        pub compression_format: String,
        #[serde(rename="EncryptionConfiguration")]
        pub encryption_configuration: EncryptionConfiguration,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="ProcessingConfiguration")]
        pub processing_configuration: ProcessingConfiguration,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
        #[serde(rename="S3BackupConfiguration")]
        pub s3_backup_configuration: S3DestinationConfiguration,
        #[serde(rename="S3BackupMode")]
        pub s3_backup_mode: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KMSEncryptionConfig {
        #[serde(rename="AWSKMSKeyARN")]
        pub awskms_key_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisStreamSourceConfiguration {
        #[serde(rename="KinesisStreamARN")]
        pub kinesis_stream_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProcessingConfiguration {
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Processors")]
        pub processors: Vec<Processor>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Processor {
        #[serde(rename="Parameters")]
        pub parameters: Vec<ProcessorParameter>,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProcessorParameter {
        #[serde(rename="ParameterName")]
        pub parameter_name: String,
        #[serde(rename="ParameterValue")]
        pub parameter_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedshiftDestinationConfiguration {
        #[serde(rename="CloudWatchLoggingOptions")]
        pub cloud_watch_logging_options: CloudWatchLoggingOptions,
        #[serde(rename="ClusterJDBCURL")]
        pub cluster_jdbcurl: String,
        #[serde(rename="CopyCommand")]
        pub copy_command: CopyCommand,
        #[serde(rename="Password")]
        pub password: String,
        #[serde(rename="ProcessingConfiguration")]
        pub processing_configuration: ProcessingConfiguration,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
        #[serde(rename="S3Configuration")]
        pub s3_configuration: S3DestinationConfiguration,
        #[serde(rename="Username")]
        pub username: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct S3DestinationConfiguration {
        #[serde(rename="BucketARN")]
        pub bucket_arn: String,
        #[serde(rename="BufferingHints")]
        pub buffering_hints: BufferingHints,
        #[serde(rename="CloudWatchLoggingOptions")]
        pub cloud_watch_logging_options: CloudWatchLoggingOptions,
        #[serde(rename="CompressionFormat")]
        pub compression_format: String,
        #[serde(rename="EncryptionConfiguration")]
        pub encryption_configuration: EncryptionConfiguration,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

}

