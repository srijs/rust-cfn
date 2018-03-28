/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Serialize, Deserialize)]
pub struct DBClusterProperties {
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: (),
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: (),
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: (),
    #[serde(rename="DBClusterParameterGroupName")]
    pub db_cluster_parameter_group_name: (),
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: (),
    #[serde(rename="DatabaseName")]
    pub database_name: (),
    #[serde(rename="Engine")]
    pub engine: (),
    #[serde(rename="EngineVersion")]
    pub engine_version: (),
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: (),
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: (),
    #[serde(rename="MasterUsername")]
    pub master_username: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: (),
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: (),
    #[serde(rename="ReplicationSourceIdentifier")]
    pub replication_source_identifier: (),
    #[serde(rename="SnapshotIdentifier")]
    pub snapshot_identifier: (),
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: (),
}

impl<'a> ::Resource<'a> for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::RDS::DBCluster";
    fn properties(&self) -> &DBClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterProperties {
        &mut self.properties
    }
}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBClusterParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Family")]
    pub family: (),
    #[serde(rename="Parameters")]
    pub parameters: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for DBClusterParameterGroup {
    type Properties = DBClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBClusterParameterGroup";
    fn properties(&self) -> &DBClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Serialize, Deserialize)]
pub struct DBInstanceProperties {
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: (),
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: (),
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: (),
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: (),
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: (),
    #[serde(rename="CharacterSetName")]
    pub character_set_name: (),
    #[serde(rename="CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: (),
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: (),
    #[serde(rename="DBInstanceClass")]
    pub db_instance_class: (),
    #[serde(rename="DBInstanceIdentifier")]
    pub db_instance_identifier: (),
    #[serde(rename="DBName")]
    pub db_name: (),
    #[serde(rename="DBParameterGroupName")]
    pub db_parameter_group_name: (),
    #[serde(rename="DBSecurityGroups")]
    pub db_security_groups: (),
    #[serde(rename="DBSnapshotIdentifier")]
    pub db_snapshot_identifier: (),
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: (),
    #[serde(rename="Domain")]
    pub domain: (),
    #[serde(rename="DomainIAMRoleName")]
    pub domain_iam_role_name: (),
    #[serde(rename="Engine")]
    pub engine: (),
    #[serde(rename="EngineVersion")]
    pub engine_version: (),
    #[serde(rename="Iops")]
    pub iops: (),
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: (),
    #[serde(rename="LicenseModel")]
    pub license_model: (),
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: (),
    #[serde(rename="MasterUsername")]
    pub master_username: (),
    #[serde(rename="MonitoringInterval")]
    pub monitoring_interval: (),
    #[serde(rename="MonitoringRoleArn")]
    pub monitoring_role_arn: (),
    #[serde(rename="MultiAZ")]
    pub multi_az: (),
    #[serde(rename="OptionGroupName")]
    pub option_group_name: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: (),
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: (),
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: (),
    #[serde(rename="SourceDBInstanceIdentifier")]
    pub source_db_instance_identifier: (),
    #[serde(rename="SourceRegion")]
    pub source_region: (),
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: (),
    #[serde(rename="StorageType")]
    pub storage_type: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="Timezone")]
    pub timezone: (),
    #[serde(rename="VPCSecurityGroups")]
    pub vpc_security_groups: (),
}

impl<'a> ::Resource<'a> for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::RDS::DBInstance";
    fn properties(&self) -> &DBInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBInstanceProperties {
        &mut self.properties
    }
}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Family")]
    pub family: (),
    #[serde(rename="Parameters")]
    pub parameters: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for DBParameterGroup {
    type Properties = DBParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBParameterGroup";
    fn properties(&self) -> &DBParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBParameterGroupProperties {
        &mut self.properties
    }
}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroupProperties {
    #[serde(rename="DBSecurityGroupIngress")]
    pub db_security_group_ingress: (),
    #[serde(rename="EC2VpcId")]
    pub ec2_vpc_id: (),
    #[serde(rename="GroupDescription")]
    pub group_description: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for DBSecurityGroup {
    type Properties = DBSecurityGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroup";
    fn properties(&self) -> &DBSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupProperties {
        &mut self.properties
    }
}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroupIngressProperties {
    #[serde(rename="CIDRIP")]
    pub cidrip: (),
    #[serde(rename="DBSecurityGroupName")]
    pub db_security_group_name: (),
    #[serde(rename="EC2SecurityGroupId")]
    pub ec2_security_group_id: (),
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: (),
    #[serde(rename="EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: (),
}

impl<'a> ::Resource<'a> for DBSecurityGroupIngress {
    type Properties = DBSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroupIngress";
    fn properties(&self) -> &DBSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSubnetGroupProperties {
    #[serde(rename="DBSubnetGroupDescription")]
    pub db_subnet_group_description: (),
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: (),
    #[serde(rename="SubnetIds")]
    pub subnet_ids: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for DBSubnetGroup {
    type Properties = DBSubnetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSubnetGroup";
    fn properties(&self) -> &DBSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSubnetGroupProperties {
        &mut self.properties
    }
}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource.
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
}

impl<'a> ::Resource<'a> for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::RDS::EventSubscription";
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

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct OptionGroupProperties {
    #[serde(rename="EngineName")]
    pub engine_name: (),
    #[serde(rename="MajorEngineVersion")]
    pub major_engine_version: (),
    #[serde(rename="OptionConfigurations")]
    pub option_configurations: (),
    #[serde(rename="OptionGroupDescription")]
    pub option_group_description: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for OptionGroup {
    type Properties = OptionGroupProperties;
    const TYPE: &'static str = "AWS::RDS::OptionGroup";
    fn properties(&self) -> &OptionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OptionGroupProperties {
        &mut self.properties
    }
}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

