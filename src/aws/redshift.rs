//! Types for the `Redshift` service.

/// The [`AWS::Redshift::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`AllowVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-allowversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_version_upgrade: Option<::Value<bool>>,
    /// Property [`AutomatedSnapshotRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-automatedsnapshotretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automated_snapshot_retention_period: Option<::Value<u32>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_identifier: Option<::Value<String>>,
    /// Property [`ClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusterparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_parameter_group_name: Option<::Value<String>>,
    /// Property [`ClusterSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustersecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_security_groups: Option<::ValueList<String>>,
    /// Property [`ClusterSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustersubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_subnet_group_name: Option<::Value<String>>,
    /// Property [`ClusterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_type: ::Value<String>,
    /// Property [`ClusterVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusterversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_version: Option<::Value<String>>,
    /// Property [`DBName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-dbname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_name: ::Value<String>,
    /// Property [`ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-elasticip).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub elastic_ip: Option<::Value<String>>,
    /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-encrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encrypted: Option<::Value<bool>>,
    /// Property [`HsmClientCertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-hsmclientcertidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hsm_client_certificate_identifier: Option<::Value<String>>,
    /// Property [`HsmConfigurationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-HsmConfigurationIdentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hsm_configuration_identifier: Option<::Value<String>>,
    /// Property [`IamRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-iamroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_roles: Option<::ValueList<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LoggingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-loggingproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_properties: Option<::Value<self::cluster::LoggingProperties>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: ::Value<String>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: ::Value<String>,
    /// Property [`NodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-nodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_type: ::Value<String>,
    /// Property [`NumberOfNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-nodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property [`OwnerAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-owneraccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub owner_account: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`SnapshotClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_cluster_identifier: Option<::Value<String>>,
    /// Property [`SnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_version_upgrade) = self.allow_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowVersionUpgrade", allow_version_upgrade)?;
        }
        if let Some(ref automated_snapshot_retention_period) = self.automated_snapshot_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomatedSnapshotRetentionPeriod", automated_snapshot_retention_period)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref cluster_identifier) = self.cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", cluster_identifier)?;
        }
        if let Some(ref cluster_parameter_group_name) = self.cluster_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterParameterGroupName", cluster_parameter_group_name)?;
        }
        if let Some(ref cluster_security_groups) = self.cluster_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroups", cluster_security_groups)?;
        }
        if let Some(ref cluster_subnet_group_name) = self.cluster_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSubnetGroupName", cluster_subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterType", &self.cluster_type)?;
        if let Some(ref cluster_version) = self.cluster_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterVersion", cluster_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", &self.db_name)?;
        if let Some(ref elastic_ip) = self.elastic_ip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIp", elastic_ip)?;
        }
        if let Some(ref encrypted) = self.encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
        }
        if let Some(ref hsm_client_certificate_identifier) = self.hsm_client_certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmClientCertificateIdentifier", hsm_client_certificate_identifier)?;
        }
        if let Some(ref hsm_configuration_identifier) = self.hsm_configuration_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmConfigurationIdentifier", hsm_configuration_identifier)?;
        }
        if let Some(ref iam_roles) = self.iam_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", iam_roles)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref logging_properties) = self.logging_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingProperties", logging_properties)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", &self.master_user_password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeType", &self.node_type)?;
        if let Some(ref number_of_nodes) = self.number_of_nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", number_of_nodes)?;
        }
        if let Some(ref owner_account) = self.owner_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerAccount", owner_account)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref snapshot_cluster_identifier) = self.snapshot_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotClusterIdentifier", snapshot_cluster_identifier)?;
        }
        if let Some(ref snapshot_identifier) = self.snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", snapshot_identifier)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
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
                let mut allow_version_upgrade: Option<::Value<bool>> = None;
                let mut automated_snapshot_retention_period: Option<::Value<u32>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut cluster_identifier: Option<::Value<String>> = None;
                let mut cluster_parameter_group_name: Option<::Value<String>> = None;
                let mut cluster_security_groups: Option<::ValueList<String>> = None;
                let mut cluster_subnet_group_name: Option<::Value<String>> = None;
                let mut cluster_type: Option<::Value<String>> = None;
                let mut cluster_version: Option<::Value<String>> = None;
                let mut db_name: Option<::Value<String>> = None;
                let mut elastic_ip: Option<::Value<String>> = None;
                let mut encrypted: Option<::Value<bool>> = None;
                let mut hsm_client_certificate_identifier: Option<::Value<String>> = None;
                let mut hsm_configuration_identifier: Option<::Value<String>> = None;
                let mut iam_roles: Option<::ValueList<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut logging_properties: Option<::Value<self::cluster::LoggingProperties>> = None;
                let mut master_user_password: Option<::Value<String>> = None;
                let mut master_username: Option<::Value<String>> = None;
                let mut node_type: Option<::Value<String>> = None;
                let mut number_of_nodes: Option<::Value<u32>> = None;
                let mut owner_account: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut snapshot_cluster_identifier: Option<::Value<String>> = None;
                let mut snapshot_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowVersionUpgrade" => {
                            allow_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutomatedSnapshotRetentionPeriod" => {
                            automated_snapshot_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterIdentifier" => {
                            cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterParameterGroupName" => {
                            cluster_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSecurityGroups" => {
                            cluster_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSubnetGroupName" => {
                            cluster_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterType" => {
                            cluster_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterVersion" => {
                            cluster_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticIp" => {
                            elastic_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Encrypted" => {
                            encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HsmClientCertificateIdentifier" => {
                            hsm_client_certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HsmConfigurationIdentifier" => {
                            hsm_configuration_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoles" => {
                            iam_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingProperties" => {
                            logging_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeType" => {
                            node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfNodes" => {
                            number_of_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerAccount" => {
                            owner_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotClusterIdentifier" => {
                            snapshot_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Cluster {
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
#[derive(Debug, Default)]
pub struct ClusterParameterGroup {
    properties: ClusterParameterGroupProperties
}

/// Properties for the `ClusterParameterGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterParameterGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`ParameterGroupFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-parametergroupfamily).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parameter_group_family: ::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupFamily", &self.parameter_group_family)?;
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut parameter_group_family: Option<::Value<String>> = None;
                let mut parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterGroupFamily" => {
                            parameter_group_family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ClusterParameterGroup {
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
#[derive(Debug, Default)]
pub struct ClusterSecurityGroup {
    properties: ClusterSecurityGroupProperties
}

/// Properties for the `ClusterSecurityGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html#cfn-redshift-clustersecuritygroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html#cfn-redshift-clustersecuritygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ClusterSecurityGroup {
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
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupIngress {
    properties: ClusterSecurityGroupIngressProperties
}

/// Properties for the `ClusterSecurityGroupIngress` resource.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupIngressProperties {
    /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-cidrip).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cidrip: Option<::Value<String>>,
    /// Property [`ClusterSecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-clustersecuritygroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_security_group_name: ::Value<String>,
    /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-ec2securitygroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-ec2securitygroupownerid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ClusterSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cidrip) = self.cidrip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroupName", &self.cluster_security_group_name)?;
        if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
        }
        if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
        }
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
                let mut cidrip: Option<::Value<String>> = None;
                let mut cluster_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_owner_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSecurityGroupName" => {
                            cluster_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupName" => {
                            ec2_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupOwnerId" => {
                            ec2_security_group_owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ClusterSecurityGroupIngress {
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
#[derive(Debug, Default)]
pub struct ClusterSubnetGroup {
    properties: ClusterSubnetGroupProperties
}

/// Properties for the `ClusterSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterSubnetGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ClusterSubnetGroup {
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
    #[derive(Debug, Default)]
    pub struct LoggingProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html#cfn-redshift-cluster-loggingproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html#cfn-redshift-cluster-loggingproperties-s3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref s3_key_prefix) = self.s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
            }
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
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut s3_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Parameter {
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-property-redshift-clusterparametergroup-parameter.html#cfn-redshift-clusterparametergroup-parameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-property-redshift-clusterparametergroup-parameter.html#cfn-redshift-clusterparametergroup-parameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
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
