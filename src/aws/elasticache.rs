//! Types for the `ElastiCache` service.

/// The [`AWS::ElastiCache::CacheCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct CacheCluster {
    properties: CacheClusterProperties
}

/// Properties for the `CacheCluster` resource.
#[derive(Debug, Default)]
pub struct CacheClusterProperties {
    /// Property [`AZMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-azmode).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub az_mode: Option<::Value<String>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property [`CacheNodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-cachenodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_node_type: ::Value<String>,
    /// Property [`CacheParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-cacheparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property [`CacheSecurityGroupNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-cachesecuritygroupnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property [`CacheSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-cachesubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: Option<::Value<String>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`IpDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-ipdiscovery).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_discovery: Option<::Value<String>>,
    /// Property [`LogDeliveryConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-logdeliveryconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_delivery_configurations: Option<::ValueList<self::cache_cluster::LogDeliveryConfigurationRequest>>,
    /// Property [`NetworkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-networktype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_type: Option<::Value<String>>,
    /// Property [`NotificationTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-notificationtopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property [`NumCacheNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-numcachenodes).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub num_cache_nodes: ::Value<u32>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PreferredAvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-preferredavailabilityzone).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub preferred_availability_zone: Option<::Value<String>>,
    /// Property [`PreferredAvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-preferredavailabilityzones).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub preferred_availability_zones: Option<::ValueList<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`SnapshotArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-snapshotarns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property [`SnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-snapshotname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_name: Option<::Value<String>>,
    /// Property [`SnapshotRetentionLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-snapshotretentionlimit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property [`SnapshotWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-snapshotwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_window: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TransitEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-transitencryptionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transit_encryption_enabled: Option<::Value<bool>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html#cfn-elasticache-cachecluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for CacheClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref az_mode) = self.az_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AZMode", az_mode)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNodeType", &self.cache_node_type)?;
        if let Some(ref cache_parameter_group_name) = self.cache_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupName", cache_parameter_group_name)?;
        }
        if let Some(ref cache_security_group_names) = self.cache_security_group_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupNames", cache_security_group_names)?;
        }
        if let Some(ref cache_subnet_group_name) = self.cache_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", cache_subnet_group_name)?;
        }
        if let Some(ref cluster_name) = self.cluster_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", cluster_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref ip_discovery) = self.ip_discovery {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpDiscovery", ip_discovery)?;
        }
        if let Some(ref log_delivery_configurations) = self.log_delivery_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDeliveryConfigurations", log_delivery_configurations)?;
        }
        if let Some(ref network_type) = self.network_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkType", network_type)?;
        }
        if let Some(ref notification_topic_arn) = self.notification_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicArn", notification_topic_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumCacheNodes", &self.num_cache_nodes)?;
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_availability_zone) = self.preferred_availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredAvailabilityZone", preferred_availability_zone)?;
        }
        if let Some(ref preferred_availability_zones) = self.preferred_availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredAvailabilityZones", preferred_availability_zones)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref snapshot_arns) = self.snapshot_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotArns", snapshot_arns)?;
        }
        if let Some(ref snapshot_name) = self.snapshot_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotName", snapshot_name)?;
        }
        if let Some(ref snapshot_retention_limit) = self.snapshot_retention_limit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotRetentionLimit", snapshot_retention_limit)?;
        }
        if let Some(ref snapshot_window) = self.snapshot_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotWindow", snapshot_window)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref transit_encryption_enabled) = self.transit_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionEnabled", transit_encryption_enabled)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
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
                let mut az_mode: Option<::Value<String>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut cache_node_type: Option<::Value<String>> = None;
                let mut cache_parameter_group_name: Option<::Value<String>> = None;
                let mut cache_security_group_names: Option<::ValueList<String>> = None;
                let mut cache_subnet_group_name: Option<::Value<String>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut ip_discovery: Option<::Value<String>> = None;
                let mut log_delivery_configurations: Option<::ValueList<self::cache_cluster::LogDeliveryConfigurationRequest>> = None;
                let mut network_type: Option<::Value<String>> = None;
                let mut notification_topic_arn: Option<::Value<String>> = None;
                let mut num_cache_nodes: Option<::Value<u32>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut preferred_availability_zone: Option<::Value<String>> = None;
                let mut preferred_availability_zones: Option<::ValueList<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut snapshot_arns: Option<::ValueList<String>> = None;
                let mut snapshot_name: Option<::Value<String>> = None;
                let mut snapshot_retention_limit: Option<::Value<u32>> = None;
                let mut snapshot_window: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transit_encryption_enabled: Option<::Value<bool>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AZMode" => {
                            az_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheNodeType" => {
                            cache_node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheParameterGroupName" => {
                            cache_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheSecurityGroupNames" => {
                            cache_security_group_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpDiscovery" => {
                            ip_discovery = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogDeliveryConfigurations" => {
                            log_delivery_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkType" => {
                            network_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTopicArn" => {
                            notification_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumCacheNodes" => {
                            num_cache_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredAvailabilityZone" => {
                            preferred_availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredAvailabilityZones" => {
                            preferred_availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotArns" => {
                            snapshot_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotName" => {
                            snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotRetentionLimit" => {
                            snapshot_retention_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotWindow" => {
                            snapshot_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitEncryptionEnabled" => {
                            transit_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    ip_discovery: ip_discovery,
                    log_delivery_configurations: log_delivery_configurations,
                    network_type: network_type,
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
                    transit_encryption_enabled: transit_encryption_enabled,
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

/// The [`AWS::ElastiCache::GlobalReplicationGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html) resource type.
#[derive(Debug, Default)]
pub struct GlobalReplicationGroup {
    properties: GlobalReplicationGroupProperties
}

