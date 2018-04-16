//! Types for the `Events` service.

/// The [`AWS::Events::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html) resource type.
#[derive(Debug, Default)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Default)]
pub struct RuleProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-eventpattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_pattern: Option<::Value<::json::Value>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-scheduleexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression: Option<::Value<String>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: Option<::ValueList<self::rule::Target>>,
}

impl ::serde::Serialize for RuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_pattern) = self.event_pattern {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventPattern", event_pattern)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref schedule_expression) = self.schedule_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref targets) = self.targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut event_pattern: Option<::Value<::json::Value>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut schedule_expression: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::rule::Target>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventPattern" => {
                            event_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpression" => {
                            schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleProperties {
                    description: description,
                    event_pattern: event_pattern,
                    name: name,
                    role_arn: role_arn,
                    schedule_expression: schedule_expression,
                    state: state,
                    targets: targets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Rule {
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
    #[derive(Debug, Default)]
    pub struct EcsParameters {
        /// Property [`TaskCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-taskcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_count: Option<::Value<u32>>,
        /// Property [`TaskDefinitionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-taskdefinitionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_definition_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for EcsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref task_count) = self.task_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskCount", task_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinitionArn", &self.task_definition_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut task_count: Option<::Value<u32>> = None;
                    let mut task_definition_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TaskCount" => {
                                task_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskDefinitionArn" => {
                                task_definition_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsParameters {
                        task_count: task_count,
                        task_definition_arn: task_definition_arn.ok_or(::serde::de::Error::missing_field("TaskDefinitionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html) property type.
    #[derive(Debug, Default)]
    pub struct InputTransformer {
        /// Property [`InputPathsMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html#cfn-events-rule-inputtransformer-inputpathsmap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_paths_map: Option<::ValueMap<String>>,
        /// Property [`InputTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html#cfn-events-rule-inputtransformer-inputtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_template: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputTransformer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_paths_map) = self.input_paths_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPathsMap", input_paths_map)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTemplate", &self.input_template)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputTransformer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputTransformer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputTransformer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputTransformer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_paths_map: Option<::ValueMap<String>> = None;
                    let mut input_template: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputPathsMap" => {
                                input_paths_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTemplate" => {
                                input_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputTransformer {
                        input_paths_map: input_paths_map,
                        input_template: input_template.ok_or(::serde::de::Error::missing_field("InputTemplate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisParameters {
        /// Property [`PartitionKeyPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html#cfn-events-rule-kinesisparameters-partitionkeypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKeyPath", &self.partition_key_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKeyPath" => {
                                partition_key_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisParameters {
                        partition_key_path: partition_key_path.ok_or(::serde::de::Error::missing_field("PartitionKeyPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RunCommandParameters {
        /// Property [`RunCommandTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html#cfn-events-rule-runcommandparameters-runcommandtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub run_command_targets: ::ValueList<RunCommandTarget>,
    }

    impl ::codec::SerializeValue for RunCommandParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunCommandTargets", &self.run_command_targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunCommandParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunCommandParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunCommandParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunCommandParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut run_command_targets: Option<::ValueList<RunCommandTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RunCommandTargets" => {
                                run_command_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunCommandParameters {
                        run_command_targets: run_command_targets.ok_or(::serde::de::Error::missing_field("RunCommandTargets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RunCommandTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct RunCommandTarget {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html#cfn-events-rule-runcommandtarget-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html#cfn-events-rule-runcommandtarget-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for RunCommandTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunCommandTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunCommandTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunCommandTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunCommandTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunCommandTarget {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-ecsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecs_parameters: Option<::Value<EcsParameters>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-input).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input: Option<::Value<String>>,
        /// Property [`InputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-inputpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_path: Option<::Value<String>>,
        /// Property [`InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-inputtransformer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_transformer: Option<::Value<InputTransformer>>,
        /// Property [`KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-kinesisparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_parameters: Option<::Value<KinesisParameters>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-runcommandparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub run_command_parameters: Option<::Value<RunCommandParameters>>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref ecs_parameters) = self.ecs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsParameters", ecs_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref input) = self.input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", input)?;
            }
            if let Some(ref input_path) = self.input_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPath", input_path)?;
            }
            if let Some(ref input_transformer) = self.input_transformer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTransformer", input_transformer)?;
            }
            if let Some(ref kinesis_parameters) = self.kinesis_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisParameters", kinesis_parameters)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref run_command_parameters) = self.run_command_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunCommandParameters", run_command_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut ecs_parameters: Option<::Value<EcsParameters>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut input: Option<::Value<String>> = None;
                    let mut input_path: Option<::Value<String>> = None;
                    let mut input_transformer: Option<::Value<InputTransformer>> = None;
                    let mut kinesis_parameters: Option<::Value<KinesisParameters>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut run_command_parameters: Option<::Value<RunCommandParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcsParameters" => {
                                ecs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Input" => {
                                input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputPath" => {
                                input_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTransformer" => {
                                input_transformer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisParameters" => {
                                kinesis_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunCommandParameters" => {
                                run_command_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        ecs_parameters: ecs_parameters,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        input: input,
                        input_path: input_path,
                        input_transformer: input_transformer,
                        kinesis_parameters: kinesis_parameters,
                        role_arn: role_arn,
                        run_command_parameters: run_command_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
