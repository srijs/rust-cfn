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
    pub allow_version_upgrade: bool,
    /// Property `AutomatedSnapshotRetentionPeriod`.
    #[serde(rename="AutomatedSnapshotRetentionPeriod")]
    pub automated_snapshot_retention_period: u32,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: String,
    /// Property `ClusterIdentifier`.
    #[serde(rename="ClusterIdentifier")]
    pub cluster_identifier: String,
    /// Property `ClusterParameterGroupName`.
    #[serde(rename="ClusterParameterGroupName")]
    pub cluster_parameter_group_name: String,
    /// Property `ClusterSecurityGroups`.
    #[serde(rename="ClusterSecurityGroups")]
    pub cluster_security_groups: Vec<String>,
    /// Property `ClusterSubnetGroupName`.
    #[serde(rename="ClusterSubnetGroupName")]
    pub cluster_subnet_group_name: String,
    /// Property `ClusterType`.
    #[serde(rename="ClusterType")]
    pub cluster_type: String,
    /// Property `ClusterVersion`.
    #[serde(rename="ClusterVersion")]
    pub cluster_version: String,
    /// Property `DBName`.
    #[serde(rename="DBName")]
    pub db_name: String,
    /// Property `ElasticIp`.
    #[serde(rename="ElasticIp")]
    pub elastic_ip: String,
    /// Property `Encrypted`.
    #[serde(rename="Encrypted")]
    pub encrypted: bool,
    /// Property `HsmClientCertificateIdentifier`.
    #[serde(rename="HsmClientCertificateIdentifier")]
    pub hsm_client_certificate_identifier: String,
    /// Property `HsmConfigurationIdentifier`.
    #[serde(rename="HsmConfigurationIdentifier")]
    pub hsm_configuration_identifier: String,
    /// Property `IamRoles`.
    #[serde(rename="IamRoles")]
    pub iam_roles: Vec<String>,
    /// Property `KmsKeyId`.
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    /// Property `LoggingProperties`.
    #[serde(rename="LoggingProperties")]
    pub logging_properties: self::cluster::LoggingProperties,
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
    pub number_of_nodes: u32,
    /// Property `OwnerAccount`.
    #[serde(rename="OwnerAccount")]
    pub owner_account: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `PubliclyAccessible`.
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: bool,
    /// Property `SnapshotClusterIdentifier`.
    #[serde(rename="SnapshotClusterIdentifier")]
    pub snapshot_cluster_identifier: String,
    /// Property `SnapshotIdentifier`.
    #[serde(rename="SnapshotIdentifier")]
    pub snapshot_identifier: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
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
    pub parameters: Vec<self::cluster_parameter_group::Parameter>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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
    pub tags: ::Tags,
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
    pub cidrip: String,
    /// Property `ClusterSecurityGroupName`.
    #[serde(rename="ClusterSecurityGroupName")]
    pub cluster_security_group_name: String,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: String,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename="EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: String,
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
    pub tags: ::Tags,
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
        pub s3_key_prefix: String,
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