/// Properties for the `GlobalReplicationGroup` resource.
#[derive(Debug, Default)]
pub struct GlobalReplicationGroupProperties {
    /// Property [`AutomaticFailoverEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-automaticfailoverenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automatic_failover_enabled: Option<::Value<bool>>,
    /// Property [`CacheNodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-cachenodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_node_type: Option<::Value<String>>,
    /// Property [`CacheParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-cacheparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`GlobalNodeGroupCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-globalnodegroupcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_node_group_count: Option<::Value<u32>>,
    /// Property [`GlobalReplicationGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-globalreplicationgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_replication_group_description: Option<::Value<String>>,
    /// Property [`GlobalReplicationGroupIdSuffix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-globalreplicationgroupidsuffix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_replication_group_id_suffix: Option<::Value<String>>,
    /// Property [`Members`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-members).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub members: ::ValueList<self::global_replication_group::GlobalReplicationGroupMember>,
    /// Property [`RegionalConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html#cfn-elasticache-globalreplicationgroup-regionalconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regional_configurations: Option<::ValueList<self::global_replication_group::RegionalConfiguration>>,
}

impl ::serde::Serialize for GlobalReplicationGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref automatic_failover_enabled) = self.automatic_failover_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticFailoverEnabled", automatic_failover_enabled)?;
        }
        if let Some(ref cache_node_type) = self.cache_node_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNodeType", cache_node_type)?;
        }
        if let Some(ref cache_parameter_group_name) = self.cache_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupName", cache_parameter_group_name)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_node_group_count) = self.global_node_group_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNodeGroupCount", global_node_group_count)?;
        }
        if let Some(ref global_replication_group_description) = self.global_replication_group_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalReplicationGroupDescription", global_replication_group_description)?;
        }
        if let Some(ref global_replication_group_id_suffix) = self.global_replication_group_id_suffix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalReplicationGroupIdSuffix", global_replication_group_id_suffix)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Members", &self.members)?;
        if let Some(ref regional_configurations) = self.regional_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionalConfigurations", regional_configurations)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GlobalReplicationGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalReplicationGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalReplicationGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GlobalReplicationGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut automatic_failover_enabled: Option<::Value<bool>> = None;
                let mut cache_node_type: Option<::Value<String>> = None;
                let mut cache_parameter_group_name: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut global_node_group_count: Option<::Value<u32>> = None;
                let mut global_replication_group_description: Option<::Value<String>> = None;
                let mut global_replication_group_id_suffix: Option<::Value<String>> = None;
                let mut members: Option<::ValueList<self::global_replication_group::GlobalReplicationGroupMember>> = None;
                let mut regional_configurations: Option<::ValueList<self::global_replication_group::RegionalConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutomaticFailoverEnabled" => {
                            automatic_failover_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheNodeType" => {
                            cache_node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheParameterGroupName" => {
                            cache_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNodeGroupCount" => {
                            global_node_group_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalReplicationGroupDescription" => {
                            global_replication_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalReplicationGroupIdSuffix" => {
                            global_replication_group_id_suffix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Members" => {
                            members = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegionalConfigurations" => {
                            regional_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalReplicationGroupProperties {
                    automatic_failover_enabled: automatic_failover_enabled,
                    cache_node_type: cache_node_type,
                    cache_parameter_group_name: cache_parameter_group_name,
                    engine_version: engine_version,
                    global_node_group_count: global_node_group_count,
                    global_replication_group_description: global_replication_group_description,
                    global_replication_group_id_suffix: global_replication_group_id_suffix,
                    members: members.ok_or(::serde::de::Error::missing_field("Members"))?,
                    regional_configurations: regional_configurations,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GlobalReplicationGroup {
    type Properties = GlobalReplicationGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::GlobalReplicationGroup";
    fn properties(&self) -> &GlobalReplicationGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GlobalReplicationGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GlobalReplicationGroup {}

impl From<GlobalReplicationGroupProperties> for GlobalReplicationGroup {
    fn from(properties: GlobalReplicationGroupProperties) -> GlobalReplicationGroup {
        GlobalReplicationGroup { properties }
    }
}

/// The [`AWS::ElastiCache::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html) resource type.
#[derive(Debug, Default)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Debug, Default)]
pub struct ParameterGroupProperties {
    /// Property [`CacheParameterGroupFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html#cfn-elasticache-parametergroup-cacheparametergroupfamily).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cache_parameter_group_family: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html#cfn-elasticache-parametergroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html#cfn-elasticache-parametergroup-properties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub properties: Option<::ValueMap<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-parameter-group.html#cfn-elasticache-parametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupFamily", &self.cache_parameter_group_family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref properties) = self.properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", properties)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut cache_parameter_group_family: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut properties: Option<::ValueMap<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheParameterGroupFamily" => {
                            cache_parameter_group_family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Properties" => {
                            properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ParameterGroupProperties {
                    cache_parameter_group_family: cache_parameter_group_family.ok_or(::serde::de::Error::missing_field("CacheParameterGroupFamily"))?,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    properties: properties,
                    tags: tags,
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
#[derive(Debug, Default)]
pub struct ReplicationGroup {
    properties: ReplicationGroupProperties
}

/// Properties for the `ReplicationGroup` resource.
#[derive(Debug, Default)]
pub struct ReplicationGroupProperties {
    /// Property [`AtRestEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-atrestencryptionenabled).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub at_rest_encryption_enabled: Option<::Value<bool>>,
    /// Property [`AuthToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-authtoken).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub auth_token: Option<::Value<String>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property [`AutomaticFailoverEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-automaticfailoverenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automatic_failover_enabled: Option<::Value<bool>>,
    /// Property [`CacheNodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-cachenodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_node_type: Option<::Value<String>>,
    /// Property [`CacheParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-cacheparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_parameter_group_name: Option<::Value<String>>,
    /// Property [`CacheSecurityGroupNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-cachesecuritygroupnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_security_group_names: Option<::ValueList<String>>,
    /// Property [`CacheSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-cachesubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property [`ClusterMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-clustermode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_mode: Option<::Value<String>>,
    /// Property [`DataTieringEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-datatieringenabled).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_tiering_enabled: Option<::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`GlobalReplicationGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-globalreplicationgroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_replication_group_id: Option<::Value<String>>,
    /// Property [`IpDiscovery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-ipdiscovery).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_discovery: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LogDeliveryConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-logdeliveryconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_delivery_configurations: Option<::ValueList<self::replication_group::LogDeliveryConfigurationRequest>>,
    /// Property [`MultiAZEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-multiazenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub multi_az_enabled: Option<::Value<bool>>,
    /// Property [`NetworkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-networktype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_type: Option<::Value<String>>,
    /// Property [`NodeGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-nodegroupconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub node_group_configuration: Option<::ValueList<self::replication_group::NodeGroupConfiguration>>,
    /// Property [`NotificationTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-notificationtopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_topic_arn: Option<::Value<String>>,
    /// Property [`NumCacheClusters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-numcacheclusters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub num_cache_clusters: Option<::Value<u32>>,
    /// Property [`NumNodeGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-numnodegroups).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub num_node_groups: Option<::Value<u32>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PreferredCacheClusterAZs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-preferredcacheclusterazs).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub preferred_cache_cluster_a_zs: Option<::ValueList<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PrimaryClusterId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-primaryclusterid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub primary_cluster_id: Option<::Value<String>>,
    /// Property [`ReplicasPerNodeGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-replicaspernodegroup).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replicas_per_node_group: Option<::Value<u32>>,
    /// Property [`ReplicationGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-replicationgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_group_description: ::Value<String>,
    /// Property [`ReplicationGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-replicationgroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replication_group_id: Option<::Value<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SnapshotArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-snapshotarns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_arns: Option<::ValueList<String>>,
    /// Property [`SnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-snapshotname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_name: Option<::Value<String>>,
    /// Property [`SnapshotRetentionLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-snapshotretentionlimit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property [`SnapshotWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-snapshotwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_window: Option<::Value<String>>,
    /// Property [`SnapshottingClusterId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-snapshottingclusterid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshotting_cluster_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TransitEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-transitencryptionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transit_encryption_enabled: Option<::Value<bool>>,
    /// Property [`TransitEncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-transitencryptionmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transit_encryption_mode: Option<::Value<String>>,
    /// Property [`UserGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html#cfn-elasticache-replicationgroup-usergroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ReplicationGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref at_rest_encryption_enabled) = self.at_rest_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AtRestEncryptionEnabled", at_rest_encryption_enabled)?;
        }
        if let Some(ref auth_token) = self.auth_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthToken", auth_token)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref automatic_failover_enabled) = self.automatic_failover_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticFailoverEnabled", automatic_failover_enabled)?;
        }
        if let Some(ref cache_node_type) = self.cache_node_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNodeType", cache_node_type)?;
        }
        if let Some(ref cache_parameter_group_name) = self.cache_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheParameterGroupName", cache_parameter_group_name)?;
        }
        if let Some(ref cache_security_group_names) = self.cache_security_group_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupNames", cache_security_group_names)?;
        }
        if let Some(ref cache_subnet_group_name) = self.cache_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", cache_subnet_group_name)?;
        }
        if let Some(ref cluster_mode) = self.cluster_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterMode", cluster_mode)?;
        }
        if let Some(ref data_tiering_enabled) = self.data_tiering_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTieringEnabled", data_tiering_enabled)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_replication_group_id) = self.global_replication_group_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalReplicationGroupId", global_replication_group_id)?;
        }
        if let Some(ref ip_discovery) = self.ip_discovery {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpDiscovery", ip_discovery)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref log_delivery_configurations) = self.log_delivery_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDeliveryConfigurations", log_delivery_configurations)?;
        }
        if let Some(ref multi_az_enabled) = self.multi_az_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZEnabled", multi_az_enabled)?;
        }
        if let Some(ref network_type) = self.network_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkType", network_type)?;
        }
        if let Some(ref node_group_configuration) = self.node_group_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeGroupConfiguration", node_group_configuration)?;
        }
        if let Some(ref notification_topic_arn) = self.notification_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicArn", notification_topic_arn)?;
        }
        if let Some(ref num_cache_clusters) = self.num_cache_clusters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumCacheClusters", num_cache_clusters)?;
        }
        if let Some(ref num_node_groups) = self.num_node_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumNodeGroups", num_node_groups)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_cache_cluster_a_zs) = self.preferred_cache_cluster_a_zs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredCacheClusterAZs", preferred_cache_cluster_a_zs)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref primary_cluster_id) = self.primary_cluster_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryClusterId", primary_cluster_id)?;
        }
        if let Some(ref replicas_per_node_group) = self.replicas_per_node_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicasPerNodeGroup", replicas_per_node_group)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupDescription", &self.replication_group_description)?;
        if let Some(ref replication_group_id) = self.replication_group_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupId", replication_group_id)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref snapshot_arns) = self.snapshot_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotArns", snapshot_arns)?;
        }
        if let Some(ref snapshot_name) = self.snapshot_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotName", snapshot_name)?;
        }
        if let Some(ref snapshot_retention_limit) = self.snapshot_retention_limit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotRetentionLimit", snapshot_retention_limit)?;
        }
        if let Some(ref snapshot_window) = self.snapshot_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotWindow", snapshot_window)?;
        }
        if let Some(ref snapshotting_cluster_id) = self.snapshotting_cluster_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshottingClusterId", snapshotting_cluster_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref transit_encryption_enabled) = self.transit_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionEnabled", transit_encryption_enabled)?;
        }
        if let Some(ref transit_encryption_mode) = self.transit_encryption_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionMode", transit_encryption_mode)?;
        }
        if let Some(ref user_group_ids) = self.user_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserGroupIds", user_group_ids)?;
        }
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
                let mut at_rest_encryption_enabled: Option<::Value<bool>> = None;
                let mut auth_token: Option<::Value<String>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut automatic_failover_enabled: Option<::Value<bool>> = None;
                let mut cache_node_type: Option<::Value<String>> = None;
                let mut cache_parameter_group_name: Option<::Value<String>> = None;
                let mut cache_security_group_names: Option<::ValueList<String>> = None;
                let mut cache_subnet_group_name: Option<::Value<String>> = None;
                let mut cluster_mode: Option<::Value<String>> = None;
                let mut data_tiering_enabled: Option<::Value<bool>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut global_replication_group_id: Option<::Value<String>> = None;
                let mut ip_discovery: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut log_delivery_configurations: Option<::ValueList<self::replication_group::LogDeliveryConfigurationRequest>> = None;
                let mut multi_az_enabled: Option<::Value<bool>> = None;
                let mut network_type: Option<::Value<String>> = None;
                let mut node_group_configuration: Option<::ValueList<self::replication_group::NodeGroupConfiguration>> = None;
                let mut notification_topic_arn: Option<::Value<String>> = None;
                let mut num_cache_clusters: Option<::Value<u32>> = None;
                let mut num_node_groups: Option<::Value<u32>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut preferred_cache_cluster_a_zs: Option<::ValueList<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut primary_cluster_id: Option<::Value<String>> = None;
                let mut replicas_per_node_group: Option<::Value<u32>> = None;
                let mut replication_group_description: Option<::Value<String>> = None;
                let mut replication_group_id: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut snapshot_arns: Option<::ValueList<String>> = None;
                let mut snapshot_name: Option<::Value<String>> = None;
                let mut snapshot_retention_limit: Option<::Value<u32>> = None;
                let mut snapshot_window: Option<::Value<String>> = None;
                let mut snapshotting_cluster_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transit_encryption_enabled: Option<::Value<bool>> = None;
                let mut transit_encryption_mode: Option<::Value<String>> = None;
                let mut user_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AtRestEncryptionEnabled" => {
                            at_rest_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthToken" => {
                            auth_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutomaticFailoverEnabled" => {
                            automatic_failover_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheNodeType" => {
                            cache_node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheParameterGroupName" => {
                            cache_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheSecurityGroupNames" => {
                            cache_security_group_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterMode" => {
                            cluster_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataTieringEnabled" => {
                            data_tiering_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalReplicationGroupId" => {
                            global_replication_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpDiscovery" => {
                            ip_discovery = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogDeliveryConfigurations" => {
                            log_delivery_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiAZEnabled" => {
                            multi_az_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkType" => {
                            network_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeGroupConfiguration" => {
                            node_group_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationTopicArn" => {
                            notification_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumCacheClusters" => {
                            num_cache_clusters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumNodeGroups" => {
                            num_node_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredCacheClusterAZs" => {
                            preferred_cache_cluster_a_zs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrimaryClusterId" => {
                            primary_cluster_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicasPerNodeGroup" => {
                            replicas_per_node_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationGroupDescription" => {
                            replication_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationGroupId" => {
                            replication_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotArns" => {
                            snapshot_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotName" => {
                            snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotRetentionLimit" => {
                            snapshot_retention_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotWindow" => {
                            snapshot_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshottingClusterId" => {
                            snapshotting_cluster_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitEncryptionEnabled" => {
                            transit_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitEncryptionMode" => {
                            transit_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserGroupIds" => {
                            user_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    cluster_mode: cluster_mode,
                    data_tiering_enabled: data_tiering_enabled,
                    engine: engine,
                    engine_version: engine_version,
                    global_replication_group_id: global_replication_group_id,
                    ip_discovery: ip_discovery,
                    kms_key_id: kms_key_id,
                    log_delivery_configurations: log_delivery_configurations,
                    multi_az_enabled: multi_az_enabled,
                    network_type: network_type,
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
                    transit_encryption_mode: transit_encryption_mode,
                    user_group_ids: user_group_ids,
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
#[derive(Debug, Default)]
pub struct SecurityGroup {
    properties: SecurityGroupProperties
}

