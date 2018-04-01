//! Types for the `RDS` service.

/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
#[derive(Debug)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterProperties {
    /// Property `AvailabilityZones`.
    #[serde(rename = "AvailabilityZones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `BackupRetentionPeriod`.
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<::Value<u32>>,
    /// Property `DBClusterIdentifier`.
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property `DBClusterParameterGroupName`.
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_cluster_parameter_group_name: Option<::Value<String>>,
    /// Property `DBSubnetGroupName`.
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `DatabaseName`.
    #[serde(rename = "DatabaseName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<::Value<String>>,
    /// Property `Engine`.
    #[serde(rename = "Engine")]
    pub engine: ::Value<String>,
    /// Property `EngineVersion`.
    #[serde(rename = "EngineVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MasterUserPassword`.
    #[serde(rename = "MasterUserPassword")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<::Value<String>>,
    /// Property `MasterUsername`.
    #[serde(rename = "MasterUsername")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_username: Option<::Value<String>>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<u32>>,
    /// Property `PreferredBackupWindow`.
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `ReplicationSourceIdentifier`.
    #[serde(rename = "ReplicationSourceIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_source_identifier: Option<::Value<String>>,
    /// Property `SnapshotIdentifier`.
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property `StorageEncrypted`.
    #[serde(rename = "StorageEncrypted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<::ValueList<String>>,
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

impl ::private::Sealed for DBCluster {}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBClusterParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `Family`.
    #[serde(rename = "Family")]
    pub family: ::Value<String>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    pub parameters: ::Value<::json::Value>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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

impl ::private::Sealed for DBClusterParameterGroup {}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource type.
#[derive(Debug)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBInstanceProperties {
    /// Property `AllocatedStorage`.
    #[serde(rename = "AllocatedStorage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<::Value<String>>,
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
    /// Property `BackupRetentionPeriod`.
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<::Value<String>>,
    /// Property `CharacterSetName`.
    #[serde(rename = "CharacterSetName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<::Value<String>>,
    /// Property `CopyTagsToSnapshot`.
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<::Value<bool>>,
    /// Property `DBClusterIdentifier`.
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property `DBInstanceClass`.
    #[serde(rename = "DBInstanceClass")]
    pub db_instance_class: ::Value<String>,
    /// Property `DBInstanceIdentifier`.
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<::Value<String>>,
    /// Property `DBName`.
    #[serde(rename = "DBName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_name: Option<::Value<String>>,
    /// Property `DBParameterGroupName`.
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_name: Option<::Value<String>>,
    /// Property `DBSecurityGroups`.
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_security_groups: Option<::ValueList<String>>,
    /// Property `DBSnapshotIdentifier`.
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<::Value<String>>,
    /// Property `DBSubnetGroupName`.
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `Domain`.
    #[serde(rename = "Domain")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<::Value<String>>,
    /// Property `DomainIAMRoleName`.
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_iam_role_name: Option<::Value<String>>,
    /// Property `Engine`.
    #[serde(rename = "Engine")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<::Value<String>>,
    /// Property `EngineVersion`.
    #[serde(rename = "EngineVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<::Value<String>>,
    /// Property `Iops`.
    #[serde(rename = "Iops")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<::Value<u32>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `LicenseModel`.
    #[serde(rename = "LicenseModel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<::Value<String>>,
    /// Property `MasterUserPassword`.
    #[serde(rename = "MasterUserPassword")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<::Value<String>>,
    /// Property `MasterUsername`.
    #[serde(rename = "MasterUsername")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_username: Option<::Value<String>>,
    /// Property `MonitoringInterval`.
    #[serde(rename = "MonitoringInterval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<::Value<u32>>,
    /// Property `MonitoringRoleArn`.
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<::Value<String>>,
    /// Property `MultiAZ`.
    #[serde(rename = "MultiAZ")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<::Value<bool>>,
    /// Property `OptionGroupName`.
    #[serde(rename = "OptionGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<::Value<String>>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<String>>,
    /// Property `PreferredBackupWindow`.
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `SourceDBInstanceIdentifier`.
    #[serde(rename = "SourceDBInstanceIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_db_instance_identifier: Option<::Value<String>>,
    /// Property `SourceRegion`.
    #[serde(rename = "SourceRegion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_region: Option<::Value<String>>,
    /// Property `StorageEncrypted`.
    #[serde(rename = "StorageEncrypted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property `StorageType`.
    #[serde(rename = "StorageType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Timezone`.
    #[serde(rename = "Timezone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<::Value<String>>,
    /// Property `VPCSecurityGroups`.
    #[serde(rename = "VPCSecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<::ValueList<String>>,
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

