/// The [`AWS::S3::Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Bucket {
    properties: BucketProperties
}

/// Properties for the `Bucket` resource.
#[derive(Serialize, Deserialize)]
pub struct BucketProperties {
    #[serde(rename="AccelerateConfiguration")]
    pub accelerate_configuration: self::bucket::AccelerateConfiguration,
    #[serde(rename="AccessControl")]
    pub access_control: String,
    #[serde(rename="AnalyticsConfigurations")]
    pub analytics_configurations: Vec<self::bucket::AnalyticsConfiguration>,
    #[serde(rename="BucketEncryption")]
    pub bucket_encryption: self::bucket::BucketEncryption,
    #[serde(rename="BucketName")]
    pub bucket_name: String,
    #[serde(rename="CorsConfiguration")]
    pub cors_configuration: self::bucket::CorsConfiguration,
    #[serde(rename="InventoryConfigurations")]
    pub inventory_configurations: Vec<self::bucket::InventoryConfiguration>,
    #[serde(rename="LifecycleConfiguration")]
    pub lifecycle_configuration: self::bucket::LifecycleConfiguration,
    #[serde(rename="LoggingConfiguration")]
    pub logging_configuration: self::bucket::LoggingConfiguration,
    #[serde(rename="MetricsConfigurations")]
    pub metrics_configurations: Vec<self::bucket::MetricsConfiguration>,
    #[serde(rename="NotificationConfiguration")]
    pub notification_configuration: self::bucket::NotificationConfiguration,
    #[serde(rename="ReplicationConfiguration")]
    pub replication_configuration: self::bucket::ReplicationConfiguration,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VersioningConfiguration")]
    pub versioning_configuration: self::bucket::VersioningConfiguration,
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

impl From<BucketProperties> for Bucket {
    fn from(properties: BucketProperties) -> Bucket {
        Bucket { properties }
    }
}

/// The [`AWS::S3::BucketPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-policy.html) resource.
#[derive(Serialize, Deserialize)]
pub struct BucketPolicy {
    properties: BucketPolicyProperties
}

/// Properties for the `BucketPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct BucketPolicyProperties {
    #[serde(rename="Bucket")]
    pub bucket: String,
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

impl From<BucketPolicyProperties> for BucketPolicy {
    fn from(properties: BucketPolicyProperties) -> BucketPolicy {
        BucketPolicy { properties }
    }
}

pub mod bucket {
    #[derive(Serialize, Deserialize)]
    pub struct AbortIncompleteMultipartUpload {
        #[serde(rename="DaysAfterInitiation")]
        pub days_after_initiation: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AccelerateConfiguration {
        #[serde(rename="AccelerationStatus")]
        pub acceleration_status: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AccessControlTranslation {
        #[serde(rename="Owner")]
        pub owner: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AnalyticsConfiguration {
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="StorageClassAnalysis")]
        pub storage_class_analysis: StorageClassAnalysis,
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BucketEncryption {
        #[serde(rename="ServerSideEncryptionConfiguration")]
        pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CorsConfiguration {
        #[serde(rename="CorsRules")]
        pub cors_rules: Vec<CorsRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CorsRule {
        #[serde(rename="AllowedHeaders")]
        pub allowed_headers: Vec<String>,
        #[serde(rename="AllowedMethods")]
        pub allowed_methods: Vec<String>,
        #[serde(rename="AllowedOrigins")]
        pub allowed_origins: Vec<String>,
        #[serde(rename="ExposedHeaders")]
        pub exposed_headers: Vec<String>,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="MaxAge")]
        pub max_age: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DataExport {
        #[serde(rename="Destination")]
        pub destination: Destination,
        #[serde(rename="OutputSchemaVersion")]
        pub output_schema_version: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Destination {
        #[serde(rename="BucketAccountId")]
        pub bucket_account_id: String,
        #[serde(rename="BucketArn")]
        pub bucket_arn: String,
        #[serde(rename="Format")]
        pub format: String,
        #[serde(rename="Prefix")]
        pub prefix: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EncryptionConfiguration {
        #[serde(rename="ReplicaKmsKeyID")]
        pub replica_kms_key_id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct FilterRule {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InventoryConfiguration {
        #[serde(rename="Destination")]
        pub destination: Destination,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="IncludedObjectVersions")]
        pub included_object_versions: String,
        #[serde(rename="OptionalFields")]
        pub optional_fields: Vec<String>,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="ScheduleFrequency")]
        pub schedule_frequency: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LambdaConfiguration {
        #[serde(rename="Event")]
        pub event: String,
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        #[serde(rename="Function")]
        pub function: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LifecycleConfiguration {
        #[serde(rename="Rules")]
        pub rules: Vec<Rule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LoggingConfiguration {
        #[serde(rename="DestinationBucketName")]
        pub destination_bucket_name: String,
        #[serde(rename="LogFilePrefix")]
        pub log_file_prefix: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetricsConfiguration {
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NoncurrentVersionTransition {
        #[serde(rename="StorageClass")]
        pub storage_class: String,
        #[serde(rename="TransitionInDays")]
        pub transition_in_days: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NotificationConfiguration {
        #[serde(rename="LambdaConfigurations")]
        pub lambda_configurations: Vec<LambdaConfiguration>,
        #[serde(rename="QueueConfigurations")]
        pub queue_configurations: Vec<QueueConfiguration>,
        #[serde(rename="TopicConfigurations")]
        pub topic_configurations: Vec<TopicConfiguration>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NotificationFilter {
        #[serde(rename="S3Key")]
        pub s3_key: S3KeyFilter,
    }

    #[derive(Serialize, Deserialize)]
    pub struct QueueConfiguration {
        #[serde(rename="Event")]
        pub event: String,
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        #[serde(rename="Queue")]
        pub queue: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedirectAllRequestsTo {
        #[serde(rename="HostName")]
        pub host_name: String,
        #[serde(rename="Protocol")]
        pub protocol: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedirectRule {
        #[serde(rename="HostName")]
        pub host_name: String,
        #[serde(rename="HttpRedirectCode")]
        pub http_redirect_code: String,
        #[serde(rename="Protocol")]
        pub protocol: String,
        #[serde(rename="ReplaceKeyPrefixWith")]
        pub replace_key_prefix_with: String,
        #[serde(rename="ReplaceKeyWith")]
        pub replace_key_with: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReplicationConfiguration {
        #[serde(rename="Role")]
        pub role: String,
        #[serde(rename="Rules")]
        pub rules: Vec<ReplicationRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReplicationDestination {
        #[serde(rename="AccessControlTranslation")]
        pub access_control_translation: AccessControlTranslation,
        #[serde(rename="Account")]
        pub account: String,
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="EncryptionConfiguration")]
        pub encryption_configuration: EncryptionConfiguration,
        #[serde(rename="StorageClass")]
        pub storage_class: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReplicationRule {
        #[serde(rename="Destination")]
        pub destination: ReplicationDestination,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="SourceSelectionCriteria")]
        pub source_selection_criteria: SourceSelectionCriteria,
        #[serde(rename="Status")]
        pub status: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RoutingRule {
        #[serde(rename="RedirectRule")]
        pub redirect_rule: RedirectRule,
        #[serde(rename="RoutingRuleCondition")]
        pub routing_rule_condition: RoutingRuleCondition,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RoutingRuleCondition {
        #[serde(rename="HttpErrorCodeReturnedEquals")]
        pub http_error_code_returned_equals: String,
        #[serde(rename="KeyPrefixEquals")]
        pub key_prefix_equals: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Rule {
        #[serde(rename="AbortIncompleteMultipartUpload")]
        pub abort_incomplete_multipart_upload: AbortIncompleteMultipartUpload,
        #[serde(rename="ExpirationDate")]
        pub expiration_date: String,
        #[serde(rename="ExpirationInDays")]
        pub expiration_in_days: u32,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="NoncurrentVersionExpirationInDays")]
        pub noncurrent_version_expiration_in_days: u32,
        #[serde(rename="NoncurrentVersionTransition")]
        pub noncurrent_version_transition: NoncurrentVersionTransition,
        #[serde(rename="NoncurrentVersionTransitions")]
        pub noncurrent_version_transitions: Vec<NoncurrentVersionTransition>,
        #[serde(rename="Prefix")]
        pub prefix: String,
        #[serde(rename="Status")]
        pub status: String,
        #[serde(rename="TagFilters")]
        pub tag_filters: Vec<TagFilter>,
        #[serde(rename="Transition")]
        pub transition: Transition,
        #[serde(rename="Transitions")]
        pub transitions: Vec<Transition>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct S3KeyFilter {
        #[serde(rename="Rules")]
        pub rules: Vec<FilterRule>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ServerSideEncryptionByDefault {
        #[serde(rename="KMSMasterKeyID")]
        pub kms_master_key_id: String,
        #[serde(rename="SSEAlgorithm")]
        pub sse_algorithm: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ServerSideEncryptionRule {
        #[serde(rename="ServerSideEncryptionByDefault")]
        pub server_side_encryption_by_default: ServerSideEncryptionByDefault,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SourceSelectionCriteria {
        #[serde(rename="SseKmsEncryptedObjects")]
        pub sse_kms_encrypted_objects: SseKmsEncryptedObjects,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SseKmsEncryptedObjects {
        #[serde(rename="Status")]
        pub status: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StorageClassAnalysis {
        #[serde(rename="DataExport")]
        pub data_export: DataExport,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TagFilter {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TopicConfiguration {
        #[serde(rename="Event")]
        pub event: String,
        #[serde(rename="Filter")]
        pub filter: NotificationFilter,
        #[serde(rename="Topic")]
        pub topic: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Transition {
        #[serde(rename="StorageClass")]
        pub storage_class: String,
        #[serde(rename="TransitionDate")]
        pub transition_date: String,
        #[serde(rename="TransitionInDays")]
        pub transition_in_days: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VersioningConfiguration {
        #[serde(rename="Status")]
        pub status: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct WebsiteConfiguration {
        #[serde(rename="ErrorDocument")]
        pub error_document: String,
        #[serde(rename="IndexDocument")]
        pub index_document: String,
        #[serde(rename="RedirectAllRequestsTo")]
        pub redirect_all_requests_to: RedirectAllRequestsTo,
        #[serde(rename="RoutingRules")]
        pub routing_rules: Vec<RoutingRule>,
    }

}

