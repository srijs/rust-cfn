//! Types for the `Lambda` service.

/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug)]
pub struct AliasProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `FunctionName`.
    pub function_name: ::Value<String>,
    /// Property `FunctionVersion`.
    pub function_version: ::Value<String>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `RoutingConfig`.
    pub routing_config: Option<::Value<self::alias::AliasRoutingConfiguration>>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingConfig", &self.routing_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut function_name = None;
                let mut function_version = None;
                let mut name = None;
                let mut routing_config = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionName" => {
                            function_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionVersion" => {
                            function_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoutingConfig" => {
                            routing_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AliasProperties {
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    function_version: function_version.ok_or(::serde::de::Error::missing_field("FunctionVersion"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    routing_config: routing_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Alias {
    type Properties = AliasProperties;
    const TYPE: &'static str = "AWS::Lambda::Alias";
    fn properties(&self) -> &AliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Alias {}

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::Lambda::EventSourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html) resource type.
#[derive(Debug)]
pub struct EventSourceMapping {
    properties: EventSourceMappingProperties
}

/// Properties for the `EventSourceMapping` resource.
#[derive(Debug)]
pub struct EventSourceMappingProperties {
    /// Property `BatchSize`.
    pub batch_size: Option<::Value<u32>>,
    /// Property `Enabled`.
    pub enabled: Option<::Value<bool>>,
    /// Property `EventSourceArn`.
    pub event_source_arn: ::Value<String>,
    /// Property `FunctionName`.
    pub function_name: ::Value<String>,
    /// Property `StartingPosition`.
    pub starting_position: ::Value<String>,
}

impl ::serde::Serialize for EventSourceMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", &self.batch_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceArn", &self.event_source_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", &self.starting_position)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSourceMappingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSourceMappingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSourceMappingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSourceMappingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut batch_size = None;
                let mut enabled = None;
                let mut event_source_arn = None;
                let mut function_name = None;
                let mut starting_position = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BatchSize" => {
                            batch_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Enabled" => {
                            enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventSourceArn" => {
                            event_source_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionName" => {
                            function_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StartingPosition" => {
                            starting_position = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EventSourceMappingProperties {
                    batch_size: batch_size,
                    enabled: enabled,
                    event_source_arn: event_source_arn.ok_or(::serde::de::Error::missing_field("EventSourceArn"))?,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    starting_position: starting_position.ok_or(::serde::de::Error::missing_field("StartingPosition"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for EventSourceMapping {
    type Properties = EventSourceMappingProperties;
    const TYPE: &'static str = "AWS::Lambda::EventSourceMapping";
    fn properties(&self) -> &EventSourceMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSourceMappingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSourceMapping {}

impl From<EventSourceMappingProperties> for EventSourceMapping {
    fn from(properties: EventSourceMappingProperties) -> EventSourceMapping {
        EventSourceMapping { properties }
    }
}

/// The [`AWS::Lambda::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html) resource type.
#[derive(Debug)]
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Debug)]
pub struct FunctionProperties {
    /// Property `Code`.
    pub code: ::Value<self::function::Code>,
    /// Property `DeadLetterConfig`.
    pub dead_letter_config: Option<::Value<self::function::DeadLetterConfig>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Environment`.
    pub environment: Option<::Value<self::function::Environment>>,
    /// Property `FunctionName`.
    pub function_name: Option<::Value<String>>,
    /// Property `Handler`.
    pub handler: ::Value<String>,
    /// Property `KmsKeyArn`.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property `MemorySize`.
    pub memory_size: Option<::Value<u32>>,
    /// Property `ReservedConcurrentExecutions`.
    pub reserved_concurrent_executions: Option<::Value<u32>>,
    /// Property `Role`.
    pub role: ::Value<String>,
    /// Property `Runtime`.
    pub runtime: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Timeout`.
    pub timeout: Option<::Value<u32>>,
    /// Property `TracingConfig`.
    pub tracing_config: Option<::Value<self::function::TracingConfig>>,
    /// Property `VpcConfig`.
    pub vpc_config: Option<::Value<self::function::VpcConfig>>,
}

impl ::serde::Serialize for FunctionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", &self.dead_letter_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handler", &self.handler)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", &self.kms_key_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", &self.memory_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservedConcurrentExecutions", &self.reserved_concurrent_executions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", &self.runtime)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", &self.timeout)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TracingConfig", &self.tracing_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", &self.vpc_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut code = None;
                let mut dead_letter_config = None;
                let mut description = None;
                let mut environment = None;
                let mut function_name = None;
                let mut handler = None;
                let mut kms_key_arn = None;
                let mut memory_size = None;
                let mut reserved_concurrent_executions = None;
                let mut role = None;
                let mut runtime = None;
                let mut tags = None;
                let mut timeout = None;
                let mut tracing_config = None;
                let mut vpc_config = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Code" => {
                            code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeadLetterConfig" => {
                            dead_letter_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Environment" => {
                            environment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionName" => {
                            function_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Handler" => {
                            handler = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MemorySize" => {
                            memory_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReservedConcurrentExecutions" => {
                            reserved_concurrent_executions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Role" => {
                            role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Runtime" => {
                            runtime = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Timeout" => {
                            timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TracingConfig" => {
                            tracing_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcConfig" => {
                            vpc_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(FunctionProperties {
                    code: code.ok_or(::serde::de::Error::missing_field("Code"))?,
                    dead_letter_config: dead_letter_config,
                    description: description,
                    environment: environment,
                    function_name: function_name,
                    handler: handler.ok_or(::serde::de::Error::missing_field("Handler"))?,
                    kms_key_arn: kms_key_arn,
                    memory_size: memory_size,
                    reserved_concurrent_executions: reserved_concurrent_executions,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    runtime: runtime.ok_or(::serde::de::Error::missing_field("Runtime"))?,
                    tags: tags,
                    timeout: timeout,
                    tracing_config: tracing_config,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Function {
    type Properties = FunctionProperties;
    const TYPE: &'static str = "AWS::Lambda::Function";
    fn properties(&self) -> &FunctionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Function {}

impl From<FunctionProperties> for Function {
    fn from(properties: FunctionProperties) -> Function {
        Function { properties }
    }
}

/// The [`AWS::Lambda::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html) resource type.
#[derive(Debug)]
pub struct Permission {
    properties: PermissionProperties
}

/// Properties for the `Permission` resource.
#[derive(Debug)]
pub struct PermissionProperties {
    /// Property `Action`.
    pub action: ::Value<String>,
    /// Property `EventSourceToken`.
    pub event_source_token: Option<::Value<String>>,
    /// Property `FunctionName`.
    pub function_name: ::Value<String>,
    /// Property `Principal`.
    pub principal: ::Value<String>,
    /// Property `SourceAccount`.
    pub source_account: Option<::Value<String>>,
    /// Property `SourceArn`.
    pub source_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for PermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceToken", &self.event_source_token)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccount", &self.source_account)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action = None;
                let mut event_source_token = None;
                let mut function_name = None;
                let mut principal = None;
                let mut source_account = None;
                let mut source_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventSourceToken" => {
                            event_source_token = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionName" => {
                            function_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Principal" => {
                            principal = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceAccount" => {
                            source_account = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceArn" => {
                            source_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(PermissionProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    event_source_token: event_source_token,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    source_account: source_account,
                    source_arn: source_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Permission {
    type Properties = PermissionProperties;
    const TYPE: &'static str = "AWS::Lambda::Permission";
    fn properties(&self) -> &PermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Permission {}

impl From<PermissionProperties> for Permission {
    fn from(properties: PermissionProperties) -> Permission {
        Permission { properties }
    }
}

/// The [`AWS::Lambda::Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html) resource type.
#[derive(Debug)]
pub struct Version {
    properties: VersionProperties
}

/// Properties for the `Version` resource.
#[derive(Debug)]
pub struct VersionProperties {
    /// Property `CodeSha256`.
    pub code_sha256: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `FunctionName`.
    pub function_name: ::Value<String>,
}

impl ::serde::Serialize for VersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSha256", &self.code_sha256)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut code_sha256 = None;
                let mut description = None;
                let mut function_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CodeSha256" => {
                            code_sha256 = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FunctionName" => {
                            function_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VersionProperties {
                    code_sha256: code_sha256,
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Version {
    type Properties = VersionProperties;
    const TYPE: &'static str = "AWS::Lambda::Version";
    fn properties(&self) -> &VersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Version {}

impl From<VersionProperties> for Version {
    fn from(properties: VersionProperties) -> Version {
        Version { properties }
    }
}

pub mod alias {
    //! Property types for the `Alias` resource.

    /// The [`AWS::Lambda::Alias.AliasRoutingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html) property type.
    #[derive(Debug)]
    pub struct AliasRoutingConfiguration {
        /// Property `AdditionalVersionWeights`.
        pub additional_version_weights: ::ValueList<VersionWeight>,
    }

    impl ::codec::SerializeValue for AliasRoutingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalVersionWeights", &self.additional_version_weights)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AliasRoutingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AliasRoutingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AliasRoutingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AliasRoutingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_version_weights = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalVersionWeights" => {
                                additional_version_weights = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AliasRoutingConfiguration {
                        additional_version_weights: additional_version_weights.ok_or(::serde::de::Error::missing_field("AdditionalVersionWeights"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Debug)]
    pub struct VersionWeight {
        /// Property `FunctionVersion`.
        pub function_version: ::Value<String>,
        /// Property `FunctionWeight`.
        pub function_weight: ::Value<f64>,
    }

    impl ::codec::SerializeValue for VersionWeight {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionWeight", &self.function_weight)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VersionWeight {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VersionWeight, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VersionWeight;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VersionWeight")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_version = None;
                    let mut function_weight = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionVersion" => {
                                function_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FunctionWeight" => {
                                function_weight = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VersionWeight {
                        function_version: function_version.ok_or(::serde::de::Error::missing_field("FunctionVersion"))?,
                        function_weight: function_weight.ok_or(::serde::de::Error::missing_field("FunctionWeight"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::Lambda::Function.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html) property type.
    #[derive(Debug)]
    pub struct Code {
        /// Property `S3Bucket`.
        pub s3_bucket: Option<::Value<String>>,
        /// Property `S3Key`.
        pub s3_key: Option<::Value<String>>,
        /// Property `S3ObjectVersion`.
        pub s3_object_version: Option<::Value<String>>,
        /// Property `ZipFile`.
        pub zip_file: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Code {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", &self.s3_object_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZipFile", &self.zip_file)?;
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
                    let mut s3_bucket = None;
                    let mut s3_key = None;
                    let mut s3_object_version = None;
                    let mut zip_file = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Key" => {
                                s3_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ZipFile" => {
                                zip_file = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Code {
                        s3_bucket: s3_bucket,
                        s3_key: s3_key,
                        s3_object_version: s3_object_version,
                        zip_file: zip_file,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html) property type.
    #[derive(Debug)]
    pub struct DeadLetterConfig {
        /// Property `TargetArn`.
        pub target_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
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
                    let mut target_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetArn" => {
                                target_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DeadLetterConfig {
                        target_arn: target_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html) property type.
    #[derive(Debug)]
    pub struct Environment {
        /// Property `Variables`.
        pub variables: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", &self.variables)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut variables = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Variables" => {
                                variables = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Debug)]
    pub struct TracingConfig {
        /// Property `Mode`.
        pub mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TracingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TracingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TracingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TracingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TracingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TracingConfig {
                        mode: mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html) property type.
    #[derive(Debug)]
    pub struct VpcConfig {
        /// Property `SecurityGroupIds`.
        pub security_group_ids: ::ValueList<String>,
        /// Property `SubnetIds`.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids = None;
                    let mut subnet_ids = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubnetIds" => {
                                subnet_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
