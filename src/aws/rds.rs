/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Serialize, Deserialize)]
pub struct DBClusterProperties {
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: u32,
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: String,
    #[serde(rename="DBClusterParameterGroupName")]
    pub db_cluster_parameter_group_name: String,
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[serde(rename="Engine")]
    pub engine: String,
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: String,
    #[serde(rename="MasterUsername")]
    pub master_username: String,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: String,
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    #[serde(rename="ReplicationSourceIdentifier")]
    pub replication_source_identifier: String,
    #[serde(rename="SnapshotIdentifier")]
    pub snapshot_identifier: String,
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: bool,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
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

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBClusterParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Family")]
    pub family: String,
    #[serde(rename="Parameters")]
    pub parameters: ::json::Value,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource type.
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Serialize, Deserialize)]
pub struct DBInstanceProperties {
    #[serde(rename="AllocatedStorage")]
    pub allocated_storage: String,
    #[serde(rename="AllowMajorVersionUpgrade")]
    pub allow_major_version_upgrade: bool,
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    #[serde(rename="BackupRetentionPeriod")]
    pub backup_retention_period: String,
    #[serde(rename="CharacterSetName")]
    pub character_set_name: String,
    #[serde(rename="CopyTagsToSnapshot")]
    pub copy_tags_to_snapshot: bool,
    #[serde(rename="DBClusterIdentifier")]
    pub db_cluster_identifier: String,
    #[serde(rename="DBInstanceClass")]
    pub db_instance_class: String,
    #[serde(rename="DBInstanceIdentifier")]
    pub db_instance_identifier: String,
    #[serde(rename="DBName")]
    pub db_name: String,
    #[serde(rename="DBParameterGroupName")]
    pub db_parameter_group_name: String,
    #[serde(rename="DBSecurityGroups")]
    pub db_security_groups: Vec<String>,
    #[serde(rename="DBSnapshotIdentifier")]
    pub db_snapshot_identifier: String,
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    #[serde(rename="Domain")]
    pub domain: String,
    #[serde(rename="DomainIAMRoleName")]
    pub domain_iam_role_name: String,
    #[serde(rename="Engine")]
    pub engine: String,
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    #[serde(rename="Iops")]
    pub iops: u32,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="LicenseModel")]
    pub license_model: String,
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: String,
    #[serde(rename="MasterUsername")]
    pub master_username: String,
    #[serde(rename="MonitoringInterval")]
    pub monitoring_interval: u32,
    #[serde(rename="MonitoringRoleArn")]
    pub monitoring_role_arn: String,
    #[serde(rename="MultiAZ")]
    pub multi_az: bool,
    #[serde(rename="OptionGroupName")]
    pub option_group_name: String,
    #[serde(rename="Port")]
    pub port: String,
    #[serde(rename="PreferredBackupWindow")]
    pub preferred_backup_window: String,
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: bool,
    #[serde(rename="SourceDBInstanceIdentifier")]
    pub source_db_instance_identifier: String,
    #[serde(rename="SourceRegion")]
    pub source_region: String,
    #[serde(rename="StorageEncrypted")]
    pub storage_encrypted: bool,
    #[serde(rename="StorageType")]
    pub storage_type: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Timezone")]
    pub timezone: String,
    #[serde(rename="VPCSecurityGroups")]
    pub vpc_security_groups: Vec<String>,
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

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource type.
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Family")]
    pub family: String,
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, String>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroupProperties {
    #[serde(rename="DBSecurityGroupIngress")]
    pub db_security_group_ingress: Vec<self::db_security_group::Ingress>,
    #[serde(rename="EC2VpcId")]
    pub ec2_vpc_id: String,
    #[serde(rename="GroupDescription")]
    pub group_description: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSecurityGroupIngressProperties {
    #[serde(rename="CIDRIP")]
    pub cidrip: String,
    #[serde(rename="DBSecurityGroupName")]
    pub db_security_group_name: String,
    #[serde(rename="EC2SecurityGroupId")]
    pub ec2_security_group_id: String,
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: String,
    #[serde(rename="EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: String,
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

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource type.
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct DBSubnetGroupProperties {
    #[serde(rename="DBSubnetGroupDescription")]
    pub db_subnet_group_description: String,
    #[serde(rename="DBSubnetGroupName")]
    pub db_subnet_group_name: String,
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
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

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct OptionGroupProperties {
    #[serde(rename="EngineName")]
    pub engine_name: String,
    #[serde(rename="MajorEngineVersion")]
    pub major_engine_version: String,
    #[serde(rename="OptionConfigurations")]
    pub option_configurations: Vec<self::option_group::OptionConfiguration>,
    #[serde(rename="OptionGroupDescription")]
    pub option_group_description: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

pub mod db_security_group {
    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Ingress {
        #[serde(rename="CIDRIP")]
        pub cidrip: String,
        #[serde(rename="EC2SecurityGroupId")]
        pub ec2_security_group_id: String,
        #[serde(rename="EC2SecurityGroupName")]
        pub ec2_security_group_name: String,
        #[serde(rename="EC2SecurityGroupOwnerId")]
        pub ec2_security_group_owner_id: String,
    }

}

pub mod option_group {
    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct OptionConfiguration {
        #[serde(rename="DBSecurityGroupMemberships")]
        pub db_security_group_memberships: Vec<String>,
        #[serde(rename="OptionName")]
        pub option_name: String,
        #[serde(rename="OptionSettings")]
        pub option_settings: OptionSetting,
        #[serde(rename="OptionVersion")]
        pub option_version: String,
        #[serde(rename="Port")]
        pub port: u32,
        #[serde(rename="VpcSecurityGroupMemberships")]
        pub vpc_security_group_memberships: Vec<String>,
    }

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct OptionSetting {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

