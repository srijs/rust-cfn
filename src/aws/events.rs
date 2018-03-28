/// The [`AWS::Events::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Serialize, Deserialize)]
pub struct RuleProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="EventPattern")]
    pub event_pattern: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RoleArn")]
    pub role_arn: (),
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: (),
    #[serde(rename="State")]
    pub state: (),
    #[serde(rename="Targets")]
    pub targets: (),
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

