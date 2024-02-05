//! Types for the `MSK` service.

/// The [`AWS::MSK::BatchScramSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-batchscramsecret.html) resource type.
#[derive(Debug, Default)]
pub struct BatchScramSecret {
    properties: BatchScramSecretProperties
}

/// Properties for the `BatchScramSecret` resource.
#[derive(Debug, Default)]
pub struct BatchScramSecretProperties {
    /// Property [`ClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-batchscramsecret.html#cfn-msk-batchscramsecret-clusterarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_arn: ::Value<String>,
    /// Property [`SecretArnList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-batchscramsecret.html#cfn-msk-batchscramsecret-secretarnlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secret_arn_list: Option<::ValueList<String>>,
}

impl ::serde::Serialize for BatchScramSecretProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterArn", &self.cluster_arn)?;
        if let Some(ref secret_arn_list) = self.secret_arn_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArnList", secret_arn_list)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BatchScramSecretProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchScramSecretProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BatchScramSecretProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BatchScramSecretProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_arn: Option<::Value<String>> = None;
                let mut secret_arn_list: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterArn" => {
                            cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretArnList" => {
                            secret_arn_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BatchScramSecretProperties {
                    cluster_arn: cluster_arn.ok_or(::serde::de::Error::missing_field("ClusterArn"))?,
                    secret_arn_list: secret_arn_list,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BatchScramSecret {
    type Properties = BatchScramSecretProperties;
    const TYPE: &'static str = "AWS::MSK::BatchScramSecret";
    fn properties(&self) -> &BatchScramSecretProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BatchScramSecretProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BatchScramSecret {}

impl From<BatchScramSecretProperties> for BatchScramSecret {
    fn from(properties: BatchScramSecretProperties) -> BatchScramSecret {
        BatchScramSecret { properties }
    }
}

/// The [`AWS::MSK::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`BrokerNodeGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-brokernodegroupinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub broker_node_group_info: ::Value<self::cluster::BrokerNodeGroupInfo>,
    /// Property [`ClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-clientauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_authentication: Option<::Value<self::cluster::ClientAuthentication>>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`ConfigurationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-configurationinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration_info: Option<::Value<self::cluster::ConfigurationInfo>>,
    /// Property [`CurrentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-currentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub current_version: Option<::Value<String>>,
    /// Property [`EncryptionInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-encryptioninfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_info: Option<::Value<self::cluster::EncryptionInfo>>,
    /// Property [`EnhancedMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-enhancedmonitoring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enhanced_monitoring: Option<::Value<String>>,
    /// Property [`KafkaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-kafkaversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kafka_version: ::Value<String>,
    /// Property [`LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-logginginfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_info: Option<::Value<self::cluster::LoggingInfo>>,
    /// Property [`NumberOfBrokerNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-numberofbrokernodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_broker_nodes: ::Value<u32>,
    /// Property [`OpenMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-openmonitoring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_monitoring: Option<::Value<self::cluster::OpenMonitoring>>,
    /// Property [`StorageMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-storagemode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_mode: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html#cfn-msk-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrokerNodeGroupInfo", &self.broker_node_group_info)?;
        if let Some(ref client_authentication) = self.client_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAuthentication", client_authentication)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref configuration_info) = self.configuration_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationInfo", configuration_info)?;
        }
        if let Some(ref current_version) = self.current_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentVersion", current_version)?;
        }
        if let Some(ref encryption_info) = self.encryption_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionInfo", encryption_info)?;
        }
        if let Some(ref enhanced_monitoring) = self.enhanced_monitoring {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedMonitoring", enhanced_monitoring)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaVersion", &self.kafka_version)?;
        if let Some(ref logging_info) = self.logging_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingInfo", logging_info)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfBrokerNodes", &self.number_of_broker_nodes)?;
        if let Some(ref open_monitoring) = self.open_monitoring {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenMonitoring", open_monitoring)?;
        }
        if let Some(ref storage_mode) = self.storage_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageMode", storage_mode)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut broker_node_group_info: Option<::Value<self::cluster::BrokerNodeGroupInfo>> = None;
                let mut client_authentication: Option<::Value<self::cluster::ClientAuthentication>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut configuration_info: Option<::Value<self::cluster::ConfigurationInfo>> = None;
                let mut current_version: Option<::Value<String>> = None;
                let mut encryption_info: Option<::Value<self::cluster::EncryptionInfo>> = None;
                let mut enhanced_monitoring: Option<::Value<String>> = None;
                let mut kafka_version: Option<::Value<String>> = None;
                let mut logging_info: Option<::Value<self::cluster::LoggingInfo>> = None;
                let mut number_of_broker_nodes: Option<::Value<u32>> = None;
                let mut open_monitoring: Option<::Value<self::cluster::OpenMonitoring>> = None;
                let mut storage_mode: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BrokerNodeGroupInfo" => {
                            broker_node_group_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientAuthentication" => {
                            client_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationInfo" => {
                            configuration_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CurrentVersion" => {
                            current_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionInfo" => {
                            encryption_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedMonitoring" => {
                            enhanced_monitoring = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaVersion" => {
                            kafka_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingInfo" => {
                            logging_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfBrokerNodes" => {
                            number_of_broker_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenMonitoring" => {
                            open_monitoring = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageMode" => {
                            storage_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    broker_node_group_info: broker_node_group_info.ok_or(::serde::de::Error::missing_field("BrokerNodeGroupInfo"))?,
                    client_authentication: client_authentication,
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    configuration_info: configuration_info,
                    current_version: current_version,
                    encryption_info: encryption_info,
                    enhanced_monitoring: enhanced_monitoring,
                    kafka_version: kafka_version.ok_or(::serde::de::Error::missing_field("KafkaVersion"))?,
                    logging_info: logging_info,
                    number_of_broker_nodes: number_of_broker_nodes.ok_or(::serde::de::Error::missing_field("NumberOfBrokerNodes"))?,
                    open_monitoring: open_monitoring,
                    storage_mode: storage_mode,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::MSK::Cluster";
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