impl ::private::Sealed for DBInstance {}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `Family`.
    #[serde(rename = "Family")]
    pub family: ::Value<String>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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

impl ::private::Sealed for DBParameterGroup {}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSecurityGroupProperties {
    /// Property `DBSecurityGroupIngress`.
    #[serde(rename = "DBSecurityGroupIngress")]
    pub db_security_group_ingress: ::ValueList<self::db_security_group::Ingress>,
    /// Property `EC2VpcId`.
    #[serde(rename = "EC2VpcId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_vpc_id: Option<::Value<String>>,
    /// Property `GroupDescription`.
    #[serde(rename = "GroupDescription")]
    pub group_description: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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

impl ::private::Sealed for DBSecurityGroup {}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSecurityGroupIngressProperties {
    /// Property `CIDRIP`.
    #[serde(rename = "CIDRIP")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidrip: Option<::Value<String>>,
    /// Property `DBSecurityGroupName`.
    #[serde(rename = "DBSecurityGroupName")]
    pub db_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupId`.
    #[serde(rename = "EC2SecurityGroupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_id: Option<::Value<String>>,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<::Value<String>>,
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

impl ::private::Sealed for DBSecurityGroupIngress {}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource type.
#[derive(Debug)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DBSubnetGroupProperties {
    /// Property `DBSubnetGroupDescription`.
    #[serde(rename = "DBSubnetGroupDescription")]
    pub db_subnet_group_description: ::Value<String>,
    /// Property `DBSubnetGroupName`.
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `SubnetIds`.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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

impl ::private::Sealed for DBSubnetGroup {}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
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

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
#[derive(Debug)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionGroupProperties {
    /// Property `EngineName`.
    #[serde(rename = "EngineName")]
    pub engine_name: ::Value<String>,
    /// Property `MajorEngineVersion`.
    #[serde(rename = "MajorEngineVersion")]
    pub major_engine_version: ::Value<String>,
    /// Property `OptionConfigurations`.
    #[serde(rename = "OptionConfigurations")]
    pub option_configurations: ::ValueList<self::option_group::OptionConfiguration>,
    /// Property `OptionGroupDescription`.
    #[serde(rename = "OptionGroupDescription")]
    pub option_group_description: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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

impl ::private::Sealed for OptionGroup {}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

pub mod db_security_group {
    //! Property types for the `DBSecurityGroup` resource.

    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ingress {
        /// Property `CIDRIP`.
        #[serde(rename = "CIDRIP")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cidrip: Option<::Value<String>>,
        /// Property `EC2SecurityGroupId`.
        #[serde(rename = "EC2SecurityGroupId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_security_group_id: Option<::Value<String>>,
        /// Property `EC2SecurityGroupName`.
        #[serde(rename = "EC2SecurityGroupName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_security_group_name: Option<::Value<String>>,
        /// Property `EC2SecurityGroupOwnerId`.
        #[serde(rename = "EC2SecurityGroupOwnerId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ec2_security_group_owner_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Ingress);
}

pub mod option_group {
    //! Property types for the `OptionGroup` resource.

    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionConfiguration {
        /// Property `DBSecurityGroupMemberships`.
        #[serde(rename = "DBSecurityGroupMemberships")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub db_security_group_memberships: Option<::ValueList<String>>,
        /// Property `OptionName`.
        #[serde(rename = "OptionName")]
        pub option_name: ::Value<String>,
        /// Property `OptionSettings`.
        #[serde(rename = "OptionSettings")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub option_settings: Option<::Value<OptionSetting>>,
        /// Property `OptionVersion`.
        #[serde(rename = "OptionVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub option_version: Option<::Value<String>>,
        /// Property `Port`.
        #[serde(rename = "Port")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port: Option<::Value<u32>>,
        /// Property `VpcSecurityGroupMemberships`.
        #[serde(rename = "VpcSecurityGroupMemberships")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vpc_security_group_memberships: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(OptionConfiguration);

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OptionSetting {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(OptionSetting);
}
