/// The [`AWS::Elasticsearch::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Serialize, Deserialize)]
pub struct DomainProperties {
    #[serde(rename="AccessPolicies")]
    pub access_policies: String,
    #[serde(rename="AdvancedOptions")]
    pub advanced_options: ::std::collections::HashMap<String, String>,
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="EBSOptions")]
    pub ebs_options: (),
    #[serde(rename="ElasticsearchClusterConfig")]
    pub elasticsearch_cluster_config: (),
    #[serde(rename="ElasticsearchVersion")]
    pub elasticsearch_version: String,
    #[serde(rename="SnapshotOptions")]
    pub snapshot_options: (),
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="VPCOptions")]
    pub vpc_options: (),
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

