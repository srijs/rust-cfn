//! Types for the `Lambda` service.

/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug)]
pub struct AliasProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`FunctionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-functionversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_version: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoutingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-routingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_config: Option<::Value<self::alias::AliasRoutingConfiguration>>,
}

impl ::serde::Serialize for AliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref routing_config) = self.routing_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingConfig", routing_config)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut function_version: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut routing_config: Option<::Value<self::alias::AliasRoutingConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionVersion" => {
                            function_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingConfig" => {
                            routing_config = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Alias {
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
    /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-batchsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub batch_size: Option<::Value<u32>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`EventSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-eventsourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_arn: ::Value<String>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-functionname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-startingposition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub starting_position: ::Value<String>,
}

impl ::serde::Serialize for EventSourceMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref batch_size) = self.batch_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
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
                let mut batch_size: Option<::Value<u32>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut event_source_arn: Option<::Value<String>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut starting_position: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BatchSize" => {
                            batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSourceArn" => {
                            event_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartingPosition" => {
                            starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for EventSourceMapping {
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
    /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-code).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code: ::Value<self::function::Code>,
    /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-deadletterconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dead_letter_config: Option<::Value<self::function::DeadLetterConfig>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<::Value<self::function::Environment>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: Option<::Value<String>>,
    /// Property [`Handler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-handler).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub handler: ::Value<String>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`MemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-memorysize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub memory_size: Option<::Value<u32>>,
    /// Property [`ReservedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-reservedconcurrentexecutions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reserved_concurrent_executions: Option<::Value<u32>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-runtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub runtime: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<::Value<u32>>,
    /// Property [`TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-tracingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tracing_config: Option<::Value<self::function::TracingConfig>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-vpcconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_config: Option<::Value<self::function::VpcConfig>>,
}

impl ::serde::Serialize for FunctionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
        if let Some(ref dead_letter_config) = self.dead_letter_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref function_name) = self.function_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", function_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handler", &self.handler)?;
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref memory_size) = self.memory_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", memory_size)?;
        }
        if let Some(ref reserved_concurrent_executions) = self.reserved_concurrent_executions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservedConcurrentExecutions", reserved_concurrent_executions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", &self.runtime)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        if let Some(ref tracing_config) = self.tracing_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TracingConfig", tracing_config)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
        }
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
                let mut code: Option<::Value<self::function::Code>> = None;
                let mut dead_letter_config: Option<::Value<self::function::DeadLetterConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut environment: Option<::Value<self::function::Environment>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut handler: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut memory_size: Option<::Value<u32>> = None;
                let mut reserved_concurrent_executions: Option<::Value<u32>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut runtime: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout: Option<::Value<u32>> = None;
                let mut tracing_config: Option<::Value<self::function::TracingConfig>> = None;
                let mut vpc_config: Option<::Value<self::function::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Code" => {
                            code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeadLetterConfig" => {
                            dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Handler" => {
                            handler = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemorySize" => {
                            memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReservedConcurrentExecutions" => {
                            reserved_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Runtime" => {
                            runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TracingConfig" => {
                            tracing_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Function {
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
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: ::Value<String>,
    /// Property [`EventSourceToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-eventsourcetoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_token: Option<::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`SourceAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-sourceaccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_account: Option<::Value<String>>,
    /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-sourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for PermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        if let Some(ref event_source_token) = self.event_source_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceToken", event_source_token)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        if let Some(ref source_account) = self.source_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccount", source_account)?;
        }
        if let Some(ref source_arn) = self.source_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", source_arn)?;
        }
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
                let mut action: Option<::Value<String>> = None;
                let mut event_source_token: Option<::Value<String>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;
                let mut source_account: Option<::Value<String>> = None;
                let mut source_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSourceToken" => {
                            event_source_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAccount" => {
                            source_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceArn" => {
                            source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Permission {
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
    /// Property [`CodeSha256`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-codesha256).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_sha256: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: ::Value<String>,
}

impl ::serde::Serialize for VersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref code_sha256) = self.code_sha256 {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSha256", code_sha256)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
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
                let mut code_sha256: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut function_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CodeSha256" => {
                            code_sha256 = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Version {
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
        /// Property [`AdditionalVersionWeights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html#cfn-lambda-alias-aliasroutingconfiguration-additionalversionweights).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_version_weights: ::ValueList<VersionWeight>,
    }

    impl ::codec::SerializeValue for AliasRoutingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut additional_version_weights: Option<::ValueList<VersionWeight>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalVersionWeights" => {
                                additional_version_weights = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`FunctionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html#cfn-lambda-alias-versionweight-functionversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_version: ::Value<String>,
        /// Property [`FunctionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html#cfn-lambda-alias-versionweight-functionweight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_weight: ::Value<f64>,
    }

    impl ::codec::SerializeValue for VersionWeight {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut function_version: Option<::Value<String>> = None;
                    let mut function_weight: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionVersion" => {
                                function_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FunctionWeight" => {
                                function_weight = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: Option<::Value<String>>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key: Option<::Value<String>>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-s3objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_version: Option<::Value<String>>,
        /// Property [`ZipFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-zipfile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zip_file: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Code {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_bucket) = self.s3_bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", s3_bucket)?;
            }
            if let Some(ref s3_key) = self.s3_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", s3_key)?;
            }
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            if let Some(ref zip_file) = self.zip_file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZipFile", zip_file)?;
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
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;
                    let mut s3_object_version: Option<::Value<String>> = None;
                    let mut zip_file: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Key" => {
                                s3_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZipFile" => {
                                zip_file = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html#cfn-lambda-function-deadletterconfig-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_arn) = self.target_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", target_arn)?;
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
                    let mut target_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html#cfn-lambda-function-environment-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
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
                    let mut variables: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html#cfn-lambda-function-tracingconfig-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TracingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
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
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
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
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
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
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
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
