//! Types for the `Athena` service.

/// The [`AWS::Athena::NamedQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html) resource type.
#[derive(Debug)]
pub struct NamedQuery {
    properties: NamedQueryProperties
}

/// Properties for the `NamedQuery` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct NamedQueryProperties {
    /// Property `Database`.
    #[serde(rename = "Database")]
    pub database: ::Value<String>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `QueryString`.
    #[serde(rename = "QueryString")]
    pub query_string: ::Value<String>,
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
