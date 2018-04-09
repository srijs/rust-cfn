//! Types for the `ElastiCache` service.

/// The [`AWS::ElastiCache::CacheCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html) resource type.
#[derive(Debug)]
pub struct CacheCluster {
    properties: CacheClusterProperties
}

/// Properties for the `CacheCluster` resource.
#[derive(Debug)]
pub struct CacheClusterProperties {
    /// Property `AZMode`.
    pub az_mode: Option<::Value<String>>,
    /// Property `AutoMinorVersionUpgrade`.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `CacheNodeType`.
    pub cache_node_type: ::Value<String>,
    /// Property `CacheParameterGroupName`.
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property `CacheSecurityGroupNames`.
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property `CacheSubnetGroupName`.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `ClusterName`.
    pub cluster_name: Option<::Value<String>>,
    /// Property `Engine`.
    pub engine: ::Value<String>,
    /// Property `EngineVersion`.
    pub engine_version: Option<::Value<String>>,
    /// Property `NotificationTopicArn`.
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property `NumCacheNodes`.
    pub num_cache_nodes: ::Value<u32>,
    /// Property `Port`.
    pub port: Option<::Value<u32>>,
    /// Property `PreferredAvailabilityZone`.
    pub preferred_availability_zone: Option<::Value<String>>,
    /// Property `PreferredAvailabilityZones`.
    pub preferred_availability_zones: Option<::ValueList<String>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `SnapshotArns`.
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property `SnapshotName`.
    pub snapshot_name: Option<::Value<String>>,
    /// Property `SnapshotRetentionLimit`.
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property `SnapshotWindow`.
    pub snapshot_window: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for CacheClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AZMode", &self.az_mode)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", &self.auto_minor_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNodeType", &self.cache_node_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupName", &self.cache_parameter_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupNames", &self.cache_security_group_names)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", &self.cache_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicArn", &self.notification_topic_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumCacheNodes", &self.num_cache_nodes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredAvailabilityZone", &self.preferred_availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredAvailabilityZones", &self.preferred_availability_zones)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotArns", &self.snapshot_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotName", &self.snapshot_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotRetentionLimit", &self.snapshot_retention_limit)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotWindow", &self.snapshot_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", &self.vpc_security_group_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CacheClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CacheClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CacheClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut az_mode = None;
                let mut auto_minor_version_upgrade = None;
                let mut cache_node_type = None;
                let mut cache_parameter_group_name = None;
                let mut cache_security_group_names = None;
                let mut cache_subnet_group_name = None;
                let mut cluster_name = None;
                let mut engine = None;
                let mut engine_version = None;
                let mut notification_topic_arn = None;
                let mut num_cache_nodes = None;
                let mut port = None;
                let mut preferred_availability_zone = None;
                let mut preferred_availability_zones = None;
                let mut preferred_maintenance_window = None;
                let mut snapshot_arns = None;
                let mut snapshot_name = None;
                let mut snapshot_retention_limit = None;
                let mut snapshot_window = None;
                let mut tags = None;
                let mut vpc_security_group_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AZMode" => {
                            az_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheNodeType" => {
                            cache_node_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheParameterGroupName" => {
                            cache_parameter_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheSecurityGroupNames" => {
                            cache_security_group_names = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClusterName" => {
                            cluster_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Engine" => {
                            engine = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineVersion" => {
                            engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NotificationTopicArn" => {
                            notification_topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NumCacheNodes" => {
                            num_cache_nodes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredAvailabilityZone" => {
                            preferred_availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredAvailabilityZones" => {
                            preferred_availability_zones = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotArns" => {
                            snapshot_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotName" => {
                            snapshot_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotRetentionLimit" => {
                            snapshot_retention_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotWindow" => {
                            snapshot_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

                Ok(CacheClusterProperties {
                    az_mode: az_mode,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    cache_node_type: cache_node_type.ok_or(::serde::de::Error::missing_field("CacheNodeType"))?,
                    cache_parameter_group_name: cache_parameter_group_name,
                    cache_security_group_names: cache_security_group_names,
                    cache_subnet_group_name: cache_subnet_group_name,
                    cluster_name: cluster_name,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    engine_version: engine_version,
                    notification_topic_arn: notification_topic_arn,
                    num_cache_nodes: num_cache_nodes.ok_or(::serde::de::Error::missing_field("NumCacheNodes"))?,
                    port: port,
                    preferred_availability_zone: preferred_availability_zone,
                    preferred_availability_zones: preferred_availability_zones,
                    preferred_maintenance_window: preferred_maintenance_window,
                    snapshot_arns: snapshot_arns,
                    snapshot_name: snapshot_name,
                    snapshot_retention_limit: snapshot_retention_limit,
                    snapshot_window: snapshot_window,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CacheCluster {
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
#[derive(Debug)]
pub struct ParameterGroupProperties {
    /// Property `CacheParameterGroupFamily`.
    pub cache_parameter_group_family: ::Value<String>,
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `Properties`.
    pub properties: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupFamily", &self.cache_parameter_group_family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", &self.properties)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_parameter_group_family = None;
                let mut description = None;
                let mut properties = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheParameterGroupFamily" => {
                            cache_parameter_group_family = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Properties" => {
                            properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ParameterGroupProperties {
                    cache_parameter_group_family: cache_parameter_group_family.ok_or(::serde::de::Error::missing_field("CacheParameterGroupFamily"))?,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    properties: properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ParameterGroup {
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
#[derive(Debug)]
pub struct ReplicationGroupProperties {
    /// Property `AtRestEncryptionEnabled`.
    pub at_rest_encryption_enabled: Option<::Value<bool>>,
    /// Property `AuthToken`.
    pub auth_token: Option<::Value<String>>,
    /// Property `AutoMinorVersionUpgrade`.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `AutomaticFailoverEnabled`.
    pub automatic_failover_enabled: Option<::Value<bool>>,
    /// Property `CacheNodeType`.
    pub cache_node_type: Option<::Value<String>>,
    /// Property `CacheParameterGroupName`.
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property `CacheSecurityGroupNames`.
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property `CacheSubnetGroupName`.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `Engine`.
    pub engine: Option<::Value<String>>,
    /// Property `EngineVersion`.
    pub engine_version: Option<::Value<String>>,
    /// Property `NodeGroupConfiguration`.
    pub node_group_configuration: Option<::ValueList<self::replication_group::NodeGroupConfiguration>>,
    /// Property `NotificationTopicArn`.
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property `NumCacheClusters`.
    pub num_cache_clusters: Option<::Value<u32>>,
    /// Property `NumNodeGroups`.
    pub num_node_groups: Option<::Value<u32>>,
    /// Property `Port`.
    pub port: Option<::Value<u32>>,
    /// Property `PreferredCacheClusterAZs`.
    pub preferred_cache_cluster_a_zs: Option<::ValueList<String>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PrimaryClusterId`.
    pub primary_cluster_id: Option<::Value<String>>,
    /// Property `ReplicasPerNodeGroup`.
    pub replicas_per_node_group: Option<::Value<u32>>,
    /// Property `ReplicationGroupDescription`.
    pub replication_group_description: ::Value<String>,
    /// Property `ReplicationGroupId`.
    pub replication_group_id: Option<::Value<String>>,
    /// Property `SecurityGroupIds`.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SnapshotArns`.
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property `SnapshotName`.
    pub snapshot_name: Option<::Value<String>>,
    /// Property `SnapshotRetentionLimit`.
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property `SnapshotWindow`.
    pub snapshot_window: Option<::Value<String>>,
    /// Property `SnapshottingClusterId`.
    pub snapshotting_cluster_id: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TransitEncryptionEnabled`.
    pub transit_encryption_enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for ReplicationGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AtRestEncryptionEnabled", &self.at_rest_encryption_enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthToken", &self.auth_token)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", &self.auto_minor_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticFailoverEnabled", &self.automatic_failover_enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNodeType", &self.cache_node_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupName", &self.cache_parameter_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupNames", &self.cache_security_group_names)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", &self.cache_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeGroupConfiguration", &self.node_group_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicArn", &self.notification_topic_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumCacheClusters", &self.num_cache_clusters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumNodeGroups", &self.num_node_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredCacheClusterAZs", &self.preferred_cache_cluster_a_zs)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryClusterId", &self.primary_cluster_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicasPerNodeGroup", &self.replicas_per_node_group)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupDescription", &self.replication_group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupId", &self.replication_group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotArns", &self.snapshot_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotName", &self.snapshot_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotRetentionLimit", &self.snapshot_retention_limit)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotWindow", &self.snapshot_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshottingClusterId", &self.snapshotting_cluster_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionEnabled", &self.transit_encryption_enabled)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut at_rest_encryption_enabled = None;
                let mut auth_token = None;
                let mut auto_minor_version_upgrade = None;
                let mut automatic_failover_enabled = None;
                let mut cache_node_type = None;
                let mut cache_parameter_group_name = None;
                let mut cache_security_group_names = None;
                let mut cache_subnet_group_name = None;
                let mut engine = None;
                let mut engine_version = None;
                let mut node_group_configuration = None;
                let mut notification_topic_arn = None;
                let mut num_cache_clusters = None;
                let mut num_node_groups = None;
                let mut port = None;
                let mut preferred_cache_cluster_a_zs = None;
                let mut preferred_maintenance_window = None;
                let mut primary_cluster_id = None;
                let mut replicas_per_node_group = None;
                let mut replication_group_description = None;
                let mut replication_group_id = None;
                let mut security_group_ids = None;
                let mut snapshot_arns = None;
                let mut snapshot_name = None;
                let mut snapshot_retention_limit = None;
                let mut snapshot_window = None;
                let mut snapshotting_cluster_id = None;
                let mut tags = None;
                let mut transit_encryption_enabled = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AtRestEncryptionEnabled" => {
                            at_rest_encryption_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthToken" => {
                            auth_token = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutomaticFailoverEnabled" => {
                            automatic_failover_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheNodeType" => {
                            cache_node_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheParameterGroupName" => {
                            cache_parameter_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheSecurityGroupNames" => {
                            cache_security_group_names = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Engine" => {
                            engine = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineVersion" => {
                            engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NodeGroupConfiguration" => {
                            node_group_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NotificationTopicArn" => {
                            notification_topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NumCacheClusters" => {
                            num_cache_clusters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NumNodeGroups" => {
                            num_node_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredCacheClusterAZs" => {
                            preferred_cache_cluster_a_zs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PrimaryClusterId" => {
                            primary_cluster_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicasPerNodeGroup" => {
                            replicas_per_node_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationGroupDescription" => {
                            replication_group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationGroupId" => {
                            replication_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotArns" => {
                            snapshot_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotName" => {
                            snapshot_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotRetentionLimit" => {
                            snapshot_retention_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotWindow" => {
                            snapshot_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshottingClusterId" => {
                            snapshotting_cluster_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TransitEncryptionEnabled" => {
                            transit_encryption_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationGroupProperties {
                    at_rest_encryption_enabled: at_rest_encryption_enabled,
                    auth_token: auth_token,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    automatic_failover_enabled: automatic_failover_enabled,
                    cache_node_type: cache_node_type,
                    cache_parameter_group_name: cache_parameter_group_name,
                    cache_security_group_names: cache_security_group_names,
                    cache_subnet_group_name: cache_subnet_group_name,
                    engine: engine,
                    engine_version: engine_version,
                    node_group_configuration: node_group_configuration,
                    notification_topic_arn: notification_topic_arn,
                    num_cache_clusters: num_cache_clusters,
                    num_node_groups: num_node_groups,
                    port: port,
                    preferred_cache_cluster_a_zs: preferred_cache_cluster_a_zs,
                    preferred_maintenance_window: preferred_maintenance_window,
                    primary_cluster_id: primary_cluster_id,
                    replicas_per_node_group: replicas_per_node_group,
                    replication_group_description: replication_group_description.ok_or(::serde::de::Error::missing_field("ReplicationGroupDescription"))?,
                    replication_group_id: replication_group_id,
                    security_group_ids: security_group_ids,
                    snapshot_arns: snapshot_arns,
                    snapshot_name: snapshot_name,
                    snapshot_retention_limit: snapshot_retention_limit,
                    snapshot_window: snapshot_window,
                    snapshotting_cluster_id: snapshotting_cluster_id,
                    tags: tags,
                    transit_encryption_enabled: transit_encryption_enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationGroup {
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
#[derive(Debug)]
pub struct SecurityGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
}

impl ::serde::Serialize for SecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SecurityGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityGroup {
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
#[derive(Debug)]
pub struct SecurityGroupIngressProperties {
    /// Property `CacheSecurityGroupName`.
    pub cache_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupName`.
    pub ec2_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupOwnerId`.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for SecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupName", &self.cache_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", &self.ec2_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", &self.ec2_security_group_owner_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_security_group_name = None;
                let mut ec2_security_group_name = None;
                let mut ec2_security_group_owner_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheSecurityGroupName" => {
                            cache_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

                Ok(SecurityGroupIngressProperties {
                    cache_security_group_name: cache_security_group_name.ok_or(::serde::de::Error::missing_field("CacheSecurityGroupName"))?,
                    ec2_security_group_name: ec2_security_group_name.ok_or(::serde::de::Error::missing_field("EC2SecurityGroupName"))?,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityGroupIngress {
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
#[derive(Debug)]
pub struct SubnetGroupProperties {
    /// Property `CacheSubnetGroupName`.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `SubnetIds`.
    pub subnet_ids: ::ValueList<String>,
}

impl ::serde::Serialize for SubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", &self.cache_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_subnet_group_name = None;
                let mut description = None;
                let mut subnet_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetIds" => {
                            subnet_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(SubnetGroupProperties {
                    cache_subnet_group_name: cache_subnet_group_name,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubnetGroup {
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
    #[derive(Debug)]
    pub struct NodeGroupConfiguration {
        /// Property `PrimaryAvailabilityZone`.
        pub primary_availability_zone: Option<::Value<String>>,
        /// Property `ReplicaAvailabilityZones`.
        pub replica_availability_zones: Option<::ValueList<String>>,
        /// Property `ReplicaCount`.
        pub replica_count: Option<::Value<u32>>,
        /// Property `Slots`.
        pub slots: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NodeGroupConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryAvailabilityZone", &self.primary_availability_zone)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaAvailabilityZones", &self.replica_availability_zones)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaCount", &self.replica_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slots", &self.slots)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NodeGroupConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeGroupConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeGroupConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeGroupConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary_availability_zone = None;
                    let mut replica_availability_zones = None;
                    let mut replica_count = None;
                    let mut slots = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrimaryAvailabilityZone" => {
                                primary_availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReplicaAvailabilityZones" => {
                                replica_availability_zones = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReplicaCount" => {
                                replica_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Slots" => {
                                slots = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeGroupConfiguration {
                        primary_availability_zone: primary_availability_zone,
                        replica_availability_zones: replica_availability_zones,
                        replica_count: replica_count,
                        slots: slots,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
