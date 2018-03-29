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
    pub az_mode: String,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    /// Property `CacheNodeType`.
    #[serde(rename="CacheNodeType")]
    pub cache_node_type: String,
    /// Property `CacheParameterGroupName`.
    #[serde(rename="CacheParameterGroupName")]
    pub cache_parameter_group_name: String,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename="CacheSecurityGroupNames")]
    pub cache_security_group_names: Vec<String>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename="CacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    /// Property `ClusterName`.
    #[serde(rename="ClusterName")]
    pub cluster_name: String,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    pub engine: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    /// Property `NotificationTopicArn`.
    #[serde(rename="NotificationTopicArn")]
    pub notification_topic_arn: String,
    /// Property `NumCacheNodes`.
    #[serde(rename="NumCacheNodes")]
    pub num_cache_nodes: u32,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `PreferredAvailabilityZone`.
    #[serde(rename="PreferredAvailabilityZone")]
    pub preferred_availability_zone: String,
    /// Property `PreferredAvailabilityZones`.
    #[serde(rename="PreferredAvailabilityZones")]
    pub preferred_availability_zones: Vec<String>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `SnapshotArns`.
    #[serde(rename="SnapshotArns")]
    pub snapshot_arns: Vec<String>,
    /// Property `SnapshotName`.
    #[serde(rename="SnapshotName")]
    pub snapshot_name: String,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename="SnapshotRetentionLimit")]
    pub snapshot_retention_limit: u32,
    /// Property `SnapshotWindow`.
    #[serde(rename="SnapshotWindow")]
    pub snapshot_window: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VpcSecurityGroupIds`.
    #[serde(rename="VpcSecurityGroupIds")]
    pub vpc_security_group_ids: Vec<String>,
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
    pub properties: ::std::collections::HashMap<String, String>,
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
    pub at_rest_encryption_enabled: bool,
    /// Property `AuthToken`.
    #[serde(rename="AuthToken")]
    pub auth_token: String,
    /// Property `AutoMinorVersionUpgrade`.
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    /// Property `AutomaticFailoverEnabled`.
    #[serde(rename="AutomaticFailoverEnabled")]
    pub automatic_failover_enabled: bool,
    /// Property `CacheNodeType`.
    #[serde(rename="CacheNodeType")]
    pub cache_node_type: String,
    /// Property `CacheParameterGroupName`.
    #[serde(rename="CacheParameterGroupName")]
    pub cache_parameter_group_name: String,
    /// Property `CacheSecurityGroupNames`.
    #[serde(rename="CacheSecurityGroupNames")]
    pub cache_security_group_names: Vec<String>,
    /// Property `CacheSubnetGroupName`.
    #[serde(rename="CacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    /// Property `Engine`.
    #[serde(rename="Engine")]
    pub engine: String,
    /// Property `EngineVersion`.
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    /// Property `NodeGroupConfiguration`.
    #[serde(rename="NodeGroupConfiguration")]
    pub node_group_configuration: Vec<self::replication_group::NodeGroupConfiguration>,
    /// Property `NotificationTopicArn`.
    #[serde(rename="NotificationTopicArn")]
    pub notification_topic_arn: String,
    /// Property `NumCacheClusters`.
    #[serde(rename="NumCacheClusters")]
    pub num_cache_clusters: u32,
    /// Property `NumNodeGroups`.
    #[serde(rename="NumNodeGroups")]
    pub num_node_groups: u32,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `PreferredCacheClusterAZs`.
    #[serde(rename="PreferredCacheClusterAZs")]
    pub preferred_cache_cluster_a_zs: Vec<String>,
    /// Property `PreferredMaintenanceWindow`.
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    /// Property `PrimaryClusterId`.
    #[serde(rename="PrimaryClusterId")]
    pub primary_cluster_id: String,
    /// Property `ReplicasPerNodeGroup`.
    #[serde(rename="ReplicasPerNodeGroup")]
    pub replicas_per_node_group: u32,
    /// Property `ReplicationGroupDescription`.
    #[serde(rename="ReplicationGroupDescription")]
    pub replication_group_description: String,
    /// Property `ReplicationGroupId`.
    #[serde(rename="ReplicationGroupId")]
    pub replication_group_id: String,
    /// Property `SecurityGroupIds`.
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// Property `SnapshotArns`.
    #[serde(rename="SnapshotArns")]
    pub snapshot_arns: Vec<String>,
    /// Property `SnapshotName`.
    #[serde(rename="SnapshotName")]
    pub snapshot_name: String,
    /// Property `SnapshotRetentionLimit`.
    #[serde(rename="SnapshotRetentionLimit")]
    pub snapshot_retention_limit: u32,
    /// Property `SnapshotWindow`.
    #[serde(rename="SnapshotWindow")]
    pub snapshot_window: String,
    /// Property `SnapshottingClusterId`.
    #[serde(rename="SnapshottingClusterId")]
    pub snapshotting_cluster_id: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `TransitEncryptionEnabled`.
    #[serde(rename="TransitEncryptionEnabled")]
    pub transit_encryption_enabled: bool,
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
    pub ec2_security_group_owner_id: String,
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
    pub cache_subnet_group_name: String,
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
        pub primary_availability_zone: String,
        /// Property `ReplicaAvailabilityZones`.
        #[serde(rename="ReplicaAvailabilityZones")]
        pub replica_availability_zones: Vec<String>,
        /// Property `ReplicaCount`.
        #[serde(rename="ReplicaCount")]
        pub replica_count: u32,
        /// Property `Slots`.
        #[serde(rename="Slots")]
        pub slots: String,
    }
}
