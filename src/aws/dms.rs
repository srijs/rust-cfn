/// The [`AWS::DMS::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html) resource type.
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(rename="CertificateIdentifier")]
    pub certificate_identifier: String,
    #[serde(rename="CertificatePem")]
    pub certificate_pem: String,
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

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::DMS::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html) resource type.
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(rename="CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[serde(rename="DynamoDbSettings")]
    pub dynamo_db_settings: self::endpoint::DynamoDbSettings,
    #[serde(rename="EndpointIdentifier")]
    pub endpoint_identifier: String,
    #[serde(rename="EndpointType")]
    pub endpoint_type: String,
    #[serde(rename="EngineName")]
    pub engine_name: String,
    #[serde(rename="ExtraConnectionAttributes")]
    pub extra_connection_attributes: String,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="MongoDbSettings")]
    pub mongo_db_settings: self::endpoint::MongoDbSettings,
    #[serde(rename="Password")]
    pub password: String,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="S3Settings")]
    pub s3_settings: self::endpoint::S3Settings,
    #[serde(rename="ServerName")]
    pub server_name: String,
    #[serde(rename="SslMode")]
    pub ssl_mode: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<EndpointProperties> for Endpoint {
    fn from(properties: EndpointProperties) -> Endpoint {
        Endpoint { properties }
    }
}

/// The [`AWS::DMS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html) resource type.
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    #[serde(rename="Enabled")]
    pub enabled: bool,
    #[serde(rename="EventCategories")]
    pub event_categories: Vec<String>,
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: String,
    #[serde(rename="SourceIds")]
    pub source_ids: Vec<String>,
    #[serde(rename="SourceType")]
    pub source_type: String,
    #[serde(rename="SubscriptionName")]
    pub subscription_name: String,
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

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::DMS::ReplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html) resource type.
pub struct ReplicationInstance {
    properties: ReplicationInstanceProperties
}

/// Properties for the `ReplicationInstance` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationInstanceProperties {
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: u32,
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: bool,
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="MultiAZ")]
    pub multi_az: bool,
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: bool,
    #[serde(rename="ReplicationInstanceClass")]
    pub replication_instance_class: String,
    #[serde(rename="ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: String,
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<ReplicationInstanceProperties> for ReplicationInstance {
    fn from(properties: ReplicationInstanceProperties) -> ReplicationInstance {
        ReplicationInstance { properties }
    }
}

/// The [`AWS::DMS::ReplicationSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html) resource type.
pub struct ReplicationSubnetGroup {
    properties: ReplicationSubnetGroupProperties
}

/// Properties for the `ReplicationSubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationSubnetGroupProperties {
    #[serde(rename="ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
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

impl From<ReplicationSubnetGroupProperties> for ReplicationSubnetGroup {
    fn from(properties: ReplicationSubnetGroupProperties) -> ReplicationSubnetGroup {
        ReplicationSubnetGroup { properties }
    }
}

/// The [`AWS::DMS::ReplicationTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html) resource type.
pub struct ReplicationTask {
    properties: ReplicationTaskProperties
}

/// Properties for the `ReplicationTask` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationTaskProperties {
    #[serde(rename="CdcStartTime")]
    pub cdc_start_time: f64,
    #[serde(rename="MigrationType")]
    pub migration_type: String,
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    #[serde(rename="ReplicationTaskIdentifier")]
    pub replication_task_identifier: String,
    #[serde(rename="ReplicationTaskSettings")]
    pub replication_task_settings: String,
    #[serde(rename="SourceEndpointArn")]
    pub source_endpoint_arn: String,
    #[serde(rename="TableMappings")]
    pub table_mappings: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<ReplicationTaskProperties> for ReplicationTask {
    fn from(properties: ReplicationTaskProperties) -> ReplicationTask {
        ReplicationTask { properties }
    }
}

pub mod endpoint {
    /// The [`AWS::DMS::Endpoint.DynamoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DynamoDbSettings {
        #[serde(rename="ServiceAccessRoleArn")]
        pub service_access_role_arn: String,
    }

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MongoDbSettings {
        #[serde(rename="AuthMechanism")]
        pub auth_mechanism: String,
        #[serde(rename="AuthSource")]
        pub auth_source: String,
        #[serde(rename="AuthType")]
        pub auth_type: String,
        #[serde(rename="DatabaseName")]
        pub database_name: String,
        #[serde(rename="DocsToInvestigate")]
        pub docs_to_investigate: String,
        #[serde(rename="ExtractDocId")]
        pub extract_doc_id: String,
        #[serde(rename="NestingLevel")]
        pub nesting_level: String,
        #[serde(rename="Password")]
        pub password: String,
        #[serde(rename="Port")]
        pub port: u32,
        #[serde(rename="ServerName")]
        pub server_name: String,
        #[serde(rename="Username")]
        pub username: String,
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3Settings {
        #[serde(rename="BucketFolder")]
        pub bucket_folder: String,
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        #[serde(rename="CompressionType")]
        pub compression_type: String,
        #[serde(rename="CsvDelimiter")]
        pub csv_delimiter: String,
        #[serde(rename="CsvRowDelimiter")]
        pub csv_row_delimiter: String,
        #[serde(rename="ExternalTableDefinition")]
        pub external_table_definition: String,
        #[serde(rename="ServiceAccessRoleArn")]
        pub service_access_role_arn: String,
    }

}

