//! Types for the `DMS` service.

/// The [`AWS::DMS::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html) resource type.
#[derive(Debug)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateProperties {
    /// Property `CertificateIdentifier`.
    #[serde(rename="CertificateIdentifier")]
    pub certificate_identifier: String,
    /// Property `CertificatePem`.
    #[serde(rename="CertificatePem")]
    pub certificate_pem: String,
    /// Property `CertificateWallet`.
    #[serde(rename="CertificateWallet")]
    pub certificate_wallet: String,
}

impl<'a> ::Resource<'a> for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::DMS::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::DMS::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html) resource type.
#[derive(Debug)]
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointProperties {
    /// Property `CertificateArn`.
    #[serde(rename="CertificateArn")]
    pub certificate_arn: String,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `DynamoDbSettings`.
    #[serde(rename="DynamoDbSettings")]
    pub dynamo_db_settings: self::endpoint::DynamoDbSettings,
    /// Property `EndpointIdentifier`.
    #[serde(rename="EndpointIdentifier")]
    pub endpoint_identifier: String,
    /// Property `EndpointType`.
    #[serde(rename="EndpointType")]
    pub endpoint_type: String,
    /// Property `EngineName`.
    #[serde(rename="EngineName")]
    pub engine_name: String,
    /// Property `ExtraConnectionAttributes`.
    #[serde(rename="ExtraConnectionAttributes")]
    pub extra_connection_attributes: String,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    /// Property `MongoDbSettings`.
    #[serde(rename="MongoDbSettings")]
    pub mongo_db_settings: self::endpoint::MongoDbSettings,
    /// Property `Password`.
    #[serde(rename="Password")]
    pub password: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `S3Settings`.
    #[serde(rename="S3Settings")]
    pub s3_settings: self::endpoint::S3Settings,
    /// Property `ServerName`.
    #[serde(rename="ServerName")]
    pub server_name: String,
    /// Property `SslMode`.
    #[serde(rename="SslMode")]
    pub ssl_mode: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `Username`.
    #[serde(rename="Username")]
    pub username: String,
}

impl<'a> ::Resource<'a> for Endpoint {
    type Properties = EndpointProperties;
    const TYPE: &'static str = "AWS::DMS::Endpoint";
    fn properties(&self) -> &EndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Endpoint {}

impl From<EndpointProperties> for Endpoint {
    fn from(properties: EndpointProperties) -> Endpoint {
        Endpoint { properties }
    }
}

/// The [`AWS::DMS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html) resource type.
#[derive(Debug)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    /// Property `Enabled`.
    #[serde(rename="Enabled")]
    pub enabled: bool,
    /// Property `EventCategories`.
    #[serde(rename="EventCategories")]
    pub event_categories: Vec<String>,
    /// Property `SnsTopicArn`.
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: String,
    /// Property `SourceIds`.
    #[serde(rename="SourceIds")]
    pub source_ids: Vec<String>,
    /// Property `SourceType`.
    #[serde(rename="SourceType")]
    pub source_type: String,
    /// Property `SubscriptionName`.
    #[serde(rename="SubscriptionName")]
    pub subscription_name: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::DMS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::DMS::ReplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html) resource type.
#[derive(Debug)]
pub struct ReplicationInstance {
    properties: ReplicationInstanceProperties
}

/// Properties for the `ReplicationInstance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationInstanceProperties {
    /// Property `AllocatedStorage`.
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: u32,
    /// Property `AllowMajorVersionUpgrade`.
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: bool,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    /// Property `MultiAZ`.
    #[serde(rename="MultiAZ")]
    pub multi_az: bool,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `PubliclyAccessible`.
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: bool,
    /// Property `ReplicationInstanceClass`.
    #[serde(rename="ReplicationInstanceClass")]
    pub replication_instance_class: String,
    /// Property `ReplicationInstanceIdentifier`.
    #[serde(rename="ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: String,
    /// Property `ReplicationSubnetGroupIdentifier`.
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
}

impl<'a> ::Resource<'a> for ReplicationInstance {
    type Properties = ReplicationInstanceProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationInstance";
    fn properties(&self) -> &ReplicationInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationInstance {}

impl From<ReplicationInstanceProperties> for ReplicationInstance {
    fn from(properties: ReplicationInstanceProperties) -> ReplicationInstance {
        ReplicationInstance { properties }
    }
}

/// The [`AWS::DMS::ReplicationSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html) resource type.
#[derive(Debug)]
pub struct ReplicationSubnetGroup {
    properties: ReplicationSubnetGroupProperties
}

/// Properties for the `ReplicationSubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationSubnetGroupProperties {
    /// Property `ReplicationSubnetGroupDescription`.
    #[serde(rename="ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    /// Property `ReplicationSubnetGroupIdentifier`.
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
}

