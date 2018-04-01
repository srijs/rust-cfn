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
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<::Value<String>>,
    /// Property `CertificatePem`.
    #[serde(rename = "CertificatePem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<::Value<String>>,
    /// Property `CertificateWallet`.
    #[serde(rename = "CertificateWallet")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<::Value<String>>,
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
    #[serde(rename = "CertificateArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<::Value<String>>,
    /// Property `DatabaseName`.
    #[serde(rename = "DatabaseName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<::Value<String>>,
    /// Property `DynamoDbSettings`.
    #[serde(rename = "DynamoDbSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<::Value<self::endpoint::DynamoDbSettings>>,
    /// Property `EndpointIdentifier`.
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<::Value<String>>,
    /// Property `EndpointType`.
    #[serde(rename = "EndpointType")]
    pub endpoint_type: ::Value<String>,
    /// Property `EngineName`.
    #[serde(rename = "EngineName")]
    pub engine_name: ::Value<String>,
    /// Property `ExtraConnectionAttributes`.
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MongoDbSettings`.
    #[serde(rename = "MongoDbSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<::Value<self::endpoint::MongoDbSettings>>,
    /// Property `Password`.
    #[serde(rename = "Password")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<::Value<String>>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<u32>>,
    /// Property `S3Settings`.
    #[serde(rename = "S3Settings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<::Value<self::endpoint::S3Settings>>,
    /// Property `ServerName`.
    #[serde(rename = "ServerName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<::Value<String>>,
    /// Property `SslMode`.
    #[serde(rename = "SslMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Username`.
    #[serde(rename = "Username")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<::Value<String>>,
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
    #[serde(rename = "Enabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<::Value<bool>>,
    /// Property `EventCategories`.
    #[serde(rename = "EventCategories")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<::ValueList<String>>,
    /// Property `SnsTopicArn`.
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: ::Value<String>,
    /// Property `SourceIds`.
    #[serde(rename = "SourceIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<::ValueList<String>>,
    /// Property `SourceType`.
    #[serde(rename = "SourceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<::Value<String>>,
    /// Property `SubscriptionName`.
    #[serde(rename = "SubscriptionName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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
    #[serde(rename = "AllocatedStorage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<::Value<u32>>,
    /// Property `AllowMajorVersionUpgrade`.
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<::Value<bool>>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<::Value<String>>,
    /// Property `EngineVersion`.
    #[serde(rename = "EngineVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MultiAZ`.
    #[serde(rename = "MultiAZ")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<::Value<bool>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `ReplicationInstanceClass`.
    #[serde(rename = "ReplicationInstanceClass")]
    pub replication_instance_class: ::Value<String>,
    /// Property `ReplicationInstanceIdentifier`.
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<::Value<String>>,
    /// Property `ReplicationSubnetGroupIdentifier`.
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<::ValueList<String>>,
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
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: ::Value<String>,
    /// Property `ReplicationSubnetGroupIdentifier`.
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property `SubnetIds`.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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
    #[serde(rename = "CdcStartTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<::Value<f64>>,
    /// Property `MigrationType`.
    #[serde(rename = "MigrationType")]
    pub migration_type: ::Value<String>,
    /// Property `ReplicationInstanceArn`.
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: ::Value<String>,
    /// Property `ReplicationTaskIdentifier`.
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<::Value<String>>,
    /// Property `ReplicationTaskSettings`.
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<::Value<String>>,
    /// Property `SourceEndpointArn`.
    #[serde(rename = "SourceEndpointArn")]
    pub source_endpoint_arn: ::Value<String>,
    /// Property `TableMappings`.
    #[serde(rename = "TableMappings")]
    pub table_mappings: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TargetEndpointArn`.
    #[serde(rename = "TargetEndpointArn")]
    pub target_endpoint_arn: ::Value<String>,
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
        #[serde(rename = "ServiceAccessRoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service_access_role_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DynamoDbSettings);

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MongoDbSettings {
        /// Property `AuthMechanism`.
        #[serde(rename = "AuthMechanism")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth_mechanism: Option<::Value<String>>,
        /// Property `AuthSource`.
        #[serde(rename = "AuthSource")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth_source: Option<::Value<String>>,
        /// Property `AuthType`.
        #[serde(rename = "AuthType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth_type: Option<::Value<String>>,
        /// Property `DatabaseName`.
        #[serde(rename = "DatabaseName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub database_name: Option<::Value<String>>,
        /// Property `DocsToInvestigate`.
        #[serde(rename = "DocsToInvestigate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub docs_to_investigate: Option<::Value<String>>,
        /// Property `ExtractDocId`.
        #[serde(rename = "ExtractDocId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extract_doc_id: Option<::Value<String>>,
        /// Property `NestingLevel`.
        #[serde(rename = "NestingLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nesting_level: Option<::Value<String>>,
        /// Property `Password`.
        #[serde(rename = "Password")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<::Value<String>>,
        /// Property `Port`.
        #[serde(rename = "Port")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<::Value<u32>>,
        /// Property `ServerName`.
        #[serde(rename = "ServerName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub server_name: Option<::Value<String>>,
        /// Property `Username`.
        #[serde(rename = "Username")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(MongoDbSettings);

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Settings {
        /// Property `BucketFolder`.
        #[serde(rename = "BucketFolder")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket_folder: Option<::Value<String>>,
        /// Property `BucketName`.
        #[serde(rename = "BucketName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket_name: Option<::Value<String>>,
        /// Property `CompressionType`.
        #[serde(rename = "CompressionType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compression_type: Option<::Value<String>>,
        /// Property `CsvDelimiter`.
        #[serde(rename = "CsvDelimiter")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csv_delimiter: Option<::Value<String>>,
        /// Property `CsvRowDelimiter`.
        #[serde(rename = "CsvRowDelimiter")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csv_row_delimiter: Option<::Value<String>>,
        /// Property `ExternalTableDefinition`.
        #[serde(rename = "ExternalTableDefinition")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub external_table_definition: Option<::Value<String>>,
        /// Property `ServiceAccessRoleArn`.
        #[serde(rename = "ServiceAccessRoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub service_access_role_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(S3Settings);
}
