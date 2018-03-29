/// The [`AWS::SDB::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-simpledb.html) resource type.
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Serialize, Deserialize)]
pub struct DomainProperties {
    #[serde(rename="Description")]
    pub description: String,
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

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