/// Properties for the `SecurityGroup` resource.
#[derive(Debug, Default)]
pub struct SecurityGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group.html#cfn-elasticache-securitygroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group.html#cfn-elasticache-securitygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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

                Ok(SecurityGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    tags: tags,
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
#[derive(Debug, Default)]
pub struct SecurityGroupIngress {
    properties: SecurityGroupIngressProperties
}

/// Properties for the `SecurityGroupIngress` resource.
#[derive(Debug, Default)]
pub struct SecurityGroupIngressProperties {
    /// Property [`CacheSecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html#cfn-elasticache-securitygroupingress-cachesecuritygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_security_group_name: ::Value<String>,
    /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html#cfn-elasticache-securitygroupingress-ec2securitygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_name: ::Value<String>,
    /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html#cfn-elasticache-securitygroupingress-ec2securitygroupownerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for SecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSecurityGroupName", &self.cache_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", &self.ec2_security_group_name)?;
        if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
        }
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
                let mut cache_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_owner_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheSecurityGroupName" => {
                            cache_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

/// The [`AWS::ElastiCache::ServerlessCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html) resource type.
#[derive(Debug, Default)]
pub struct ServerlessCache {
    properties: ServerlessCacheProperties
}

/// Properties for the `ServerlessCache` resource.
#[derive(Debug, Default)]
pub struct ServerlessCacheProperties {
    /// Property [`CacheUsageLimits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-cacheusagelimits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_usage_limits: Option<::Value<self::serverless_cache::CacheUsageLimits>>,
    /// Property [`DailySnapshotTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-dailysnapshottime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub daily_snapshot_time: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-endpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint: Option<::Value<self::serverless_cache::Endpoint>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`FinalSnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-finalsnapshotname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub final_snapshot_name: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`MajorEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-majorengineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub major_engine_version: Option<::Value<String>>,
    /// Property [`ReaderEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-readerendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reader_endpoint: Option<::Value<self::serverless_cache::Endpoint>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`ServerlessCacheName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-serverlesscachename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub serverless_cache_name: ::Value<String>,
    /// Property [`SnapshotArnsToRestore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-snapshotarnstorestore).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_arns_to_restore: Option<::ValueList<String>>,
    /// Property [`SnapshotRetentionLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-snapshotretentionlimit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_retention_limit: Option<::Value<u32>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html#cfn-elasticache-serverlesscache-usergroupid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_group_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ServerlessCacheProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cache_usage_limits) = self.cache_usage_limits {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheUsageLimits", cache_usage_limits)?;
        }
        if let Some(ref daily_snapshot_time) = self.daily_snapshot_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailySnapshotTime", daily_snapshot_time)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref endpoint) = self.endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref final_snapshot_name) = self.final_snapshot_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FinalSnapshotName", final_snapshot_name)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref major_engine_version) = self.major_engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MajorEngineVersion", major_engine_version)?;
        }
        if let Some(ref reader_endpoint) = self.reader_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReaderEndpoint", reader_endpoint)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerlessCacheName", &self.serverless_cache_name)?;
        if let Some(ref snapshot_arns_to_restore) = self.snapshot_arns_to_restore {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotArnsToRestore", snapshot_arns_to_restore)?;
        }
        if let Some(ref snapshot_retention_limit) = self.snapshot_retention_limit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotRetentionLimit", snapshot_retention_limit)?;
        }
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_group_id) = self.user_group_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserGroupId", user_group_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServerlessCacheProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerlessCacheProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServerlessCacheProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServerlessCacheProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_usage_limits: Option<::Value<self::serverless_cache::CacheUsageLimits>> = None;
                let mut daily_snapshot_time: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut endpoint: Option<::Value<self::serverless_cache::Endpoint>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut final_snapshot_name: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut major_engine_version: Option<::Value<String>> = None;
                let mut reader_endpoint: Option<::Value<self::serverless_cache::Endpoint>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut serverless_cache_name: Option<::Value<String>> = None;
                let mut snapshot_arns_to_restore: Option<::ValueList<String>> = None;
                let mut snapshot_retention_limit: Option<::Value<u32>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_group_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheUsageLimits" => {
                            cache_usage_limits = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DailySnapshotTime" => {
                            daily_snapshot_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Endpoint" => {
                            endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FinalSnapshotName" => {
                            final_snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MajorEngineVersion" => {
                            major_engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReaderEndpoint" => {
                            reader_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerlessCacheName" => {
                            serverless_cache_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotArnsToRestore" => {
                            snapshot_arns_to_restore = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotRetentionLimit" => {
                            snapshot_retention_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserGroupId" => {
                            user_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServerlessCacheProperties {
                    cache_usage_limits: cache_usage_limits,
                    daily_snapshot_time: daily_snapshot_time,
                    description: description,
                    endpoint: endpoint,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    final_snapshot_name: final_snapshot_name,
                    kms_key_id: kms_key_id,
                    major_engine_version: major_engine_version,
                    reader_endpoint: reader_endpoint,
                    security_group_ids: security_group_ids,
                    serverless_cache_name: serverless_cache_name.ok_or(::serde::de::Error::missing_field("ServerlessCacheName"))?,
                    snapshot_arns_to_restore: snapshot_arns_to_restore,
                    snapshot_retention_limit: snapshot_retention_limit,
                    subnet_ids: subnet_ids,
                    tags: tags,
                    user_group_id: user_group_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServerlessCache {
    type Properties = ServerlessCacheProperties;
    const TYPE: &'static str = "AWS::ElastiCache::ServerlessCache";
    fn properties(&self) -> &ServerlessCacheProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServerlessCacheProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServerlessCache {}

impl From<ServerlessCacheProperties> for ServerlessCache {
    fn from(properties: ServerlessCacheProperties) -> ServerlessCache {
        ServerlessCache { properties }
    }
}

/// The [`AWS::ElastiCache::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Debug, Default)]
pub struct SubnetGroupProperties {
    /// Property [`CacheSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html#cfn-elasticache-subnetgroup-cachesubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cache_subnet_group_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html#cfn-elasticache-subnetgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html#cfn-elasticache-subnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html#cfn-elasticache-subnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cache_subnet_group_name) = self.cache_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheSubnetGroupName", cache_subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut cache_subnet_group_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheSubnetGroupName" => {
                            cache_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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

                Ok(SubnetGroupProperties {
                    cache_subnet_group_name: cache_subnet_group_name,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
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

/// The [`AWS::ElastiCache::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`AccessString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-accessstring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_string: Option<::Value<String>>,
    /// Property [`AuthenticationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-authenticationmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authentication_mode: Option<::Value<self::user::AuthenticationMode>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`NoPasswordRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-nopasswordrequired).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub no_password_required: Option<::Value<bool>>,
    /// Property [`Passwords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-passwords).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub passwords: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-userid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_id: ::Value<String>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html#cfn-elasticache-user-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_string) = self.access_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessString", access_string)?;
        }
        if let Some(ref authentication_mode) = self.authentication_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationMode", authentication_mode)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref no_password_required) = self.no_password_required {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoPasswordRequired", no_password_required)?;
        }
        if let Some(ref passwords) = self.passwords {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Passwords", passwords)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserId", &self.user_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_string: Option<::Value<String>> = None;
                let mut authentication_mode: Option<::Value<self::user::AuthenticationMode>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut no_password_required: Option<::Value<bool>> = None;
                let mut passwords: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_id: Option<::Value<String>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessString" => {
                            access_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthenticationMode" => {
                            authentication_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NoPasswordRequired" => {
                            no_password_required = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Passwords" => {
                            passwords = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserId" => {
                            user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    access_string: access_string,
                    authentication_mode: authentication_mode,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    no_password_required: no_password_required,
                    passwords: passwords,
                    tags: tags,
                    user_id: user_id.ok_or(::serde::de::Error::missing_field("UserId"))?,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::ElastiCache::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::ElastiCache::UserGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html) resource type.
#[derive(Debug, Default)]
pub struct UserGroup {
    properties: UserGroupProperties
}

/// Properties for the `UserGroup` resource.
#[derive(Debug, Default)]
pub struct UserGroupProperties {
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html#cfn-elasticache-usergroup-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html#cfn-elasticache-usergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html#cfn-elasticache-usergroup-usergroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_group_id: ::Value<String>,
    /// Property [`UserIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html#cfn-elasticache-usergroup-userids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_ids: ::ValueList<String>,
}

impl ::serde::Serialize for UserGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserGroupId", &self.user_group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserIds", &self.user_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut engine: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_group_id: Option<::Value<String>> = None;
                let mut user_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserGroupId" => {
                            user_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserIds" => {
                            user_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserGroupProperties {
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    tags: tags,
                    user_group_id: user_group_id.ok_or(::serde::de::Error::missing_field("UserGroupId"))?,
                    user_ids: user_ids.ok_or(::serde::de::Error::missing_field("UserIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserGroup {
    type Properties = UserGroupProperties;
    const TYPE: &'static str = "AWS::ElastiCache::UserGroup";
    fn properties(&self) -> &UserGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserGroup {}

impl From<UserGroupProperties> for UserGroup {
    fn from(properties: UserGroupProperties) -> UserGroup {
        UserGroup { properties }
    }
}

pub mod cache_cluster {
    //! Property types for the `CacheCluster` resource.

    /// The [`AWS::ElastiCache::CacheCluster.CloudWatchLogsDestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-cloudwatchlogsdestinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogsDestinationDetails {
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-cloudwatchlogsdestinationdetails.html#cfn-elasticache-cachecluster-cloudwatchlogsdestinationdetails-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogsDestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", &self.log_group)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogsDestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogsDestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogsDestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogsDestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogsDestinationDetails {
                        log_group: log_group.ok_or(::serde::de::Error::missing_field("LogGroup"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::CacheCluster.DestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-destinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationDetails {
        /// Property [`CloudWatchLogsDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-destinationdetails.html#cfn-elasticache-cachecluster-destinationdetails-cloudwatchlogsdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_details: Option<::Value<CloudWatchLogsDestinationDetails>>,
        /// Property [`KinesisFirehoseDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-destinationdetails.html#cfn-elasticache-cachecluster-destinationdetails-kinesisfirehosedetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_details: Option<::Value<KinesisFirehoseDestinationDetails>>,
    }

    impl ::codec::SerializeValue for DestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs_details) = self.cloud_watch_logs_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsDetails", cloud_watch_logs_details)?;
            }
            if let Some(ref kinesis_firehose_details) = self.kinesis_firehose_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseDetails", kinesis_firehose_details)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_details: Option<::Value<CloudWatchLogsDestinationDetails>> = None;
                    let mut kinesis_firehose_details: Option<::Value<KinesisFirehoseDestinationDetails>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsDetails" => {
                                cloud_watch_logs_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisFirehoseDetails" => {
                                kinesis_firehose_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationDetails {
                        cloud_watch_logs_details: cloud_watch_logs_details,
                        kinesis_firehose_details: kinesis_firehose_details,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::CacheCluster.KinesisFirehoseDestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-kinesisfirehosedestinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseDestinationDetails {
        /// Property [`DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-kinesisfirehosedestinationdetails.html#cfn-elasticache-cachecluster-kinesisfirehosedestinationdetails-deliverystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseDestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStream", &self.delivery_stream)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseDestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseDestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseDestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseDestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStream" => {
                                delivery_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseDestinationDetails {
                        delivery_stream: delivery_stream.ok_or(::serde::de::Error::missing_field("DeliveryStream"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::CacheCluster.LogDeliveryConfigurationRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct LogDeliveryConfigurationRequest {
        /// Property [`DestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html#cfn-elasticache-cachecluster-logdeliveryconfigurationrequest-destinationdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_details: ::Value<DestinationDetails>,
        /// Property [`DestinationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html#cfn-elasticache-cachecluster-logdeliveryconfigurationrequest-destinationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_type: ::Value<String>,
        /// Property [`LogFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html#cfn-elasticache-cachecluster-logdeliveryconfigurationrequest-logformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_format: ::Value<String>,
        /// Property [`LogType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html#cfn-elasticache-cachecluster-logdeliveryconfigurationrequest-logtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for LogDeliveryConfigurationRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationDetails", &self.destination_details)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationType", &self.destination_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogFormat", &self.log_format)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogType", &self.log_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogDeliveryConfigurationRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogDeliveryConfigurationRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogDeliveryConfigurationRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogDeliveryConfigurationRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_details: Option<::Value<DestinationDetails>> = None;
                    let mut destination_type: Option<::Value<String>> = None;
                    let mut log_format: Option<::Value<String>> = None;
                    let mut log_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationDetails" => {
                                destination_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationType" => {
                                destination_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogFormat" => {
                                log_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogType" => {
                                log_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogDeliveryConfigurationRequest {
                        destination_details: destination_details.ok_or(::serde::de::Error::missing_field("DestinationDetails"))?,
                        destination_type: destination_type.ok_or(::serde::de::Error::missing_field("DestinationType"))?,
                        log_format: log_format.ok_or(::serde::de::Error::missing_field("LogFormat"))?,
                        log_type: log_type.ok_or(::serde::de::Error::missing_field("LogType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod global_replication_group {
    //! Property types for the `GlobalReplicationGroup` resource.

    /// The [`AWS::ElastiCache::GlobalReplicationGroup.GlobalReplicationGroupMember`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-globalreplicationgroupmember.html) property type.
    #[derive(Debug, Default)]
    pub struct GlobalReplicationGroupMember {
        /// Property [`ReplicationGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-globalreplicationgroupmember.html#cfn-elasticache-globalreplicationgroup-globalreplicationgroupmember-replicationgroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replication_group_id: Option<::Value<String>>,
        /// Property [`ReplicationGroupRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-globalreplicationgroupmember.html#cfn-elasticache-globalreplicationgroup-globalreplicationgroupmember-replicationgroupregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replication_group_region: Option<::Value<String>>,
        /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-globalreplicationgroupmember.html#cfn-elasticache-globalreplicationgroup-globalreplicationgroupmember-role).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GlobalReplicationGroupMember {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref replication_group_id) = self.replication_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupId", replication_group_id)?;
            }
            if let Some(ref replication_group_region) = self.replication_group_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupRegion", replication_group_region)?;
            }
            if let Some(ref role) = self.role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", role)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlobalReplicationGroupMember {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalReplicationGroupMember, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlobalReplicationGroupMember;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlobalReplicationGroupMember")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut replication_group_id: Option<::Value<String>> = None;
                    let mut replication_group_region: Option<::Value<String>> = None;
                    let mut role: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReplicationGroupId" => {
                                replication_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicationGroupRegion" => {
                                replication_group_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Role" => {
                                role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlobalReplicationGroupMember {
                        replication_group_id: replication_group_id,
                        replication_group_region: replication_group_region,
                        role: role,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::GlobalReplicationGroup.RegionalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-regionalconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RegionalConfiguration {
        /// Property [`ReplicationGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-regionalconfiguration.html#cfn-elasticache-globalreplicationgroup-regionalconfiguration-replicationgroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replication_group_id: Option<::Value<String>>,
        /// Property [`ReplicationGroupRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-regionalconfiguration.html#cfn-elasticache-globalreplicationgroup-regionalconfiguration-replicationgroupregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replication_group_region: Option<::Value<String>>,
        /// Property [`ReshardingConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-regionalconfiguration.html#cfn-elasticache-globalreplicationgroup-regionalconfiguration-reshardingconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resharding_configurations: Option<::ValueList<ReshardingConfiguration>>,
    }

    impl ::codec::SerializeValue for RegionalConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref replication_group_id) = self.replication_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupId", replication_group_id)?;
            }
            if let Some(ref replication_group_region) = self.replication_group_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationGroupRegion", replication_group_region)?;
            }
            if let Some(ref resharding_configurations) = self.resharding_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReshardingConfigurations", resharding_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegionalConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegionalConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegionalConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegionalConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut replication_group_id: Option<::Value<String>> = None;
                    let mut replication_group_region: Option<::Value<String>> = None;
                    let mut resharding_configurations: Option<::ValueList<ReshardingConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReplicationGroupId" => {
                                replication_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicationGroupRegion" => {
                                replication_group_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReshardingConfigurations" => {
                                resharding_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegionalConfiguration {
                        replication_group_id: replication_group_id,
                        replication_group_region: replication_group_region,
                        resharding_configurations: resharding_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::GlobalReplicationGroup.ReshardingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-reshardingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ReshardingConfiguration {
        /// Property [`NodeGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-reshardingconfiguration.html#cfn-elasticache-globalreplicationgroup-reshardingconfiguration-nodegroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub node_group_id: Option<::Value<String>>,
        /// Property [`PreferredAvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-reshardingconfiguration.html#cfn-elasticache-globalreplicationgroup-reshardingconfiguration-preferredavailabilityzones).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preferred_availability_zones: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ReshardingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref node_group_id) = self.node_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeGroupId", node_group_id)?;
            }
            if let Some(ref preferred_availability_zones) = self.preferred_availability_zones {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredAvailabilityZones", preferred_availability_zones)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReshardingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReshardingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReshardingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReshardingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut node_group_id: Option<::Value<String>> = None;
                    let mut preferred_availability_zones: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NodeGroupId" => {
                                node_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreferredAvailabilityZones" => {
                                preferred_availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReshardingConfiguration {
                        node_group_id: node_group_id,
                        preferred_availability_zones: preferred_availability_zones,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod replication_group {
    //! Property types for the `ReplicationGroup` resource.

    /// The [`AWS::ElastiCache::ReplicationGroup.CloudWatchLogsDestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-cloudwatchlogsdestinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogsDestinationDetails {
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-cloudwatchlogsdestinationdetails.html#cfn-elasticache-replicationgroup-cloudwatchlogsdestinationdetails-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogsDestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", &self.log_group)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogsDestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogsDestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogsDestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogsDestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogsDestinationDetails {
                        log_group: log_group.ok_or(::serde::de::Error::missing_field("LogGroup"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ReplicationGroup.DestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-destinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationDetails {
        /// Property [`CloudWatchLogsDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-destinationdetails.html#cfn-elasticache-replicationgroup-destinationdetails-cloudwatchlogsdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_details: Option<::Value<CloudWatchLogsDestinationDetails>>,
        /// Property [`KinesisFirehoseDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-destinationdetails.html#cfn-elasticache-replicationgroup-destinationdetails-kinesisfirehosedetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_details: Option<::Value<KinesisFirehoseDestinationDetails>>,
    }

    impl ::codec::SerializeValue for DestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs_details) = self.cloud_watch_logs_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsDetails", cloud_watch_logs_details)?;
            }
            if let Some(ref kinesis_firehose_details) = self.kinesis_firehose_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseDetails", kinesis_firehose_details)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_details: Option<::Value<CloudWatchLogsDestinationDetails>> = None;
                    let mut kinesis_firehose_details: Option<::Value<KinesisFirehoseDestinationDetails>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsDetails" => {
                                cloud_watch_logs_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisFirehoseDetails" => {
                                kinesis_firehose_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationDetails {
                        cloud_watch_logs_details: cloud_watch_logs_details,
                        kinesis_firehose_details: kinesis_firehose_details,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ReplicationGroup.KinesisFirehoseDestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-kinesisfirehosedestinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseDestinationDetails {
        /// Property [`DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-kinesisfirehosedestinationdetails.html#cfn-elasticache-replicationgroup-kinesisfirehosedestinationdetails-deliverystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseDestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStream", &self.delivery_stream)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseDestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseDestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseDestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseDestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStream" => {
                                delivery_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseDestinationDetails {
                        delivery_stream: delivery_stream.ok_or(::serde::de::Error::missing_field("DeliveryStream"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ReplicationGroup.LogDeliveryConfigurationRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct LogDeliveryConfigurationRequest {
        /// Property [`DestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html#cfn-elasticache-replicationgroup-logdeliveryconfigurationrequest-destinationdetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_details: ::Value<DestinationDetails>,
        /// Property [`DestinationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html#cfn-elasticache-replicationgroup-logdeliveryconfigurationrequest-destinationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_type: ::Value<String>,
        /// Property [`LogFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html#cfn-elasticache-replicationgroup-logdeliveryconfigurationrequest-logformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_format: ::Value<String>,
        /// Property [`LogType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html#cfn-elasticache-replicationgroup-logdeliveryconfigurationrequest-logtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for LogDeliveryConfigurationRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationDetails", &self.destination_details)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationType", &self.destination_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogFormat", &self.log_format)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogType", &self.log_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogDeliveryConfigurationRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogDeliveryConfigurationRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogDeliveryConfigurationRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogDeliveryConfigurationRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_details: Option<::Value<DestinationDetails>> = None;
                    let mut destination_type: Option<::Value<String>> = None;
                    let mut log_format: Option<::Value<String>> = None;
                    let mut log_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationDetails" => {
                                destination_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationType" => {
                                destination_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogFormat" => {
                                log_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogType" => {
                                log_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogDeliveryConfigurationRequest {
                        destination_details: destination_details.ok_or(::serde::de::Error::missing_field("DestinationDetails"))?,
                        destination_type: destination_type.ok_or(::serde::de::Error::missing_field("DestinationType"))?,
                        log_format: log_format.ok_or(::serde::de::Error::missing_field("LogFormat"))?,
                        log_type: log_type.ok_or(::serde::de::Error::missing_field("LogType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ReplicationGroup.NodeGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeGroupConfiguration {
        /// Property [`NodeGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html#cfn-elasticache-replicationgroup-nodegroupconfiguration-nodegroupid).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub node_group_id: Option<::Value<String>>,
        /// Property [`PrimaryAvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html#cfn-elasticache-replicationgroup-nodegroupconfiguration-primaryavailabilityzone).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub primary_availability_zone: Option<::Value<String>>,
        /// Property [`ReplicaAvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html#cfn-elasticache-replicationgroup-nodegroupconfiguration-replicaavailabilityzones).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub replica_availability_zones: Option<::ValueList<String>>,
        /// Property [`ReplicaCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html#cfn-elasticache-replicationgroup-nodegroupconfiguration-replicacount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub replica_count: Option<::Value<u32>>,
        /// Property [`Slots`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html#cfn-elasticache-replicationgroup-nodegroupconfiguration-slots).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub slots: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NodeGroupConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref node_group_id) = self.node_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeGroupId", node_group_id)?;
            }
            if let Some(ref primary_availability_zone) = self.primary_availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryAvailabilityZone", primary_availability_zone)?;
            }
            if let Some(ref replica_availability_zones) = self.replica_availability_zones {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaAvailabilityZones", replica_availability_zones)?;
            }
            if let Some(ref replica_count) = self.replica_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaCount", replica_count)?;
            }
            if let Some(ref slots) = self.slots {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slots", slots)?;
            }
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
                    let mut node_group_id: Option<::Value<String>> = None;
                    let mut primary_availability_zone: Option<::Value<String>> = None;
                    let mut replica_availability_zones: Option<::ValueList<String>> = None;
                    let mut replica_count: Option<::Value<u32>> = None;
                    let mut slots: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NodeGroupId" => {
                                node_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryAvailabilityZone" => {
                                primary_availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicaAvailabilityZones" => {
                                replica_availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicaCount" => {
                                replica_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slots" => {
                                slots = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeGroupConfiguration {
                        node_group_id: node_group_id,
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

pub mod serverless_cache {
    //! Property types for the `ServerlessCache` resource.

    /// The [`AWS::ElastiCache::ServerlessCache.CacheUsageLimits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-cacheusagelimits.html) property type.
    #[derive(Debug, Default)]
    pub struct CacheUsageLimits {
        /// Property [`DataStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-cacheusagelimits.html#cfn-elasticache-serverlesscache-cacheusagelimits-datastorage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_storage: Option<::Value<DataStorage>>,
        /// Property [`ECPUPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-cacheusagelimits.html#cfn-elasticache-serverlesscache-cacheusagelimits-ecpupersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecpu_per_second: Option<::Value<ECPUPerSecond>>,
    }

    impl ::codec::SerializeValue for CacheUsageLimits {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_storage) = self.data_storage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataStorage", data_storage)?;
            }
            if let Some(ref ecpu_per_second) = self.ecpu_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ECPUPerSecond", ecpu_per_second)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CacheUsageLimits {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheUsageLimits, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheUsageLimits;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheUsageLimits")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_storage: Option<::Value<DataStorage>> = None;
                    let mut ecpu_per_second: Option<::Value<ECPUPerSecond>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataStorage" => {
                                data_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ECPUPerSecond" => {
                                ecpu_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheUsageLimits {
                        data_storage: data_storage,
                        ecpu_per_second: ecpu_per_second,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ServerlessCache.DataStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-datastorage.html) property type.
    #[derive(Debug, Default)]
    pub struct DataStorage {
        /// Property [`Maximum`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-datastorage.html#cfn-elasticache-serverlesscache-datastorage-maximum).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum: ::Value<u32>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-datastorage.html#cfn-elasticache-serverlesscache-datastorage-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Maximum", &self.maximum)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum: Option<::Value<u32>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Maximum" => {
                                maximum = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataStorage {
                        maximum: maximum.ok_or(::serde::de::Error::missing_field("Maximum"))?,
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ServerlessCache.ECPUPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-ecpupersecond.html) property type.
    #[derive(Debug, Default)]
    pub struct ECPUPerSecond {
        /// Property [`Maximum`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-ecpupersecond.html#cfn-elasticache-serverlesscache-ecpupersecond-maximum).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ECPUPerSecond {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Maximum", &self.maximum)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ECPUPerSecond {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ECPUPerSecond, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ECPUPerSecond;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ECPUPerSecond")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Maximum" => {
                                maximum = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ECPUPerSecond {
                        maximum: maximum.ok_or(::serde::de::Error::missing_field("Maximum"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElastiCache::ServerlessCache.Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-endpoint.html#cfn-elasticache-serverlesscache-endpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-endpoint.html#cfn-elasticache-serverlesscache-endpoint-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Endpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Endpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoint {
                        address: address,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::ElastiCache::User.AuthenticationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-user-authenticationmode.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticationMode {
        /// Property [`Passwords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-user-authenticationmode.html#cfn-elasticache-user-authenticationmode-passwords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub passwords: Option<::ValueList<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-user-authenticationmode.html#cfn-elasticache-user-authenticationmode-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticationMode {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref passwords) = self.passwords {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Passwords", passwords)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticationMode {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticationMode, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticationMode;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticationMode")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut passwords: Option<::ValueList<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Passwords" => {
                                passwords = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticationMode {
                        passwords: passwords,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
