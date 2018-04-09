//! Types for the `Redshift` service.

/// The [`AWS::Redshift::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug)]
pub struct ClusterProperties {
    /// Property `AllowVersionUpgrade`.
    pub allow_version_upgrade: Option<::Value<bool>>,
    /// Property `AutomatedSnapshotRetentionPeriod`.
    pub automated_snapshot_retention_period: Option<::Value<u32>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `ClusterIdentifier`.
    pub cluster_identifier: Option<::Value<String>>,
    /// Property `ClusterParameterGroupName`.
    pub cluster_parameter_group_name: Option<::Value<String>>,
    /// Property `ClusterSecurityGroups`.
    pub cluster_security_groups: Option<::ValueList<String>>,
    /// Property `ClusterSubnetGroupName`.
    pub cluster_subnet_group_name: Option<::Value<String>>,
    /// Property `ClusterType`.
    pub cluster_type: ::Value<String>,
    /// Property `ClusterVersion`.
    pub cluster_version: Option<::Value<String>>,
    /// Property `DBName`.
    pub db_name: ::Value<String>,
    /// Property `ElasticIp`.
    pub elastic_ip: Option<::Value<String>>,
    /// Property `Encrypted`.
    pub encrypted: Option<::Value<bool>>,
    /// Property `HsmClientCertificateIdentifier`.
    pub hsm_client_certificate_identifier: Option<::Value<String>>,
    /// Property `HsmConfigurationIdentifier`.
    pub hsm_configuration_identifier: Option<::Value<String>>,
    /// Property `IamRoles`.
    pub iam_roles: Option<::ValueList<String>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `LoggingProperties`.
    pub logging_properties: Option<::Value<self::cluster::LoggingProperties>>,
    /// Property `MasterUserPassword`.
    pub master_user_password: ::Value<String>,
    /// Property `MasterUsername`.
    pub master_username: ::Value<String>,
    /// Property `NodeType`.
    pub node_type: ::Value<String>,
    /// Property `NumberOfNodes`.
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property `OwnerAccount`.
    pub owner_account: Option<::Value<String>>,
    /// Property `Port`.
    pub port: Option<::Value<u32>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `SnapshotClusterIdentifier`.
    pub snapshot_cluster_identifier: Option<::Value<String>>,
    /// Property `SnapshotIdentifier`.
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowVersionUpgrade", &self.allow_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomatedSnapshotRetentionPeriod", &self.automated_snapshot_retention_period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterParameterGroupName", &self.cluster_parameter_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroups", &self.cluster_security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSubnetGroupName", &self.cluster_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterType", &self.cluster_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterVersion", &self.cluster_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", &self.db_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIp", &self.elastic_ip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmClientCertificateIdentifier", &self.hsm_client_certificate_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmConfigurationIdentifier", &self.hsm_configuration_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", &self.iam_roles)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingProperties", &self.logging_properties)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", &self.master_user_password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeType", &self.node_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", &self.number_of_nodes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerAccount", &self.owner_account)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", &self.publicly_accessible)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotClusterIdentifier", &self.snapshot_cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", &self.snapshot_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", &self.vpc_security_group_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_version_upgrade = None;
                let mut automated_snapshot_retention_period = None;
                let mut availability_zone = None;
                let mut cluster_identifier = None;
                let mut cluster_parameter_group_name = None;
                let mut cluster_security_groups = None;
                let mut cluster_subnet_group_name = None;
                let mut cluster_type = None;
                let mut cluster_version = None;
                let mut db_name = None;
                let mut elastic_ip = None;
                let mut encrypted = None;
                let mut hsm_client_certificate_identifier = None;
                let mut hsm_configuration_identifier = None;
                let mut iam_roles = None;
                let mut kms_key_id = None;
                let mut logging_properties = None;
                let mut master_user_password = None;
                let mut master_username = None;
                let mut node_type = None;
                let mut number_of_nodes = None;
                let mut owner_account = None;
                let mut port = None;
                let mut preferred_maintenance_window = None;
                let mut publicly_accessible = None;
                let mut snapshot_cluster_identifier = None;
                let mut snapshot_identifier = None;
                let mut tags = None;
                let mut vpc_security_group_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowVersionUpgrade" => {
                            allow_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutomatedSnapshotRetentionPeriod" => {
                            automated_snapshot_retention_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterIdentifier" => {
                            cluster_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterParameterGroupName" => {
                            cluster_parameter_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterSecurityGroups" => {
                            cluster_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterSubnetGroupName" => {
                            cluster_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterType" => {
                            cluster_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterVersion" => {
                            cluster_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBName" => {
                            db_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ElasticIp" => {
                            elastic_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Encrypted" => {
                            encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HsmClientCertificateIdentifier" => {
                            hsm_client_certificate_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HsmConfigurationIdentifier" => {
                            hsm_configuration_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IamRoles" => {
                            iam_roles = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoggingProperties" => {
                            logging_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUserPassword" => {
                            master_user_password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUsername" => {
                            master_username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NodeType" => {
                            node_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NumberOfNodes" => {
                            number_of_nodes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OwnerAccount" => {
                            owner_account = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotClusterIdentifier" => {
                            snapshot_cluster_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    allow_version_upgrade: allow_version_upgrade,
                    automated_snapshot_retention_period: automated_snapshot_retention_period,
                    availability_zone: availability_zone,
                    cluster_identifier: cluster_identifier,
                    cluster_parameter_group_name: cluster_parameter_group_name,
                    cluster_security_groups: cluster_security_groups,
                    cluster_subnet_group_name: cluster_subnet_group_name,
                    cluster_type: cluster_type.ok_or(::serde::de::Error::missing_field("ClusterType"))?,
                    cluster_version: cluster_version,
                    db_name: db_name.ok_or(::serde::de::Error::missing_field("DBName"))?,
                    elastic_ip: elastic_ip,
                    encrypted: encrypted,
                    hsm_client_certificate_identifier: hsm_client_certificate_identifier,
                    hsm_configuration_identifier: hsm_configuration_identifier,
                    iam_roles: iam_roles,
                    kms_key_id: kms_key_id,
                    logging_properties: logging_properties,
                    master_user_password: master_user_password.ok_or(::serde::de::Error::missing_field("MasterUserPassword"))?,
                    master_username: master_username.ok_or(::serde::de::Error::missing_field("MasterUsername"))?,
                    node_type: node_type.ok_or(::serde::de::Error::missing_field("NodeType"))?,
                    number_of_nodes: number_of_nodes,
                    owner_account: owner_account,
                    port: port,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    snapshot_cluster_identifier: snapshot_cluster_identifier,
                    snapshot_identifier: snapshot_identifier,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ClusterParameterGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `ParameterGroupFamily`.
    pub parameter_group_family: ::Value<String>,
    /// Property `Parameters`.
    pub parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupFamily", &self.parameter_group_family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut parameter_group_family = None;
                let mut parameters = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ParameterGroupFamily" => {
                            parameter_group_family = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    parameter_group_family: parameter_group_family.ok_or(::serde::de::Error::missing_field("ParameterGroupFamily"))?,
                    parameters: parameters,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ClusterSecurityGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterSecurityGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ClusterSecurityGroupIngressProperties {
    /// Property `CIDRIP`.
    pub cidrip: Option<::Value<String>>,
    /// Property `ClusterSecurityGroupName`.
    pub cluster_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupName`.
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property `EC2SecurityGroupOwnerId`.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ClusterSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", &self.cidrip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroupName", &self.cluster_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", &self.ec2_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", &self.ec2_security_group_owner_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidrip = None;
                let mut cluster_security_group_name = None;
                let mut ec2_security_group_name = None;
                let mut ec2_security_group_owner_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterSecurityGroupName" => {
                            cluster_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2SecurityGroupName" => {
                            ec2_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2SecurityGroupOwnerId" => {
                            ec2_security_group_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterSecurityGroupIngressProperties {
                    cidrip: cidrip,
                    cluster_security_group_name: cluster_security_group_name.ok_or(::serde::de::Error::missing_field("ClusterSecurityGroupName"))?,
                    ec2_security_group_name: ec2_security_group_name,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ClusterSubnetGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `SubnetIds`.
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut subnet_ids = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetIds" => {
                            subnet_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterSubnetGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
    #[derive(Debug)]
    pub struct LoggingProperties {
        /// Property `BucketName`.
        pub bucket_name: ::Value<String>,
        /// Property `S3KeyPrefix`.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", &self.s3_key_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name = None;
                    let mut s3_key_prefix = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        s3_key_prefix: s3_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cluster_parameter_group {
    //! Property types for the `ClusterParameterGroup` resource.

    /// The [`AWS::Redshift::ClusterParameterGroup.Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-property-redshift-clusterparametergroup-parameter.html) property type.
    #[derive(Debug)]
    pub struct Parameter {
        /// Property `ParameterName`.
        pub parameter_name: ::Value<String>,
        /// Property `ParameterValue`.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Parameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name = None;
                    let mut parameter_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ParameterValue" => {
                                parameter_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