impl<'a> ::Resource<'a> for ReplicationSubnetGroup {
    type Properties = ReplicationSubnetGroupProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationSubnetGroup";
    fn properties(&self) -> &ReplicationSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationSubnetGroup {}

impl From<ReplicationSubnetGroupProperties> for ReplicationSubnetGroup {
    fn from(properties: ReplicationSubnetGroupProperties) -> ReplicationSubnetGroup {
        ReplicationSubnetGroup { properties }
    }
}

/// The [`AWS::DMS::ReplicationTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html) resource type.
#[derive(Debug)]
pub struct ReplicationTask {
    properties: ReplicationTaskProperties
}

/// Properties for the `ReplicationTask` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationTaskProperties {
    /// Property `CdcStartTime`.
    #[serde(rename="CdcStartTime")]
    pub cdc_start_time: f64,
    /// Property `MigrationType`.
    #[serde(rename="MigrationType")]
    pub migration_type: String,
    /// Property `ReplicationInstanceArn`.
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// Property `ReplicationTaskIdentifier`.
    #[serde(rename="ReplicationTaskIdentifier")]
    pub replication_task_identifier: String,
    /// Property `ReplicationTaskSettings`.
    #[serde(rename="ReplicationTaskSettings")]
    pub replication_task_settings: String,
    /// Property `SourceEndpointArn`.
    #[serde(rename="SourceEndpointArn")]
    pub source_endpoint_arn: String,
    /// Property `TableMappings`.
    #[serde(rename="TableMappings")]
    pub table_mappings: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `TargetEndpointArn`.
    #[serde(rename="TargetEndpointArn")]
    pub target_endpoint_arn: String,
}

impl<'a> ::Resource<'a> for ReplicationTask {
    type Properties = ReplicationTaskProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationTask";
    fn properties(&self) -> &ReplicationTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationTask {}

impl From<ReplicationTaskProperties> for ReplicationTask {
    fn from(properties: ReplicationTaskProperties) -> ReplicationTask {
        ReplicationTask { properties }
    }
}

pub mod endpoint {
    //! Property types for the `Endpoint` resource.

    /// The [`AWS::DMS::Endpoint.DynamoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DynamoDbSettings {
        /// Property `ServiceAccessRoleArn`.
        #[serde(rename="ServiceAccessRoleArn")]
        pub service_access_role_arn: String,
    }

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MongoDbSettings {
        /// Property `AuthMechanism`.
        #[serde(rename="AuthMechanism")]
        pub auth_mechanism: String,
        /// Property `AuthSource`.
        #[serde(rename="AuthSource")]
        pub auth_source: String,
        /// Property `AuthType`.
        #[serde(rename="AuthType")]
        pub auth_type: String,
        /// Property `DatabaseName`.
        #[serde(rename="DatabaseName")]
        pub database_name: String,
        /// Property `DocsToInvestigate`.
        #[serde(rename="DocsToInvestigate")]
        pub docs_to_investigate: String,
        /// Property `ExtractDocId`.
        #[serde(rename="ExtractDocId")]
        pub extract_doc_id: String,
        /// Property `NestingLevel`.
        #[serde(rename="NestingLevel")]
        pub nesting_level: String,
        /// Property `Password`.
        #[serde(rename="Password")]
        pub password: String,
        /// Property `Port`.
        #[serde(rename="Port")]
        pub port: u32,
        /// Property `ServerName`.
        #[serde(rename="ServerName")]
        pub server_name: String,
        /// Property `Username`.
        #[serde(rename="Username")]
        pub username: String,
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Settings {
        /// Property `BucketFolder`.
        #[serde(rename="BucketFolder")]
        pub bucket_folder: String,
        /// Property `BucketName`.
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        /// Property `CompressionType`.
        #[serde(rename="CompressionType")]
        pub compression_type: String,
        /// Property `CsvDelimiter`.
        #[serde(rename="CsvDelimiter")]
        pub csv_delimiter: String,
        /// Property `CsvRowDelimiter`.
        #[serde(rename="CsvRowDelimiter")]
        pub csv_row_delimiter: String,
        /// Property `ExternalTableDefinition`.
        #[serde(rename="ExternalTableDefinition")]
        pub external_table_definition: String,
        /// Property `ServiceAccessRoleArn`.
        #[serde(rename="ServiceAccessRoleArn")]
        pub service_access_role_arn: String,
    }
}