/// The [`AWS::MSK::ClusterPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-clusterpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterPolicy {
    properties: ClusterPolicyProperties
}

/// Properties for the `ClusterPolicy` resource.
#[derive(Debug, Default)]
pub struct ClusterPolicyProperties {
    /// Property [`ClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-clusterpolicy.html#cfn-msk-clusterpolicy-clusterarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_arn: ::Value<String>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-clusterpolicy.html#cfn-msk-clusterpolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::Value<::json::Value>,
}

impl ::serde::Serialize for ClusterPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterArn", &self.cluster_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_arn: Option<::Value<String>> = None;
                let mut policy: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterArn" => {
                            cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterPolicyProperties {
                    cluster_arn: cluster_arn.ok_or(::serde::de::Error::missing_field("ClusterArn"))?,
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterPolicy {
    type Properties = ClusterPolicyProperties;
    const TYPE: &'static str = "AWS::MSK::ClusterPolicy";
    fn properties(&self) -> &ClusterPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterPolicy {}

impl From<ClusterPolicyProperties> for ClusterPolicy {
    fn from(properties: ClusterPolicyProperties) -> ClusterPolicy {
        ClusterPolicy { properties }
    }
}

/// The [`AWS::MSK::Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html) resource type.
#[derive(Debug, Default)]
pub struct Configuration {
    properties: ConfigurationProperties
}

/// Properties for the `Configuration` resource.
#[derive(Debug, Default)]
pub struct ConfigurationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html#cfn-msk-configuration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`KafkaVersionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html#cfn-msk-configuration-kafkaversionslist).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_versions_list: Option<::ValueList<String>>,
    /// Property [`LatestRevision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html#cfn-msk-configuration-latestrevision).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub latest_revision: Option<::Value<self::configuration::LatestRevision>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html#cfn-msk-configuration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ServerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html#cfn-msk-configuration-serverproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_properties: ::Value<String>,
}

impl ::serde::Serialize for ConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref kafka_versions_list) = self.kafka_versions_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaVersionsList", kafka_versions_list)?;
        }
        if let Some(ref latest_revision) = self.latest_revision {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LatestRevision", latest_revision)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerProperties", &self.server_properties)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut kafka_versions_list: Option<::ValueList<String>> = None;
                let mut latest_revision: Option<::Value<self::configuration::LatestRevision>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut server_properties: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaVersionsList" => {
                            kafka_versions_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LatestRevision" => {
                            latest_revision = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerProperties" => {
                            server_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationProperties {
                    description: description,
                    kafka_versions_list: kafka_versions_list,
                    latest_revision: latest_revision,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    server_properties: server_properties.ok_or(::serde::de::Error::missing_field("ServerProperties"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Configuration {
    type Properties = ConfigurationProperties;
    const TYPE: &'static str = "AWS::MSK::Configuration";
    fn properties(&self) -> &ConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Configuration {}

impl From<ConfigurationProperties> for Configuration {
    fn from(properties: ConfigurationProperties) -> Configuration {
        Configuration { properties }
    }
}

/// The [`AWS::MSK::Replicator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html) resource type.
#[derive(Debug, Default)]
pub struct Replicator {
    properties: ReplicatorProperties
}

/// Properties for the `Replicator` resource.
#[derive(Debug, Default)]
pub struct ReplicatorProperties {
    /// Property [`CurrentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-currentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub current_version: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`KafkaClusters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-kafkaclusters).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_clusters: ::ValueList<self::replicator::KafkaCluster>,
    /// Property [`ReplicationInfoList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-replicationinfolist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_info_list: ::ValueList<self::replicator::ReplicationInfo>,
    /// Property [`ReplicatorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-replicatorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replicator_name: ::Value<String>,
    /// Property [`ServiceExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-serviceexecutionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_execution_role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html#cfn-msk-replicator-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ReplicatorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref current_version) = self.current_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentVersion", current_version)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaClusters", &self.kafka_clusters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInfoList", &self.replication_info_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicatorName", &self.replicator_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceExecutionRoleArn", &self.service_execution_role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicatorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicatorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicatorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicatorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut current_version: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut kafka_clusters: Option<::ValueList<self::replicator::KafkaCluster>> = None;
                let mut replication_info_list: Option<::ValueList<self::replicator::ReplicationInfo>> = None;
                let mut replicator_name: Option<::Value<String>> = None;
                let mut service_execution_role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CurrentVersion" => {
                            current_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaClusters" => {
                            kafka_clusters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationInfoList" => {
                            replication_info_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicatorName" => {
                            replicator_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceExecutionRoleArn" => {
                            service_execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicatorProperties {
                    current_version: current_version,
                    description: description,
                    kafka_clusters: kafka_clusters.ok_or(::serde::de::Error::missing_field("KafkaClusters"))?,
                    replication_info_list: replication_info_list.ok_or(::serde::de::Error::missing_field("ReplicationInfoList"))?,
                    replicator_name: replicator_name.ok_or(::serde::de::Error::missing_field("ReplicatorName"))?,
                    service_execution_role_arn: service_execution_role_arn.ok_or(::serde::de::Error::missing_field("ServiceExecutionRoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Replicator {
    type Properties = ReplicatorProperties;
    const TYPE: &'static str = "AWS::MSK::Replicator";
    fn properties(&self) -> &ReplicatorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicatorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Replicator {}

impl From<ReplicatorProperties> for Replicator {
    fn from(properties: ReplicatorProperties) -> Replicator {
        Replicator { properties }
    }
}

/// The [`AWS::MSK::ServerlessCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html) resource type.
#[derive(Debug, Default)]
pub struct ServerlessCluster {
    properties: ServerlessClusterProperties
}

/// Properties for the `ServerlessCluster` resource.
#[derive(Debug, Default)]
pub struct ServerlessClusterProperties {
    /// Property [`ClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html#cfn-msk-serverlesscluster-clientauthentication).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_authentication: ::Value<self::serverless_cluster::ClientAuthentication>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html#cfn-msk-serverlesscluster-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html#cfn-msk-serverlesscluster-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`VpcConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html#cfn-msk-serverlesscluster-vpcconfigs).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_configs: ::ValueList<self::serverless_cluster::VpcConfig>,
}

impl ::serde::Serialize for ServerlessClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAuthentication", &self.client_authentication)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfigs", &self.vpc_configs)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServerlessClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerlessClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServerlessClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServerlessClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut client_authentication: Option<::Value<self::serverless_cluster::ClientAuthentication>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut vpc_configs: Option<::ValueList<self::serverless_cluster::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientAuthentication" => {
                            client_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfigs" => {
                            vpc_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServerlessClusterProperties {
                    client_authentication: client_authentication.ok_or(::serde::de::Error::missing_field("ClientAuthentication"))?,
                    cluster_name: cluster_name.ok_or(::serde::de::Error::missing_field("ClusterName"))?,
                    tags: tags,
                    vpc_configs: vpc_configs.ok_or(::serde::de::Error::missing_field("VpcConfigs"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServerlessCluster {
    type Properties = ServerlessClusterProperties;
    const TYPE: &'static str = "AWS::MSK::ServerlessCluster";
    fn properties(&self) -> &ServerlessClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServerlessClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServerlessCluster {}

impl From<ServerlessClusterProperties> for ServerlessCluster {
    fn from(properties: ServerlessClusterProperties) -> ServerlessCluster {
        ServerlessCluster { properties }
    }
}

/// The [`AWS::MSK::VpcConnection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html) resource type.
#[derive(Debug, Default)]
pub struct VpcConnection {
    properties: VpcConnectionProperties
}

/// Properties for the `VpcConnection` resource.
#[derive(Debug, Default)]
pub struct VpcConnectionProperties {
    /// Property [`Authentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-authentication).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authentication: ::Value<String>,
    /// Property [`ClientSubnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-clientsubnets).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_subnets: ::ValueList<String>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-securitygroups).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_groups: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`TargetClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-targetclusterarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_cluster_arn: ::Value<String>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html#cfn-msk-vpcconnection-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for VpcConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Authentication", &self.authentication)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSubnets", &self.client_subnets)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetClusterArn", &self.target_cluster_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VpcConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VpcConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VpcConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authentication: Option<::Value<String>> = None;
                let mut client_subnets: Option<::ValueList<String>> = None;
                let mut security_groups: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut target_cluster_arn: Option<::Value<String>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Authentication" => {
                            authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientSubnets" => {
                            client_subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetClusterArn" => {
                            target_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VpcConnectionProperties {
                    authentication: authentication.ok_or(::serde::de::Error::missing_field("Authentication"))?,
                    client_subnets: client_subnets.ok_or(::serde::de::Error::missing_field("ClientSubnets"))?,
                    security_groups: security_groups.ok_or(::serde::de::Error::missing_field("SecurityGroups"))?,
                    tags: tags,
                    target_cluster_arn: target_cluster_arn.ok_or(::serde::de::Error::missing_field("TargetClusterArn"))?,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VpcConnection {
    type Properties = VpcConnectionProperties;
    const TYPE: &'static str = "AWS::MSK::VpcConnection";
    fn properties(&self) -> &VpcConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VpcConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VpcConnection {}

impl From<VpcConnectionProperties> for VpcConnection {
    fn from(properties: VpcConnectionProperties) -> VpcConnection {
        VpcConnection { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::MSK::Cluster.BrokerLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokerlogs.html) property type.
    #[derive(Debug, Default)]
    pub struct BrokerLogs {
        /// Property [`CloudWatchLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokerlogs.html#cfn-msk-cluster-brokerlogs-cloudwatchlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs: Option<::Value<CloudWatchLogs>>,
        /// Property [`Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokerlogs.html#cfn-msk-cluster-brokerlogs-firehose).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose: Option<::Value<Firehose>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokerlogs.html#cfn-msk-cluster-brokerlogs-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3>>,
    }

    impl ::codec::SerializeValue for BrokerLogs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs) = self.cloud_watch_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogs", cloud_watch_logs)?;
            }
            if let Some(ref firehose) = self.firehose {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Firehose", firehose)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BrokerLogs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BrokerLogs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BrokerLogs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BrokerLogs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs: Option<::Value<CloudWatchLogs>> = None;
                    let mut firehose: Option<::Value<Firehose>> = None;
                    let mut s3: Option<::Value<S3>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogs" => {
                                cloud_watch_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Firehose" => {
                                firehose = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BrokerLogs {
                        cloud_watch_logs: cloud_watch_logs,
                        firehose: firehose,
                        s3: s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.BrokerNodeGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct BrokerNodeGroupInfo {
        /// Property [`BrokerAZDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-brokerazdistribution).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub broker_az_distribution: Option<::Value<String>>,
        /// Property [`ClientSubnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-clientsubnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub client_subnets: ::ValueList<String>,
        /// Property [`ConnectivityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-connectivityinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connectivity_info: Option<::Value<ConnectivityInfo>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-securitygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`StorageInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html#cfn-msk-cluster-brokernodegroupinfo-storageinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_info: Option<::Value<StorageInfo>>,
    }

    impl ::codec::SerializeValue for BrokerNodeGroupInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref broker_az_distribution) = self.broker_az_distribution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrokerAZDistribution", broker_az_distribution)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSubnets", &self.client_subnets)?;
            if let Some(ref connectivity_info) = self.connectivity_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectivityInfo", connectivity_info)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            if let Some(ref storage_info) = self.storage_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageInfo", storage_info)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BrokerNodeGroupInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BrokerNodeGroupInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BrokerNodeGroupInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BrokerNodeGroupInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut broker_az_distribution: Option<::Value<String>> = None;
                    let mut client_subnets: Option<::ValueList<String>> = None;
                    let mut connectivity_info: Option<::Value<ConnectivityInfo>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut storage_info: Option<::Value<StorageInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BrokerAZDistribution" => {
                                broker_az_distribution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSubnets" => {
                                client_subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectivityInfo" => {
                                connectivity_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageInfo" => {
                                storage_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BrokerNodeGroupInfo {
                        broker_az_distribution: broker_az_distribution,
                        client_subnets: client_subnets.ok_or(::serde::de::Error::missing_field("ClientSubnets"))?,
                        connectivity_info: connectivity_info,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        security_groups: security_groups,
                        storage_info: storage_info,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.ClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-clientauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientAuthentication {
        /// Property [`Sasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-clientauthentication.html#cfn-msk-cluster-clientauthentication-sasl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl: Option<::Value<Sasl>>,
        /// Property [`Tls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-clientauthentication.html#cfn-msk-cluster-clientauthentication-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<Tls>>,
        /// Property [`Unauthenticated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-clientauthentication.html#cfn-msk-cluster-clientauthentication-unauthenticated).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unauthenticated: Option<::Value<Unauthenticated>>,
    }

    impl ::codec::SerializeValue for ClientAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sasl) = self.sasl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sasl", sasl)?;
            }
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tls", tls)?;
            }
            if let Some(ref unauthenticated) = self.unauthenticated {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unauthenticated", unauthenticated)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sasl: Option<::Value<Sasl>> = None;
                    let mut tls: Option<::Value<Tls>> = None;
                    let mut unauthenticated: Option<::Value<Unauthenticated>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Sasl" => {
                                sasl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tls" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unauthenticated" => {
                                unauthenticated = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientAuthentication {
                        sasl: sasl,
                        tls: tls,
                        unauthenticated: unauthenticated,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.CloudWatchLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-cloudwatchlogs.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogs {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-cloudwatchlogs.html#cfn-msk-cluster-cloudwatchlogs-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-cloudwatchlogs.html#cfn-msk-cluster-cloudwatchlogs-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchLogs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref log_group) = self.log_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", log_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut log_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogs {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        log_group: log_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.ConfigurationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-configurationinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationInfo {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-configurationinfo.html#cfn-msk-cluster-configurationinfo-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-configurationinfo.html#cfn-msk-cluster-configurationinfo-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ConfigurationInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationInfo {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.ConnectivityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-connectivityinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectivityInfo {
        /// Property [`PublicAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-connectivityinfo.html#cfn-msk-cluster-connectivityinfo-publicaccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub public_access: Option<::Value<PublicAccess>>,
        /// Property [`VpcConnectivity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-connectivityinfo.html#cfn-msk-cluster-connectivityinfo-vpcconnectivity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_connectivity: Option<::Value<VpcConnectivity>>,
    }

    impl ::codec::SerializeValue for ConnectivityInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref public_access) = self.public_access {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicAccess", public_access)?;
            }
            if let Some(ref vpc_connectivity) = self.vpc_connectivity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConnectivity", vpc_connectivity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectivityInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectivityInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectivityInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectivityInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut public_access: Option<::Value<PublicAccess>> = None;
                    let mut vpc_connectivity: Option<::Value<VpcConnectivity>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PublicAccess" => {
                                public_access = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConnectivity" => {
                                vpc_connectivity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectivityInfo {
                        public_access: public_access,
                        vpc_connectivity: vpc_connectivity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.EBSStorageInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-ebsstorageinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct EBSStorageInfo {
        /// Property [`ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-ebsstorageinfo.html#cfn-msk-cluster-ebsstorageinfo-provisionedthroughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_throughput: Option<::Value<ProvisionedThroughput>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-ebsstorageinfo.html#cfn-msk-cluster-ebsstorageinfo-volumesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EBSStorageInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref provisioned_throughput) = self.provisioned_throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", provisioned_throughput)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EBSStorageInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EBSStorageInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EBSStorageInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EBSStorageInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provisioned_throughput: Option<::Value<ProvisionedThroughput>> = None;
                    let mut volume_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProvisionedThroughput" => {
                                provisioned_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EBSStorageInfo {
                        provisioned_throughput: provisioned_throughput,
                        volume_size: volume_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.EncryptionAtRest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionatrest.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionAtRest {
        /// Property [`DataVolumeKMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionatrest.html#cfn-msk-cluster-encryptionatrest-datavolumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_volume_kms_key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionAtRest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataVolumeKMSKeyId", &self.data_volume_kms_key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionAtRest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionAtRest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionAtRest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionAtRest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_volume_kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataVolumeKMSKeyId" => {
                                data_volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionAtRest {
                        data_volume_kms_key_id: data_volume_kms_key_id.ok_or(::serde::de::Error::missing_field("DataVolumeKMSKeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.EncryptionInTransit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionintransit.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionInTransit {
        /// Property [`ClientBroker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionintransit.html#cfn-msk-cluster-encryptionintransit-clientbroker).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_broker: Option<::Value<String>>,
        /// Property [`InCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionintransit.html#cfn-msk-cluster-encryptionintransit-incluster).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub in_cluster: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EncryptionInTransit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_broker) = self.client_broker {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientBroker", client_broker)?;
            }
            if let Some(ref in_cluster) = self.in_cluster {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InCluster", in_cluster)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionInTransit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionInTransit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionInTransit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionInTransit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_broker: Option<::Value<String>> = None;
                    let mut in_cluster: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientBroker" => {
                                client_broker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InCluster" => {
                                in_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionInTransit {
                        client_broker: client_broker,
                        in_cluster: in_cluster,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.EncryptionInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptioninfo.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionInfo {
        /// Property [`EncryptionAtRest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptioninfo.html#cfn-msk-cluster-encryptioninfo-encryptionatrest).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encryption_at_rest: Option<::Value<EncryptionAtRest>>,
        /// Property [`EncryptionInTransit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptioninfo.html#cfn-msk-cluster-encryptioninfo-encryptionintransit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_in_transit: Option<::Value<EncryptionInTransit>>,
    }

    impl ::codec::SerializeValue for EncryptionInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption_at_rest) = self.encryption_at_rest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAtRest", encryption_at_rest)?;
            }
            if let Some(ref encryption_in_transit) = self.encryption_in_transit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionInTransit", encryption_in_transit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_at_rest: Option<::Value<EncryptionAtRest>> = None;
                    let mut encryption_in_transit: Option<::Value<EncryptionInTransit>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionAtRest" => {
                                encryption_at_rest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionInTransit" => {
                                encryption_in_transit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionInfo {
                        encryption_at_rest: encryption_at_rest,
                        encryption_in_transit: encryption_in_transit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-firehose.html) property type.
    #[derive(Debug, Default)]
    pub struct Firehose {
        /// Property [`DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-firehose.html#cfn-msk-cluster-firehose-deliverystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-firehose.html#cfn-msk-cluster-firehose-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for Firehose {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delivery_stream) = self.delivery_stream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStream", delivery_stream)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Firehose {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Firehose, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Firehose;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Firehose")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStream" => {
                                delivery_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Firehose {
                        delivery_stream: delivery_stream,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-iam.html) property type.
    #[derive(Debug, Default)]
    pub struct Iam {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-iam.html#cfn-msk-cluster-iam-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for Iam {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Iam {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Iam, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Iam;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Iam")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Iam {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.JmxExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-jmxexporter.html) property type.
    #[derive(Debug, Default)]
    pub struct JmxExporter {
        /// Property [`EnabledInBroker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-jmxexporter.html#cfn-msk-cluster-jmxexporter-enabledinbroker).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled_in_broker: ::Value<bool>,
    }

    impl ::codec::SerializeValue for JmxExporter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnabledInBroker", &self.enabled_in_broker)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JmxExporter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JmxExporter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JmxExporter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JmxExporter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled_in_broker: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnabledInBroker" => {
                                enabled_in_broker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JmxExporter {
                        enabled_in_broker: enabled_in_broker.ok_or(::serde::de::Error::missing_field("EnabledInBroker"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-logginginfo.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingInfo {
        /// Property [`BrokerLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-logginginfo.html#cfn-msk-cluster-logginginfo-brokerlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub broker_logs: ::Value<BrokerLogs>,
    }

    impl ::codec::SerializeValue for LoggingInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrokerLogs", &self.broker_logs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut broker_logs: Option<::Value<BrokerLogs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BrokerLogs" => {
                                broker_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingInfo {
                        broker_logs: broker_logs.ok_or(::serde::de::Error::missing_field("BrokerLogs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.NodeExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-nodeexporter.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeExporter {
        /// Property [`EnabledInBroker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-nodeexporter.html#cfn-msk-cluster-nodeexporter-enabledinbroker).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled_in_broker: ::Value<bool>,
    }

    impl ::codec::SerializeValue for NodeExporter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnabledInBroker", &self.enabled_in_broker)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NodeExporter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeExporter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeExporter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeExporter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled_in_broker: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnabledInBroker" => {
                                enabled_in_broker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeExporter {
                        enabled_in_broker: enabled_in_broker.ok_or(::serde::de::Error::missing_field("EnabledInBroker"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.OpenMonitoring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-openmonitoring.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenMonitoring {
        /// Property [`Prometheus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-openmonitoring.html#cfn-msk-cluster-openmonitoring-prometheus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prometheus: ::Value<Prometheus>,
    }

    impl ::codec::SerializeValue for OpenMonitoring {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prometheus", &self.prometheus)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenMonitoring {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenMonitoring, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenMonitoring;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenMonitoring")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut prometheus: Option<::Value<Prometheus>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Prometheus" => {
                                prometheus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenMonitoring {
                        prometheus: prometheus.ok_or(::serde::de::Error::missing_field("Prometheus"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Prometheus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-prometheus.html) property type.
    #[derive(Debug, Default)]
    pub struct Prometheus {
        /// Property [`JmxExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-prometheus.html#cfn-msk-cluster-prometheus-jmxexporter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jmx_exporter: Option<::Value<JmxExporter>>,
        /// Property [`NodeExporter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-prometheus.html#cfn-msk-cluster-prometheus-nodeexporter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub node_exporter: Option<::Value<NodeExporter>>,
    }

    impl ::codec::SerializeValue for Prometheus {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref jmx_exporter) = self.jmx_exporter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JmxExporter", jmx_exporter)?;
            }
            if let Some(ref node_exporter) = self.node_exporter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeExporter", node_exporter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Prometheus {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Prometheus, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Prometheus;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Prometheus")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut jmx_exporter: Option<::Value<JmxExporter>> = None;
                    let mut node_exporter: Option<::Value<NodeExporter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JmxExporter" => {
                                jmx_exporter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NodeExporter" => {
                                node_exporter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Prometheus {
                        jmx_exporter: jmx_exporter,
                        node_exporter: node_exporter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-provisionedthroughput.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedThroughput {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-provisionedthroughput.html#cfn-msk-cluster-provisionedthroughput-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`VolumeThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-provisionedthroughput.html#cfn-msk-cluster-provisionedthroughput-volumethroughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_throughput: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ProvisionedThroughput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref volume_throughput) = self.volume_throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeThroughput", volume_throughput)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionedThroughput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedThroughput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedThroughput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedThroughput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut volume_throughput: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeThroughput" => {
                                volume_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedThroughput {
                        enabled: enabled,
                        volume_throughput: volume_throughput,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.PublicAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-publicaccess.html) property type.
    #[derive(Debug, Default)]
    pub struct PublicAccess {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-publicaccess.html#cfn-msk-cluster-publicaccess-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PublicAccess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublicAccess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicAccess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublicAccess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublicAccess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublicAccess {
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-s3.html) property type.
    #[derive(Debug, Default)]
    pub struct S3 {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-s3.html#cfn-msk-cluster-s3-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-s3.html#cfn-msk-cluster-s3-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-s3.html#cfn-msk-cluster-s3-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3 {
                        bucket: bucket,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Sasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-sasl.html) property type.
    #[derive(Debug, Default)]
    pub struct Sasl {
        /// Property [`Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-sasl.html#cfn-msk-cluster-sasl-iam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam: Option<::Value<Iam>>,
        /// Property [`Scram`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-sasl.html#cfn-msk-cluster-sasl-scram).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scram: Option<::Value<Scram>>,
    }

    impl ::codec::SerializeValue for Sasl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iam) = self.iam {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iam", iam)?;
            }
            if let Some(ref scram) = self.scram {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scram", scram)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Sasl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sasl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sasl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sasl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iam: Option<::Value<Iam>> = None;
                    let mut scram: Option<::Value<Scram>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iam" => {
                                iam = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scram" => {
                                scram = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sasl {
                        iam: iam,
                        scram: scram,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Scram`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-scram.html) property type.
    #[derive(Debug, Default)]
    pub struct Scram {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-scram.html#cfn-msk-cluster-scram-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for Scram {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scram {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scram, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scram;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scram")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scram {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.StorageInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-storageinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct StorageInfo {
        /// Property [`EBSStorageInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-storageinfo.html#cfn-msk-cluster-storageinfo-ebsstorageinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs_storage_info: Option<::Value<EBSStorageInfo>>,
    }

    impl ::codec::SerializeValue for StorageInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ebs_storage_info) = self.ebs_storage_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EBSStorageInfo", ebs_storage_info)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StorageInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StorageInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StorageInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ebs_storage_info: Option<::Value<EBSStorageInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EBSStorageInfo" => {
                                ebs_storage_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StorageInfo {
                        ebs_storage_info: ebs_storage_info,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Tls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-tls.html) property type.
    #[derive(Debug, Default)]
    pub struct Tls {
        /// Property [`CertificateAuthorityArnList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-tls.html#cfn-msk-cluster-tls-certificateauthorityarnlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_authority_arn_list: Option<::ValueList<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-tls.html#cfn-msk-cluster-tls-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Tls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_authority_arn_list) = self.certificate_authority_arn_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArnList", certificate_authority_arn_list)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_authority_arn_list: Option<::ValueList<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateAuthorityArnList" => {
                                certificate_authority_arn_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tls {
                        certificate_authority_arn_list: certificate_authority_arn_list,
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.Unauthenticated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-unauthenticated.html) property type.
    #[derive(Debug, Default)]
    pub struct Unauthenticated {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-unauthenticated.html#cfn-msk-cluster-unauthenticated-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for Unauthenticated {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Unauthenticated {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Unauthenticated, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Unauthenticated;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Unauthenticated")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Unauthenticated {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivity.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivity {
        /// Property [`ClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivity.html#cfn-msk-cluster-vpcconnectivity-clientauthentication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_authentication: Option<::Value<VpcConnectivityClientAuthentication>>,
    }

    impl ::codec::SerializeValue for VpcConnectivity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_authentication) = self.client_authentication {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAuthentication", client_authentication)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_authentication: Option<::Value<VpcConnectivityClientAuthentication>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientAuthentication" => {
                                client_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivity {
                        client_authentication: client_authentication,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivityClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityclientauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivityClientAuthentication {
        /// Property [`Sasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityclientauthentication.html#cfn-msk-cluster-vpcconnectivityclientauthentication-sasl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl: Option<::Value<VpcConnectivitySasl>>,
        /// Property [`Tls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityclientauthentication.html#cfn-msk-cluster-vpcconnectivityclientauthentication-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<VpcConnectivityTls>>,
    }

    impl ::codec::SerializeValue for VpcConnectivityClientAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sasl) = self.sasl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sasl", sasl)?;
            }
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tls", tls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivityClientAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivityClientAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivityClientAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivityClientAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sasl: Option<::Value<VpcConnectivitySasl>> = None;
                    let mut tls: Option<::Value<VpcConnectivityTls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Sasl" => {
                                sasl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tls" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivityClientAuthentication {
                        sasl: sasl,
                        tls: tls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivityIam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityiam.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivityIam {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityiam.html#cfn-msk-cluster-vpcconnectivityiam-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for VpcConnectivityIam {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivityIam {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivityIam, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivityIam;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivityIam")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivityIam {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivitySasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitysasl.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivitySasl {
        /// Property [`Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitysasl.html#cfn-msk-cluster-vpcconnectivitysasl-iam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam: Option<::Value<VpcConnectivityIam>>,
        /// Property [`Scram`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitysasl.html#cfn-msk-cluster-vpcconnectivitysasl-scram).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scram: Option<::Value<VpcConnectivityScram>>,
    }

    impl ::codec::SerializeValue for VpcConnectivitySasl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iam) = self.iam {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iam", iam)?;
            }
            if let Some(ref scram) = self.scram {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scram", scram)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivitySasl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivitySasl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivitySasl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivitySasl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iam: Option<::Value<VpcConnectivityIam>> = None;
                    let mut scram: Option<::Value<VpcConnectivityScram>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iam" => {
                                iam = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scram" => {
                                scram = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivitySasl {
                        iam: iam,
                        scram: scram,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivityScram`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityscram.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivityScram {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityscram.html#cfn-msk-cluster-vpcconnectivityscram-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for VpcConnectivityScram {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivityScram {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivityScram, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivityScram;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivityScram")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivityScram {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Cluster.VpcConnectivityTls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitytls.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectivityTls {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitytls.html#cfn-msk-cluster-vpcconnectivitytls-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for VpcConnectivityTls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectivityTls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectivityTls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectivityTls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectivityTls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectivityTls {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration {
    //! Property types for the `Configuration` resource.

    /// The [`AWS::MSK::Configuration.LatestRevision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-configuration-latestrevision.html) property type.
    #[derive(Debug, Default)]
    pub struct LatestRevision {
        /// Property [`CreationTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-configuration-latestrevision.html#cfn-msk-configuration-latestrevision-creationtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-configuration-latestrevision.html#cfn-msk-configuration-latestrevision-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-configuration-latestrevision.html#cfn-msk-configuration-latestrevision-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for LatestRevision {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref creation_time) = self.creation_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationTime", creation_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref revision) = self.revision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", revision)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LatestRevision {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LatestRevision, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LatestRevision;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LatestRevision")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut creation_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreationTime" => {
                                creation_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LatestRevision {
                        creation_time: creation_time,
                        description: description,
                        revision: revision,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod replicator {
    //! Property types for the `Replicator` resource.

    /// The [`AWS::MSK::Replicator.AmazonMskCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-amazonmskcluster.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonMskCluster {
        /// Property [`MskClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-amazonmskcluster.html#cfn-msk-replicator-amazonmskcluster-mskclusterarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub msk_cluster_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for AmazonMskCluster {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MskClusterArn", &self.msk_cluster_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonMskCluster {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonMskCluster, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonMskCluster;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonMskCluster")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut msk_cluster_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MskClusterArn" => {
                                msk_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmazonMskCluster {
                        msk_cluster_arn: msk_cluster_arn.ok_or(::serde::de::Error::missing_field("MskClusterArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Replicator.ConsumerGroupReplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html) property type.
    #[derive(Debug, Default)]
    pub struct ConsumerGroupReplication {
        /// Property [`ConsumerGroupsToExclude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html#cfn-msk-replicator-consumergroupreplication-consumergroupstoexclude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consumer_groups_to_exclude: Option<::ValueList<String>>,
        /// Property [`ConsumerGroupsToReplicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html#cfn-msk-replicator-consumergroupreplication-consumergroupstoreplicate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consumer_groups_to_replicate: ::ValueList<String>,
        /// Property [`DetectAndCopyNewConsumerGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html#cfn-msk-replicator-consumergroupreplication-detectandcopynewconsumergroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detect_and_copy_new_consumer_groups: Option<::Value<bool>>,
        /// Property [`SynchroniseConsumerGroupOffsets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html#cfn-msk-replicator-consumergroupreplication-synchroniseconsumergroupoffsets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub synchronise_consumer_group_offsets: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ConsumerGroupReplication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref consumer_groups_to_exclude) = self.consumer_groups_to_exclude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupsToExclude", consumer_groups_to_exclude)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupsToReplicate", &self.consumer_groups_to_replicate)?;
            if let Some(ref detect_and_copy_new_consumer_groups) = self.detect_and_copy_new_consumer_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectAndCopyNewConsumerGroups", detect_and_copy_new_consumer_groups)?;
            }
            if let Some(ref synchronise_consumer_group_offsets) = self.synchronise_consumer_group_offsets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SynchroniseConsumerGroupOffsets", synchronise_consumer_group_offsets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConsumerGroupReplication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConsumerGroupReplication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConsumerGroupReplication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConsumerGroupReplication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut consumer_groups_to_exclude: Option<::ValueList<String>> = None;
                    let mut consumer_groups_to_replicate: Option<::ValueList<String>> = None;
                    let mut detect_and_copy_new_consumer_groups: Option<::Value<bool>> = None;
                    let mut synchronise_consumer_group_offsets: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConsumerGroupsToExclude" => {
                                consumer_groups_to_exclude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConsumerGroupsToReplicate" => {
                                consumer_groups_to_replicate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DetectAndCopyNewConsumerGroups" => {
                                detect_and_copy_new_consumer_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SynchroniseConsumerGroupOffsets" => {
                                synchronise_consumer_group_offsets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConsumerGroupReplication {
                        consumer_groups_to_exclude: consumer_groups_to_exclude,
                        consumer_groups_to_replicate: consumer_groups_to_replicate.ok_or(::serde::de::Error::missing_field("ConsumerGroupsToReplicate"))?,
                        detect_and_copy_new_consumer_groups: detect_and_copy_new_consumer_groups,
                        synchronise_consumer_group_offsets: synchronise_consumer_group_offsets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Replicator.KafkaCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkacluster.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaCluster {
        /// Property [`AmazonMskCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkacluster.html#cfn-msk-replicator-kafkacluster-amazonmskcluster).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub amazon_msk_cluster: ::Value<AmazonMskCluster>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkacluster.html#cfn-msk-replicator-kafkacluster-vpcconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_config: ::Value<KafkaClusterClientVpcConfig>,
    }

    impl ::codec::SerializeValue for KafkaCluster {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonMskCluster", &self.amazon_msk_cluster)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", &self.vpc_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaCluster {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaCluster, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaCluster;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaCluster")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amazon_msk_cluster: Option<::Value<AmazonMskCluster>> = None;
                    let mut vpc_config: Option<::Value<KafkaClusterClientVpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AmazonMskCluster" => {
                                amazon_msk_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaCluster {
                        amazon_msk_cluster: amazon_msk_cluster.ok_or(::serde::de::Error::missing_field("AmazonMskCluster"))?,
                        vpc_config: vpc_config.ok_or(::serde::de::Error::missing_field("VpcConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Replicator.KafkaClusterClientVpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkaclusterclientvpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaClusterClientVpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkaclusterclientvpcconfig.html#cfn-msk-replicator-kafkaclusterclientvpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkaclusterclientvpcconfig.html#cfn-msk-replicator-kafkaclusterclientvpcconfig-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for KafkaClusterClientVpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaClusterClientVpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaClusterClientVpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaClusterClientVpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaClusterClientVpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaClusterClientVpcConfig {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Replicator.ReplicationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationInfo {
        /// Property [`ConsumerGroupReplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html#cfn-msk-replicator-replicationinfo-consumergroupreplication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consumer_group_replication: ::Value<ConsumerGroupReplication>,
        /// Property [`SourceKafkaClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html#cfn-msk-replicator-replicationinfo-sourcekafkaclusterarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_kafka_cluster_arn: ::Value<String>,
        /// Property [`TargetCompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html#cfn-msk-replicator-replicationinfo-targetcompressiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_compression_type: ::Value<String>,
        /// Property [`TargetKafkaClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html#cfn-msk-replicator-replicationinfo-targetkafkaclusterarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_kafka_cluster_arn: ::Value<String>,
        /// Property [`TopicReplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html#cfn-msk-replicator-replicationinfo-topicreplication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_replication: ::Value<TopicReplication>,
    }

    impl ::codec::SerializeValue for ReplicationInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupReplication", &self.consumer_group_replication)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceKafkaClusterArn", &self.source_kafka_cluster_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetCompressionType", &self.target_compression_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetKafkaClusterArn", &self.target_kafka_cluster_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicReplication", &self.topic_replication)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut consumer_group_replication: Option<::Value<ConsumerGroupReplication>> = None;
                    let mut source_kafka_cluster_arn: Option<::Value<String>> = None;
                    let mut target_compression_type: Option<::Value<String>> = None;
                    let mut target_kafka_cluster_arn: Option<::Value<String>> = None;
                    let mut topic_replication: Option<::Value<TopicReplication>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConsumerGroupReplication" => {
                                consumer_group_replication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceKafkaClusterArn" => {
                                source_kafka_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetCompressionType" => {
                                target_compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetKafkaClusterArn" => {
                                target_kafka_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicReplication" => {
                                topic_replication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationInfo {
                        consumer_group_replication: consumer_group_replication.ok_or(::serde::de::Error::missing_field("ConsumerGroupReplication"))?,
                        source_kafka_cluster_arn: source_kafka_cluster_arn.ok_or(::serde::de::Error::missing_field("SourceKafkaClusterArn"))?,
                        target_compression_type: target_compression_type.ok_or(::serde::de::Error::missing_field("TargetCompressionType"))?,
                        target_kafka_cluster_arn: target_kafka_cluster_arn.ok_or(::serde::de::Error::missing_field("TargetKafkaClusterArn"))?,
                        topic_replication: topic_replication.ok_or(::serde::de::Error::missing_field("TopicReplication"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::Replicator.TopicReplication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html) property type.
    #[derive(Debug, Default)]
    pub struct TopicReplication {
        /// Property [`CopyAccessControlListsForTopics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html#cfn-msk-replicator-topicreplication-copyaccesscontrollistsfortopics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_access_control_lists_for_topics: Option<::Value<bool>>,
        /// Property [`CopyTopicConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html#cfn-msk-replicator-topicreplication-copytopicconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_topic_configurations: Option<::Value<bool>>,
        /// Property [`DetectAndCopyNewTopics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html#cfn-msk-replicator-topicreplication-detectandcopynewtopics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detect_and_copy_new_topics: Option<::Value<bool>>,
        /// Property [`TopicsToExclude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html#cfn-msk-replicator-topicreplication-topicstoexclude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topics_to_exclude: Option<::ValueList<String>>,
        /// Property [`TopicsToReplicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html#cfn-msk-replicator-topicreplication-topicstoreplicate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topics_to_replicate: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for TopicReplication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_access_control_lists_for_topics) = self.copy_access_control_lists_for_topics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyAccessControlListsForTopics", copy_access_control_lists_for_topics)?;
            }
            if let Some(ref copy_topic_configurations) = self.copy_topic_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTopicConfigurations", copy_topic_configurations)?;
            }
            if let Some(ref detect_and_copy_new_topics) = self.detect_and_copy_new_topics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectAndCopyNewTopics", detect_and_copy_new_topics)?;
            }
            if let Some(ref topics_to_exclude) = self.topics_to_exclude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicsToExclude", topics_to_exclude)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicsToReplicate", &self.topics_to_replicate)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TopicReplication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicReplication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TopicReplication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TopicReplication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_access_control_lists_for_topics: Option<::Value<bool>> = None;
                    let mut copy_topic_configurations: Option<::Value<bool>> = None;
                    let mut detect_and_copy_new_topics: Option<::Value<bool>> = None;
                    let mut topics_to_exclude: Option<::ValueList<String>> = None;
                    let mut topics_to_replicate: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyAccessControlListsForTopics" => {
                                copy_access_control_lists_for_topics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTopicConfigurations" => {
                                copy_topic_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DetectAndCopyNewTopics" => {
                                detect_and_copy_new_topics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicsToExclude" => {
                                topics_to_exclude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicsToReplicate" => {
                                topics_to_replicate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TopicReplication {
                        copy_access_control_lists_for_topics: copy_access_control_lists_for_topics,
                        copy_topic_configurations: copy_topic_configurations,
                        detect_and_copy_new_topics: detect_and_copy_new_topics,
                        topics_to_exclude: topics_to_exclude,
                        topics_to_replicate: topics_to_replicate.ok_or(::serde::de::Error::missing_field("TopicsToReplicate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod serverless_cluster {
    //! Property types for the `ServerlessCluster` resource.

    /// The [`AWS::MSK::ServerlessCluster.ClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-clientauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientAuthentication {
        /// Property [`Sasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-clientauthentication.html#cfn-msk-serverlesscluster-clientauthentication-sasl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub sasl: ::Value<Sasl>,
    }

    impl ::codec::SerializeValue for ClientAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sasl", &self.sasl)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sasl: Option<::Value<Sasl>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Sasl" => {
                                sasl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientAuthentication {
                        sasl: sasl.ok_or(::serde::de::Error::missing_field("Sasl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::ServerlessCluster.Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-iam.html) property type.
    #[derive(Debug, Default)]
    pub struct Iam {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-iam.html#cfn-msk-serverlesscluster-iam-enabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for Iam {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Iam {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Iam, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Iam;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Iam")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Iam {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::ServerlessCluster.Sasl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-sasl.html) property type.
    #[derive(Debug, Default)]
    pub struct Sasl {
        /// Property [`Iam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-sasl.html#cfn-msk-serverlesscluster-sasl-iam).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iam: ::Value<Iam>,
    }

    impl ::codec::SerializeValue for Sasl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iam", &self.iam)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Sasl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sasl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sasl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sasl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iam: Option<::Value<Iam>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iam" => {
                                iam = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sasl {
                        iam: iam.ok_or(::serde::de::Error::missing_field("Iam"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MSK::ServerlessCluster.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-vpcconfig.html#cfn-msk-serverlesscluster-vpcconfig-securitygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-vpcconfig.html#cfn-msk-serverlesscluster-vpcconfig-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_groups: security_groups,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
