/// The [`AWS::Elasticsearch::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Serialize, Deserialize)]
pub struct DomainProperties {
    #[serde(rename="AccessPolicies")]
    pub access_policies: ::json::Value,
    #[serde(rename="AdvancedOptions")]
    pub advanced_options: ::std::collections::HashMap<String, String>,
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="EBSOptions")]
    pub ebs_options: self::domain::EBSOptions,
    #[serde(rename="ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: self::domain::ElasticsearchClusterConfig,
    #[serde(rename="ElasticsearchVersion")]
    pub elasticsearch_version: String,
    #[serde(rename="SnapshotOptions")]
    pub snapshot_options: self::domain::SnapshotOptions,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

pub mod domain {
    #[derive(Serialize, Deserialize)]
    pub struct EBSOptions {
        #[serde(rename="EBSEnabled")]
        pub ebs_enabled: bool,
        #[serde(rename="Iops")]
        pub iops: u32,
        #[serde(rename="VolumeSize")]
        pub volume_size: u32,
        #[serde(rename="VolumeType")]
        pub volume_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ElasticsearchClusterConfig {
        #[serde(rename="DedicatedMasterCount")]
        pub dedicated_master_count: u32,
        #[serde(rename="DedicatedMasterEnabled")]
        pub dedicated_master_enabled: bool,
        #[serde(rename="DedicatedMasterType")]
        pub dedicated_master_type: String,
        #[serde(rename="InstanceCount")]
        pub instance_count: u32,
        #[serde(rename="InstanceType")]
        pub instance_type: String,
        #[serde(rename="ZoneAwarenessEnabled")]
        pub zone_awareness_enabled: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SnapshotOptions {
        #[serde(rename="AutomatedSnapshotStartHour")]
        pub automated_snapshot_start_hour: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VPCOptions {
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
    }

}

