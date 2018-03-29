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
    #[serde(rename="AllowVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    /// Property `AutomatedSnapshotRetentionPeriod`.
    #[serde(rename="AutomatedSnapshotRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<u32>,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// Property `ClusterIdentifier`.
    #[serde(rename="ClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    /// Property `ClusterParameterGroupName`.
    #[serde(rename="ClusterParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_name: Option<String>,
    /// Property `ClusterSecurityGroups`.
    #[serde(rename="ClusterSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<Vec<String>>,
    /// Property `ClusterSubnetGroupName`.
    #[serde(rename="ClusterSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    /// Property `ClusterType`.
    #[serde(rename="ClusterType")]
    pub cluster_type: String,
    /// Property `ClusterVersion`.
    #[serde(rename="ClusterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// Property `DBName`.
    #[serde(rename="DBName")]
    pub db_name: String,
    /// Property `ElasticIp`.
    #[serde(rename="ElasticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    /// Property `Encrypted`.
    #[serde(rename="Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Property `HsmClientCertificateIdentifier`.
    #[serde(rename="HsmClientCertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    /// Property `HsmConfigurationIdentifier`.
    #[serde(rename="HsmConfigurationIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    /// Property `IamRoles`.
    #[serde(rename="IamRoles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<Vec<String>>,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// Property `LoggingProperties`.
    #[serde(rename="LoggingProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_properties: Option<self::cluster::LoggingProperties>,
    /// Property `MasterUserPassword`.
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: String,
    /// Property `MasterUsername`.
    #[serde(rename="MasterUsername")]
    pub master_username: String,
    /// Property `NodeType`.
    #[serde(rename="NodeType")]
    pub node_type: String,
    /// Property `NumberOfNodes`.
    #[serde(rename="NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<u32>,
    /// Property `OwnerAccount`.
    #[serde(rename="OwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// Property `Port`.
    #[serde(rename="Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// Property `PubliclyAccessible`.
    #[serde(rename="PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// Property `SnapshotClusterIdentifier`.
    #[serde(rename="SnapshotClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    /// Property `SnapshotIdentifier`.
    #[serde(rename="SnapshotIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `ParameterGroupFamily`.
    #[serde(rename="ParameterGroupFamily")]
    pub parameter_group_family: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<self::cluster_parameter_group::Parameter>>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(rename="CIDRIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrip: Option<String>,
    /// Property `ClusterSecurityGroupName`.
    #[serde(rename="ClusterSecurityGroupName")]
    pub cluster_security_group_name: String,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename="EC2SecurityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_name: Option<String>,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename="EC2SecurityGroupOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<String>,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
        #[serde(rename="BucketName")]
        pub bucket_name: String,
        /// Property `S3KeyPrefix`.
        #[serde(rename="S3KeyPrefix")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_key_prefix: Option<String>,
    }
}

pub mod cluster_parameter_group {
    //! Property types for the `ClusterParameterGroup` resource.

    /// The [`AWS::Redshift::ClusterParameterGroup.Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-property-redshift-clusterparametergroup-parameter.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Parameter {
        /// Property `ParameterName`.
        #[serde(rename="ParameterName")]
        pub parameter_name: String,
        /// Property `ParameterValue`.
        #[serde(rename="ParameterValue")]
        pub parameter_value: String,
    }
}
