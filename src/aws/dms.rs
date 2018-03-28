/// The [`AWS::DMS::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Serialize, Deserialize)]
pub struct CertificateProperties {
    #[serde(rename="CertificateIdentifier")]
    pub certificate_identifier: (),
    #[serde(rename="CertificatePem")]
    pub certificate_pem: (),
    #[serde(rename="CertificateWallet")]
    pub certificate_wallet: (),
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

/// The [`AWS::DMS::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(rename="CertificateArn")]
    pub certificate_arn: (),
    #[serde(rename="DatabaseName")]
    pub database_name: (),
    #[serde(rename="DynamoDbSettings")]
    pub dynamo_db_settings: (),
    #[serde(rename="EndpointIdentifier")]
    pub endpoint_identifier: (),
    #[serde(rename="EndpointType")]
    pub endpoint_type: (),
    #[serde(rename="EngineName")]
    pub engine_name: (),
    #[serde(rename="ExtraConnectionAttributes")]
    pub extra_connection_attributes: (),
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: (),
    #[serde(rename="MongoDbSettings")]
    pub mongo_db_settings: (),
    #[serde(rename="Password")]
    pub password: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="S3Settings")]
    pub s3_settings: (),
    #[serde(rename="ServerName")]
    pub server_name: (),
    #[serde(rename="SslMode")]
    pub ssl_mode: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="Username")]
    pub username: (),
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

/// The [`AWS::DMS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html) resource.
#[derive(Serialize, Deserialize)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Serialize, Deserialize)]
pub struct EventSubscriptionProperties {
    #[serde(rename="Enabled")]
    pub enabled: (),
    #[serde(rename="EventCategories")]
    pub event_categories: (),
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: (),
    #[serde(rename="SourceIds")]
    pub source_ids: (),
    #[serde(rename="SourceType")]
    pub source_type: (),
    #[serde(rename="SubscriptionName")]
    pub subscription_name: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

/// The [`AWS::DMS::ReplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationInstance {
    properties: ReplicationInstanceProperties
}

/// Properties for the `ReplicationInstance` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationInstanceProperties {
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: (),
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: (),
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: (),
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: (),
    #[serde(rename="EngineVersion")]
    pub engine_version: (),
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: (),
    #[serde(rename="MultiAZ")]
    pub multi_az: (),
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: (),
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: (),
    #[serde(rename="ReplicationInstanceClass")]
    pub replication_instance_class: (),
    #[serde(rename="ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: (),
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: (),
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

/// The [`AWS::DMS::ReplicationSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationSubnetGroup {
    properties: ReplicationSubnetGroupProperties
}

/// Properties for the `ReplicationSubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationSubnetGroupProperties {
    #[serde(rename="ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: (),
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: (),
    #[serde(rename="SubnetIds")]
    pub subnet_ids: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

/// The [`AWS::DMS::ReplicationTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationTask {
    properties: ReplicationTaskProperties
}

/// Properties for the `ReplicationTask` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationTaskProperties {
    #[serde(rename="CdcStartTime")]
    pub cdc_start_time: (),
    #[serde(rename="MigrationType")]
    pub migration_type: (),
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: (),
    #[serde(rename="ReplicationTaskIdentifier")]
    pub replication_task_identifier: (),
    #[serde(rename="ReplicationTaskSettings")]
    pub replication_task_settings: (),
    #[serde(rename="SourceEndpointArn")]
    pub source_endpoint_arn: (),
    #[serde(rename="TableMappings")]
    pub table_mappings: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="TargetEndpointArn")]
    pub target_endpoint_arn: (),
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

