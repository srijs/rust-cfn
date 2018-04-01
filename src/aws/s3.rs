//! Types for the `S3` service.

/// The [`AWS::S3::Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html) resource type.
#[derive(Debug)]
pub struct Bucket {
    properties: BucketProperties
}

/// Properties for the `Bucket` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketProperties {
    /// Property `AccelerateConfiguration`.
    #[serde(rename = "AccelerateConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accelerate_configuration: Option<::Value<self::bucket::AccelerateConfiguration>>,
    /// Property `AccessControl`.
    #[serde(rename = "AccessControl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<::Value<String>>,
    /// Property `AnalyticsConfigurations`.
    #[serde(rename = "AnalyticsConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics_configurations: Option<::ValueList<self::bucket::AnalyticsConfiguration>>,
    /// Property `BucketEncryption`.
    #[serde(rename = "BucketEncryption")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_encryption: Option<::Value<self::bucket::BucketEncryption>>,
    /// Property `BucketName`.
    #[serde(rename = "BucketName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<::Value<String>>,
    /// Property `CorsConfiguration`.
    #[serde(rename = "CorsConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<::Value<self::bucket::CorsConfiguration>>,
    /// Property `InventoryConfigurations`.
    #[serde(rename = "InventoryConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_configurations: Option<::ValueList<self::bucket::InventoryConfiguration>>,
    /// Property `LifecycleConfiguration`.
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<::Value<self::bucket::LifecycleConfiguration>>,
    /// Property `LoggingConfiguration`.
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<::Value<self::bucket::LoggingConfiguration>>,
    /// Property `MetricsConfigurations`.
    #[serde(rename = "MetricsConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics_configurations: Option<::ValueList<self::bucket::MetricsConfiguration>>,
    /// Property `NotificationConfiguration`.
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<::Value<self::bucket::NotificationConfiguration>>,
    /// Property `ReplicationConfiguration`.
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<::Value<self::bucket::ReplicationConfiguration>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VersioningConfiguration`.
    #[serde(rename = "VersioningConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<::Value<self::bucket::VersioningConfiguration>>,
    /// Property `WebsiteConfiguration`.
    #[serde(rename = "WebsiteConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_configuration: Option<::Value<self::bucket::WebsiteConfiguration>>,
}

impl<'a> ::Resource<'a> for Bucket {
    type Properties = BucketProperties;
    const TYPE: &'static str = "AWS::S3::Bucket";
    fn properties(&self) -> &BucketProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BucketProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Bucket {}

impl From<BucketProperties> for Bucket {
    fn from(properties: BucketProperties) -> Bucket {
        Bucket { properties }
    }
}

/// The [`AWS::S3::BucketPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html) resource type.
#[derive(Debug)]
pub struct BucketPolicy {
    properties: BucketPolicyProperties
}

/// Properties for the `BucketPolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketPolicyProperties {
    /// Property `Bucket`.
    #[serde(rename = "Bucket")]
    pub bucket: ::Value<String>,
    /// Property `PolicyDocument`.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: ::Value<::json::Value>,
}

impl<'a> ::Resource<'a> for BucketPolicy {
    type Properties = BucketPolicyProperties;
    const TYPE: &'static str = "AWS::S3::BucketPolicy";
    fn properties(&self) -> &BucketPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BucketPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BucketPolicy {}

impl From<BucketPolicyProperties> for BucketPolicy {
    fn from(properties: BucketPolicyProperties) -> BucketPolicy {
        BucketPolicy { properties }
    }
}

pub mod bucket {
    //! Property types for the `Bucket` resource.

    /// The [`AWS::S3::Bucket.AbortIncompleteMultipartUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-abortincompletemultipartupload.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AbortIncompleteMultipartUpload {
        /// Property `DaysAfterInitiation`.
        #[serde(rename = "DaysAfterInitiation")]
        pub days_after_initiation: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(AbortIncompleteMultipartUpload);

    /// The [`AWS::S3::Bucket.AccelerateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accelerateconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccelerateConfiguration {
        /// Property `AccelerationStatus`.
        #[serde(rename = "AccelerationStatus")]
        pub acceleration_status: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(AccelerateConfiguration);

