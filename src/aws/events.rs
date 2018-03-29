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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `EventPattern`.
    #[serde(rename="EventPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<::json::Value>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// Property `ScheduleExpression`.
    #[serde(rename="ScheduleExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    /// Property `State`.
    #[serde(rename="State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<self::rule::Target>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub task_count: Option<u32>,
        /// Property `TaskDefinitionArn`.
        #[serde(rename="TaskDefinitionArn")]
        pub task_definition_arn: String,
    }

    /// The [`AWS::Events::Rule.InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputTransformer {
        /// Property `InputPathsMap`.
        #[serde(rename="InputPathsMap")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input_paths_map: Option<::std::collections::HashMap<String, String>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ecs_parameters: Option<EcsParameters>,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Input`.
        #[serde(rename="Input")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input: Option<String>,
        /// Property `InputPath`.
        #[serde(rename="InputPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input_path: Option<String>,
        /// Property `InputTransformer`.
        #[serde(rename="InputTransformer")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input_transformer: Option<InputTransformer>,
        /// Property `KinesisParameters`.
        #[serde(rename="KinesisParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kinesis_parameters: Option<KinesisParameters>,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<String>,
        /// Property `RunCommandParameters`.
        #[serde(rename="RunCommandParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub run_command_parameters: Option<RunCommandParameters>,
    }
}
