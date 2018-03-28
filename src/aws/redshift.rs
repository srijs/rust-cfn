/// The [`AWS::Redshift::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="AllowVersionUpgrade")]
    pub allow_version_upgrade: (),
    #[serde(rename="AutomatedSnapshotRetentionPeriod")]
    pub automated_snapshot_retention_period: (),
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: (),
    #[serde(rename="ClusterIdentifier")]
    pub cluster_identifier: (),
    #[serde(rename="ClusterParameterGroupName")]
    pub cluster_parameter_group_name: (),
    #[serde(rename="ClusterSecurityGroups")]
    pub cluster_security_groups: (),
    #[serde(rename="ClusterSubnetGroupName")]
    pub cluster_subnet_group_name: (),
    #[serde(rename="ClusterType")]
    pub cluster_type: (),
    #[serde(rename="ClusterVersion")]
    pub cluster_version: (),
    #[serde(rename="DBName")]
    pub db_name: (),
    #[serde(rename="ElasticIp")]
    pub elastic_ip: (),
    #[serde(rename="Encrypted")]
    pub encrypted: (),
    #[serde(rename="HsmClientCertificateIdentifier")]
    pub hsm_client_certificate_identifier: (),
    #[serde(rename="HsmConfigurationIdentifier")]
    pub hsm_configuration_identifier: (),
    #[serde(rename="IamRoles")]
    pub iam_roles: (),
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: (),
    #[serde(rename="LoggingProperties")]
    pub logging_properties: (),
    #[serde(rename="MasterUserPassword")]
    pub master_user_password: (),
    #[serde(rename="MasterUsername")]
    pub master_username: (),
    #[serde(rename="NodeType")]
    pub node_type: (),
    #[serde(rename="NumberOfNodes")]
    pub number_of_nodes: (),
    #[serde(rename="OwnerAccount")]
    pub owner_account: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: (),
    #[serde(rename="PubliclyAccessible")]
    pub publicly_accessible: (),
    #[serde(rename="SnapshotClusterIdentifier")]
    pub snapshot_cluster_identifier: (),
    #[serde(rename="SnapshotIdentifier")]
    pub snapshot_identifier: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: (),
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

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::Redshift::ClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterParameterGroup {
    properties: ClusterParameterGroupProperties
}

/// Properties for the `ClusterParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="ParameterGroupFamily")]
    pub parameter_group_family: (),
    #[serde(rename="Parameters")]
    pub parameters: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

impl From<ClusterParameterGroupProperties> for ClusterParameterGroup {
    fn from(properties: ClusterParameterGroupProperties) -> ClusterParameterGroup {
        ClusterParameterGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSecurityGroup {
    properties: ClusterSecurityGroupProperties
}

/// Properties for the `ClusterSecurityGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSecurityGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

impl From<ClusterSecurityGroupProperties> for ClusterSecurityGroup {
    fn from(properties: ClusterSecurityGroupProperties) -> ClusterSecurityGroup {
        ClusterSecurityGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSecurityGroupIngress {
    properties: ClusterSecurityGroupIngressProperties
}

/// Properties for the `ClusterSecurityGroupIngress` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSecurityGroupIngressProperties {
    #[serde(rename="CIDRIP")]
    pub cidrip: (),
    #[serde(rename="ClusterSecurityGroupName")]
    pub cluster_security_group_name: (),
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: (),
    #[serde(rename="EC2SecurityGroupOwnerId")]
    pub ec2_security_group_owner_id: (),
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

impl From<ClusterSecurityGroupIngressProperties> for ClusterSecurityGroupIngress {
    fn from(properties: ClusterSecurityGroupIngressProperties) -> ClusterSecurityGroupIngress {
        ClusterSecurityGroupIngress { properties }
    }
}

/// The [`AWS::Redshift::ClusterSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSubnetGroup {
    properties: ClusterSubnetGroupProperties
}

/// Properties for the `ClusterSubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterSubnetGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="SubnetIds")]
    pub subnet_ids: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

impl From<ClusterSubnetGroupProperties> for ClusterSubnetGroup {
    fn from(properties: ClusterSubnetGroupProperties) -> ClusterSubnetGroup {
        ClusterSubnetGroup { properties }
    }
}

