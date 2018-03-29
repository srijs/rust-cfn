/// The [`AWS::ElastiCache::CacheCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct CacheCluster {
    properties: CacheClusterProperties
}

/// Properties for the `CacheCluster` resource.
#[derive(Serialize, Deserialize)]
pub struct CacheClusterProperties {
    #[serde(rename="AZMode")]
    pub az_mode: String,
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    #[serde(rename="CacheNodeType")]
    pub cache_node_type: String,
    #[serde(rename="CacheParameterGroupName")]
    pub cache_parameter_group_name: String,
    #[serde(rename="CacheSecurityGroupNames")]
    pub cache_security_group_names: Vec<String>,
    #[serde(rename="CacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    #[serde(rename="ClusterName")]
    pub cluster_name: String,
    #[serde(rename="Engine")]
    pub engine: String,
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    #[serde(rename="NotificationTopicArn")]
    pub notification_topic_arn: String,
    #[serde(rename="NumCacheNodes")]
    pub num_cache_nodes: u32,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="PreferredAvailabilityZone")]
    pub preferred_availability_zone: String,
    #[serde(rename="PreferredAvailabilityZones")]
    pub preferred_availability_zones: Vec<String>,
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    #[serde(rename="SnapshotArns")]
    pub snapshot_arns: Vec<String>,
    #[serde(rename="SnapshotName")]
    pub snapshot_name: String,
    #[serde(rename="SnapshotRetentionLimit")]
    pub snapshot_retention_limit: u32,
    #[serde(rename="SnapshotWindow")]
    pub snapshot_window: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<CacheClusterProperties> for CacheCluster {
    fn from(properties: CacheClusterProperties) -> CacheCluster {
        CacheCluster { properties }
    }
}

/// The [`AWS::ElastiCache::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ParameterGroupProperties {
    #[serde(rename="CacheParameterGroupFamily")]
    pub cache_parameter_group_family: String,
    #[serde(rename="Description")]
    pub description: String,
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

impl From<ParameterGroupProperties> for ParameterGroup {
    fn from(properties: ParameterGroupProperties) -> ParameterGroup {
        ParameterGroup { properties }
    }
}

/// The [`AWS::ElastiCache::ReplicationGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationGroup {
    properties: ReplicationGroupProperties
}

/// Properties for the `ReplicationGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ReplicationGroupProperties {
    #[serde(rename="AtRestEncryptionEnabled")]
    pub at_rest_encryption_enabled: bool,
    #[serde(rename="AuthToken")]
    pub auth_token: String,
    #[serde(rename="AutoMinorVersionUpgrade")]
    pub auto_minor_version_upgrade: bool,
    #[serde(rename="AutomaticFailoverEnabled")]
    pub automatic_failover_enabled: bool,
    #[serde(rename="CacheNodeType")]
    pub cache_node_type: String,
    #[serde(rename="CacheParameterGroupName")]
    pub cache_parameter_group_name: String,
    #[serde(rename="CacheSecurityGroupNames")]
    pub cache_security_group_names: Vec<String>,
    #[serde(rename="CacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    #[serde(rename="Engine")]
    pub engine: String,
    #[serde(rename="EngineVersion")]
    pub engine_version: String,
    #[serde(rename="NodeGroupConfiguration")]
    pub node_group_configuration: Vec<self::replication_group::NodeGroupConfiguration>,
    #[serde(rename="NotificationTopicArn")]
    pub notification_topic_arn: String,
    #[serde(rename="NumCacheClusters")]
    pub num_cache_clusters: u32,
    #[serde(rename="NumNodeGroups")]
    pub num_node_groups: u32,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="PreferredCacheClusterAZs")]
    pub preferred_cache_cluster_a_zs: Vec<String>,
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: String,
    #[serde(rename="PrimaryClusterId")]
    pub primary_cluster_id: String,
    #[serde(rename="ReplicasPerNodeGroup")]
    pub replicas_per_node_group: u32,
    #[serde(rename="ReplicationGroupDescription")]
    pub replication_group_description: String,
    #[serde(rename="ReplicationGroupId")]
    pub replication_group_id: String,
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    #[serde(rename="SnapshotArns")]
    pub snapshot_arns: Vec<String>,
    #[serde(rename="SnapshotName")]
    pub snapshot_name: String,
    #[serde(rename="SnapshotRetentionLimit")]
    pub snapshot_retention_limit: u32,
    #[serde(rename="SnapshotWindow")]
    pub snapshot_window: String,
    #[serde(rename="SnapshottingClusterId")]
    pub snapshotting_cluster_id: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<ReplicationGroupProperties> for ReplicationGroup {
    fn from(properties: ReplicationGroupProperties) -> ReplicationGroup {
        ReplicationGroup { properties }
    }
}

/// The [`AWS::ElastiCache::SecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroup {
    properties: SecurityGroupProperties
}

/// Properties for the `SecurityGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupProperties {
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

impl From<SecurityGroupProperties> for SecurityGroup {
    fn from(properties: SecurityGroupProperties) -> SecurityGroup {
        SecurityGroup { properties }
    }
}

/// The [`AWS::ElastiCache::SecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupIngress {
    properties: SecurityGroupIngressProperties
}

/// Properties for the `SecurityGroupIngress` resource.
#[derive(Serialize, Deserialize)]
pub struct SecurityGroupIngressProperties {
    #[serde(rename="CacheSecurityGroupName")]
    pub cache_security_group_name: String,
    #[serde(rename="EC2SecurityGroupName")]
    pub ec2_security_group_name: String,
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

impl From<SecurityGroupIngressProperties> for SecurityGroupIngress {
    fn from(properties: SecurityGroupIngressProperties) -> SecurityGroupIngress {
        SecurityGroupIngress { properties }
    }
}

/// The [`AWS::ElastiCache::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-subnetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetGroupProperties {
    #[serde(rename="CacheSubnetGroupName")]
    pub cache_subnet_group_name: String,
    #[serde(rename="Description")]
    pub description: String,
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

impl From<SubnetGroupProperties> for SubnetGroup {
    fn from(properties: SubnetGroupProperties) -> SubnetGroup {
        SubnetGroup { properties }
    }
}

pub mod replication_group {
    #[derive(Serialize, Deserialize)]
    pub struct NodeGroupConfiguration {
        #[serde(rename="PrimaryAvailabilityZone")]
        pub primary_availability_zone: String,
        #[serde(rename="ReplicaAvailabilityZones")]
        pub replica_availability_zones: Vec<String>,
        #[serde(rename="ReplicaCount")]
        pub replica_count: u32,
        #[serde(rename="Slots")]
        pub slots: String,
    }

}

