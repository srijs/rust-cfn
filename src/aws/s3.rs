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
    #[serde(rename="AccelerateConfiguration")]
    pub accelerate_configuration: self::bucket::AccelerateConfiguration,
    /// Property `AccessControl`.
    #[serde(rename="AccessControl")]
    pub access_control: String,
    /// Property `AnalyticsConfigurations`.
    #[serde(rename="AnalyticsConfigurations")]
    pub analytics_configurations: Vec<self::bucket::AnalyticsConfiguration>,
    /// Property `BucketEncryption`.
    #[serde(rename="BucketEncryption")]
    pub bucket_encryption: self::bucket::BucketEncryption,
    /// Property `BucketName`.
    #[serde(rename="BucketName")]
    pub bucket_name: String,
    /// Property `CorsConfiguration`.
    #[serde(rename="CorsConfiguration")]
    pub cors_configuration: self::bucket::CorsConfiguration,
    /// Property `InventoryConfigurations`.
    #[serde(rename="InventoryConfigurations")]
    pub inventory_configurations: Vec<self::bucket::InventoryConfiguration>,
    /// Property `LifecycleConfiguration`.
    #[serde(rename="LifecycleConfiguration")]
    pub lifecycle_configuration: self::bucket::LifecycleConfiguration,
    /// Property `LoggingConfiguration`.
    #[serde(rename="LoggingConfiguration")]
    pub logging_configuration: self::bucket::LoggingConfiguration,
    /// Property `MetricsConfigurations`.
    #[serde(rename="MetricsConfigurations")]
    pub metrics_configurations: Vec<self::bucket::MetricsConfiguration>,
    /// Property `NotificationConfiguration`.
    #[serde(rename="NotificationConfiguration")]
    pub notification_configuration: self::bucket::NotificationConfiguration,
    /// Property `ReplicationConfiguration`.
    #[serde(rename="ReplicationConfiguration")]
    pub replication_configuration: self::bucket::ReplicationConfiguration,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VersioningConfiguration`.
    #[serde(rename="VersioningConfiguration")]
    pub versioning_configuration: self::bucket::VersioningConfiguration,
    /// Property `WebsiteConfiguration`.
    #[serde(rename="WebsiteConfiguration")]
    pub website_configuration: self::bucket::WebsiteConfiguration,
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
    #[serde(rename="Bucket")]
    pub bucket: String,
    /// Property `PolicyDocument`.
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
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
        #[serde(rename="DaysAfterInitiation")]
        pub days_after_initiation: u32,
    }

    /// The [`AWS::S3::Bucket.AccelerateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accelerateconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccelerateConfiguration {
        /// Property `AccelerationStatus`.
        #[serde(rename="AccelerationStatus")]
        pub acceleration_status: String,
    }

    /// The [`AWS::S3::Bucket.AccessControlTranslation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accesscontroltranslation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccessControlTranslation {
        /// Property `Owner`.
        #[serde(rename="Owner")]
        pub owner: String,
    }

    /// The [`AWS::S3::Bucket.AnalyticsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnalyticsConfiguration {
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
        /// Property `StorageClassAnalysis`.
        #[serde(rename="StorageClassAnalysis")]
        pub storage_class_analysis: StorageClassAnalysis,
        /// Property `TagFilters`.
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
    }

    /// The [`AWS::S3::Bucket.BucketEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-bucketencryption.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BucketEncryption {
        /// Property `ServerSideEncryptionConfiguration`.
        #[serde(rename="ServerSideEncryptionConfiguration")]
        pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,
    }

    /// The [`AWS::S3::Bucket.CorsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CorsConfiguration {
        /// Property `CorsRules`.
        #[serde(rename="CorsRules")]
        pub cors_rules: Vec<CorsRule>,
    }

    /// The [`AWS::S3::Bucket.CorsRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-cors-corsrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CorsRule {
        /// Property `AllowedHeaders`.
        #[serde(rename="AllowedHeaders")]
        pub allowed_headers: Vec<String>,
        /// Property `AllowedMethods`.
        #[serde(rename="AllowedMethods")]
        pub allowed_methods: Vec<String>,
        /// Property `AllowedOrigins`.
        #[serde(rename="AllowedOrigins")]
        pub allowed_origins: Vec<String>,
        /// Property `ExposedHeaders`.
        #[serde(rename="ExposedHeaders")]
        pub exposed_headers: Vec<String>,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `MaxAge`.
        #[serde(rename="MaxAge")]
        pub max_age: u32,
    }

    /// The [`AWS::S3::Bucket.DataExport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataExport {
        /// Property `Destination`.
        #[serde(rename="Destination")]
        pub destination: Destination,
        /// Property `OutputSchemaVersion`.
        #[serde(rename="OutputSchemaVersion")]
        pub output_schema_version: String,
    }

    /// The [`AWS::S3::Bucket.Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Destination {
        /// Property `BucketAccountId`.
        #[serde(rename="BucketAccountId")]
        pub bucket_account_id: String,
        /// Property `BucketArn`.
        #[serde(rename="BucketArn")]
        pub bucket_arn: String,
        /// Property `Format`.
        #[serde(rename="Format")]
        pub format: String,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
    }

    /// The [`AWS::S3::Bucket.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-encryptionconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EncryptionConfiguration {
        /// Property `ReplicaKmsKeyID`.
        #[serde(rename="ReplicaKmsKeyID")]
        pub replica_kms_key_id: String,
    }

    /// The [`AWS::S3::Bucket.FilterRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key-rules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct FilterRule {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::S3::Bucket.InventoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryConfiguration {
        /// Property `Destination`.
        #[serde(rename="Destination")]
        pub destination: Destination,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `IncludedObjectVersions`.
        #[serde(rename="IncludedObjectVersions")]
        pub included_object_versions: String,
        /// Property `OptionalFields`.
        #[serde(rename="OptionalFields")]
        pub optional_fields: Vec<String>,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
        /// Property `ScheduleFrequency`.
        #[serde(rename="ScheduleFrequency")]
        pub schedule_frequency: String,
    }

    /// The [`AWS::S3::Bucket.LambdaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-lambdaconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaConfiguration {
        /// Property `Event`.
        #[serde(rename="Event")]
        pub event: String,
        /// Property `Filter`.
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        /// Property `Function`.
        #[serde(rename="Function")]
        pub function: String,
    }

    /// The [`AWS::S3::Bucket.LifecycleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleConfiguration {
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<Rule>,
    }

    /// The [`AWS::S3::Bucket.LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoggingConfiguration {
        /// Property `DestinationBucketName`.
        #[serde(rename="DestinationBucketName")]
        pub destination_bucket_name: String,
        /// Property `LogFilePrefix`.
        #[serde(rename="LogFilePrefix")]
        pub log_file_prefix: String,
    }

    /// The [`AWS::S3::Bucket.MetricsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricsConfiguration {
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
        /// Property `TagFilters`.
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
    }

    /// The [`AWS::S3::Bucket.NoncurrentVersionTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-noncurrentversiontransition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NoncurrentVersionTransition {
        /// Property `StorageClass`.
        #[serde(rename="StorageClass")]
        pub storage_class: String,
        /// Property `TransitionInDays`.
        #[serde(rename="TransitionInDays")]
        pub transition_in_days: u32,
    }

    /// The [`AWS::S3::Bucket.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        /// Property `LambdaConfigurations`.
        #[serde(rename="LambdaConfigurations")]
        pub lambda_configurations: Vec<LambdaConfiguration>,
        /// Property `QueueConfigurations`.
        #[serde(rename="QueueConfigurations")]
        pub queue_configurations: Vec<QueueConfiguration>,
        /// Property `TopicConfigurations`.
        #[serde(rename="TopicConfigurations")]
        pub topic_configurations: Vec<TopicConfiguration>,
    }

    /// The [`AWS::S3::Bucket.NotificationFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NotificationFilter {
        /// Property `S3Key`.
        #[serde(rename="S3Key")]
        pub s3_key: S3KeyFilter,
    }

    /// The [`AWS::S3::Bucket.QueueConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-queueconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueueConfiguration {
        /// Property `Event`.
        #[serde(rename="Event")]
        pub event: String,
        /// Property `Filter`.
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        /// Property `Queue`.
        #[serde(rename="Queue")]
        pub queue: String,
    }

    /// The [`AWS::S3::Bucket.RedirectAllRequestsTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-redirectallrequeststo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RedirectAllRequestsTo {
        /// Property `HostName`.
        #[serde(rename="HostName")]
        pub host_name: String,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
    }

    /// The [`AWS::S3::Bucket.RedirectRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-redirectrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RedirectRule {
        /// Property `HostName`.
        #[serde(rename="HostName")]
        pub host_name: String,
        /// Property `HttpRedirectCode`.
        #[serde(rename="HttpRedirectCode")]
        pub http_redirect_code: String,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
        /// Property `ReplaceKeyPrefixWith`.
        #[serde(rename="ReplaceKeyPrefixWith")]
        pub replace_key_prefix_with: String,
        /// Property `ReplaceKeyWith`.
        #[serde(rename="ReplaceKeyWith")]
        pub replace_key_with: String,
    }

    /// The [`AWS::S3::Bucket.ReplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationConfiguration {
        /// Property `Role`.
        #[serde(rename="Role")]
        pub role: String,
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<ReplicationRule>,
    }

    /// The [`AWS::S3::Bucket.ReplicationDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules-destination.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationDestination {
        /// Property `AccessControlTranslation`.
        #[serde(rename="AccessControlTranslation")]
        pub access_control_translation: AccessControlTranslation,
        /// Property `Account`.
        #[serde(rename="Account")]
        pub account: String,
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `EncryptionConfiguration`.
        #[serde(rename="EncryptionConfiguration")]
        pub encryption_configuration: EncryptionConfiguration,
        /// Property `StorageClass`.
        #[serde(rename="StorageClass")]
        pub storage_class: String,
    }

    /// The [`AWS::S3::Bucket.ReplicationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration-rules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReplicationRule {
        /// Property `Destination`.
        #[serde(rename="Destination")]
        pub destination: ReplicationDestination,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
        /// Property `SourceSelectionCriteria`.
        #[serde(rename="SourceSelectionCriteria")]
        pub source_selection_criteria: SourceSelectionCriteria,
        /// Property `Status`.
        #[serde(rename="Status")]
        pub status: String,
    }

    /// The [`AWS::S3::Bucket.RoutingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoutingRule {
        /// Property `RedirectRule`.
        #[serde(rename="RedirectRule")]
        pub redirect_rule: RedirectRule,
        /// Property `RoutingRuleCondition`.
        #[serde(rename="RoutingRuleCondition")]
        pub routing_rule_condition: RoutingRuleCondition,
    }

    /// The [`AWS::S3::Bucket.RoutingRuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration-routingrules-routingrulecondition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoutingRuleCondition {
        /// Property `HttpErrorCodeReturnedEquals`.
        #[serde(rename="HttpErrorCodeReturnedEquals")]
        pub http_error_code_returned_equals: String,
        /// Property `KeyPrefixEquals`.
        #[serde(rename="KeyPrefixEquals")]
        pub key_prefix_equals: String,
    }

    /// The [`AWS::S3::Bucket.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rule {
        /// Property `AbortIncompleteMultipartUpload`.
        #[serde(rename="AbortIncompleteMultipartUpload")]
        pub abort_incomplete_multipart_upload: AbortIncompleteMultipartUpload,
        /// Property `ExpirationDate`.
        #[serde(rename="ExpirationDate")]
        pub expiration_date: String,
        /// Property `ExpirationInDays`.
        #[serde(rename="ExpirationInDays")]
        pub expiration_in_days: u32,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `NoncurrentVersionExpirationInDays`.
        #[serde(rename="NoncurrentVersionExpirationInDays")]
        pub noncurrent_version_expiration_in_days: u32,
        /// Property `NoncurrentVersionTransition`.
        #[serde(rename="NoncurrentVersionTransition")]
        pub noncurrent_version_transition: NoncurrentVersionTransition,
        /// Property `NoncurrentVersionTransitions`.
        #[serde(rename="NoncurrentVersionTransitions")]
        pub noncurrent_version_transitions: Vec<NoncurrentVersionTransition>,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
        /// Property `Status`.
        #[serde(rename="Status")]
        pub status: String,
        /// Property `TagFilters`.
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
        /// Property `Transition`.
        #[serde(rename="Transition")]
        pub transition: Transition,
        /// Property `Transitions`.
        #[serde(rename="Transitions")]
        pub transitions: Vec<Transition>,
    }

    /// The [`AWS::S3::Bucket.S3KeyFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration-config-filter-s3key.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3KeyFilter {
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<FilterRule>,
    }

    /// The [`AWS::S3::Bucket.ServerSideEncryptionByDefault`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerSideEncryptionByDefault {
        /// Property `KMSMasterKeyID`.
        #[serde(rename="KMSMasterKeyID")]
        pub kms_master_key_id: String,
        /// Property `SSEAlgorithm`.
        #[serde(rename="SSEAlgorithm")]
        pub sse_algorithm: String,
    }

    /// The [`AWS::S3::Bucket.ServerSideEncryptionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ServerSideEncryptionRule {
        /// Property `ServerSideEncryptionByDefault`.
        #[serde(rename="ServerSideEncryptionByDefault")]
        pub server_side_encryption_by_default: ServerSideEncryptionByDefault,
    }

    /// The [`AWS::S3::Bucket.SourceSelectionCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-sourceselectioncriteria.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceSelectionCriteria {
        /// Property `SseKmsEncryptedObjects`.
        #[serde(rename="SseKmsEncryptedObjects")]
        pub sse_kms_encrypted_objects: SseKmsEncryptedObjects,
    }

    /// The [`AWS::S3::Bucket.SseKmsEncryptedObjects`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ssekmsencryptedobjects.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SseKmsEncryptedObjects {
        /// Property `Status`.
        #[serde(rename="Status")]
        pub status: String,
    }

    /// The [`AWS::S3::Bucket.StorageClassAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-storageclassanalysis.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageClassAnalysis {
        /// Property `DataExport`.
        #[serde(rename="DataExport")]
        pub data_export: DataExport,
    }

    /// The [`AWS::S3::Bucket.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TagFilter {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::S3::Bucket.TopicConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfig-topicconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TopicConfiguration {
        /// Property `Event`.
        #[serde(rename="Event")]
        pub event: String,
        /// Property `Filter`.
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        /// Property `Topic`.
        #[serde(rename="Topic")]
        pub topic: String,
    }

    /// The [`AWS::S3::Bucket.Transition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfig-rule-transition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Transition {
        /// Property `StorageClass`.
        #[serde(rename="StorageClass")]
        pub storage_class: String,
        /// Property `TransitionDate`.
        #[serde(rename="TransitionDate")]
        pub transition_date: String,
        /// Property `TransitionInDays`.
        #[serde(rename="TransitionInDays")]
        pub transition_in_days: u32,
    }

    /// The [`AWS::S3::Bucket.VersioningConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-versioningconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersioningConfiguration {
        /// Property `Status`.
        #[serde(rename="Status")]
        pub status: String,
    }

    /// The [`AWS::S3::Bucket.WebsiteConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-websiteconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct WebsiteConfiguration {
        /// Property `ErrorDocument`.
        #[serde(rename="ErrorDocument")]
        pub error_document: String,
        /// Property `IndexDocument`.
        #[serde(rename="IndexDocument")]
        pub index_document: String,
        /// Property `RedirectAllRequestsTo`.
        #[serde(rename="RedirectAllRequestsTo")]
        pub redirect_all_requests_to: RedirectAllRequestsTo,
        /// Property `RoutingRules`.
        #[serde(rename="RoutingRules")]
        pub routing_rules: Vec<RoutingRule>,
    }
}
