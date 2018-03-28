/// The [`AWS::S3::Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Bucket {
    properties: BucketProperties
}

/// Properties for the `Bucket` resource.
#[derive(Serialize, Deserialize)]
pub struct BucketProperties {
    #[serde(rename="AccelerateConfiguration")]
    pub accelerate_configuration: (),
    #[serde(rename="AccessControl")]
    pub access_control: String,
    #[serde(rename="AnalyticsConfigurations")]
    pub analytics_configurations: Vec<()>,
    #[serde(rename="BucketEncryption")]
    pub bucket_encryption: (),
    #[serde(rename="BucketName")]
    pub bucket_name: String,
    #[serde(rename="CorsConfiguration")]
    pub cors_configuration: (),
    #[serde(rename="InventoryConfigurations")]
    pub inventory_configurations: Vec<()>,
    #[serde(rename="LifecycleConfiguration")]
    pub lifecycle_configuration: (),
    #[serde(rename="LoggingConfiguration")]
    pub logging_configuration: (),
    #[serde(rename="MetricsConfigurations")]
    pub metrics_configurations: Vec<()>,
    #[serde(rename="NotificationConfiguration")]
    pub notification_configuration: (),
    #[serde(rename="ReplicationConfiguration")]
    pub replication_configuration: (),
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="VersioningConfiguration")]
    pub versioning_configuration: (),
    #[serde(rename="WebsiteConfiguration")]
    pub website_configuration: (),
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
    pub policy_document: String,
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

