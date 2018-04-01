//! Types for the `SDB` service.

/// The [`AWS::SDB::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simpledb.html) resource type.
#[derive(Debug)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::SDB::Domain";
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
