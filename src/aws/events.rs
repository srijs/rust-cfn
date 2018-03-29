//! Types for the `Events` service.

/// The [`AWS::Events::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html) resource type.
#[derive(Debug)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleProperties {
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `EventPattern`.
    #[serde(rename="EventPattern")]
    pub event_pattern: ::json::Value,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    /// Property `ScheduleExpression`.
    #[serde(rename="ScheduleExpression")]
    pub schedule_expression: String,
    /// Property `State`.
    #[serde(rename="State")]
    pub state: String,
    /// Property `Targets`.
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

impl ::private::Sealed for Rule {}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::Events::Rule.EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EcsParameters {
        /// Property `TaskCount`.
        #[serde(rename="TaskCount")]
        pub task_count: u32,
        /// Property `TaskDefinitionArn`.
        #[serde(rename="TaskDefinitionArn")]
        pub task_definition_arn: String,
    }

    /// The [`AWS::Events::Rule.InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputTransformer {
        /// Property `InputPathsMap`.
        #[serde(rename="InputPathsMap")]
        pub input_paths_map: ::std::collections::HashMap<String, String>,
        /// Property `InputTemplate`.
        #[serde(rename="InputTemplate")]
        pub input_template: String,
    }

    /// The [`AWS::Events::Rule.KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisParameters {
        /// Property `PartitionKeyPath`.
        #[serde(rename="PartitionKeyPath")]
        pub partition_key_path: String,
    }

    /// The [`AWS::Events::Rule.RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RunCommandParameters {
        /// Property `RunCommandTargets`.
        #[serde(rename="RunCommandTargets")]
        pub run_command_targets: Vec<RunCommandTarget>,
    }

    /// The [`AWS::Events::Rule.RunCommandTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RunCommandTarget {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::Events::Rule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Arn`.
        #[serde(rename="Arn")]
        pub arn: String,
        /// Property `EcsParameters`.
        #[serde(rename="EcsParameters")]
        pub ecs_parameters: EcsParameters,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Input`.
        #[serde(rename="Input")]
        pub input: String,
        /// Property `InputPath`.
        #[serde(rename="InputPath")]
        pub input_path: String,
        /// Property `InputTransformer`.
        #[serde(rename="InputTransformer")]
        pub input_transformer: InputTransformer,
        /// Property `KinesisParameters`.
        #[serde(rename="KinesisParameters")]
        pub kinesis_parameters: KinesisParameters,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `RunCommandParameters`.
        #[serde(rename="RunCommandParameters")]
        pub run_command_parameters: RunCommandParameters,
    }
}
