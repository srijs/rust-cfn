//! Types for the `Pipes` service.

/// The [`AWS::Pipes::Pipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html) resource type.
#[derive(Debug, Default)]
pub struct Pipe {
    properties: PipeProperties
}

/// Properties for the `Pipe` resource.
#[derive(Debug, Default)]
pub struct PipeProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DesiredState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-desiredstate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_state: Option<::Value<String>>,
    /// Property [`Enrichment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-enrichment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enrichment: Option<::Value<String>>,
    /// Property [`EnrichmentParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-enrichmentparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enrichment_parameters: Option<::Value<self::pipe::PipeEnrichmentParameters>>,
    /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-logconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_configuration: Option<::Value<self::pipe::PipeLogConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-source).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source: ::Value<String>,
    /// Property [`SourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-sourceparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_parameters: Option<::Value<self::pipe::PipeSourceParameters>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-target).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target: ::Value<String>,
    /// Property [`TargetParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html#cfn-pipes-pipe-targetparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_parameters: Option<::Value<self::pipe::PipeTargetParameters>>,
}

impl ::serde::Serialize for PipeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref desired_state) = self.desired_state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredState", desired_state)?;
        }
        if let Some(ref enrichment) = self.enrichment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enrichment", enrichment)?;
        }
        if let Some(ref enrichment_parameters) = self.enrichment_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnrichmentParameters", enrichment_parameters)?;
        }
        if let Some(ref log_configuration) = self.log_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref source_parameters) = self.source_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceParameters", source_parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
        if let Some(ref target_parameters) = self.target_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetParameters", target_parameters)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut desired_state: Option<::Value<String>> = None;
                let mut enrichment: Option<::Value<String>> = None;
                let mut enrichment_parameters: Option<::Value<self::pipe::PipeEnrichmentParameters>> = None;
                let mut log_configuration: Option<::Value<self::pipe::PipeLogConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut source: Option<::Value<String>> = None;
                let mut source_parameters: Option<::Value<self::pipe::PipeSourceParameters>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut target: Option<::Value<String>> = None;
                let mut target_parameters: Option<::Value<self::pipe::PipeTargetParameters>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredState" => {
                            desired_state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enrichment" => {
                            enrichment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnrichmentParameters" => {
                            enrichment_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogConfiguration" => {
                            log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceParameters" => {
                            source_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Target" => {
                            target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetParameters" => {
                            target_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PipeProperties {
                    description: description,
                    desired_state: desired_state,
                    enrichment: enrichment,
                    enrichment_parameters: enrichment_parameters,
                    log_configuration: log_configuration,
                    name: name,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    source_parameters: source_parameters,
                    tags: tags,
                    target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    target_parameters: target_parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Pipe {
    type Properties = PipeProperties;
    const TYPE: &'static str = "AWS::Pipes::Pipe";
    fn properties(&self) -> &PipeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipe {}

impl From<PipeProperties> for Pipe {
    fn from(properties: PipeProperties) -> Pipe {
        Pipe { properties }
    }
}

pub mod pipe {
    //! Property types for the `Pipe` resource.

    /// The [`AWS::Pipes::Pipe.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-awsvpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsVpcConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-awsvpcconfiguration.html#cfn-pipes-pipe-awsvpcconfiguration-assignpublicip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-awsvpcconfiguration.html#cfn-pipes-pipe-awsvpcconfiguration-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-awsvpcconfiguration.html#cfn-pipes-pipe-awsvpcconfiguration-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_public_ip: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsVpcConfiguration {
                        assign_public_ip: assign_public_ip,
                        security_groups: security_groups,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchArrayProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batcharrayproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchArrayProperties {
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batcharrayproperties.html#cfn-pipes-pipe-batcharrayproperties-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BatchArrayProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref size) = self.size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchArrayProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchArrayProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchArrayProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchArrayProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchArrayProperties {
                        size: size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchContainerOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchContainerOverrides {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html#cfn-pipes-pipe-batchcontaineroverrides-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html#cfn-pipes-pipe-batchcontaineroverrides-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<::ValueList<BatchEnvironmentVariable>>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html#cfn-pipes-pipe-batchcontaineroverrides-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`ResourceRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html#cfn-pipes-pipe-batchcontaineroverrides-resourcerequirements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_requirements: Option<::ValueList<BatchResourceRequirement>>,
    }

    impl ::codec::SerializeValue for BatchContainerOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref resource_requirements) = self.resource_requirements {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRequirements", resource_requirements)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchContainerOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchContainerOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchContainerOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchContainerOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<::ValueList<String>> = None;
                    let mut environment: Option<::ValueList<BatchEnvironmentVariable>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut resource_requirements: Option<::ValueList<BatchResourceRequirement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRequirements" => {
                                resource_requirements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchContainerOverrides {
                        command: command,
                        environment: environment,
                        instance_type: instance_type,
                        resource_requirements: resource_requirements,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchEnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchenvironmentvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchEnvironmentVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchenvironmentvariable.html#cfn-pipes-pipe-batchenvironmentvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchenvironmentvariable.html#cfn-pipes-pipe-batchenvironmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BatchEnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchEnvironmentVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchEnvironmentVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchEnvironmentVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchEnvironmentVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchEnvironmentVariable {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchJobDependency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchjobdependency.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchJobDependency {
        /// Property [`JobId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchjobdependency.html#cfn-pipes-pipe-batchjobdependency-jobid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_id: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchjobdependency.html#cfn-pipes-pipe-batchjobdependency-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BatchJobDependency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref job_id) = self.job_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobId", job_id)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchJobDependency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchJobDependency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchJobDependency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchJobDependency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut job_id: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JobId" => {
                                job_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchJobDependency {
                        job_id: job_id,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchResourceRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchresourcerequirement.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchResourceRequirement {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchresourcerequirement.html#cfn-pipes-pipe-batchresourcerequirement-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchresourcerequirement.html#cfn-pipes-pipe-batchresourcerequirement-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for BatchResourceRequirement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchResourceRequirement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchResourceRequirement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchResourceRequirement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchResourceRequirement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchResourceRequirement {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.BatchRetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchretrystrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchRetryStrategy {
        /// Property [`Attempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchretrystrategy.html#cfn-pipes-pipe-batchretrystrategy-attempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attempts: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BatchRetryStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attempts) = self.attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attempts", attempts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchRetryStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchRetryStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchRetryStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchRetryStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attempts: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attempts" => {
                                attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchRetryStrategy {
                        attempts: attempts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.CapacityProviderStrategyItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-capacityproviderstrategyitem.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityProviderStrategyItem {
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-capacityproviderstrategyitem.html#cfn-pipes-pipe-capacityproviderstrategyitem-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<u32>>,
        /// Property [`CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-capacityproviderstrategyitem.html#cfn-pipes-pipe-capacityproviderstrategyitem-capacityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-capacityproviderstrategyitem.html#cfn-pipes-pipe-capacityproviderstrategyitem-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CapacityProviderStrategyItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProvider", &self.capacity_provider)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityProviderStrategyItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderStrategyItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityProviderStrategyItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityProviderStrategyItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base: Option<::Value<u32>> = None;
                    let mut capacity_provider: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapacityProvider" => {
                                capacity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityProviderStrategyItem {
                        base: base,
                        capacity_provider: capacity_provider.ok_or(::serde::de::Error::missing_field("CapacityProvider"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.CloudwatchLogsLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-cloudwatchlogslogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchLogsLogDestination {
        /// Property [`LogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-cloudwatchlogslogdestination.html#cfn-pipes-pipe-cloudwatchlogslogdestination-loggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudwatchLogsLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_arn) = self.log_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupArn", log_group_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchLogsLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchLogsLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchLogsLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchLogsLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupArn" => {
                                log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchLogsLogDestination {
                        log_group_arn: log_group_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-deadletterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeadLetterConfig {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-deadletterconfig.html#cfn-pipes-pipe-deadletterconfig-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeadLetterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeadLetterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeadLetterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeadLetterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeadLetterConfig {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsContainerOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsContainerOverride {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-cpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu: Option<::Value<u32>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<::ValueList<EcsEnvironmentVariable>>,
        /// Property [`EnvironmentFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-environmentfiles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_files: Option<::ValueList<EcsEnvironmentFile>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: Option<::Value<u32>>,
        /// Property [`MemoryReservation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-memoryreservation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory_reservation: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ResourceRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html#cfn-pipes-pipe-ecscontaineroverride-resourcerequirements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_requirements: Option<::ValueList<EcsResourceRequirement>>,
    }

    impl ::codec::SerializeValue for EcsContainerOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref cpu) = self.cpu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", cpu)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref environment_files) = self.environment_files {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentFiles", environment_files)?;
            }
            if let Some(ref memory) = self.memory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
            }
            if let Some(ref memory_reservation) = self.memory_reservation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryReservation", memory_reservation)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref resource_requirements) = self.resource_requirements {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRequirements", resource_requirements)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsContainerOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsContainerOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsContainerOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsContainerOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<::ValueList<String>> = None;
                    let mut cpu: Option<::Value<u32>> = None;
                    let mut environment: Option<::ValueList<EcsEnvironmentVariable>> = None;
                    let mut environment_files: Option<::ValueList<EcsEnvironmentFile>> = None;
                    let mut memory: Option<::Value<u32>> = None;
                    let mut memory_reservation: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut resource_requirements: Option<::ValueList<EcsResourceRequirement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentFiles" => {
                                environment_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemoryReservation" => {
                                memory_reservation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRequirements" => {
                                resource_requirements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsContainerOverride {
                        command: command,
                        cpu: cpu,
                        environment: environment,
                        environment_files: environment_files,
                        memory: memory,
                        memory_reservation: memory_reservation,
                        name: name,
                        resource_requirements: resource_requirements,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsEnvironmentFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentfile.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsEnvironmentFile {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentfile.html#cfn-pipes-pipe-ecsenvironmentfile-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentfile.html#cfn-pipes-pipe-ecsenvironmentfile-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EcsEnvironmentFile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsEnvironmentFile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsEnvironmentFile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsEnvironmentFile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsEnvironmentFile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsEnvironmentFile {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsEnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsEnvironmentVariable {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentvariable.html#cfn-pipes-pipe-ecsenvironmentvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentvariable.html#cfn-pipes-pipe-ecsenvironmentvariable-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EcsEnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsEnvironmentVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsEnvironmentVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsEnvironmentVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsEnvironmentVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsEnvironmentVariable {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsEphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsephemeralstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsEphemeralStorage {
        /// Property [`SizeInGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsephemeralstorage.html#cfn-pipes-pipe-ecsephemeralstorage-sizeingib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_gi_b: ::Value<u32>,
    }

    impl ::codec::SerializeValue for EcsEphemeralStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGiB", &self.size_in_gi_b)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsEphemeralStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsEphemeralStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsEphemeralStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsEphemeralStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut size_in_gi_b: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SizeInGiB" => {
                                size_in_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsEphemeralStorage {
                        size_in_gi_b: size_in_gi_b.ok_or(::serde::de::Error::missing_field("SizeInGiB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsInferenceAcceleratorOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsinferenceacceleratoroverride.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsInferenceAcceleratorOverride {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsinferenceacceleratoroverride.html#cfn-pipes-pipe-ecsinferenceacceleratoroverride-devicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_name: Option<::Value<String>>,
        /// Property [`DeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsinferenceacceleratoroverride.html#cfn-pipes-pipe-ecsinferenceacceleratoroverride-devicetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EcsInferenceAcceleratorOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref device_name) = self.device_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", device_name)?;
            }
            if let Some(ref device_type) = self.device_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceType", device_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsInferenceAcceleratorOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsInferenceAcceleratorOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsInferenceAcceleratorOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsInferenceAcceleratorOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name: Option<::Value<String>> = None;
                    let mut device_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceType" => {
                                device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsInferenceAcceleratorOverride {
                        device_name: device_name,
                        device_type: device_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsResourceRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsresourcerequirement.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsResourceRequirement {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsresourcerequirement.html#cfn-pipes-pipe-ecsresourcerequirement-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsresourcerequirement.html#cfn-pipes-pipe-ecsresourcerequirement-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EcsResourceRequirement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsResourceRequirement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsResourceRequirement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsResourceRequirement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsResourceRequirement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsResourceRequirement {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.EcsTaskOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsTaskOverride {
        /// Property [`ContainerOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-containeroverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_overrides: Option<::ValueList<EcsContainerOverride>>,
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-cpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu: Option<::Value<String>>,
        /// Property [`EphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-ephemeralstorage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ephemeral_storage: Option<::Value<EcsEphemeralStorage>>,
        /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-executionrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role_arn: Option<::Value<String>>,
        /// Property [`InferenceAcceleratorOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-inferenceacceleratoroverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inference_accelerator_overrides: Option<::ValueList<EcsInferenceAcceleratorOverride>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: Option<::Value<String>>,
        /// Property [`TaskRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html#cfn-pipes-pipe-ecstaskoverride-taskrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EcsTaskOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_overrides) = self.container_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerOverrides", container_overrides)?;
            }
            if let Some(ref cpu) = self.cpu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", cpu)?;
            }
            if let Some(ref ephemeral_storage) = self.ephemeral_storage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EphemeralStorage", ephemeral_storage)?;
            }
            if let Some(ref execution_role_arn) = self.execution_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
            }
            if let Some(ref inference_accelerator_overrides) = self.inference_accelerator_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceAcceleratorOverrides", inference_accelerator_overrides)?;
            }
            if let Some(ref memory) = self.memory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
            }
            if let Some(ref task_role_arn) = self.task_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskRoleArn", task_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsTaskOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsTaskOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsTaskOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsTaskOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_overrides: Option<::ValueList<EcsContainerOverride>> = None;
                    let mut cpu: Option<::Value<String>> = None;
                    let mut ephemeral_storage: Option<::Value<EcsEphemeralStorage>> = None;
                    let mut execution_role_arn: Option<::Value<String>> = None;
                    let mut inference_accelerator_overrides: Option<::ValueList<EcsInferenceAcceleratorOverride>> = None;
                    let mut memory: Option<::Value<String>> = None;
                    let mut task_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerOverrides" => {
                                container_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EphemeralStorage" => {
                                ephemeral_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecutionRoleArn" => {
                                execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceAcceleratorOverrides" => {
                                inference_accelerator_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskRoleArn" => {
                                task_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsTaskOverride {
                        container_overrides: container_overrides,
                        cpu: cpu,
                        ephemeral_storage: ephemeral_storage,
                        execution_role_arn: execution_role_arn,
                        inference_accelerator_overrides: inference_accelerator_overrides,
                        memory: memory,
                        task_role_arn: task_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filter.html#cfn-pipes-pipe-filter-pattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pattern) = self.pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", pattern)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pattern: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pattern" => {
                                pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        pattern: pattern,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filtercriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterCriteria {
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filtercriteria.html#cfn-pipes-pipe-filtercriteria-filters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filters: Option<::ValueList<Filter>>,
    }

    impl ::codec::SerializeValue for FilterCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref filters) = self.filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut filters: Option<::ValueList<Filter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Filters" => {
                                filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterCriteria {
                        filters: filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.FirehoseLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-firehoselogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct FirehoseLogDestination {
        /// Property [`DeliveryStreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-firehoselogdestination.html#cfn-pipes-pipe-firehoselogdestination-deliverystreamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirehoseLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delivery_stream_arn) = self.delivery_stream_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamArn", delivery_stream_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirehoseLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirehoseLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirehoseLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirehoseLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStreamArn" => {
                                delivery_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirehoseLogDestination {
                        delivery_stream_arn: delivery_stream_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.MQBrokerAccessCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mqbrokeraccesscredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct MQBrokerAccessCredentials {
        /// Property [`BasicAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mqbrokeraccesscredentials.html#cfn-pipes-pipe-mqbrokeraccesscredentials-basicauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_auth: ::Value<String>,
    }

    impl ::codec::SerializeValue for MQBrokerAccessCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuth", &self.basic_auth)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MQBrokerAccessCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MQBrokerAccessCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MQBrokerAccessCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MQBrokerAccessCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut basic_auth: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasicAuth" => {
                                basic_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MQBrokerAccessCredentials {
                        basic_auth: basic_auth.ok_or(::serde::de::Error::missing_field("BasicAuth"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.MSKAccessCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mskaccesscredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct MSKAccessCredentials {
        /// Property [`ClientCertificateTlsAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mskaccesscredentials.html#cfn-pipes-pipe-mskaccesscredentials-clientcertificatetlsauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_certificate_tls_auth: Option<::Value<String>>,
        /// Property [`SaslScram512Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mskaccesscredentials.html#cfn-pipes-pipe-mskaccesscredentials-saslscram512auth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl_scram512_auth: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MSKAccessCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_certificate_tls_auth) = self.client_certificate_tls_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateTlsAuth", client_certificate_tls_auth)?;
            }
            if let Some(ref sasl_scram512_auth) = self.sasl_scram512_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SaslScram512Auth", sasl_scram512_auth)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MSKAccessCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MSKAccessCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MSKAccessCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MSKAccessCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_certificate_tls_auth: Option<::Value<String>> = None;
                    let mut sasl_scram512_auth: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientCertificateTlsAuth" => {
                                client_certificate_tls_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SaslScram512Auth" => {
                                sasl_scram512_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MSKAccessCredentials {
                        client_certificate_tls_auth: client_certificate_tls_auth,
                        sasl_scram512_auth: sasl_scram512_auth,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AwsvpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-networkconfiguration.html#cfn-pipes-pipe-networkconfiguration-awsvpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub awsvpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref awsvpc_configuration) = self.awsvpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsvpcConfiguration", awsvpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut awsvpc_configuration: Option<::Value<AwsVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsvpcConfiguration" => {
                                awsvpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        awsvpc_configuration: awsvpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeEnrichmentHttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmenthttpparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeEnrichmentHttpParameters {
        /// Property [`HeaderParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmenthttpparameters.html#cfn-pipes-pipe-pipeenrichmenthttpparameters-headerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_parameters: Option<::ValueMap<String>>,
        /// Property [`PathParameterValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmenthttpparameters.html#cfn-pipes-pipe-pipeenrichmenthttpparameters-pathparametervalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_parameter_values: Option<::ValueList<String>>,
        /// Property [`QueryStringParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmenthttpparameters.html#cfn-pipes-pipe-pipeenrichmenthttpparameters-querystringparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_parameters: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for PipeEnrichmentHttpParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref header_parameters) = self.header_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderParameters", header_parameters)?;
            }
            if let Some(ref path_parameter_values) = self.path_parameter_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathParameterValues", path_parameter_values)?;
            }
            if let Some(ref query_string_parameters) = self.query_string_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringParameters", query_string_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeEnrichmentHttpParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeEnrichmentHttpParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeEnrichmentHttpParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeEnrichmentHttpParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_parameters: Option<::ValueMap<String>> = None;
                    let mut path_parameter_values: Option<::ValueList<String>> = None;
                    let mut query_string_parameters: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderParameters" => {
                                header_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathParameterValues" => {
                                path_parameter_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringParameters" => {
                                query_string_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeEnrichmentHttpParameters {
                        header_parameters: header_parameters,
                        path_parameter_values: path_parameter_values,
                        query_string_parameters: query_string_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeEnrichmentParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmentparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeEnrichmentParameters {
        /// Property [`HttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmentparameters.html#cfn-pipes-pipe-pipeenrichmentparameters-httpparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_parameters: Option<::Value<PipeEnrichmentHttpParameters>>,
        /// Property [`InputTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmentparameters.html#cfn-pipes-pipe-pipeenrichmentparameters-inputtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_template: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeEnrichmentParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_parameters) = self.http_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpParameters", http_parameters)?;
            }
            if let Some(ref input_template) = self.input_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTemplate", input_template)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeEnrichmentParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeEnrichmentParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeEnrichmentParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeEnrichmentParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_parameters: Option<::Value<PipeEnrichmentHttpParameters>> = None;
                    let mut input_template: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpParameters" => {
                                http_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTemplate" => {
                                input_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeEnrichmentParameters {
                        http_parameters: http_parameters,
                        input_template: input_template,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeLogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeLogConfiguration {
        /// Property [`CloudwatchLogsLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html#cfn-pipes-pipe-pipelogconfiguration-cloudwatchlogslogdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_logs_log_destination: Option<::Value<CloudwatchLogsLogDestination>>,
        /// Property [`FirehoseLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html#cfn-pipes-pipe-pipelogconfiguration-firehoselogdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose_log_destination: Option<::Value<FirehoseLogDestination>>,
        /// Property [`IncludeExecutionData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html#cfn-pipes-pipe-pipelogconfiguration-includeexecutiondata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_execution_data: Option<::ValueList<String>>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html#cfn-pipes-pipe-pipelogconfiguration-level).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub level: Option<::Value<String>>,
        /// Property [`S3LogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html#cfn-pipes-pipe-pipelogconfiguration-s3logdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_log_destination: Option<::Value<S3LogDestination>>,
    }

    impl ::codec::SerializeValue for PipeLogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloudwatch_logs_log_destination) = self.cloudwatch_logs_log_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchLogsLogDestination", cloudwatch_logs_log_destination)?;
            }
            if let Some(ref firehose_log_destination) = self.firehose_log_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirehoseLogDestination", firehose_log_destination)?;
            }
            if let Some(ref include_execution_data) = self.include_execution_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeExecutionData", include_execution_data)?;
            }
            if let Some(ref level) = self.level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", level)?;
            }
            if let Some(ref s3_log_destination) = self.s3_log_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3LogDestination", s3_log_destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeLogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeLogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeLogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeLogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloudwatch_logs_log_destination: Option<::Value<CloudwatchLogsLogDestination>> = None;
                    let mut firehose_log_destination: Option<::Value<FirehoseLogDestination>> = None;
                    let mut include_execution_data: Option<::ValueList<String>> = None;
                    let mut level: Option<::Value<String>> = None;
                    let mut s3_log_destination: Option<::Value<S3LogDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudwatchLogsLogDestination" => {
                                cloudwatch_logs_log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirehoseLogDestination" => {
                                firehose_log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeExecutionData" => {
                                include_execution_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3LogDestination" => {
                                s3_log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeLogConfiguration {
                        cloudwatch_logs_log_destination: cloudwatch_logs_log_destination,
                        firehose_log_destination: firehose_log_destination,
                        include_execution_data: include_execution_data,
                        level: level,
                        s3_log_destination: s3_log_destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceActiveMQBrokerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceActiveMQBrokerParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html#cfn-pipes-pipe-pipesourceactivemqbrokerparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html#cfn-pipes-pipe-pipesourceactivemqbrokerparameters-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: ::Value<MQBrokerAccessCredentials>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html#cfn-pipes-pipe-pipesourceactivemqbrokerparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`QueueName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html#cfn-pipes-pipe-pipesourceactivemqbrokerparameters-queuename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub queue_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipeSourceActiveMQBrokerParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", &self.credentials)?;
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueName", &self.queue_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceActiveMQBrokerParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceActiveMQBrokerParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceActiveMQBrokerParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceActiveMQBrokerParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut credentials: Option<::Value<MQBrokerAccessCredentials>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut queue_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueName" => {
                                queue_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceActiveMQBrokerParameters {
                        batch_size: batch_size,
                        credentials: credentials.ok_or(::serde::de::Error::missing_field("Credentials"))?,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        queue_name: queue_name.ok_or(::serde::de::Error::missing_field("QueueName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceDynamoDBStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceDynamoDBStreamParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-deadletterconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dead_letter_config: Option<::Value<DeadLetterConfig>>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`MaximumRecordAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-maximumrecordageinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_record_age_in_seconds: Option<::Value<u32>>,
        /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-maximumretryattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_retry_attempts: Option<::Value<u32>>,
        /// Property [`OnPartialBatchItemFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-onpartialbatchitemfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_partial_batch_item_failure: Option<::Value<String>>,
        /// Property [`ParallelizationFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-parallelizationfactor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelization_factor: Option<::Value<u32>>,
        /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html#cfn-pipes-pipe-pipesourcedynamodbstreamparameters-startingposition).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub starting_position: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipeSourceDynamoDBStreamParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref dead_letter_config) = self.dead_letter_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
            }
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            if let Some(ref maximum_record_age_in_seconds) = self.maximum_record_age_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRecordAgeInSeconds", maximum_record_age_in_seconds)?;
            }
            if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
            }
            if let Some(ref on_partial_batch_item_failure) = self.on_partial_batch_item_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnPartialBatchItemFailure", on_partial_batch_item_failure)?;
            }
            if let Some(ref parallelization_factor) = self.parallelization_factor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelizationFactor", parallelization_factor)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", &self.starting_position)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceDynamoDBStreamParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceDynamoDBStreamParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceDynamoDBStreamParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceDynamoDBStreamParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut dead_letter_config: Option<::Value<DeadLetterConfig>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut maximum_record_age_in_seconds: Option<::Value<u32>> = None;
                    let mut maximum_retry_attempts: Option<::Value<u32>> = None;
                    let mut on_partial_batch_item_failure: Option<::Value<String>> = None;
                    let mut parallelization_factor: Option<::Value<u32>> = None;
                    let mut starting_position: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeadLetterConfig" => {
                                dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRecordAgeInSeconds" => {
                                maximum_record_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRetryAttempts" => {
                                maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnPartialBatchItemFailure" => {
                                on_partial_batch_item_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelizationFactor" => {
                                parallelization_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartingPosition" => {
                                starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceDynamoDBStreamParameters {
                        batch_size: batch_size,
                        dead_letter_config: dead_letter_config,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        maximum_record_age_in_seconds: maximum_record_age_in_seconds,
                        maximum_retry_attempts: maximum_retry_attempts,
                        on_partial_batch_item_failure: on_partial_batch_item_failure,
                        parallelization_factor: parallelization_factor,
                        starting_position: starting_position.ok_or(::serde::de::Error::missing_field("StartingPosition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceKinesisStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceKinesisStreamParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-deadletterconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dead_letter_config: Option<::Value<DeadLetterConfig>>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`MaximumRecordAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-maximumrecordageinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_record_age_in_seconds: Option<::Value<u32>>,
        /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-maximumretryattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_retry_attempts: Option<::Value<u32>>,
        /// Property [`OnPartialBatchItemFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-onpartialbatchitemfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_partial_batch_item_failure: Option<::Value<String>>,
        /// Property [`ParallelizationFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-parallelizationfactor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelization_factor: Option<::Value<u32>>,
        /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-startingposition).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub starting_position: ::Value<String>,
        /// Property [`StartingPositionTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html#cfn-pipes-pipe-pipesourcekinesisstreamparameters-startingpositiontimestamp).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub starting_position_timestamp: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeSourceKinesisStreamParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref dead_letter_config) = self.dead_letter_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
            }
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            if let Some(ref maximum_record_age_in_seconds) = self.maximum_record_age_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRecordAgeInSeconds", maximum_record_age_in_seconds)?;
            }
            if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
            }
            if let Some(ref on_partial_batch_item_failure) = self.on_partial_batch_item_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnPartialBatchItemFailure", on_partial_batch_item_failure)?;
            }
            if let Some(ref parallelization_factor) = self.parallelization_factor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelizationFactor", parallelization_factor)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", &self.starting_position)?;
            if let Some(ref starting_position_timestamp) = self.starting_position_timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPositionTimestamp", starting_position_timestamp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceKinesisStreamParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceKinesisStreamParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceKinesisStreamParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceKinesisStreamParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut dead_letter_config: Option<::Value<DeadLetterConfig>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut maximum_record_age_in_seconds: Option<::Value<u32>> = None;
                    let mut maximum_retry_attempts: Option<::Value<u32>> = None;
                    let mut on_partial_batch_item_failure: Option<::Value<String>> = None;
                    let mut parallelization_factor: Option<::Value<u32>> = None;
                    let mut starting_position: Option<::Value<String>> = None;
                    let mut starting_position_timestamp: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeadLetterConfig" => {
                                dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRecordAgeInSeconds" => {
                                maximum_record_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRetryAttempts" => {
                                maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnPartialBatchItemFailure" => {
                                on_partial_batch_item_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelizationFactor" => {
                                parallelization_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartingPosition" => {
                                starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartingPositionTimestamp" => {
                                starting_position_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceKinesisStreamParameters {
                        batch_size: batch_size,
                        dead_letter_config: dead_letter_config,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        maximum_record_age_in_seconds: maximum_record_age_in_seconds,
                        maximum_retry_attempts: maximum_retry_attempts,
                        on_partial_batch_item_failure: on_partial_batch_item_failure,
                        parallelization_factor: parallelization_factor,
                        starting_position: starting_position.ok_or(::serde::de::Error::missing_field("StartingPosition"))?,
                        starting_position_timestamp: starting_position_timestamp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceManagedStreamingKafkaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceManagedStreamingKafkaParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`ConsumerGroupID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-consumergroupid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub consumer_group_id: Option<::Value<String>>,
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: Option<::Value<MSKAccessCredentials>>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-startingposition).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub starting_position: Option<::Value<String>>,
        /// Property [`TopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html#cfn-pipes-pipe-pipesourcemanagedstreamingkafkaparameters-topicname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub topic_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipeSourceManagedStreamingKafkaParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref consumer_group_id) = self.consumer_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupID", consumer_group_id)?;
            }
            if let Some(ref credentials) = self.credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
            }
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            if let Some(ref starting_position) = self.starting_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", starting_position)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicName", &self.topic_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceManagedStreamingKafkaParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceManagedStreamingKafkaParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceManagedStreamingKafkaParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceManagedStreamingKafkaParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut consumer_group_id: Option<::Value<String>> = None;
                    let mut credentials: Option<::Value<MSKAccessCredentials>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut starting_position: Option<::Value<String>> = None;
                    let mut topic_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConsumerGroupID" => {
                                consumer_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartingPosition" => {
                                starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicName" => {
                                topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceManagedStreamingKafkaParameters {
                        batch_size: batch_size,
                        consumer_group_id: consumer_group_id,
                        credentials: credentials,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        starting_position: starting_position,
                        topic_name: topic_name.ok_or(::serde::de::Error::missing_field("TopicName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceParameters {
        /// Property [`ActiveMQBrokerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-activemqbrokerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub active_mq_broker_parameters: Option<::Value<PipeSourceActiveMQBrokerParameters>>,
        /// Property [`DynamoDBStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-dynamodbstreamparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_db_stream_parameters: Option<::Value<PipeSourceDynamoDBStreamParameters>>,
        /// Property [`FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-filtercriteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_criteria: Option<::Value<FilterCriteria>>,
        /// Property [`KinesisStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-kinesisstreamparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_stream_parameters: Option<::Value<PipeSourceKinesisStreamParameters>>,
        /// Property [`ManagedStreamingKafkaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-managedstreamingkafkaparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_streaming_kafka_parameters: Option<::Value<PipeSourceManagedStreamingKafkaParameters>>,
        /// Property [`RabbitMQBrokerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-rabbitmqbrokerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rabbit_mq_broker_parameters: Option<::Value<PipeSourceRabbitMQBrokerParameters>>,
        /// Property [`SelfManagedKafkaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-selfmanagedkafkaparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub self_managed_kafka_parameters: Option<::Value<PipeSourceSelfManagedKafkaParameters>>,
        /// Property [`SqsQueueParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html#cfn-pipes-pipe-pipesourceparameters-sqsqueueparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs_queue_parameters: Option<::Value<PipeSourceSqsQueueParameters>>,
    }

    impl ::codec::SerializeValue for PipeSourceParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref active_mq_broker_parameters) = self.active_mq_broker_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveMQBrokerParameters", active_mq_broker_parameters)?;
            }
            if let Some(ref dynamo_db_stream_parameters) = self.dynamo_db_stream_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBStreamParameters", dynamo_db_stream_parameters)?;
            }
            if let Some(ref filter_criteria) = self.filter_criteria {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterCriteria", filter_criteria)?;
            }
            if let Some(ref kinesis_stream_parameters) = self.kinesis_stream_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamParameters", kinesis_stream_parameters)?;
            }
            if let Some(ref managed_streaming_kafka_parameters) = self.managed_streaming_kafka_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedStreamingKafkaParameters", managed_streaming_kafka_parameters)?;
            }
            if let Some(ref rabbit_mq_broker_parameters) = self.rabbit_mq_broker_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RabbitMQBrokerParameters", rabbit_mq_broker_parameters)?;
            }
            if let Some(ref self_managed_kafka_parameters) = self.self_managed_kafka_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedKafkaParameters", self_managed_kafka_parameters)?;
            }
            if let Some(ref sqs_queue_parameters) = self.sqs_queue_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqsQueueParameters", sqs_queue_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active_mq_broker_parameters: Option<::Value<PipeSourceActiveMQBrokerParameters>> = None;
                    let mut dynamo_db_stream_parameters: Option<::Value<PipeSourceDynamoDBStreamParameters>> = None;
                    let mut filter_criteria: Option<::Value<FilterCriteria>> = None;
                    let mut kinesis_stream_parameters: Option<::Value<PipeSourceKinesisStreamParameters>> = None;
                    let mut managed_streaming_kafka_parameters: Option<::Value<PipeSourceManagedStreamingKafkaParameters>> = None;
                    let mut rabbit_mq_broker_parameters: Option<::Value<PipeSourceRabbitMQBrokerParameters>> = None;
                    let mut self_managed_kafka_parameters: Option<::Value<PipeSourceSelfManagedKafkaParameters>> = None;
                    let mut sqs_queue_parameters: Option<::Value<PipeSourceSqsQueueParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActiveMQBrokerParameters" => {
                                active_mq_broker_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDBStreamParameters" => {
                                dynamo_db_stream_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterCriteria" => {
                                filter_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisStreamParameters" => {
                                kinesis_stream_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedStreamingKafkaParameters" => {
                                managed_streaming_kafka_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RabbitMQBrokerParameters" => {
                                rabbit_mq_broker_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelfManagedKafkaParameters" => {
                                self_managed_kafka_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqsQueueParameters" => {
                                sqs_queue_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceParameters {
                        active_mq_broker_parameters: active_mq_broker_parameters,
                        dynamo_db_stream_parameters: dynamo_db_stream_parameters,
                        filter_criteria: filter_criteria,
                        kinesis_stream_parameters: kinesis_stream_parameters,
                        managed_streaming_kafka_parameters: managed_streaming_kafka_parameters,
                        rabbit_mq_broker_parameters: rabbit_mq_broker_parameters,
                        self_managed_kafka_parameters: self_managed_kafka_parameters,
                        sqs_queue_parameters: sqs_queue_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceRabbitMQBrokerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceRabbitMQBrokerParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html#cfn-pipes-pipe-pipesourcerabbitmqbrokerparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html#cfn-pipes-pipe-pipesourcerabbitmqbrokerparameters-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: ::Value<MQBrokerAccessCredentials>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html#cfn-pipes-pipe-pipesourcerabbitmqbrokerparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`QueueName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html#cfn-pipes-pipe-pipesourcerabbitmqbrokerparameters-queuename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub queue_name: ::Value<String>,
        /// Property [`VirtualHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html#cfn-pipes-pipe-pipesourcerabbitmqbrokerparameters-virtualhost).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub virtual_host: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeSourceRabbitMQBrokerParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", &self.credentials)?;
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueName", &self.queue_name)?;
            if let Some(ref virtual_host) = self.virtual_host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualHost", virtual_host)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceRabbitMQBrokerParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceRabbitMQBrokerParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceRabbitMQBrokerParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceRabbitMQBrokerParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut credentials: Option<::Value<MQBrokerAccessCredentials>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut queue_name: Option<::Value<String>> = None;
                    let mut virtual_host: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueName" => {
                                queue_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualHost" => {
                                virtual_host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceRabbitMQBrokerParameters {
                        batch_size: batch_size,
                        credentials: credentials.ok_or(::serde::de::Error::missing_field("Credentials"))?,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        queue_name: queue_name.ok_or(::serde::de::Error::missing_field("QueueName"))?,
                        virtual_host: virtual_host,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceSelfManagedKafkaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceSelfManagedKafkaParameters {
        /// Property [`AdditionalBootstrapServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-additionalbootstrapservers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_bootstrap_servers: Option<::ValueList<String>>,
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`ConsumerGroupID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-consumergroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consumer_group_id: Option<::Value<String>>,
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: Option<::Value<SelfManagedKafkaAccessConfigurationCredentials>>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
        /// Property [`ServerRootCaCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-serverrootcacertificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_root_ca_certificate: Option<::Value<String>>,
        /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-startingposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub starting_position: Option<::Value<String>>,
        /// Property [`TopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-topicname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_name: ::Value<String>,
        /// Property [`Vpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html#cfn-pipes-pipe-pipesourceselfmanagedkafkaparameters-vpc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc: Option<::Value<SelfManagedKafkaAccessConfigurationVpc>>,
    }

    impl ::codec::SerializeValue for PipeSourceSelfManagedKafkaParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref additional_bootstrap_servers) = self.additional_bootstrap_servers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalBootstrapServers", additional_bootstrap_servers)?;
            }
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref consumer_group_id) = self.consumer_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupID", consumer_group_id)?;
            }
            if let Some(ref credentials) = self.credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
            }
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            if let Some(ref server_root_ca_certificate) = self.server_root_ca_certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerRootCaCertificate", server_root_ca_certificate)?;
            }
            if let Some(ref starting_position) = self.starting_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", starting_position)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicName", &self.topic_name)?;
            if let Some(ref vpc) = self.vpc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vpc", vpc)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceSelfManagedKafkaParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceSelfManagedKafkaParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceSelfManagedKafkaParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceSelfManagedKafkaParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_bootstrap_servers: Option<::ValueList<String>> = None;
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut consumer_group_id: Option<::Value<String>> = None;
                    let mut credentials: Option<::Value<SelfManagedKafkaAccessConfigurationCredentials>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                    let mut server_root_ca_certificate: Option<::Value<String>> = None;
                    let mut starting_position: Option<::Value<String>> = None;
                    let mut topic_name: Option<::Value<String>> = None;
                    let mut vpc: Option<::Value<SelfManagedKafkaAccessConfigurationVpc>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalBootstrapServers" => {
                                additional_bootstrap_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConsumerGroupID" => {
                                consumer_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerRootCaCertificate" => {
                                server_root_ca_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartingPosition" => {
                                starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicName" => {
                                topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Vpc" => {
                                vpc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceSelfManagedKafkaParameters {
                        additional_bootstrap_servers: additional_bootstrap_servers,
                        batch_size: batch_size,
                        consumer_group_id: consumer_group_id,
                        credentials: credentials,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                        server_root_ca_certificate: server_root_ca_certificate,
                        starting_position: starting_position,
                        topic_name: topic_name.ok_or(::serde::de::Error::missing_field("TopicName"))?,
                        vpc: vpc,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeSourceSqsQueueParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcesqsqueueparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeSourceSqsQueueParameters {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcesqsqueueparameters.html#cfn-pipes-pipe-pipesourcesqsqueueparameters-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: Option<::Value<u32>>,
        /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcesqsqueueparameters.html#cfn-pipes-pipe-pipesourcesqsqueueparameters-maximumbatchingwindowinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for PipeSourceSqsQueueParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_size) = self.batch_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
            }
            if let Some(ref maximum_batching_window_in_seconds) = self.maximum_batching_window_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBatchingWindowInSeconds", maximum_batching_window_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeSourceSqsQueueParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeSourceSqsQueueParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeSourceSqsQueueParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeSourceSqsQueueParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBatchingWindowInSeconds" => {
                                maximum_batching_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeSourceSqsQueueParameters {
                        batch_size: batch_size,
                        maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetBatchJobParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetBatchJobParameters {
        /// Property [`ArrayProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-arrayproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub array_properties: Option<::Value<BatchArrayProperties>>,
        /// Property [`ContainerOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-containeroverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_overrides: Option<::Value<BatchContainerOverrides>>,
        /// Property [`DependsOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-dependson).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub depends_on: Option<::ValueList<BatchJobDependency>>,
        /// Property [`JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-jobdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_definition: ::Value<String>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: ::Value<String>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueMap<String>>,
        /// Property [`RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html#cfn-pipes-pipe-pipetargetbatchjobparameters-retrystrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_strategy: Option<::Value<BatchRetryStrategy>>,
    }

    impl ::codec::SerializeValue for PipeTargetBatchJobParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref array_properties) = self.array_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArrayProperties", array_properties)?;
            }
            if let Some(ref container_overrides) = self.container_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerOverrides", container_overrides)?;
            }
            if let Some(ref depends_on) = self.depends_on {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DependsOn", depends_on)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinition", &self.job_definition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", &self.job_name)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref retry_strategy) = self.retry_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryStrategy", retry_strategy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetBatchJobParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetBatchJobParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetBatchJobParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetBatchJobParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut array_properties: Option<::Value<BatchArrayProperties>> = None;
                    let mut container_overrides: Option<::Value<BatchContainerOverrides>> = None;
                    let mut depends_on: Option<::ValueList<BatchJobDependency>> = None;
                    let mut job_definition: Option<::Value<String>> = None;
                    let mut job_name: Option<::Value<String>> = None;
                    let mut parameters: Option<::ValueMap<String>> = None;
                    let mut retry_strategy: Option<::Value<BatchRetryStrategy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArrayProperties" => {
                                array_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerOverrides" => {
                                container_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DependsOn" => {
                                depends_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobDefinition" => {
                                job_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryStrategy" => {
                                retry_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetBatchJobParameters {
                        array_properties: array_properties,
                        container_overrides: container_overrides,
                        depends_on: depends_on,
                        job_definition: job_definition.ok_or(::serde::de::Error::missing_field("JobDefinition"))?,
                        job_name: job_name.ok_or(::serde::de::Error::missing_field("JobName"))?,
                        parameters: parameters,
                        retry_strategy: retry_strategy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetCloudWatchLogsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetcloudwatchlogsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetCloudWatchLogsParameters {
        /// Property [`LogStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetcloudwatchlogsparameters.html#cfn-pipes-pipe-pipetargetcloudwatchlogsparameters-logstreamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_stream_name: Option<::Value<String>>,
        /// Property [`Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetcloudwatchlogsparameters.html#cfn-pipes-pipe-pipetargetcloudwatchlogsparameters-timestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetCloudWatchLogsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_stream_name) = self.log_stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamName", log_stream_name)?;
            }
            if let Some(ref timestamp) = self.timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestamp", timestamp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetCloudWatchLogsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetCloudWatchLogsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetCloudWatchLogsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetCloudWatchLogsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_stream_name: Option<::Value<String>> = None;
                    let mut timestamp: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogStreamName" => {
                                log_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestamp" => {
                                timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetCloudWatchLogsParameters {
                        log_stream_name: log_stream_name,
                        timestamp: timestamp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetEcsTaskParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetEcsTaskParameters {
        /// Property [`CapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-capacityproviderstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider_strategy: Option<::ValueList<CapacityProviderStrategyItem>>,
        /// Property [`EnableECSManagedTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-enableecsmanagedtags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_ecs_managed_tags: Option<::Value<bool>>,
        /// Property [`EnableExecuteCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-enableexecutecommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_execute_command: Option<::Value<bool>>,
        /// Property [`Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-group).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group: Option<::Value<String>>,
        /// Property [`LaunchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-launchtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_type: Option<::Value<String>>,
        /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-networkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_configuration: Option<::Value<NetworkConfiguration>>,
        /// Property [`Overrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-overrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overrides: Option<::Value<EcsTaskOverride>>,
        /// Property [`PlacementConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-placementconstraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub placement_constraints: Option<::ValueList<PlacementConstraint>>,
        /// Property [`PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-placementstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub placement_strategy: Option<::ValueList<PlacementStrategy>>,
        /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-platformversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform_version: Option<::Value<String>>,
        /// Property [`PropagateTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-propagatetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub propagate_tags: Option<::Value<String>>,
        /// Property [`ReferenceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-referenceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reference_id: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
        /// Property [`TaskCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-taskcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_count: Option<::Value<u32>>,
        /// Property [`TaskDefinitionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html#cfn-pipes-pipe-pipetargetecstaskparameters-taskdefinitionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_definition_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipeTargetEcsTaskParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capacity_provider_strategy) = self.capacity_provider_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProviderStrategy", capacity_provider_strategy)?;
            }
            if let Some(ref enable_ecs_managed_tags) = self.enable_ecs_managed_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableECSManagedTags", enable_ecs_managed_tags)?;
            }
            if let Some(ref enable_execute_command) = self.enable_execute_command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableExecuteCommand", enable_execute_command)?;
            }
            if let Some(ref group) = self.group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Group", group)?;
            }
            if let Some(ref launch_type) = self.launch_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", launch_type)?;
            }
            if let Some(ref network_configuration) = self.network_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
            }
            if let Some(ref overrides) = self.overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overrides", overrides)?;
            }
            if let Some(ref placement_constraints) = self.placement_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", placement_constraints)?;
            }
            if let Some(ref placement_strategy) = self.placement_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementStrategy", placement_strategy)?;
            }
            if let Some(ref platform_version) = self.platform_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
            }
            if let Some(ref propagate_tags) = self.propagate_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateTags", propagate_tags)?;
            }
            if let Some(ref reference_id) = self.reference_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceId", reference_id)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            if let Some(ref task_count) = self.task_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskCount", task_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinitionArn", &self.task_definition_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetEcsTaskParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetEcsTaskParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetEcsTaskParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetEcsTaskParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capacity_provider_strategy: Option<::ValueList<CapacityProviderStrategyItem>> = None;
                    let mut enable_ecs_managed_tags: Option<::Value<bool>> = None;
                    let mut enable_execute_command: Option<::Value<bool>> = None;
                    let mut group: Option<::Value<String>> = None;
                    let mut launch_type: Option<::Value<String>> = None;
                    let mut network_configuration: Option<::Value<NetworkConfiguration>> = None;
                    let mut overrides: Option<::Value<EcsTaskOverride>> = None;
                    let mut placement_constraints: Option<::ValueList<PlacementConstraint>> = None;
                    let mut placement_strategy: Option<::ValueList<PlacementStrategy>> = None;
                    let mut platform_version: Option<::Value<String>> = None;
                    let mut propagate_tags: Option<::Value<String>> = None;
                    let mut reference_id: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;
                    let mut task_count: Option<::Value<u32>> = None;
                    let mut task_definition_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CapacityProviderStrategy" => {
                                capacity_provider_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableECSManagedTags" => {
                                enable_ecs_managed_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableExecuteCommand" => {
                                enable_execute_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Group" => {
                                group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchType" => {
                                launch_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkConfiguration" => {
                                network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overrides" => {
                                overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlacementConstraints" => {
                                placement_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlacementStrategy" => {
                                placement_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlatformVersion" => {
                                platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropagateTags" => {
                                propagate_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReferenceId" => {
                                reference_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskCount" => {
                                task_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskDefinitionArn" => {
                                task_definition_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetEcsTaskParameters {
                        capacity_provider_strategy: capacity_provider_strategy,
                        enable_ecs_managed_tags: enable_ecs_managed_tags,
                        enable_execute_command: enable_execute_command,
                        group: group,
                        launch_type: launch_type,
                        network_configuration: network_configuration,
                        overrides: overrides,
                        placement_constraints: placement_constraints,
                        placement_strategy: placement_strategy,
                        platform_version: platform_version,
                        propagate_tags: propagate_tags,
                        reference_id: reference_id,
                        tags: tags,
                        task_count: task_count,
                        task_definition_arn: task_definition_arn.ok_or(::serde::de::Error::missing_field("TaskDefinitionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetEventBridgeEventBusParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetEventBridgeEventBusParameters {
        /// Property [`DetailType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html#cfn-pipes-pipe-pipetargeteventbridgeeventbusparameters-detailtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detail_type: Option<::Value<String>>,
        /// Property [`EndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html#cfn-pipes-pipe-pipetargeteventbridgeeventbusparameters-endpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_id: Option<::Value<String>>,
        /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html#cfn-pipes-pipe-pipetargeteventbridgeeventbusparameters-resources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resources: Option<::ValueList<String>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html#cfn-pipes-pipe-pipetargeteventbridgeeventbusparameters-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<String>>,
        /// Property [`Time`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html#cfn-pipes-pipe-pipetargeteventbridgeeventbusparameters-time).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetEventBridgeEventBusParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref detail_type) = self.detail_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailType", detail_type)?;
            }
            if let Some(ref endpoint_id) = self.endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointId", endpoint_id)?;
            }
            if let Some(ref resources) = self.resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", resources)?;
            }
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            if let Some(ref time) = self.time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Time", time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetEventBridgeEventBusParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetEventBridgeEventBusParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetEventBridgeEventBusParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetEventBridgeEventBusParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut detail_type: Option<::Value<String>> = None;
                    let mut endpoint_id: Option<::Value<String>> = None;
                    let mut resources: Option<::ValueList<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DetailType" => {
                                detail_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointId" => {
                                endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Resources" => {
                                resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Time" => {
                                time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetEventBridgeEventBusParameters {
                        detail_type: detail_type,
                        endpoint_id: endpoint_id,
                        resources: resources,
                        source: source,
                        time: time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetHttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargethttpparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetHttpParameters {
        /// Property [`HeaderParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargethttpparameters.html#cfn-pipes-pipe-pipetargethttpparameters-headerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_parameters: Option<::ValueMap<String>>,
        /// Property [`PathParameterValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargethttpparameters.html#cfn-pipes-pipe-pipetargethttpparameters-pathparametervalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_parameter_values: Option<::ValueList<String>>,
        /// Property [`QueryStringParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargethttpparameters.html#cfn-pipes-pipe-pipetargethttpparameters-querystringparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_parameters: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetHttpParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref header_parameters) = self.header_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderParameters", header_parameters)?;
            }
            if let Some(ref path_parameter_values) = self.path_parameter_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathParameterValues", path_parameter_values)?;
            }
            if let Some(ref query_string_parameters) = self.query_string_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringParameters", query_string_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetHttpParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetHttpParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetHttpParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetHttpParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_parameters: Option<::ValueMap<String>> = None;
                    let mut path_parameter_values: Option<::ValueList<String>> = None;
                    let mut query_string_parameters: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderParameters" => {
                                header_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathParameterValues" => {
                                path_parameter_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringParameters" => {
                                query_string_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetHttpParameters {
                        header_parameters: header_parameters,
                        path_parameter_values: path_parameter_values,
                        query_string_parameters: query_string_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetKinesisStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetkinesisstreamparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetKinesisStreamParameters {
        /// Property [`PartitionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetkinesisstreamparameters.html#cfn-pipes-pipe-pipetargetkinesisstreamparameters-partitionkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipeTargetKinesisStreamParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKey", &self.partition_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetKinesisStreamParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetKinesisStreamParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetKinesisStreamParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetKinesisStreamParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKey" => {
                                partition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetKinesisStreamParameters {
                        partition_key: partition_key.ok_or(::serde::de::Error::missing_field("PartitionKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetLambdaFunctionParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetlambdafunctionparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetLambdaFunctionParameters {
        /// Property [`InvocationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetlambdafunctionparameters.html#cfn-pipes-pipe-pipetargetlambdafunctionparameters-invocationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetLambdaFunctionParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invocation_type) = self.invocation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationType", invocation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetLambdaFunctionParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetLambdaFunctionParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetLambdaFunctionParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetLambdaFunctionParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invocation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvocationType" => {
                                invocation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetLambdaFunctionParameters {
                        invocation_type: invocation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetParameters {
        /// Property [`BatchJobParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-batchjobparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_job_parameters: Option<::Value<PipeTargetBatchJobParameters>>,
        /// Property [`CloudWatchLogsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-cloudwatchlogsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_parameters: Option<::Value<PipeTargetCloudWatchLogsParameters>>,
        /// Property [`EcsTaskParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-ecstaskparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecs_task_parameters: Option<::Value<PipeTargetEcsTaskParameters>>,
        /// Property [`EventBridgeEventBusParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-eventbridgeeventbusparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_bridge_event_bus_parameters: Option<::Value<PipeTargetEventBridgeEventBusParameters>>,
        /// Property [`HttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-httpparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_parameters: Option<::Value<PipeTargetHttpParameters>>,
        /// Property [`InputTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-inputtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_template: Option<::Value<String>>,
        /// Property [`KinesisStreamParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-kinesisstreamparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_stream_parameters: Option<::Value<PipeTargetKinesisStreamParameters>>,
        /// Property [`LambdaFunctionParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-lambdafunctionparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_parameters: Option<::Value<PipeTargetLambdaFunctionParameters>>,
        /// Property [`RedshiftDataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-redshiftdataparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_data_parameters: Option<::Value<PipeTargetRedshiftDataParameters>>,
        /// Property [`SageMakerPipelineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-sagemakerpipelineparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_pipeline_parameters: Option<::Value<PipeTargetSageMakerPipelineParameters>>,
        /// Property [`SqsQueueParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-sqsqueueparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs_queue_parameters: Option<::Value<PipeTargetSqsQueueParameters>>,
        /// Property [`StepFunctionStateMachineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html#cfn-pipes-pipe-pipetargetparameters-stepfunctionstatemachineparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub step_function_state_machine_parameters: Option<::Value<PipeTargetStateMachineParameters>>,
    }

    impl ::codec::SerializeValue for PipeTargetParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_job_parameters) = self.batch_job_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchJobParameters", batch_job_parameters)?;
            }
            if let Some(ref cloud_watch_logs_parameters) = self.cloud_watch_logs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsParameters", cloud_watch_logs_parameters)?;
            }
            if let Some(ref ecs_task_parameters) = self.ecs_task_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsTaskParameters", ecs_task_parameters)?;
            }
            if let Some(ref event_bridge_event_bus_parameters) = self.event_bridge_event_bus_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeEventBusParameters", event_bridge_event_bus_parameters)?;
            }
            if let Some(ref http_parameters) = self.http_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpParameters", http_parameters)?;
            }
            if let Some(ref input_template) = self.input_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTemplate", input_template)?;
            }
            if let Some(ref kinesis_stream_parameters) = self.kinesis_stream_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamParameters", kinesis_stream_parameters)?;
            }
            if let Some(ref lambda_function_parameters) = self.lambda_function_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionParameters", lambda_function_parameters)?;
            }
            if let Some(ref redshift_data_parameters) = self.redshift_data_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftDataParameters", redshift_data_parameters)?;
            }
            if let Some(ref sage_maker_pipeline_parameters) = self.sage_maker_pipeline_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerPipelineParameters", sage_maker_pipeline_parameters)?;
            }
            if let Some(ref sqs_queue_parameters) = self.sqs_queue_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqsQueueParameters", sqs_queue_parameters)?;
            }
            if let Some(ref step_function_state_machine_parameters) = self.step_function_state_machine_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepFunctionStateMachineParameters", step_function_state_machine_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_job_parameters: Option<::Value<PipeTargetBatchJobParameters>> = None;
                    let mut cloud_watch_logs_parameters: Option<::Value<PipeTargetCloudWatchLogsParameters>> = None;
                    let mut ecs_task_parameters: Option<::Value<PipeTargetEcsTaskParameters>> = None;
                    let mut event_bridge_event_bus_parameters: Option<::Value<PipeTargetEventBridgeEventBusParameters>> = None;
                    let mut http_parameters: Option<::Value<PipeTargetHttpParameters>> = None;
                    let mut input_template: Option<::Value<String>> = None;
                    let mut kinesis_stream_parameters: Option<::Value<PipeTargetKinesisStreamParameters>> = None;
                    let mut lambda_function_parameters: Option<::Value<PipeTargetLambdaFunctionParameters>> = None;
                    let mut redshift_data_parameters: Option<::Value<PipeTargetRedshiftDataParameters>> = None;
                    let mut sage_maker_pipeline_parameters: Option<::Value<PipeTargetSageMakerPipelineParameters>> = None;
                    let mut sqs_queue_parameters: Option<::Value<PipeTargetSqsQueueParameters>> = None;
                    let mut step_function_state_machine_parameters: Option<::Value<PipeTargetStateMachineParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchJobParameters" => {
                                batch_job_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLogsParameters" => {
                                cloud_watch_logs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcsTaskParameters" => {
                                ecs_task_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventBridgeEventBusParameters" => {
                                event_bridge_event_bus_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpParameters" => {
                                http_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTemplate" => {
                                input_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisStreamParameters" => {
                                kinesis_stream_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionParameters" => {
                                lambda_function_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftDataParameters" => {
                                redshift_data_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerPipelineParameters" => {
                                sage_maker_pipeline_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqsQueueParameters" => {
                                sqs_queue_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepFunctionStateMachineParameters" => {
                                step_function_state_machine_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetParameters {
                        batch_job_parameters: batch_job_parameters,
                        cloud_watch_logs_parameters: cloud_watch_logs_parameters,
                        ecs_task_parameters: ecs_task_parameters,
                        event_bridge_event_bus_parameters: event_bridge_event_bus_parameters,
                        http_parameters: http_parameters,
                        input_template: input_template,
                        kinesis_stream_parameters: kinesis_stream_parameters,
                        lambda_function_parameters: lambda_function_parameters,
                        redshift_data_parameters: redshift_data_parameters,
                        sage_maker_pipeline_parameters: sage_maker_pipeline_parameters,
                        sqs_queue_parameters: sqs_queue_parameters,
                        step_function_state_machine_parameters: step_function_state_machine_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetRedshiftDataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetRedshiftDataParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`DbUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-dbuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_user: Option<::Value<String>>,
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: Option<::Value<String>>,
        /// Property [`Sqls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-sqls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqls: ::ValueList<String>,
        /// Property [`StatementName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-statementname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement_name: Option<::Value<String>>,
        /// Property [`WithEvent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html#cfn-pipes-pipe-pipetargetredshiftdataparameters-withevent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub with_event: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PipeTargetRedshiftDataParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            if let Some(ref db_user) = self.db_user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbUser", db_user)?;
            }
            if let Some(ref secret_manager_arn) = self.secret_manager_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", secret_manager_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sqls", &self.sqls)?;
            if let Some(ref statement_name) = self.statement_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatementName", statement_name)?;
            }
            if let Some(ref with_event) = self.with_event {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WithEvent", with_event)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetRedshiftDataParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetRedshiftDataParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetRedshiftDataParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetRedshiftDataParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut db_user: Option<::Value<String>> = None;
                    let mut secret_manager_arn: Option<::Value<String>> = None;
                    let mut sqls: Option<::ValueList<String>> = None;
                    let mut statement_name: Option<::Value<String>> = None;
                    let mut with_event: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DbUser" => {
                                db_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sqls" => {
                                sqls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatementName" => {
                                statement_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WithEvent" => {
                                with_event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetRedshiftDataParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        db_user: db_user,
                        secret_manager_arn: secret_manager_arn,
                        sqls: sqls.ok_or(::serde::de::Error::missing_field("Sqls"))?,
                        statement_name: statement_name,
                        with_event: with_event,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetSageMakerPipelineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsagemakerpipelineparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetSageMakerPipelineParameters {
        /// Property [`PipelineParameterList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsagemakerpipelineparameters.html#cfn-pipes-pipe-pipetargetsagemakerpipelineparameters-pipelineparameterlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pipeline_parameter_list: Option<::ValueList<SageMakerPipelineParameter>>,
    }

    impl ::codec::SerializeValue for PipeTargetSageMakerPipelineParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pipeline_parameter_list) = self.pipeline_parameter_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineParameterList", pipeline_parameter_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetSageMakerPipelineParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetSageMakerPipelineParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetSageMakerPipelineParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetSageMakerPipelineParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pipeline_parameter_list: Option<::ValueList<SageMakerPipelineParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PipelineParameterList" => {
                                pipeline_parameter_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetSageMakerPipelineParameters {
                        pipeline_parameter_list: pipeline_parameter_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetSqsQueueParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsqsqueueparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetSqsQueueParameters {
        /// Property [`MessageDeduplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsqsqueueparameters.html#cfn-pipes-pipe-pipetargetsqsqueueparameters-messagededuplicationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_deduplication_id: Option<::Value<String>>,
        /// Property [`MessageGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsqsqueueparameters.html#cfn-pipes-pipe-pipetargetsqsqueueparameters-messagegroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_group_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetSqsQueueParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_deduplication_id) = self.message_deduplication_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageDeduplicationId", message_deduplication_id)?;
            }
            if let Some(ref message_group_id) = self.message_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupId", message_group_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetSqsQueueParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetSqsQueueParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetSqsQueueParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetSqsQueueParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_deduplication_id: Option<::Value<String>> = None;
                    let mut message_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageDeduplicationId" => {
                                message_deduplication_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroupId" => {
                                message_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetSqsQueueParameters {
                        message_deduplication_id: message_deduplication_id,
                        message_group_id: message_group_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PipeTargetStateMachineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetstatemachineparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PipeTargetStateMachineParameters {
        /// Property [`InvocationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetstatemachineparameters.html#cfn-pipes-pipe-pipetargetstatemachineparameters-invocationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PipeTargetStateMachineParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invocation_type) = self.invocation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationType", invocation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipeTargetStateMachineParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipeTargetStateMachineParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipeTargetStateMachineParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipeTargetStateMachineParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invocation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvocationType" => {
                                invocation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipeTargetStateMachineParameters {
                        invocation_type: invocation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementconstraint.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementConstraint {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementconstraint.html#cfn-pipes-pipe-placementconstraint-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementconstraint.html#cfn-pipes-pipe-placementconstraint-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementConstraint {
                        expression: expression,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementStrategy {
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementstrategy.html#cfn-pipes-pipe-placementstrategy-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementstrategy.html#cfn-pipes-pipe-placementstrategy-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PlacementStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementStrategy {
                        field: field,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.S3LogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3LogDestination {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html#cfn-pipes-pipe-s3logdestination-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`BucketOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html#cfn-pipes-pipe-s3logdestination-bucketowner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_owner: Option<::Value<String>>,
        /// Property [`OutputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html#cfn-pipes-pipe-s3logdestination-outputformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_format: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html#cfn-pipes-pipe-s3logdestination-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3LogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref bucket_owner) = self.bucket_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketOwner", bucket_owner)?;
            }
            if let Some(ref output_format) = self.output_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFormat", output_format)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3LogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3LogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3LogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3LogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_owner: Option<::Value<String>> = None;
                    let mut output_format: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketOwner" => {
                                bucket_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputFormat" => {
                                output_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3LogDestination {
                        bucket_name: bucket_name,
                        bucket_owner: bucket_owner,
                        output_format: output_format,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.SageMakerPipelineParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-sagemakerpipelineparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct SageMakerPipelineParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-sagemakerpipelineparameter.html#cfn-pipes-pipe-sagemakerpipelineparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-sagemakerpipelineparameter.html#cfn-pipes-pipe-sagemakerpipelineparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SageMakerPipelineParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SageMakerPipelineParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SageMakerPipelineParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SageMakerPipelineParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SageMakerPipelineParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SageMakerPipelineParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.SelfManagedKafkaAccessConfigurationCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedKafkaAccessConfigurationCredentials {
        /// Property [`BasicAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials-basicauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_auth: Option<::Value<String>>,
        /// Property [`ClientCertificateTlsAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials-clientcertificatetlsauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_certificate_tls_auth: Option<::Value<String>>,
        /// Property [`SaslScram256Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials-saslscram256auth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl_scram256_auth: Option<::Value<String>>,
        /// Property [`SaslScram512Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials-saslscram512auth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl_scram512_auth: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SelfManagedKafkaAccessConfigurationCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref basic_auth) = self.basic_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuth", basic_auth)?;
            }
            if let Some(ref client_certificate_tls_auth) = self.client_certificate_tls_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateTlsAuth", client_certificate_tls_auth)?;
            }
            if let Some(ref sasl_scram256_auth) = self.sasl_scram256_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SaslScram256Auth", sasl_scram256_auth)?;
            }
            if let Some(ref sasl_scram512_auth) = self.sasl_scram512_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SaslScram512Auth", sasl_scram512_auth)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedKafkaAccessConfigurationCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedKafkaAccessConfigurationCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedKafkaAccessConfigurationCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedKafkaAccessConfigurationCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut basic_auth: Option<::Value<String>> = None;
                    let mut client_certificate_tls_auth: Option<::Value<String>> = None;
                    let mut sasl_scram256_auth: Option<::Value<String>> = None;
                    let mut sasl_scram512_auth: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasicAuth" => {
                                basic_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientCertificateTlsAuth" => {
                                client_certificate_tls_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SaslScram256Auth" => {
                                sasl_scram256_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SaslScram512Auth" => {
                                sasl_scram512_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedKafkaAccessConfigurationCredentials {
                        basic_auth: basic_auth,
                        client_certificate_tls_auth: client_certificate_tls_auth,
                        sasl_scram256_auth: sasl_scram256_auth,
                        sasl_scram512_auth: sasl_scram512_auth,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pipes::Pipe.SelfManagedKafkaAccessConfigurationVpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedKafkaAccessConfigurationVpc {
        /// Property [`SecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc-securitygroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc.html#cfn-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SelfManagedKafkaAccessConfigurationVpc {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group) = self.security_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroup", security_group)?;
            }
            if let Some(ref subnets) = self.subnets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedKafkaAccessConfigurationVpc {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedKafkaAccessConfigurationVpc, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedKafkaAccessConfigurationVpc;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedKafkaAccessConfigurationVpc")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroup" => {
                                security_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedKafkaAccessConfigurationVpc {
                        security_group: security_group,
                        subnets: subnets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
