//! Types for the `ElastiCache` service.

/// The [`AWS::ElastiCache::CacheCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html) resource type.
#[derive(Debug)]
pub struct CacheCluster {
    properties: CacheClusterProperties
}

/// Properties for the `CacheCluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheClusterProperties {
    /// Property `AZMode`.
    #[serde(rename = "AZMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub az_mode: Option<::Value<String>>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `CacheNodeType`.
    #[serde(rename = "CacheNodeType")]
    pub cache_node_type: ::Value<String>,
    /// Property `CacheParameterGroupName`.
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `ClusterName`.
    #[serde(rename = "ClusterName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<::Value<String>>,
    /// Property `Engine`.
    #[serde(rename = "Engine")]
    pub engine: ::Value<String>,
    /// Property `EngineVersion`.
    #[serde(rename = "EngineVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<::Value<String>>,
    /// Property `NotificationTopicArn`.
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property `NumCacheNodes`.
    #[serde(rename = "NumCacheNodes")]
    pub num_cache_nodes: ::Value<u32>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<u32>>,
    /// Property `PreferredAvailabilityZone`.
    #[serde(rename = "PreferredAvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<::Value<String>>,
    /// Property `PreferredAvailabilityZones`.
    #[serde(rename = "PreferredAvailabilityZones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<::ValueList<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `SnapshotArns`.
    #[serde(rename = "SnapshotArns")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property `SnapshotName`.
    #[serde(rename = "SnapshotName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<::Value<String>>,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property `SnapshotWindow`.
    #[serde(rename = "SnapshotWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl<'a> ::Resource<'a> for CacheCluster {
    type Properties = CacheClusterProperties;
    const TYPE: &'static str = "AWS::ElastiCache::CacheCluster";
    fn properties(&self) -> &CacheClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CacheClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CacheCluster {}

impl From<CacheClusterProperties> for CacheCluster {
    fn from(properties: CacheClusterProperties) -> CacheCluster {
        CacheCluster { properties }
    }
}

/// The [`AWS::ElastiCache::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html) resource type.
#[derive(Debug)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterGroupProperties {
    /// Property `CacheParameterGroupFamily`.
    #[serde(rename = "CacheParameterGroupFamily")]
    pub cache_parameter_group_family: ::Value<String>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `Properties`.
    #[serde(rename = "Properties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, ::Value<String>>>,
}

impl<'a> ::Resource<'a> for ParameterGroup {
    type Properties = ParameterGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::ParameterGroup";
    fn properties(&self) -> &ParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ParameterGroup {}

impl From<ParameterGroupProperties> for ParameterGroup {
    fn from(properties: ParameterGroupProperties) -> ParameterGroup {
        ParameterGroup { properties }
    }
}

/// The [`AWS::ElastiCache::ReplicationGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html) resource type.
#[derive(Debug)]
pub struct ReplicationGroup {
    properties: ReplicationGroupProperties
}

/// Properties for the `ReplicationGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicationGroupProperties {
    /// Property `AtRestEncryptionEnabled`.
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<::Value<bool>>,
    /// Property `AuthToken`.
    #[serde(rename = "AuthToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<::Value<String>>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `AutomaticFailoverEnabled`.
    #[serde(rename = "AutomaticFailoverEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_failover_enabled: Option<::Value<bool>>,
    /// Property `CacheNodeType`.
    #[serde(rename = "CacheNodeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<::Value<String>>,
    /// Property `CacheParameterGroupName`.
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `Engine`.
    #[serde(rename = "Engine")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<::Value<String>>,
    /// Property `EngineVersion`.
    #[serde(rename = "EngineVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<::Value<String>>,
    /// Property `NodeGroupConfiguration`.
    #[serde(rename = "NodeGroupConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_group_configuration: Option<::ValueList<self::replication_group::NodeGroupConfiguration>>,
    /// Property `NotificationTopicArn`.
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property `NumCacheClusters`.
    #[serde(rename = "NumCacheClusters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_cache_clusters: Option<::Value<u32>>,
    /// Property `NumNodeGroups`.
    #[serde(rename = "NumNodeGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_node_groups: Option<::Value<u32>>,
    /// Property `Port`.
    #[serde(rename = "Port")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<::Value<u32>>,
    /// Property `PreferredCacheClusterAZs`.
    #[serde(rename = "PreferredCacheClusterAZs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_cache_cluster_a_zs: Option<::ValueList<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PrimaryClusterId`.
    #[serde(rename = "PrimaryClusterId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_cluster_id: Option<::Value<String>>,
    /// Property `ReplicasPerNodeGroup`.
    #[serde(rename = "ReplicasPerNodeGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas_per_node_group: Option<::Value<u32>>,
    /// Property `ReplicationGroupDescription`.
    #[serde(rename = "ReplicationGroupDescription")]
    pub replication_group_description: ::Value<String>,
    /// Property `ReplicationGroupId`.
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<::Value<String>>,
    /// Property `SecurityGroupIds`.
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SnapshotArns`.
    #[serde(rename = "SnapshotArns")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property `SnapshotName`.
    #[serde(rename = "SnapshotName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<::Value<String>>,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property `SnapshotWindow`.
    #[serde(rename = "SnapshotWindow")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<::Value<String>>,
    /// Property `SnapshottingClusterId`.
    #[serde(rename = "SnapshottingClusterId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshotting_cluster_id: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TransitEncryptionEnabled`.
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<::Value<bool>>,
}

