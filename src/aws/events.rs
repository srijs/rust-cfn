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
    pub event_pattern: ::json::Value,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
    #[serde(rename="State")]
    pub state: String,
    #[serde(rename="Targets")]
    pub targets: Vec<self::rule::Target>,
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

pub mod rule {
    #[derive(Serialize, Deserialize)]
    pub struct EcsParameters {
        #[serde(rename="TaskCount")]
        pub task_count: u32,
        #[serde(rename="TaskDefinitionArn")]
        pub task_definition_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputTransformer {
        #[serde(rename="InputPathsMap")]
        pub input_paths_map: ::std::collections::HashMap<String, String>,
        #[serde(rename="InputTemplate")]
        pub input_template: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisParameters {
        #[serde(rename="PartitionKeyPath")]
        pub partition_key_path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RunCommandParameters {
        #[serde(rename="RunCommandTargets")]
        pub run_command_targets: Vec<RunCommandTarget>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RunCommandTarget {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Target {
        #[serde(rename="Arn")]
        pub arn: String,
        #[serde(rename="EcsParameters")]
        pub ecs_parameters: EcsParameters,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Input")]
        pub input: String,
        #[serde(rename="InputPath")]
        pub input_path: String,
        #[serde(rename="InputTransformer")]
        pub input_transformer: InputTransformer,
        #[serde(rename="KinesisParameters")]
        pub kinesis_parameters: KinesisParameters,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="RunCommandParameters")]
        pub run_command_parameters: RunCommandParameters,
    }

}

