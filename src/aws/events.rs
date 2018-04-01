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
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `EventPattern`.
    #[serde(rename = "EventPattern")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<::Value<::json::Value>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<::Value<String>>,
    /// Property `ScheduleExpression`.
    #[serde(rename = "ScheduleExpression")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<::Value<String>>,
    /// Property `State`.
    #[serde(rename = "State")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<::Value<String>>,
    /// Property `Targets`.
    #[serde(rename = "Targets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<::ValueList<self::rule::Target>>,
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
        #[serde(rename = "TaskCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub task_count: Option<::Value<u32>>,
        /// Property `TaskDefinitionArn`.
        #[serde(rename = "TaskDefinitionArn")]
        pub task_definition_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(EcsParameters);

    /// The [`AWS::Events::Rule.InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputTransformer {
        /// Property `InputPathsMap`.
        #[serde(rename = "InputPathsMap")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_paths_map: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `InputTemplate`.
        #[serde(rename = "InputTemplate")]
        pub input_template: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InputTransformer);

    /// The [`AWS::Events::Rule.KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisParameters {
        /// Property `PartitionKeyPath`.
        #[serde(rename = "PartitionKeyPath")]
        pub partition_key_path: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisParameters);

    /// The [`AWS::Events::Rule.RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RunCommandParameters {
        /// Property `RunCommandTargets`.
        #[serde(rename = "RunCommandTargets")]
        pub run_command_targets: ::ValueList<RunCommandTarget>,
    }

    cfn_internal__inherit_codec_impls!(RunCommandParameters);

    /// The [`AWS::Events::Rule.RunCommandTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RunCommandTarget {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Values`.
        #[serde(rename = "Values")]
        pub values: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(RunCommandTarget);

    /// The [`AWS::Events::Rule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Target {
        /// Property `Arn`.
        #[serde(rename = "Arn")]
        pub arn: ::Value<String>,
        /// Property `EcsParameters`.
        #[serde(rename = "EcsParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ecs_parameters: Option<::Value<EcsParameters>>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Input`.
        #[serde(rename = "Input")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input: Option<::Value<String>>,
        /// Property `InputPath`.
        #[serde(rename = "InputPath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_path: Option<::Value<String>>,
        /// Property `InputTransformer`.
        #[serde(rename = "InputTransformer")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_transformer: Option<::Value<InputTransformer>>,
        /// Property `KinesisParameters`.
        #[serde(rename = "KinesisParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis_parameters: Option<::Value<KinesisParameters>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
        /// Property `RunCommandParameters`.
        #[serde(rename = "RunCommandParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub run_command_parameters: Option<::Value<RunCommandParameters>>,
    }

    cfn_internal__inherit_codec_impls!(Target);
}
