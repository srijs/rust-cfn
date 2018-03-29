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
    #[serde(rename="AZMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub az_mode: Option<String>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// Property `CacheNodeType`.
    #[serde(rename="CacheNodeType")]
    pub cache_node_type: String,
    /// Property `CacheParameterGroupName`.
    #[serde(rename="CacheParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename="CacheSecurityGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<Vec<String>>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename="CacheSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    /// Property `ClusterName`.
    #[serde(rename="ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    pub engine: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// Property `NotificationTopicArn`.
    #[serde(rename="NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// Property `NumCacheNodes`.
    #[serde(rename="NumCacheNodes")]
    pub num_cache_nodes: u32,
    /// Property `Port`.
    #[serde(rename="Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    /// Property `PreferredAvailabilityZone`.
    #[serde(rename="PreferredAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,
    /// Property `PreferredAvailabilityZones`.
    #[serde(rename="PreferredAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<Vec<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// Property `SnapshotArns`.
    #[serde(rename="SnapshotArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<Vec<String>>,
    /// Property `SnapshotName`.
    #[serde(rename="SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename="SnapshotRetentionLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<u32>,
    /// Property `SnapshotWindow`.
    #[serde(rename="SnapshotWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
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
    #[serde(rename="CacheParameterGroupFamily")]
    pub cache_parameter_group_family: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Properties`.
    #[serde(rename="Properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
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
    #[serde(rename="AtRestEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    /// Property `AuthToken`.
    #[serde(rename="AuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// Property `AutomaticFailoverEnabled`.
    #[serde(rename="AutomaticFailoverEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover_enabled: Option<bool>,
    /// Property `CacheNodeType`.
    #[serde(rename="CacheNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    /// Property `CacheParameterGroupName`.
    #[serde(rename="CacheParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename="CacheSecurityGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<Vec<String>>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename="CacheSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// Property `NodeGroupConfiguration`.
    #[serde(rename="NodeGroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_configuration: Option<Vec<self::replication_group::NodeGroupConfiguration>>,
    /// Property `NotificationTopicArn`.
    #[serde(rename="NotificationTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    /// Property `NumCacheClusters`.
    #[serde(rename="NumCacheClusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_clusters: Option<u32>,
    /// Property `NumNodeGroups`.
    #[serde(rename="NumNodeGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_node_groups: Option<u32>,
    /// Property `Port`.
    #[serde(rename="Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u32>,
    /// Property `PreferredCacheClusterAZs`.
    #[serde(rename="PreferredCacheClusterAZs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_cache_cluster_a_zs: Option<Vec<String>>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// Property `PrimaryClusterId`.
    #[serde(rename="PrimaryClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cluster_id: Option<String>,
    /// Property `ReplicasPerNodeGroup`.
    #[serde(rename="ReplicasPerNodeGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas_per_node_group: Option<u32>,
    /// Property `ReplicationGroupDescription`.
    #[serde(rename="ReplicationGroupDescription")]
    pub replication_group_description: String,
    /// Property `ReplicationGroupId`.
    #[serde(rename="ReplicationGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    /// Property `SecurityGroupIds`.
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// Property `SnapshotArns`.
    #[serde(rename="SnapshotArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<Vec<String>>,
    /// Property `SnapshotName`.
    #[serde(rename="SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename="SnapshotRetentionLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<u32>,
    /// Property `SnapshotWindow`.
    #[serde(rename="SnapshotWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    /// Property `SnapshottingClusterId`.
    #[serde(rename="SnapshottingClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshotting_cluster_id: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `TransitEncryptionEnabled`.
    #[serde(rename="TransitEncryptionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
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
    #[serde(rename="Description")]
    pub description: String,
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
    #[serde(rename="CacheSecurityGroupName")]
    pub cache_security_group_name: String,
    /// Property `EC2SecurityGroupName`.
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: String,
    /// Property `EC2SecurityGroupOwnerId`.
    #[serde(rename="EC2SecurityGroupOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<String>,
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
    #[serde(rename="CacheSubnetGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `SubnetIds`.
    #[serde(rename="SubnetIds")]
    pub subnet_ids: Vec<String>,
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
        #[serde(rename="PrimaryAvailabilityZone")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub primary_availability_zone: Option<String>,
        /// Property `ReplicaAvailabilityZones`.
        #[serde(rename="ReplicaAvailabilityZones")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub replica_availability_zones: Option<Vec<String>>,
        /// Property `ReplicaCount`.
        #[serde(rename="ReplicaCount")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub replica_count: Option<u32>,
        /// Property `Slots`.
        #[serde(rename="Slots")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub slots: Option<String>,
    }
}
