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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    /// Property `CertificatePem`.
    #[serde(rename="CertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// Property `CertificateWallet`.
    #[serde(rename="CertificateWallet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// Property `DynamoDbSettings`.
    #[serde(rename="DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<self::endpoint::DynamoDbSettings>,
    /// Property `EndpointIdentifier`.
    #[serde(rename="EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// Property `EndpointType`.
    #[serde(rename="EndpointType")]
    pub endpoint_type: String,
    /// Property `EngineName`.
    #[serde(rename="EngineName")]
    pub engine_name: String,
    /// Property `ExtraConnectionAttributes`.
    #[serde(rename="ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Property `MongoDbSettings`.
    #[serde(rename="MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<self::endpoint::MongoDbSettings>,
    /// Property `Password`.
    #[serde(rename="Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Property `Port`.
    #[serde(rename="Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    /// Property `S3Settings`.
    #[serde(rename="S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<self::endpoint::S3Settings>,
    /// Property `ServerName`.
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// Property `SslMode`.
    #[serde(rename="SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `Username`.
    #[serde(rename="Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Property `EventCategories`.
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// Property `SnsTopicArn`.
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: String,
    /// Property `SourceIds`.
    #[serde(rename="SourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    /// Property `SourceType`.
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// Property `SubscriptionName`.
    #[serde(rename="SubscriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<u32>,
    /// Property `AllowMajorVersionUpgrade`.
    #[serde(rename="AllowMajorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Property `MultiAZ`.
    #[serde(rename="MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// Property `PubliclyAccessible`.
    #[serde(rename="PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// Property `ReplicationInstanceClass`.
    #[serde(rename="ReplicationInstanceClass")]
    pub replication_instance_class: String,
    /// Property `ReplicationInstanceIdentifier`.
    #[serde(rename="ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// Property `ReplicationSubnetGroupIdentifier`.
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// Property `MigrationType`.
    #[serde(rename="MigrationType")]
    pub migration_type: String,
    /// Property `ReplicationInstanceArn`.
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// Property `ReplicationTaskIdentifier`.
    #[serde(rename="ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// Property `ReplicationTaskSettings`.
    #[serde(rename="ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// Property `SourceEndpointArn`.
    #[serde(rename="SourceEndpointArn")]
    pub source_endpoint_arn: String,
    /// Property `TableMappings`.
    #[serde(rename="TableMappings")]
    pub table_mappings: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_access_role_arn: Option<String>,
    }

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MongoDbSettings {
        /// Property `AuthMechanism`.
        #[serde(rename="AuthMechanism")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth_mechanism: Option<String>,
        /// Property `AuthSource`.
        #[serde(rename="AuthSource")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth_source: Option<String>,
        /// Property `AuthType`.
        #[serde(rename="AuthType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth_type: Option<String>,
        /// Property `DatabaseName`.
        #[serde(rename="DatabaseName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub database_name: Option<String>,
        /// Property `DocsToInvestigate`.
        #[serde(rename="DocsToInvestigate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub docs_to_investigate: Option<String>,
        /// Property `ExtractDocId`.
        #[serde(rename="ExtractDocId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extract_doc_id: Option<String>,
        /// Property `NestingLevel`.
        #[serde(rename="NestingLevel")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nesting_level: Option<String>,
        /// Property `Password`.
        #[serde(rename="Password")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        /// Property `Port`.
        #[serde(rename="Port")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port: Option<u32>,
        /// Property `ServerName`.
        #[serde(rename="ServerName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_name: Option<String>,
        /// Property `Username`.
        #[serde(rename="Username")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Settings {
        /// Property `BucketFolder`.
        #[serde(rename="BucketFolder")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bucket_folder: Option<String>,
        /// Property `BucketName`.
        #[serde(rename="BucketName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bucket_name: Option<String>,
        /// Property `CompressionType`.
        #[serde(rename="CompressionType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compression_type: Option<String>,
        /// Property `CsvDelimiter`.
        #[serde(rename="CsvDelimiter")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub csv_delimiter: Option<String>,
        /// Property `CsvRowDelimiter`.
        #[serde(rename="CsvRowDelimiter")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub csv_row_delimiter: Option<String>,
        /// Property `ExternalTableDefinition`.
        #[serde(rename="ExternalTableDefinition")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_table_definition: Option<String>,
        /// Property `ServiceAccessRoleArn`.
        #[serde(rename="ServiceAccessRoleArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub service_access_role_arn: Option<String>,
    }
}
