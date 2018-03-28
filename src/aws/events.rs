/// The [`AWS::Events::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Serialize, Deserialize)]
pub struct RuleProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EventPattern")]
    pub event_pattern: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename="State")]
    pub state: String,
    #[serde(rename="Targets")]
    pub targets: Vec<()>,
}

impl<'a> ::Resource<'a> for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::Events::Rule";
    fn properties(&self) -> &RuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleProperties {
        &mut self.properties
    }
}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