    /// The [`AWS::S3::Bucket.AccessControlTranslation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accesscontroltranslation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccessControlTranslation {
        /// Property `Owner`.
        #[serde(rename = "Owner")]
        pub owner: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(AccessControlTranslation);

    /// The [`AWS::S3::Bucket.AnalyticsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnalyticsConfiguration {
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
        /// Property `StorageClassAnalysis`.
        #[serde(rename = "StorageClassAnalysis")]
        pub storage_class_analysis: ::Value<StorageClassAnalysis>,
        /// Property `TagFilters`.
        #[serde(rename = "TagFilters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_filters: Option<::ValueList<TagFilter>>,
    }

    cfn_internal__inherit_codec_impls!(AnalyticsConfiguration);

    /// The [`AWS::S3::Bucket.BucketEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-bucketencryption.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BucketEncryption {
        /// Property `ServerSideEncryptionConfiguration`.
        #[serde(rename = "ServerSideEncryptionConfiguration")]
        pub server_side_encryption_configuration: ::ValueList<ServerSideEncryptionRule>,
    }

    cfn_internal__inherit_codec_impls!(BucketEncryption);

    /// The [`AWS::S3::Bucket.CorsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CorsConfiguration {
        /// Property `CorsRules`.
        #[serde(rename = "CorsRules")]
        pub cors_rules: ::ValueList<CorsRule>,
    }

    cfn_internal__inherit_codec_impls!(CorsConfiguration);

    /// The [`AWS::S3::Bucket.CorsRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CorsRule {
        /// Property `AllowedHeaders`.
        #[serde(rename = "AllowedHeaders")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allowed_headers: Option<::ValueList<String>>,
        /// Property `AllowedMethods`.
        #[serde(rename = "AllowedMethods")]
        pub allowed_methods: ::ValueList<String>,
        /// Property `AllowedOrigins`.
        #[serde(rename = "AllowedOrigins")]
        pub allowed_origins: ::ValueList<String>,
        /// Property `ExposedHeaders`.
        #[serde(rename = "ExposedHeaders")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exposed_headers: Option<::ValueList<String>>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<::Value<String>>,
        /// Property `MaxAge`.
        #[serde(rename = "MaxAge")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_age: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(CorsRule);

    /// The [`AWS::S3::Bucket.DataExport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataExport {
        /// Property `Destination`.
        #[serde(rename = "Destination")]
        pub destination: ::Value<Destination>,
        /// Property `OutputSchemaVersion`.
        #[serde(rename = "OutputSchemaVersion")]
        pub output_schema_version: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(DataExport);

    /// The [`AWS::S3::Bucket.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Destination {
        /// Property `BucketAccountId`.
        #[serde(rename = "BucketAccountId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket_account_id: Option<::Value<String>>,
        /// Property `BucketArn`.
        #[serde(rename = "BucketArn")]
        pub bucket_arn: ::Value<String>,
        /// Property `Format`.
        #[serde(rename = "Format")]
        pub format: ::Value<String>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Destination);

    /// The [`AWS::S3::Bucket.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-encryptionconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EncryptionConfiguration {
        /// Property `ReplicaKmsKeyID`.
        #[serde(rename = "ReplicaKmsKeyID")]
        pub replica_kms_key_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(EncryptionConfiguration);

    /// The [`AWS::S3::Bucket.FilterRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key-rules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FilterRule {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(FilterRule);

    /// The [`AWS::S3::Bucket.InventoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryConfiguration {
        /// Property `Destination`.
        #[serde(rename = "Destination")]
        pub destination: ::Value<Destination>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `IncludedObjectVersions`.
        #[serde(rename = "IncludedObjectVersions")]
        pub included_object_versions: ::Value<String>,
        /// Property `OptionalFields`.
        #[serde(rename = "OptionalFields")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub optional_fields: Option<::ValueList<String>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
        /// Property `ScheduleFrequency`.
        #[serde(rename = "ScheduleFrequency")]
        pub schedule_frequency: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InventoryConfiguration);

    /// The [`AWS::S3::Bucket.LambdaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaConfiguration {
        /// Property `Event`.
        #[serde(rename = "Event")]
        pub event: ::Value<String>,
        /// Property `Filter`.
        #[serde(rename = "Filter")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property `Function`.
        #[serde(rename = "Function")]
        pub function: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(LambdaConfiguration);

    /// The [`AWS::S3::Bucket.LifecycleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleConfiguration {
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<Rule>,
    }

    cfn_internal__inherit_codec_impls!(LifecycleConfiguration);

    /// The [`AWS::S3::Bucket.LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoggingConfiguration {
        /// Property `DestinationBucketName`.
        #[serde(rename = "DestinationBucketName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub destination_bucket_name: Option<::Value<String>>,
        /// Property `LogFilePrefix`.
        #[serde(rename = "LogFilePrefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log_file_prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LoggingConfiguration);

    /// The [`AWS::S3::Bucket.MetricsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricsConfiguration {
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
        /// Property `TagFilters`.
        #[serde(rename = "TagFilters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_filters: Option<::ValueList<TagFilter>>,
    }

    cfn_internal__inherit_codec_impls!(MetricsConfiguration);

    /// The [`AWS::S3::Bucket.NoncurrentVersionTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NoncurrentVersionTransition {
        /// Property `StorageClass`.
        #[serde(rename = "StorageClass")]
        pub storage_class: ::Value<String>,
        /// Property `TransitionInDays`.
        #[serde(rename = "TransitionInDays")]
        pub transition_in_days: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(NoncurrentVersionTransition);

    /// The [`AWS::S3::Bucket.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        /// Property `LambdaConfigurations`.
        #[serde(rename = "LambdaConfigurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda_configurations: Option<::ValueList<LambdaConfiguration>>,
        /// Property `QueueConfigurations`.
        #[serde(rename = "QueueConfigurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub queue_configurations: Option<::ValueList<QueueConfiguration>>,
        /// Property `TopicConfigurations`.
        #[serde(rename = "TopicConfigurations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub topic_configurations: Option<::ValueList<TopicConfiguration>>,
    }

    cfn_internal__inherit_codec_impls!(NotificationConfiguration);

    /// The [`AWS::S3::Bucket.NotificationFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationFilter {
        /// Property `S3Key`.
        #[serde(rename = "S3Key")]
        pub s3_key: ::Value<S3KeyFilter>,
    }

    cfn_internal__inherit_codec_impls!(NotificationFilter);

    /// The [`AWS::S3::Bucket.QueueConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueueConfiguration {
        /// Property `Event`.
        #[serde(rename = "Event")]
        pub event: ::Value<String>,
        /// Property `Filter`.
        #[serde(rename = "Filter")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property `Queue`.
        #[serde(rename = "Queue")]
        pub queue: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(QueueConfiguration);

    /// The [`AWS::S3::Bucket.RedirectAllRequestsTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-redirectallrequeststo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RedirectAllRequestsTo {
        /// Property `HostName`.
        #[serde(rename = "HostName")]
        pub host_name: ::Value<String>,
        /// Property `Protocol`.
        #[serde(rename = "Protocol")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(RedirectAllRequestsTo);

    /// The [`AWS::S3::Bucket.RedirectRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RedirectRule {
        /// Property `HostName`.
        #[serde(rename = "HostName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host_name: Option<::Value<String>>,
        /// Property `HttpRedirectCode`.
        #[serde(rename = "HttpRedirectCode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_redirect_code: Option<::Value<String>>,
        /// Property `Protocol`.
        #[serde(rename = "Protocol")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<::Value<String>>,
        /// Property `ReplaceKeyPrefixWith`.
        #[serde(rename = "ReplaceKeyPrefixWith")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replace_key_prefix_with: Option<::Value<String>>,
        /// Property `ReplaceKeyWith`.
        #[serde(rename = "ReplaceKeyWith")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replace_key_with: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(RedirectRule);

    /// The [`AWS::S3::Bucket.ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationConfiguration {
        /// Property `Role`.
        #[serde(rename = "Role")]
        pub role: ::Value<String>,
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<ReplicationRule>,
    }

    cfn_internal__inherit_codec_impls!(ReplicationConfiguration);

    /// The [`AWS::S3::Bucket.ReplicationDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationDestination {
        /// Property `AccessControlTranslation`.
        #[serde(rename = "AccessControlTranslation")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub access_control_translation: Option<::Value<AccessControlTranslation>>,
        /// Property `Account`.
        #[serde(rename = "Account")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub account: Option<::Value<String>>,
        /// Property `Bucket`.
        #[serde(rename = "Bucket")]
        pub bucket: ::Value<String>,
        /// Property `EncryptionConfiguration`.
        #[serde(rename = "EncryptionConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property `StorageClass`.
        #[serde(rename = "StorageClass")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub storage_class: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ReplicationDestination);

    /// The [`AWS::S3::Bucket.ReplicationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationRule {
        /// Property `Destination`.
        #[serde(rename = "Destination")]
        pub destination: ::Value<ReplicationDestination>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<::Value<String>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        pub prefix: ::Value<String>,
        /// Property `SourceSelectionCriteria`.
        #[serde(rename = "SourceSelectionCriteria")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_selection_criteria: Option<::Value<SourceSelectionCriteria>>,
        /// Property `Status`.
        #[serde(rename = "Status")]
        pub status: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ReplicationRule);

    /// The [`AWS::S3::Bucket.RoutingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoutingRule {
        /// Property `RedirectRule`.
        #[serde(rename = "RedirectRule")]
        pub redirect_rule: ::Value<RedirectRule>,
        /// Property `RoutingRuleCondition`.
        #[serde(rename = "RoutingRuleCondition")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub routing_rule_condition: Option<::Value<RoutingRuleCondition>>,
    }

    cfn_internal__inherit_codec_impls!(RoutingRule);

    /// The [`AWS::S3::Bucket.RoutingRuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-routingrulecondition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoutingRuleCondition {
        /// Property `HttpErrorCodeReturnedEquals`.
        #[serde(rename = "HttpErrorCodeReturnedEquals")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_error_code_returned_equals: Option<::Value<String>>,
        /// Property `KeyPrefixEquals`.
        #[serde(rename = "KeyPrefixEquals")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key_prefix_equals: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(RoutingRuleCondition);

    /// The [`AWS::S3::Bucket.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `AbortIncompleteMultipartUpload`.
        #[serde(rename = "AbortIncompleteMultipartUpload")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub abort_incomplete_multipart_upload: Option<::Value<AbortIncompleteMultipartUpload>>,
        /// Property `ExpirationDate`.
        #[serde(rename = "ExpirationDate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration_date: Option<::Value<String>>,
        /// Property `ExpirationInDays`.
        #[serde(rename = "ExpirationInDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration_in_days: Option<::Value<u32>>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<::Value<String>>,
        /// Property `NoncurrentVersionExpirationInDays`.
        #[serde(rename = "NoncurrentVersionExpirationInDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub noncurrent_version_expiration_in_days: Option<::Value<u32>>,
        /// Property `NoncurrentVersionTransition`.
        #[serde(rename = "NoncurrentVersionTransition")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub noncurrent_version_transition: Option<::Value<NoncurrentVersionTransition>>,
        /// Property `NoncurrentVersionTransitions`.
        #[serde(rename = "NoncurrentVersionTransitions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub noncurrent_version_transitions: Option<::ValueList<NoncurrentVersionTransition>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
        /// Property `Status`.
        #[serde(rename = "Status")]
        pub status: ::Value<String>,
        /// Property `TagFilters`.
        #[serde(rename = "TagFilters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_filters: Option<::ValueList<TagFilter>>,
        /// Property `Transition`.
        #[serde(rename = "Transition")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transition: Option<::Value<Transition>>,
        /// Property `Transitions`.
        #[serde(rename = "Transitions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transitions: Option<::ValueList<Transition>>,
    }

    cfn_internal__inherit_codec_impls!(Rule);

    /// The [`AWS::S3::Bucket.S3KeyFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3KeyFilter {
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<FilterRule>,
    }

    cfn_internal__inherit_codec_impls!(S3KeyFilter);

    /// The [`AWS::S3::Bucket.ServerSideEncryptionByDefault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerSideEncryptionByDefault {
        /// Property `KMSMasterKeyID`.
        #[serde(rename = "KMSMasterKeyID")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kms_master_key_id: Option<::Value<String>>,
        /// Property `SSEAlgorithm`.
        #[serde(rename = "SSEAlgorithm")]
        pub sse_algorithm: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ServerSideEncryptionByDefault);

    /// The [`AWS::S3::Bucket.ServerSideEncryptionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerSideEncryptionRule {
        /// Property `ServerSideEncryptionByDefault`.
        #[serde(rename = "ServerSideEncryptionByDefault")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub server_side_encryption_by_default: Option<::Value<ServerSideEncryptionByDefault>>,
    }

    cfn_internal__inherit_codec_impls!(ServerSideEncryptionRule);

    /// The [`AWS::S3::Bucket.SourceSelectionCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-sourceselectioncriteria.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceSelectionCriteria {
        /// Property `SseKmsEncryptedObjects`.
        #[serde(rename = "SseKmsEncryptedObjects")]
        pub sse_kms_encrypted_objects: ::Value<SseKmsEncryptedObjects>,
    }

    cfn_internal__inherit_codec_impls!(SourceSelectionCriteria);

    /// The [`AWS::S3::Bucket.SseKmsEncryptedObjects`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ssekmsencryptedobjects.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SseKmsEncryptedObjects {
        /// Property `Status`.
        #[serde(rename = "Status")]
        pub status: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SseKmsEncryptedObjects);

    /// The [`AWS::S3::Bucket.StorageClassAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-storageclassanalysis.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageClassAnalysis {
        /// Property `DataExport`.
        #[serde(rename = "DataExport")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data_export: Option<::Value<DataExport>>,
    }

    cfn_internal__inherit_codec_impls!(StorageClassAnalysis);

    /// The [`AWS::S3::Bucket.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagFilter {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(TagFilter);

    /// The [`AWS::S3::Bucket.TopicConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TopicConfiguration {
        /// Property `Event`.
        #[serde(rename = "Event")]
        pub event: ::Value<String>,
        /// Property `Filter`.
        #[serde(rename = "Filter")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<::Value<NotificationFilter>>,
        /// Property `Topic`.
        #[serde(rename = "Topic")]
        pub topic: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(TopicConfiguration);

    /// The [`AWS::S3::Bucket.Transition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transition {
        /// Property `StorageClass`.
        #[serde(rename = "StorageClass")]
        pub storage_class: ::Value<String>,
        /// Property `TransitionDate`.
        #[serde(rename = "TransitionDate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transition_date: Option<::Value<String>>,
        /// Property `TransitionInDays`.
        #[serde(rename = "TransitionInDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transition_in_days: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(Transition);

    /// The [`AWS::S3::Bucket.VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-versioningconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersioningConfiguration {
        /// Property `Status`.
        #[serde(rename = "Status")]
        pub status: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(VersioningConfiguration);

    /// The [`AWS::S3::Bucket.WebsiteConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WebsiteConfiguration {
        /// Property `ErrorDocument`.
        #[serde(rename = "ErrorDocument")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_document: Option<::Value<String>>,
        /// Property `IndexDocument`.
        #[serde(rename = "IndexDocument")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub index_document: Option<::Value<String>>,
        /// Property `RedirectAllRequestsTo`.
        #[serde(rename = "RedirectAllRequestsTo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub redirect_all_requests_to: Option<::Value<RedirectAllRequestsTo>>,
        /// Property `RoutingRules`.
        #[serde(rename = "RoutingRules")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub routing_rules: Option<::ValueList<RoutingRule>>,
    }

    cfn_internal__inherit_codec_impls!(WebsiteConfiguration);
}
