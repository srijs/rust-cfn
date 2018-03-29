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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<::json::Value>,
    /// Property `AdvancedOptions`.
    #[serde(rename="AdvancedOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<::std::collections::HashMap<String, String>>,
    /// Property `DomainName`.
    #[serde(rename="DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Property `EBSOptions`.
    #[serde(rename="EBSOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_options: Option<self::domain::EBSOptions>,
    /// Property `ElasticsearchClusterConfig`.
    #[serde(rename="ElasticsearchClusterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_cluster_config: Option<self::domain::ElasticsearchClusterConfig>,
    /// Property `ElasticsearchVersion`.
    #[serde(rename="ElasticsearchVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    /// Property `SnapshotOptions`.
    #[serde(rename="SnapshotOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<self::domain::SnapshotOptions>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `VPCOptions`.
    #[serde(rename="VPCOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<self::domain::VPCOptions>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ebs_enabled: Option<bool>,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iops: Option<u32>,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<u32>,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<String>,
    }

    /// The [`AWS::Elasticsearch::Domain.ElasticsearchClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticsearchClusterConfig {
        /// Property `DedicatedMasterCount`.
        #[serde(rename="DedicatedMasterCount")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dedicated_master_count: Option<u32>,
        /// Property `DedicatedMasterEnabled`.
        #[serde(rename="DedicatedMasterEnabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dedicated_master_enabled: Option<bool>,
        /// Property `DedicatedMasterType`.
        #[serde(rename="DedicatedMasterType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dedicated_master_type: Option<String>,
        /// Property `InstanceCount`.
        #[serde(rename="InstanceCount")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_count: Option<u32>,
        /// Property `InstanceType`.
        #[serde(rename="InstanceType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_type: Option<String>,
        /// Property `ZoneAwarenessEnabled`.
        #[serde(rename="ZoneAwarenessEnabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_awareness_enabled: Option<bool>,
    }

    /// The [`AWS::Elasticsearch::Domain.SnapshotOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SnapshotOptions {
        /// Property `AutomatedSnapshotStartHour`.
        #[serde(rename="AutomatedSnapshotStartHour")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub automated_snapshot_start_hour: Option<u32>,
    }

    /// The [`AWS::Elasticsearch::Domain.VPCOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VPCOptions {
        /// Property `SecurityGroupIds`.
        #[serde(rename="SecurityGroupIds")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub security_group_ids: Option<Vec<String>>,
        /// Property `SubnetIds`.
        #[serde(rename="SubnetIds")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subnet_ids: Option<Vec<String>>,
    }
}
