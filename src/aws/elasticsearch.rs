//! Types for the `Elasticsearch` service.

/// The [`AWS::Elasticsearch::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html) resource type.
#[derive(Debug)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainProperties {
    /// Property `AccessPolicies`.
    #[serde(rename="AccessPolicies")]
    pub access_policies: ::json::Value,
    /// Property `AdvancedOptions`.
    #[serde(rename="AdvancedOptions")]
    pub advanced_options: ::std::collections::HashMap<String, String>,
    /// Property `DomainName`.
    #[serde(rename="DomainName")]
    pub domain_name: String,
    /// Property `EBSOptions`.
    #[serde(rename="EBSOptions")]
    pub ebs_options: self::domain::EBSOptions,
    /// Property `ElasticsearchClusterConfig`.
    #[serde(rename="ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: self::domain::ElasticsearchClusterConfig,
    /// Property `ElasticsearchVersion`.
    #[serde(rename="ElasticsearchVersion")]
    pub elasticsearch_version: String,
    /// Property `SnapshotOptions`.
    #[serde(rename="SnapshotOptions")]
    pub snapshot_options: self::domain::SnapshotOptions,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `VPCOptions`.
    #[serde(rename="VPCOptions")]
    pub vpc_options: self::domain::VPCOptions,
}

impl<'a> ::Resource<'a> for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::Elasticsearch::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::Elasticsearch::Domain.EBSOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EBSOptions {
        /// Property `EBSEnabled`.
        #[serde(rename="EBSEnabled")]
        pub ebs_enabled: bool,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        pub iops: u32,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    /// The [`AWS::Elasticsearch::Domain.ElasticsearchClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchClusterConfig {
        /// Property `DedicatedMasterCount`.
        #[serde(rename="DedicatedMasterCount")]
        pub dedicated_master_count: u32,
        /// Property `DedicatedMasterEnabled`.
        #[serde(rename="DedicatedMasterEnabled")]
        pub dedicated_master_enabled: bool,
        /// Property `DedicatedMasterType`.
        #[serde(rename="DedicatedMasterType")]
        pub dedicated_master_type: String,
        /// Property `InstanceCount`.
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        /// Property `InstanceType`.
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        /// Property `ZoneAwarenessEnabled`.
        #[serde(rename="ZoneAwarenessEnabled")]
        pub zone_awareness_enabled: bool,
    }

    /// The [`AWS::Elasticsearch::Domain.SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SnapshotOptions {
        /// Property `AutomatedSnapshotStartHour`.
        #[serde(rename="AutomatedSnapshotStartHour")]
        pub automated_snapshot_start_hour: u32,
    }

    /// The [`AWS::Elasticsearch::Domain.VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VPCOptions {
        /// Property `SecurityGroupIds`.
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        /// Property `SubnetIds`.
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
    }
}
