//! Types for the `Synthetics` service.

/// The [`AWS::Synthetics::Canary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html) resource type.
#[derive(Debug, Default)]
pub struct Canary {
    properties: CanaryProperties
}

/// Properties for the `Canary` resource.
#[derive(Debug, Default)]
pub struct CanaryProperties {
    /// Property [`ArtifactS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-artifacts3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub artifact_s3_location: ::Value<String>,
    /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-code).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code: ::Value<self::canary::Code>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-executionrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role_arn: ::Value<String>,
    /// Property [`FailureRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-failureretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub failure_retention_period: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RunConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-runconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub run_config: Option<::Value<self::canary::RunConfig>>,
    /// Property [`RuntimeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-runtimeversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub runtime_version: ::Value<String>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: ::Value<self::canary::Schedule>,
    /// Property [`StartCanaryAfterCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-startcanaryaftercreation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_canary_after_creation: ::Value<bool>,
    /// Property [`SuccessRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-successretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub success_retention_period: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VPCConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-synthetics-canary.html#cfn-synthetics-canary-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::canary::VPCConfig>>,
}

impl ::serde::Serialize for CanaryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArtifactS3Location", &self.artifact_s3_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
        if let Some(ref failure_retention_period) = self.failure_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureRetentionPeriod", failure_retention_period)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref run_config) = self.run_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunConfig", run_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeVersion", &self.runtime_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", &self.schedule)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartCanaryAfterCreation", &self.start_canary_after_creation)?;
        if let Some(ref success_retention_period) = self.success_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessRetentionPeriod", success_retention_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCConfig", vpc_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CanaryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CanaryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CanaryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CanaryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut artifact_s3_location: Option<::Value<String>> = None;
                let mut code: Option<::Value<self::canary::Code>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut failure_retention_period: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut run_config: Option<::Value<self::canary::RunConfig>> = None;
                let mut runtime_version: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<self::canary::Schedule>> = None;
                let mut start_canary_after_creation: Option<::Value<bool>> = None;
                let mut success_retention_period: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_config: Option<::Value<self::canary::VPCConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ArtifactS3Location" => {
                            artifact_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Code" => {
                            code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FailureRetentionPeriod" => {
                            failure_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RunConfig" => {
                            run_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimeVersion" => {
                            runtime_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartCanaryAfterCreation" => {
                            start_canary_after_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SuccessRetentionPeriod" => {
                            success_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CanaryProperties {
                    artifact_s3_location: artifact_s3_location.ok_or(::serde::de::Error::missing_field("ArtifactS3Location"))?,
                    code: code.ok_or(::serde::de::Error::missing_field("Code"))?,
                    execution_role_arn: execution_role_arn.ok_or(::serde::de::Error::missing_field("ExecutionRoleArn"))?,
                    failure_retention_period: failure_retention_period,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    run_config: run_config,
                    runtime_version: runtime_version.ok_or(::serde::de::Error::missing_field("RuntimeVersion"))?,
                    schedule: schedule.ok_or(::serde::de::Error::missing_field("Schedule"))?,
                    start_canary_after_creation: start_canary_after_creation.ok_or(::serde::de::Error::missing_field("StartCanaryAfterCreation"))?,
                    success_retention_period: success_retention_period,
                    tags: tags,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Canary {
    type Properties = CanaryProperties;
    const TYPE: &'static str = "AWS::Synthetics::Canary";
    fn properties(&self) -> &CanaryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CanaryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Canary {}

impl From<CanaryProperties> for Canary {
    fn from(properties: CanaryProperties) -> Canary {
        Canary { properties }
    }
}

pub mod canary {
    //! Property types for the `Canary` resource.

    /// The [`AWS::Synthetics::Canary.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html) property type.
    #[derive(Debug, Default)]
    pub struct Code {
        /// Property [`Handler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html#cfn-synthetics-canary-code-handler).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub handler: ::Value<String>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html#cfn-synthetics-canary-code-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: Option<::Value<String>>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html#cfn-synthetics-canary-code-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: Option<::Value<String>>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html#cfn-synthetics-canary-code-s3objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_version: Option<::Value<String>>,
        /// Property [`Script`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-code.html#cfn-synthetics-canary-code-script).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub script: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Code {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handler", &self.handler)?;
            if let Some(ref s3_bucket) = self.s3_bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", s3_bucket)?;
            }
            if let Some(ref s3_key) = self.s3_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", s3_key)?;
            }
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            if let Some(ref script) = self.script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Script", script)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Code {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Code, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Code;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Code")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut handler: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;
                    let mut s3_object_version: Option<::Value<String>> = None;
                    let mut script: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Handler" => {
                                handler = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Script" => {
                                script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Code {
                        handler: handler.ok_or(::serde::de::Error::missing_field("Handler"))?,
                        s3_bucket: s3_bucket,
                        s3_key: s3_key,
                        s3_object_version: s3_object_version,
                        script: script,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Synthetics::Canary.RunConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RunConfig {
        /// Property [`ActiveTracing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html#cfn-synthetics-canary-runconfig-activetracing).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub active_tracing: Option<::Value<bool>>,
        /// Property [`EnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html#cfn-synthetics-canary-runconfig-environmentvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_variables: Option<::ValueMap<String>>,
        /// Property [`MemoryInMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html#cfn-synthetics-canary-runconfig-memoryinmb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory_in_mb: Option<::Value<u32>>,
        /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-runconfig.html#cfn-synthetics-canary-runconfig-timeoutinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RunConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref active_tracing) = self.active_tracing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveTracing", active_tracing)?;
            }
            if let Some(ref environment_variables) = self.environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentVariables", environment_variables)?;
            }
            if let Some(ref memory_in_mb) = self.memory_in_mb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryInMB", memory_in_mb)?;
            }
            if let Some(ref timeout_in_seconds) = self.timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", timeout_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active_tracing: Option<::Value<bool>> = None;
                    let mut environment_variables: Option<::ValueMap<String>> = None;
                    let mut memory_in_mb: Option<::Value<u32>> = None;
                    let mut timeout_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActiveTracing" => {
                                active_tracing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentVariables" => {
                                environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemoryInMB" => {
                                memory_in_mb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInSeconds" => {
                                timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunConfig {
                        active_tracing: active_tracing,
                        environment_variables: environment_variables,
                        memory_in_mb: memory_in_mb,
                        timeout_in_seconds: timeout_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Synthetics::Canary.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-schedule.html#cfn-synthetics-canary-schedule-durationinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_seconds: Option<::Value<String>>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-schedule.html#cfn-synthetics-canary-schedule-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_seconds) = self.duration_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", duration_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_in_seconds: Option<::Value<String>> = None;
                    let mut expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInSeconds" => {
                                duration_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        duration_in_seconds: duration_in_seconds,
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Synthetics::Canary.VPCConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VPCConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-vpcconfig.html#cfn-synthetics-canary-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-vpcconfig.html#cfn-synthetics-canary-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-synthetics-canary-vpcconfig.html#cfn-synthetics-canary-vpcconfig-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VPCConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VPCConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VPCConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VPCConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VPCConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VPCConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