impl<'a> ::Resource<'a> for ReplicationGroup {
    type Properties = ReplicationGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::ReplicationGroup";
    fn properties(&self) -> &ReplicationGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationGroup {}

impl From<ReplicationGroupProperties> for ReplicationGroup {
    fn from(properties: ReplicationGroupProperties) -> ReplicationGroup {
        ReplicationGroup { properties }
    }
}

/// The [`AWS::ElastiCache::SecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group.html) resource type.
#[derive(Debug)]
pub struct SecurityGroup {
    properties: SecurityGroupProperties
}

/// Properties for the `SecurityGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
}

impl<'a> ::Resource<'a> for SecurityGroup {
    type Properties = SecurityGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::SecurityGroup";
    fn properties(&self) -> &SecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroup {}

impl From<SecurityGroupProperties> for SecurityGroup {
    fn from(properties: SecurityGroupProperties) -> SecurityGroup {
        SecurityGroup { properties }
    }
}

/// The [`AWS::ElastiCache::SecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html) resource type.
#[derive(Debug)]
pub struct SecurityGroupIngress {
    properties: SecurityGroupIngressProperties
}

/// Properties for the `SecurityGroupIngress` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityGroupIngressProperties {
    /// Property `CacheSecurityGroupName`.
    #[serde(rename = "CacheSecurityGroupName")]
    pub cache_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename = "EC2SecurityGroupName")]
    pub ec2_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for SecurityGroupIngress {
    type Properties = SecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::ElastiCache::SecurityGroupIngress";
    fn properties(&self) -> &SecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityGroupIngress {}

impl From<SecurityGroupIngressProperties> for SecurityGroupIngress {
    fn from(properties: SecurityGroupIngressProperties) -> SecurityGroupIngress {
        SecurityGroupIngress { properties }
    }
}

/// The [`AWS::ElastiCache::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-subnetgroup.html) resource type.
#[derive(Debug)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetGroupProperties {
    /// Property `CacheSubnetGroupName`.
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    pub description: ::Value<String>,
    /// Property `SubnetIds`.
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: ::ValueList<String>,
}

impl<'a> ::Resource<'a> for SubnetGroup {
    type Properties = SubnetGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::SubnetGroup";
    fn properties(&self) -> &SubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubnetGroup {}

impl From<SubnetGroupProperties> for SubnetGroup {
    fn from(properties: SubnetGroupProperties) -> SubnetGroup {
        SubnetGroup { properties }
    }
}

pub mod replication_group {
    //! Property types for the `ReplicationGroup` resource.

    /// The [`AWS::ElastiCache::ReplicationGroup.NodeGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NodeGroupConfiguration {
        /// Property `PrimaryAvailabilityZone`.
        #[serde(rename = "PrimaryAvailabilityZone")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub primary_availability_zone: Option<::Value<String>>,
        /// Property `ReplicaAvailabilityZones`.
        #[serde(rename = "ReplicaAvailabilityZones")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replica_availability_zones: Option<::ValueList<String>>,
        /// Property `ReplicaCount`.
        #[serde(rename = "ReplicaCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub replica_count: Option<::Value<u32>>,
        /// Property `Slots`.
        #[serde(rename = "Slots")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slots: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(NodeGroupConfiguration);
}
