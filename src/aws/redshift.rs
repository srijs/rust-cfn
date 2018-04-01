//! Types for the `Redshift` service.

/// The [`AWS::Redshift::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterProperties {
    /// Property `AllowVersionUpgrade`.
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<::Value<bool>>,
    /// Property `AutomatedSnapshotRetentionPeriod`.
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<::Value<u32>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<::Value<String>>,
    /// Property `ClusterIdentifier`.
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<::Value<String>>,
    /// Property `ClusterParameterGroupName`.
    #[serde(rename = "ClusterParameterGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_name: Option<::Value<String>>,
    /// Property `ClusterSecurityGroups`.
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<::ValueList<String>>,
    /// Property `ClusterSubnetGroupName`.
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<::Value<String>>,
    /// Property `ClusterType`.
    #[serde(rename = "ClusterType")]
    pub cluster_type: ::Value<String>,
    /// Property `ClusterVersion`.
    #[serde(rename = "ClusterVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<::Value<String>>,
    /// Property `DBName`.
    #[serde(rename = "DBName")]
    pub db_name: ::Value<String>,
    /// Property `ElasticIp`.
    #[serde(rename = "ElasticIp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<::Value<String>>,
    /// Property `Encrypted`.
    #[serde(rename = "Encrypted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<::Value<bool>>,
    /// Property `HsmClientCertificateIdentifier`.
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<::Value<String>>,
    /// Property `HsmConfigurationIdentifier`.
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<::Value<String>>,
    /// Property `IamRoles`.
    #[serde(rename = "IamRoles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<::ValueList<String>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `LoggingProperties`.
    #[serde(rename = "LoggingProperties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging_properties: Option<::Value<self::cluster::LoggingProperties>>,
    /// Property `MasterUserPassword`.
    #[serde(rename = "MasterUserPassword")]
    pub master_user_password: ::Value<String>,
    /// Property `MasterUsername`.
    #[serde(rename = "MasterUsername")]
    pub master_username: ::Value<String>,
    /// Property `NodeType`.
    #[serde(rename = "NodeType")]
    pub node_type: ::Value<String>,
    /// Property `NumberOfNodes`.
    #[serde(rename = "NumberOfNodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property `OwnerAccount`.
    #[serde(rename = "OwnerAccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<::Value<String>>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<u32>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `SnapshotClusterIdentifier`.
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<::Value<String>>,
    /// Property `SnapshotIdentifier`.
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl<'a> ::Resource<'a> for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::Redshift::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::Redshift::ClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html) resource type.
#[derive(Debug)]
pub struct ClusterParameterGroup {
    properties: ClusterParameterGroupProperties
}

/// Properties for the `ClusterParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterParameterGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `ParameterGroupFamily`.
    #[serde(rename = "ParameterGroupFamily")]
    pub parameter_group_family: ::Value<String>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for ClusterParameterGroup {
    type Properties = ClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterParameterGroup";
    fn properties(&self) -> &ClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterParameterGroup {}

impl From<ClusterParameterGroupProperties> for ClusterParameterGroup {
    fn from(properties: ClusterParameterGroupProperties) -> ClusterParameterGroup {
        ClusterParameterGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html) resource type.
#[derive(Debug)]
pub struct ClusterSecurityGroup {
    properties: ClusterSecurityGroupProperties
}

/// Properties for the `ClusterSecurityGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterSecurityGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for ClusterSecurityGroup {
    type Properties = ClusterSecurityGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSecurityGroup";
    fn properties(&self) -> &ClusterSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSecurityGroup {}

impl From<ClusterSecurityGroupProperties> for ClusterSecurityGroup {
    fn from(properties: ClusterSecurityGroupProperties) -> ClusterSecurityGroup {
        ClusterSecurityGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html) resource type.
#[derive(Debug)]
pub struct ClusterSecurityGroupIngress {
    properties: ClusterSecurityGroupIngressProperties
}

/// Properties for the `ClusterSecurityGroupIngress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterSecurityGroupIngressProperties {
    /// Property `CIDRIP`.
    #[serde(rename = "CIDRIP")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidrip: Option<::Value<String>>,
    /// Property `ClusterSecurityGroupName`.
    #[serde(rename = "ClusterSecurityGroupName")]
    pub cluster_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for ClusterSecurityGroupIngress {
    type Properties = ClusterSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSecurityGroupIngress";
    fn properties(&self) -> &ClusterSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSecurityGroupIngress {}

impl From<ClusterSecurityGroupIngressProperties> for ClusterSecurityGroupIngress {
    fn from(properties: ClusterSecurityGroupIngressProperties) -> ClusterSecurityGroupIngress {
        ClusterSecurityGroupIngress { properties }
    }
}

/// The [`AWS::Redshift::ClusterSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html) resource type.
#[derive(Debug)]
pub struct ClusterSubnetGroup {
    properties: ClusterSubnetGroupProperties
}

/// Properties for the `ClusterSubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterSubnetGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `SubnetIds`.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for ClusterSubnetGroup {
    type Properties = ClusterSubnetGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSubnetGroup";
    fn properties(&self) -> &ClusterSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSubnetGroup {}

impl From<ClusterSubnetGroupProperties> for ClusterSubnetGroup {
    fn from(properties: ClusterSubnetGroupProperties) -> ClusterSubnetGroup {
        ClusterSubnetGroup { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::Redshift::Cluster.LoggingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoggingProperties {
        /// Property `BucketName`.
        #[serde(rename = "BucketName")]
        pub bucket_name: ::Value<String>,
        /// Property `S3KeyPrefix`.
        #[serde(rename = "S3KeyPrefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_key_prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LoggingProperties);
}

pub mod cluster_parameter_group {
    //! Property types for the `ClusterParameterGroup` resource.

    /// The [`AWS::Redshift::ClusterParameterGroup.Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-property-redshift-clusterparametergroup-parameter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Parameter {
        /// Property `ParameterName`.
        #[serde(rename = "ParameterName")]
        pub parameter_name: ::Value<String>,
        /// Property `ParameterValue`.
        #[serde(rename = "ParameterValue")]
        pub parameter_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Parameter);
}
