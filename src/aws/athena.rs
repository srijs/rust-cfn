/// The [`AWS::Athena::NamedQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html) resource type.
pub struct NamedQuery {
    properties: NamedQueryProperties
}

/// Properties for the `NamedQuery` resource.
#[derive(Serialize, Deserialize)]
pub struct NamedQueryProperties {
    #[serde(rename="Database")]
    pub database: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="QueryString")]
    pub query_string: String,
}

impl<'a> ::Resource<'a> for NamedQuery {
    type Properties = NamedQueryProperties;
    const TYPE: &'static str = "AWS::Athena::NamedQuery";
    fn properties(&self) -> &NamedQueryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NamedQueryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NamedQuery {}

impl From<NamedQueryProperties> for NamedQuery {
    fn from(properties: NamedQueryProperties) -> NamedQuery {
        NamedQuery { properties }
    }
}

