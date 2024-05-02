//! Types for the `Lambda` service.

/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
#[derive(Debug, Default)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug, Default)]
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
    /// Property [`ProvisionedConcurrencyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html#cfn-lambda-alias-provisionedconcurrencyconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioned_concurrency_config: Option<::Value<self::alias::ProvisionedConcurrencyConfiguration>>,
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
        if let Some(ref provisioned_concurrency_config) = self.provisioned_concurrency_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrencyConfig", provisioned_concurrency_config)?;
        }
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
                let mut provisioned_concurrency_config: Option<::Value<self::alias::ProvisionedConcurrencyConfiguration>> = None;
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
                        "ProvisionedConcurrencyConfig" => {
                            provisioned_concurrency_config = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    provisioned_concurrency_config: provisioned_concurrency_config,
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

/// The [`AWS::Lambda::CodeSigningConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html) resource type.
#[derive(Debug, Default)]
pub struct CodeSigningConfig {
    properties: CodeSigningConfigProperties
}

/// Properties for the `CodeSigningConfig` resource.
#[derive(Debug, Default)]
pub struct CodeSigningConfigProperties {
    /// Property [`AllowedPublishers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-allowedpublishers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_publishers: ::Value<self::code_signing_config::AllowedPublishers>,
    /// Property [`CodeSigningPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-codesigningpolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_signing_policies: Option<::Value<self::code_signing_config::CodeSigningPolicies>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-codesigningconfig.html#cfn-lambda-codesigningconfig-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
}

impl ::serde::Serialize for CodeSigningConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedPublishers", &self.allowed_publishers)?;
        if let Some(ref code_signing_policies) = self.code_signing_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSigningPolicies", code_signing_policies)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CodeSigningConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeSigningConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CodeSigningConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CodeSigningConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_publishers: Option<::Value<self::code_signing_config::AllowedPublishers>> = None;
                let mut code_signing_policies: Option<::Value<self::code_signing_config::CodeSigningPolicies>> = None;
                let mut description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedPublishers" => {
                            allowed_publishers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CodeSigningPolicies" => {
                            code_signing_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CodeSigningConfigProperties {
                    allowed_publishers: allowed_publishers.ok_or(::serde::de::Error::missing_field("AllowedPublishers"))?,
                    code_signing_policies: code_signing_policies,
                    description: description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CodeSigningConfig {
    type Properties = CodeSigningConfigProperties;
    const TYPE: &'static str = "AWS::Lambda::CodeSigningConfig";
    fn properties(&self) -> &CodeSigningConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CodeSigningConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CodeSigningConfig {}

impl From<CodeSigningConfigProperties> for CodeSigningConfig {
    fn from(properties: CodeSigningConfigProperties) -> CodeSigningConfig {
        CodeSigningConfig { properties }
    }
}

/// The [`AWS::Lambda::EventInvokeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html) resource type.
#[derive(Debug, Default)]
pub struct EventInvokeConfig {
    properties: EventInvokeConfigProperties
}

/// Properties for the `EventInvokeConfig` resource.
#[derive(Debug, Default)]
pub struct EventInvokeConfigProperties {
    /// Property [`DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_config: Option<::Value<self::event_invoke_config::DestinationConfig>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`MaximumEventAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-maximumeventageinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_event_age_in_seconds: Option<::Value<u32>>,
    /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-maximumretryattempts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_retry_attempts: Option<::Value<u32>>,
    /// Property [`Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventinvokeconfig.html#cfn-lambda-eventinvokeconfig-qualifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub qualifier: ::Value<String>,
}

impl ::serde::Serialize for EventInvokeConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref destination_config) = self.destination_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationConfig", destination_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref maximum_event_age_in_seconds) = self.maximum_event_age_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumEventAgeInSeconds", maximum_event_age_in_seconds)?;
        }
        if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", &self.qualifier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventInvokeConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventInvokeConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventInvokeConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventInvokeConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_config: Option<::Value<self::event_invoke_config::DestinationConfig>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut maximum_event_age_in_seconds: Option<::Value<u32>> = None;
                let mut maximum_retry_attempts: Option<::Value<u32>> = None;
                let mut qualifier: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationConfig" => {
                            destination_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumEventAgeInSeconds" => {
                            maximum_event_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumRetryAttempts" => {
                            maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Qualifier" => {
                            qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventInvokeConfigProperties {
                    destination_config: destination_config,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    maximum_event_age_in_seconds: maximum_event_age_in_seconds,
                    maximum_retry_attempts: maximum_retry_attempts,
                    qualifier: qualifier.ok_or(::serde::de::Error::missing_field("Qualifier"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventInvokeConfig {
    type Properties = EventInvokeConfigProperties;
    const TYPE: &'static str = "AWS::Lambda::EventInvokeConfig";
    fn properties(&self) -> &EventInvokeConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventInvokeConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventInvokeConfig {}

impl From<EventInvokeConfigProperties> for EventInvokeConfig {
    fn from(properties: EventInvokeConfigProperties) -> EventInvokeConfig {
        EventInvokeConfig { properties }
    }
}

/// The [`AWS::Lambda::EventSourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html) resource type.
#[derive(Debug, Default)]
pub struct EventSourceMapping {
    properties: EventSourceMappingProperties
}

/// Properties for the `EventSourceMapping` resource.
#[derive(Debug, Default)]
pub struct EventSourceMappingProperties {
    /// Property [`AmazonManagedKafkaEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-amazonmanagedkafkaeventsourceconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub amazon_managed_kafka_event_source_config: Option<::Value<self::event_source_mapping::AmazonManagedKafkaEventSourceConfig>>,
    /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-batchsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub batch_size: Option<::Value<u32>>,
    /// Property [`BisectBatchOnFunctionError`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-bisectbatchonfunctionerror).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bisect_batch_on_function_error: Option<::Value<bool>>,
    /// Property [`DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-destinationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_config: Option<::Value<self::event_source_mapping::DestinationConfig>>,
    /// Property [`DocumentDBEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-documentdbeventsourceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub document_db_event_source_config: Option<::Value<self::event_source_mapping::DocumentDBEventSourceConfig>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`EventSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-eventsourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_arn: Option<::Value<String>>,
    /// Property [`FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-filtercriteria).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_criteria: Option<::Value<self::event_source_mapping::FilterCriteria>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-functionname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`FunctionResponseTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-functionresponsetypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_response_types: Option<::ValueList<String>>,
    /// Property [`MaximumBatchingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumbatchingwindowinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_batching_window_in_seconds: Option<::Value<u32>>,
    /// Property [`MaximumRecordAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumrecordageinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_record_age_in_seconds: Option<::Value<u32>>,
    /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-maximumretryattempts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_retry_attempts: Option<::Value<u32>>,
    /// Property [`ParallelizationFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-parallelizationfactor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parallelization_factor: Option<::Value<u32>>,
    /// Property [`Queues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-queues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub queues: Option<::ValueList<String>>,
    /// Property [`ScalingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-scalingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_config: Option<::Value<self::event_source_mapping::ScalingConfig>>,
    /// Property [`SelfManagedEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-selfmanagedeventsource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub self_managed_event_source: Option<::Value<self::event_source_mapping::SelfManagedEventSource>>,
    /// Property [`SelfManagedKafkaEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-selfmanagedkafkaeventsourceconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub self_managed_kafka_event_source_config: Option<::Value<self::event_source_mapping::SelfManagedKafkaEventSourceConfig>>,
    /// Property [`SourceAccessConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-sourceaccessconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_access_configurations: Option<::ValueList<self::event_source_mapping::SourceAccessConfiguration>>,
    /// Property [`StartingPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-startingposition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub starting_position: Option<::Value<String>>,
    /// Property [`StartingPositionTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-startingpositiontimestamp).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub starting_position_timestamp: Option<::Value<f64>>,
    /// Property [`Topics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-topics).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topics: Option<::ValueList<String>>,
    /// Property [`TumblingWindowInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html#cfn-lambda-eventsourcemapping-tumblingwindowinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tumbling_window_in_seconds: Option<::Value<u32>>,
}

impl ::serde::Serialize for EventSourceMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref amazon_managed_kafka_event_source_config) = self.amazon_managed_kafka_event_source_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonManagedKafkaEventSourceConfig", amazon_managed_kafka_event_source_config)?;
        }
        if let Some(ref batch_size) = self.batch_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", batch_size)?;
        }
        if let Some(ref bisect_batch_on_function_error) = self.bisect_batch_on_function_error {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BisectBatchOnFunctionError", bisect_batch_on_function_error)?;
        }
        if let Some(ref destination_config) = self.destination_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationConfig", destination_config)?;
        }
        if let Some(ref document_db_event_source_config) = self.document_db_event_source_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDBEventSourceConfig", document_db_event_source_config)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref event_source_arn) = self.event_source_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceArn", event_source_arn)?;
        }
        if let Some(ref filter_criteria) = self.filter_criteria {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterCriteria", filter_criteria)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", &self.function_name)?;
        if let Some(ref function_response_types) = self.function_response_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionResponseTypes", function_response_types)?;
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
        if let Some(ref parallelization_factor) = self.parallelization_factor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelizationFactor", parallelization_factor)?;
        }
        if let Some(ref queues) = self.queues {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Queues", queues)?;
        }
        if let Some(ref scaling_config) = self.scaling_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingConfig", scaling_config)?;
        }
        if let Some(ref self_managed_event_source) = self.self_managed_event_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedEventSource", self_managed_event_source)?;
        }
        if let Some(ref self_managed_kafka_event_source_config) = self.self_managed_kafka_event_source_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedKafkaEventSourceConfig", self_managed_kafka_event_source_config)?;
        }
        if let Some(ref source_access_configurations) = self.source_access_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccessConfigurations", source_access_configurations)?;
        }
        if let Some(ref starting_position) = self.starting_position {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPosition", starting_position)?;
        }
        if let Some(ref starting_position_timestamp) = self.starting_position_timestamp {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartingPositionTimestamp", starting_position_timestamp)?;
        }
        if let Some(ref topics) = self.topics {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topics", topics)?;
        }
        if let Some(ref tumbling_window_in_seconds) = self.tumbling_window_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TumblingWindowInSeconds", tumbling_window_in_seconds)?;
        }
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
                let mut amazon_managed_kafka_event_source_config: Option<::Value<self::event_source_mapping::AmazonManagedKafkaEventSourceConfig>> = None;
                let mut batch_size: Option<::Value<u32>> = None;
                let mut bisect_batch_on_function_error: Option<::Value<bool>> = None;
                let mut destination_config: Option<::Value<self::event_source_mapping::DestinationConfig>> = None;
                let mut document_db_event_source_config: Option<::Value<self::event_source_mapping::DocumentDBEventSourceConfig>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut event_source_arn: Option<::Value<String>> = None;
                let mut filter_criteria: Option<::Value<self::event_source_mapping::FilterCriteria>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut function_response_types: Option<::ValueList<String>> = None;
                let mut maximum_batching_window_in_seconds: Option<::Value<u32>> = None;
                let mut maximum_record_age_in_seconds: Option<::Value<u32>> = None;
                let mut maximum_retry_attempts: Option<::Value<u32>> = None;
                let mut parallelization_factor: Option<::Value<u32>> = None;
                let mut queues: Option<::ValueList<String>> = None;
                let mut scaling_config: Option<::Value<self::event_source_mapping::ScalingConfig>> = None;
                let mut self_managed_event_source: Option<::Value<self::event_source_mapping::SelfManagedEventSource>> = None;
                let mut self_managed_kafka_event_source_config: Option<::Value<self::event_source_mapping::SelfManagedKafkaEventSourceConfig>> = None;
                let mut source_access_configurations: Option<::ValueList<self::event_source_mapping::SourceAccessConfiguration>> = None;
                let mut starting_position: Option<::Value<String>> = None;
                let mut starting_position_timestamp: Option<::Value<f64>> = None;
                let mut topics: Option<::ValueList<String>> = None;
                let mut tumbling_window_in_seconds: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AmazonManagedKafkaEventSourceConfig" => {
                            amazon_managed_kafka_event_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BatchSize" => {
                            batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BisectBatchOnFunctionError" => {
                            bisect_batch_on_function_error = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationConfig" => {
                            destination_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentDBEventSourceConfig" => {
                            document_db_event_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSourceArn" => {
                            event_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FilterCriteria" => {
                            filter_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionResponseTypes" => {
                            function_response_types = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "ParallelizationFactor" => {
                            parallelization_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Queues" => {
                            queues = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingConfig" => {
                            scaling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SelfManagedEventSource" => {
                            self_managed_event_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SelfManagedKafkaEventSourceConfig" => {
                            self_managed_kafka_event_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAccessConfigurations" => {
                            source_access_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartingPosition" => {
                            starting_position = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartingPositionTimestamp" => {
                            starting_position_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Topics" => {
                            topics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TumblingWindowInSeconds" => {
                            tumbling_window_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventSourceMappingProperties {
                    amazon_managed_kafka_event_source_config: amazon_managed_kafka_event_source_config,
                    batch_size: batch_size,
                    bisect_batch_on_function_error: bisect_batch_on_function_error,
                    destination_config: destination_config,
                    document_db_event_source_config: document_db_event_source_config,
                    enabled: enabled,
                    event_source_arn: event_source_arn,
                    filter_criteria: filter_criteria,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    function_response_types: function_response_types,
                    maximum_batching_window_in_seconds: maximum_batching_window_in_seconds,
                    maximum_record_age_in_seconds: maximum_record_age_in_seconds,
                    maximum_retry_attempts: maximum_retry_attempts,
                    parallelization_factor: parallelization_factor,
                    queues: queues,
                    scaling_config: scaling_config,
                    self_managed_event_source: self_managed_event_source,
                    self_managed_kafka_event_source_config: self_managed_kafka_event_source_config,
                    source_access_configurations: source_access_configurations,
                    starting_position: starting_position,
                    starting_position_timestamp: starting_position_timestamp,
                    topics: topics,
                    tumbling_window_in_seconds: tumbling_window_in_seconds,
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
#[derive(Debug, Default)]
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Debug, Default)]
pub struct FunctionProperties {
    /// Property [`Architectures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-architectures).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub architectures: Option<::ValueList<String>>,
    /// Property [`Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-code).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code: ::Value<self::function::Code>,
    /// Property [`CodeSigningConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-codesigningconfigarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub code_signing_config_arn: Option<::Value<String>>,
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
    /// Property [`EphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-ephemeralstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ephemeral_storage: Option<::Value<self::function::EphemeralStorage>>,
    /// Property [`FileSystemConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-filesystemconfigs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_system_configs: Option<::ValueList<self::function::FileSystemConfig>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: Option<::Value<String>>,
    /// Property [`Handler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-handler).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub handler: Option<::Value<String>>,
    /// Property [`ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-imageconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_config: Option<::Value<self::function::ImageConfig>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`Layers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-layers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub layers: Option<::ValueList<String>>,
    /// Property [`LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-loggingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_config: Option<::Value<self::function::LoggingConfig>>,
    /// Property [`MemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-memorysize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub memory_size: Option<::Value<u32>>,
    /// Property [`PackageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-packagetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub package_type: Option<::Value<String>>,
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
    pub runtime: Option<::Value<String>>,
    /// Property [`RuntimeManagementConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-runtimemanagementconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub runtime_management_config: Option<::Value<self::function::RuntimeManagementConfig>>,
    /// Property [`SnapStart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html#cfn-lambda-function-snapstart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snap_start: Option<::Value<self::function::SnapStart>>,
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
        if let Some(ref architectures) = self.architectures {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architectures", architectures)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Code", &self.code)?;
        if let Some(ref code_signing_config_arn) = self.code_signing_config_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeSigningConfigArn", code_signing_config_arn)?;
        }
        if let Some(ref dead_letter_config) = self.dead_letter_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref ephemeral_storage) = self.ephemeral_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EphemeralStorage", ephemeral_storage)?;
        }
        if let Some(ref file_system_configs) = self.file_system_configs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemConfigs", file_system_configs)?;
        }
        if let Some(ref function_name) = self.function_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionName", function_name)?;
        }
        if let Some(ref handler) = self.handler {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handler", handler)?;
        }
        if let Some(ref image_config) = self.image_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfig", image_config)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref layers) = self.layers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Layers", layers)?;
        }
        if let Some(ref logging_config) = self.logging_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfig", logging_config)?;
        }
        if let Some(ref memory_size) = self.memory_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySize", memory_size)?;
        }
        if let Some(ref package_type) = self.package_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageType", package_type)?;
        }
        if let Some(ref reserved_concurrent_executions) = self.reserved_concurrent_executions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservedConcurrentExecutions", reserved_concurrent_executions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        if let Some(ref runtime) = self.runtime {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", runtime)?;
        }
        if let Some(ref runtime_management_config) = self.runtime_management_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeManagementConfig", runtime_management_config)?;
        }
        if let Some(ref snap_start) = self.snap_start {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapStart", snap_start)?;
        }
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
                let mut architectures: Option<::ValueList<String>> = None;
                let mut code: Option<::Value<self::function::Code>> = None;
                let mut code_signing_config_arn: Option<::Value<String>> = None;
                let mut dead_letter_config: Option<::Value<self::function::DeadLetterConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut environment: Option<::Value<self::function::Environment>> = None;
                let mut ephemeral_storage: Option<::Value<self::function::EphemeralStorage>> = None;
                let mut file_system_configs: Option<::ValueList<self::function::FileSystemConfig>> = None;
                let mut function_name: Option<::Value<String>> = None;
                let mut handler: Option<::Value<String>> = None;
                let mut image_config: Option<::Value<self::function::ImageConfig>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut layers: Option<::ValueList<String>> = None;
                let mut logging_config: Option<::Value<self::function::LoggingConfig>> = None;
                let mut memory_size: Option<::Value<u32>> = None;
                let mut package_type: Option<::Value<String>> = None;
                let mut reserved_concurrent_executions: Option<::Value<u32>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut runtime: Option<::Value<String>> = None;
                let mut runtime_management_config: Option<::Value<self::function::RuntimeManagementConfig>> = None;
                let mut snap_start: Option<::Value<self::function::SnapStart>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout: Option<::Value<u32>> = None;
                let mut tracing_config: Option<::Value<self::function::TracingConfig>> = None;
                let mut vpc_config: Option<::Value<self::function::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Architectures" => {
                            architectures = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Code" => {
                            code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CodeSigningConfigArn" => {
                            code_signing_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "EphemeralStorage" => {
                            ephemeral_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemConfigs" => {
                            file_system_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionName" => {
                            function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Handler" => {
                            handler = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageConfig" => {
                            image_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Layers" => {
                            layers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfig" => {
                            logging_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemorySize" => {
                            memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageType" => {
                            package_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "RuntimeManagementConfig" => {
                            runtime_management_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapStart" => {
                            snap_start = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    architectures: architectures,
                    code: code.ok_or(::serde::de::Error::missing_field("Code"))?,
                    code_signing_config_arn: code_signing_config_arn,
                    dead_letter_config: dead_letter_config,
                    description: description,
                    environment: environment,
                    ephemeral_storage: ephemeral_storage,
                    file_system_configs: file_system_configs,
                    function_name: function_name,
                    handler: handler,
                    image_config: image_config,
                    kms_key_arn: kms_key_arn,
                    layers: layers,
                    logging_config: logging_config,
                    memory_size: memory_size,
                    package_type: package_type,
                    reserved_concurrent_executions: reserved_concurrent_executions,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    runtime: runtime,
                    runtime_management_config: runtime_management_config,
                    snap_start: snap_start,
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

/// The [`AWS::Lambda::LayerVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html) resource type.
#[derive(Debug, Default)]
pub struct LayerVersion {
    properties: LayerVersionProperties
}

/// Properties for the `LayerVersion` resource.
#[derive(Debug, Default)]
pub struct LayerVersionProperties {
    /// Property [`CompatibleArchitectures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-compatiblearchitectures).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compatible_architectures: Option<::ValueList<String>>,
    /// Property [`CompatibleRuntimes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-compatibleruntimes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compatible_runtimes: Option<::ValueList<String>>,
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-content).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub content: ::Value<self::layer_version::Content>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`LayerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-layername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub layer_name: Option<::Value<String>>,
    /// Property [`LicenseInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversion.html#cfn-lambda-layerversion-licenseinfo).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub license_info: Option<::Value<String>>,
}

impl ::serde::Serialize for LayerVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compatible_architectures) = self.compatible_architectures {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompatibleArchitectures", compatible_architectures)?;
        }
        if let Some(ref compatible_runtimes) = self.compatible_runtimes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompatibleRuntimes", compatible_runtimes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref layer_name) = self.layer_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerName", layer_name)?;
        }
        if let Some(ref license_info) = self.license_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseInfo", license_info)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LayerVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LayerVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LayerVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LayerVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compatible_architectures: Option<::ValueList<String>> = None;
                let mut compatible_runtimes: Option<::ValueList<String>> = None;
                let mut content: Option<::Value<self::layer_version::Content>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut layer_name: Option<::Value<String>> = None;
                let mut license_info: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CompatibleArchitectures" => {
                            compatible_architectures = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CompatibleRuntimes" => {
                            compatible_runtimes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerName" => {
                            layer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseInfo" => {
                            license_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LayerVersionProperties {
                    compatible_architectures: compatible_architectures,
                    compatible_runtimes: compatible_runtimes,
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    layer_name: layer_name,
                    license_info: license_info,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LayerVersion {
    type Properties = LayerVersionProperties;
    const TYPE: &'static str = "AWS::Lambda::LayerVersion";
    fn properties(&self) -> &LayerVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LayerVersion {}

impl From<LayerVersionProperties> for LayerVersion {
    fn from(properties: LayerVersionProperties) -> LayerVersion {
        LayerVersion { properties }
    }
}

/// The [`AWS::Lambda::LayerVersionPermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html) resource type.
#[derive(Debug, Default)]
pub struct LayerVersionPermission {
    properties: LayerVersionPermissionProperties
}

/// Properties for the `LayerVersionPermission` resource.
#[derive(Debug, Default)]
pub struct LayerVersionPermissionProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: ::Value<String>,
    /// Property [`LayerVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-layerversionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub layer_version_arn: ::Value<String>,
    /// Property [`OrganizationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-organizationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub organization_id: Option<::Value<String>>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-layerversionpermission.html#cfn-lambda-layerversionpermission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
}

impl ::serde::Serialize for LayerVersionPermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerVersionArn", &self.layer_version_arn)?;
        if let Some(ref organization_id) = self.organization_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationId", organization_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LayerVersionPermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LayerVersionPermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LayerVersionPermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LayerVersionPermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut layer_version_arn: Option<::Value<String>> = None;
                let mut organization_id: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerVersionArn" => {
                            layer_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationId" => {
                            organization_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LayerVersionPermissionProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    layer_version_arn: layer_version_arn.ok_or(::serde::de::Error::missing_field("LayerVersionArn"))?,
                    organization_id: organization_id,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LayerVersionPermission {
    type Properties = LayerVersionPermissionProperties;
    const TYPE: &'static str = "AWS::Lambda::LayerVersionPermission";
    fn properties(&self) -> &LayerVersionPermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerVersionPermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LayerVersionPermission {}

impl From<LayerVersionPermissionProperties> for LayerVersionPermission {
    fn from(properties: LayerVersionPermissionProperties) -> LayerVersionPermission {
        LayerVersionPermission { properties }
    }
}

/// The [`AWS::Lambda::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html) resource type.
#[derive(Debug, Default)]
pub struct Permission {
    properties: PermissionProperties
}

/// Properties for the `Permission` resource.
#[derive(Debug, Default)]
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
    /// Property [`FunctionUrlAuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-functionurlauthtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_url_auth_type: Option<::Value<String>>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`PrincipalOrgID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html#cfn-lambda-permission-principalorgid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal_org_id: Option<::Value<String>>,
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
        if let Some(ref function_url_auth_type) = self.function_url_auth_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionUrlAuthType", function_url_auth_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        if let Some(ref principal_org_id) = self.principal_org_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalOrgID", principal_org_id)?;
        }
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
                let mut function_url_auth_type: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;
                let mut principal_org_id: Option<::Value<String>> = None;
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
                        "FunctionUrlAuthType" => {
                            function_url_auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalOrgID" => {
                            principal_org_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    function_url_auth_type: function_url_auth_type,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    principal_org_id: principal_org_id,
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

/// The [`AWS::Lambda::Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html) resource type.
#[derive(Debug, Default)]
pub struct Url {
    properties: UrlProperties
}

/// Properties for the `Url` resource.
#[derive(Debug, Default)]
pub struct UrlProperties {
    /// Property [`AuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html#cfn-lambda-url-authtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth_type: ::Value<String>,
    /// Property [`Cors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html#cfn-lambda-url-cors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cors: Option<::Value<self::url::Cors>>,
    /// Property [`InvokeMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html#cfn-lambda-url-invokemode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub invoke_mode: Option<::Value<String>>,
    /// Property [`Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html#cfn-lambda-url-qualifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub qualifier: Option<::Value<String>>,
    /// Property [`TargetFunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-url.html#cfn-lambda-url-targetfunctionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_function_arn: ::Value<String>,
}

impl ::serde::Serialize for UrlProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", &self.auth_type)?;
        if let Some(ref cors) = self.cors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cors", cors)?;
        }
        if let Some(ref invoke_mode) = self.invoke_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvokeMode", invoke_mode)?;
        }
        if let Some(ref qualifier) = self.qualifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", qualifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetFunctionArn", &self.target_function_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UrlProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UrlProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UrlProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UrlProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth_type: Option<::Value<String>> = None;
                let mut cors: Option<::Value<self::url::Cors>> = None;
                let mut invoke_mode: Option<::Value<String>> = None;
                let mut qualifier: Option<::Value<String>> = None;
                let mut target_function_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthType" => {
                            auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cors" => {
                            cors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InvokeMode" => {
                            invoke_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Qualifier" => {
                            qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetFunctionArn" => {
                            target_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UrlProperties {
                    auth_type: auth_type.ok_or(::serde::de::Error::missing_field("AuthType"))?,
                    cors: cors,
                    invoke_mode: invoke_mode,
                    qualifier: qualifier,
                    target_function_arn: target_function_arn.ok_or(::serde::de::Error::missing_field("TargetFunctionArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Url {
    type Properties = UrlProperties;
    const TYPE: &'static str = "AWS::Lambda::Url";
    fn properties(&self) -> &UrlProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UrlProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Url {}

impl From<UrlProperties> for Url {
    fn from(properties: UrlProperties) -> Url {
        Url { properties }
    }
}

/// The [`AWS::Lambda::Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html) resource type.
#[derive(Debug, Default)]
pub struct Version {
    properties: VersionProperties
}

/// Properties for the `Version` resource.
#[derive(Debug, Default)]
pub struct VersionProperties {
    /// Property [`CodeSha256`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-codesha256).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub code_sha256: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-functionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub function_name: ::Value<String>,
    /// Property [`ProvisionedConcurrencyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-provisionedconcurrencyconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provisioned_concurrency_config: Option<::Value<self::version::ProvisionedConcurrencyConfiguration>>,
    /// Property [`RuntimePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html#cfn-lambda-version-runtimepolicy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub runtime_policy: Option<::Value<self::version::RuntimePolicy>>,
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
        if let Some(ref provisioned_concurrency_config) = self.provisioned_concurrency_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrencyConfig", provisioned_concurrency_config)?;
        }
        if let Some(ref runtime_policy) = self.runtime_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimePolicy", runtime_policy)?;
        }
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
                let mut provisioned_concurrency_config: Option<::Value<self::version::ProvisionedConcurrencyConfiguration>> = None;
                let mut runtime_policy: Option<::Value<self::version::RuntimePolicy>> = None;

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
                        "ProvisionedConcurrencyConfig" => {
                            provisioned_concurrency_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimePolicy" => {
                            runtime_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VersionProperties {
                    code_sha256: code_sha256,
                    description: description,
                    function_name: function_name.ok_or(::serde::de::Error::missing_field("FunctionName"))?,
                    provisioned_concurrency_config: provisioned_concurrency_config,
                    runtime_policy: runtime_policy,
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
    #[derive(Debug, Default)]
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

    /// The [`AWS::Lambda::Alias.ProvisionedConcurrencyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-provisionedconcurrencyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedConcurrencyConfiguration {
        /// Property [`ProvisionedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-provisionedconcurrencyconfiguration.html#cfn-lambda-alias-provisionedconcurrencyconfiguration-provisionedconcurrentexecutions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_concurrent_executions: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ProvisionedConcurrencyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrentExecutions", &self.provisioned_concurrent_executions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionedConcurrencyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedConcurrencyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedConcurrencyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedConcurrencyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provisioned_concurrent_executions: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProvisionedConcurrentExecutions" => {
                                provisioned_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedConcurrencyConfiguration {
                        provisioned_concurrent_executions: provisioned_concurrent_executions.ok_or(::serde::de::Error::missing_field("ProvisionedConcurrentExecutions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Debug, Default)]
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

pub mod code_signing_config {
    //! Property types for the `CodeSigningConfig` resource.

    /// The [`AWS::Lambda::CodeSigningConfig.AllowedPublishers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-allowedpublishers.html) property type.
    #[derive(Debug, Default)]
    pub struct AllowedPublishers {
        /// Property [`SigningProfileVersionArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-allowedpublishers.html#cfn-lambda-codesigningconfig-allowedpublishers-signingprofileversionarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_profile_version_arns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AllowedPublishers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningProfileVersionArns", &self.signing_profile_version_arns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AllowedPublishers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AllowedPublishers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AllowedPublishers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AllowedPublishers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut signing_profile_version_arns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SigningProfileVersionArns" => {
                                signing_profile_version_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AllowedPublishers {
                        signing_profile_version_arns: signing_profile_version_arns.ok_or(::serde::de::Error::missing_field("SigningProfileVersionArns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::CodeSigningConfig.CodeSigningPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-codesigningpolicies.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeSigningPolicies {
        /// Property [`UntrustedArtifactOnDeployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-codesigningconfig-codesigningpolicies.html#cfn-lambda-codesigningconfig-codesigningpolicies-untrustedartifactondeployment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub untrusted_artifact_on_deployment: ::Value<String>,
    }

    impl ::codec::SerializeValue for CodeSigningPolicies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UntrustedArtifactOnDeployment", &self.untrusted_artifact_on_deployment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeSigningPolicies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeSigningPolicies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeSigningPolicies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeSigningPolicies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut untrusted_artifact_on_deployment: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UntrustedArtifactOnDeployment" => {
                                untrusted_artifact_on_deployment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeSigningPolicies {
                        untrusted_artifact_on_deployment: untrusted_artifact_on_deployment.ok_or(::serde::de::Error::missing_field("UntrustedArtifactOnDeployment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_invoke_config {
    //! Property types for the `EventInvokeConfig` resource.

    /// The [`AWS::Lambda::EventInvokeConfig.DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationConfig {
        /// Property [`OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig-onfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_failure: Option<::Value<OnFailure>>,
        /// Property [`OnSuccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-destinationconfig.html#cfn-lambda-eventinvokeconfig-destinationconfig-onsuccess).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_success: Option<::Value<OnSuccess>>,
    }

    impl ::codec::SerializeValue for DestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_failure) = self.on_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnFailure", on_failure)?;
            }
            if let Some(ref on_success) = self.on_success {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnSuccess", on_success)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_failure: Option<::Value<OnFailure>> = None;
                    let mut on_success: Option<::Value<OnSuccess>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnFailure" => {
                                on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnSuccess" => {
                                on_success = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationConfig {
                        on_failure: on_failure,
                        on_success: on_success,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventInvokeConfig.OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onfailure.html) property type.
    #[derive(Debug, Default)]
    pub struct OnFailure {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onfailure.html#cfn-lambda-eventinvokeconfig-onfailure-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<String>,
    }

    impl ::codec::SerializeValue for OnFailure {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OnFailure {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnFailure, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnFailure;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnFailure")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnFailure {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventInvokeConfig.OnSuccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onsuccess.html) property type.
    #[derive(Debug, Default)]
    pub struct OnSuccess {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventinvokeconfig-onsuccess.html#cfn-lambda-eventinvokeconfig-onsuccess-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<String>,
    }

    impl ::codec::SerializeValue for OnSuccess {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OnSuccess {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnSuccess, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnSuccess;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnSuccess")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnSuccess {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_source_mapping {
    //! Property types for the `EventSourceMapping` resource.

    /// The [`AWS::Lambda::EventSourceMapping.AmazonManagedKafkaEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-amazonmanagedkafkaeventsourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonManagedKafkaEventSourceConfig {
        /// Property [`ConsumerGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-amazonmanagedkafkaeventsourceconfig.html#cfn-lambda-eventsourcemapping-amazonmanagedkafkaeventsourceconfig-consumergroupid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub consumer_group_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AmazonManagedKafkaEventSourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref consumer_group_id) = self.consumer_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupId", consumer_group_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonManagedKafkaEventSourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonManagedKafkaEventSourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonManagedKafkaEventSourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonManagedKafkaEventSourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut consumer_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConsumerGroupId" => {
                                consumer_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmazonManagedKafkaEventSourceConfig {
                        consumer_group_id: consumer_group_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.DestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-destinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationConfig {
        /// Property [`OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-destinationconfig.html#cfn-lambda-eventsourcemapping-destinationconfig-onfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_failure: Option<::Value<OnFailure>>,
    }

    impl ::codec::SerializeValue for DestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_failure) = self.on_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnFailure", on_failure)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_failure: Option<::Value<OnFailure>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnFailure" => {
                                on_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationConfig {
                        on_failure: on_failure,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.DocumentDBEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-documentdbeventsourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentDBEventSourceConfig {
        /// Property [`CollectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-documentdbeventsourceconfig.html#cfn-lambda-eventsourcemapping-documentdbeventsourceconfig-collectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub collection_name: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-documentdbeventsourceconfig.html#cfn-lambda-eventsourcemapping-documentdbeventsourceconfig-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`FullDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-documentdbeventsourceconfig.html#cfn-lambda-eventsourcemapping-documentdbeventsourceconfig-fulldocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub full_document: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentDBEventSourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref collection_name) = self.collection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionName", collection_name)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref full_document) = self.full_document {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullDocument", full_document)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentDBEventSourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentDBEventSourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentDBEventSourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentDBEventSourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut collection_name: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut full_document: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CollectionName" => {
                                collection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullDocument" => {
                                full_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentDBEventSourceConfig {
                        collection_name: collection_name,
                        database_name: database_name,
                        full_document: full_document,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.Endpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-endpoints.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoints {
        /// Property [`KafkaBootstrapServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-endpoints.html#cfn-lambda-eventsourcemapping-endpoints-kafkabootstrapservers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kafka_bootstrap_servers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Endpoints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kafka_bootstrap_servers) = self.kafka_bootstrap_servers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaBootstrapServers", kafka_bootstrap_servers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Endpoints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kafka_bootstrap_servers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KafkaBootstrapServers" => {
                                kafka_bootstrap_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoints {
                        kafka_bootstrap_servers: kafka_bootstrap_servers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filter.html#cfn-lambda-eventsourcemapping-filter-pattern).
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

    /// The [`AWS::Lambda::EventSourceMapping.FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filtercriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterCriteria {
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-filtercriteria.html#cfn-lambda-eventsourcemapping-filtercriteria-filters).
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

    /// The [`AWS::Lambda::EventSourceMapping.OnFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-onfailure.html) property type.
    #[derive(Debug, Default)]
    pub struct OnFailure {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-onfailure.html#cfn-lambda-eventsourcemapping-onfailure-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OnFailure {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OnFailure {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnFailure, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnFailure;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnFailure")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnFailure {
                        destination: destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.ScalingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-scalingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ScalingConfig {
        /// Property [`MaximumConcurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-scalingconfig.html#cfn-lambda-eventsourcemapping-scalingconfig-maximumconcurrency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_concurrency: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ScalingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_concurrency) = self.maximum_concurrency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumConcurrency", maximum_concurrency)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_concurrency: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumConcurrency" => {
                                maximum_concurrency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConfig {
                        maximum_concurrency: maximum_concurrency,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.SelfManagedEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedeventsource.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedEventSource {
        /// Property [`Endpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedeventsource.html#cfn-lambda-eventsourcemapping-selfmanagedeventsource-endpoints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoints: Option<::Value<Endpoints>>,
    }

    impl ::codec::SerializeValue for SelfManagedEventSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoints) = self.endpoints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoints", endpoints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedEventSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedEventSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedEventSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedEventSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoints: Option<::Value<Endpoints>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoints" => {
                                endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedEventSource {
                        endpoints: endpoints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.SelfManagedKafkaEventSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedkafkaeventsourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedKafkaEventSourceConfig {
        /// Property [`ConsumerGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-selfmanagedkafkaeventsourceconfig.html#cfn-lambda-eventsourcemapping-selfmanagedkafkaeventsourceconfig-consumergroupid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub consumer_group_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SelfManagedKafkaEventSourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref consumer_group_id) = self.consumer_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumerGroupId", consumer_group_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedKafkaEventSourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedKafkaEventSourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedKafkaEventSourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedKafkaEventSourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut consumer_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConsumerGroupId" => {
                                consumer_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedKafkaEventSourceConfig {
                        consumer_group_id: consumer_group_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::EventSourceMapping.SourceAccessConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceAccessConfiguration {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html#cfn-lambda-eventsourcemapping-sourceaccessconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`URI`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-eventsourcemapping-sourceaccessconfiguration.html#cfn-lambda-eventsourcemapping-sourceaccessconfiguration-uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SourceAccessConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref uri) = self.uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "URI", uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceAccessConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceAccessConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceAccessConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceAccessConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "URI" => {
                                uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAccessConfiguration {
                        r#type: r#type,
                        uri: uri,
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
    #[derive(Debug, Default)]
    pub struct Code {
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html#cfn-lambda-function-code-imageuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_uri: Option<::Value<String>>,
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
            if let Some(ref image_uri) = self.image_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", image_uri)?;
            }
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
                    let mut image_uri: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;
                    let mut s3_object_version: Option<::Value<String>> = None;
                    let mut zip_file: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "ZipFile" => {
                                zip_file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Code {
                        image_uri: image_uri,
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
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
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

    /// The [`AWS::Lambda::Function.EphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-ephemeralstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct EphemeralStorage {
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-ephemeralstorage.html#cfn-lambda-function-ephemeralstorage-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: ::Value<u32>,
    }

    impl ::codec::SerializeValue for EphemeralStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EphemeralStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EphemeralStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EphemeralStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EphemeralStorage")
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

                    Ok(EphemeralStorage {
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.FileSystemConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FileSystemConfig {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html#cfn-lambda-function-filesystemconfig-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`LocalMountPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-filesystemconfig.html#cfn-lambda-function-filesystemconfig-localmountpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub local_mount_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for FileSystemConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalMountPath", &self.local_mount_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileSystemConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSystemConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileSystemConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileSystemConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut local_mount_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalMountPath" => {
                                local_mount_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileSystemConfig {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        local_mount_path: local_mount_path.ok_or(::serde::de::Error::missing_field("LocalMountPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageConfig {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-command).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`EntryPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-entrypoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry_point: Option<::ValueList<String>>,
        /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-imageconfig.html#cfn-lambda-function-imageconfig-workingdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub working_directory: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ImageConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref entry_point) = self.entry_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryPoint", entry_point)?;
            }
            if let Some(ref working_directory) = self.working_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<::ValueList<String>> = None;
                    let mut entry_point: Option<::ValueList<String>> = None;
                    let mut working_directory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryPoint" => {
                                entry_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkingDirectory" => {
                                working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageConfig {
                        command: command,
                        entry_point: entry_point,
                        working_directory: working_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.LoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfig {
        /// Property [`ApplicationLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html#cfn-lambda-function-loggingconfig-applicationloglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_log_level: Option<::Value<String>>,
        /// Property [`LogFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html#cfn-lambda-function-loggingconfig-logformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_format: Option<::Value<String>>,
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html#cfn-lambda-function-loggingconfig-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: Option<::Value<String>>,
        /// Property [`SystemLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-loggingconfig.html#cfn-lambda-function-loggingconfig-systemloglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub system_log_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_log_level) = self.application_log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationLogLevel", application_log_level)?;
            }
            if let Some(ref log_format) = self.log_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogFormat", log_format)?;
            }
            if let Some(ref log_group) = self.log_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", log_group)?;
            }
            if let Some(ref system_log_level) = self.system_log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemLogLevel", system_log_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_log_level: Option<::Value<String>> = None;
                    let mut log_format: Option<::Value<String>> = None;
                    let mut log_group: Option<::Value<String>> = None;
                    let mut system_log_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationLogLevel" => {
                                application_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogFormat" => {
                                log_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SystemLogLevel" => {
                                system_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfig {
                        application_log_level: application_log_level,
                        log_format: log_format,
                        log_group: log_group,
                        system_log_level: system_log_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.RuntimeManagementConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-runtimemanagementconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RuntimeManagementConfig {
        /// Property [`RuntimeVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-runtimemanagementconfig.html#cfn-lambda-function-runtimemanagementconfig-runtimeversionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime_version_arn: Option<::Value<String>>,
        /// Property [`UpdateRuntimeOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-runtimemanagementconfig.html#cfn-lambda-function-runtimemanagementconfig-updateruntimeon).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_runtime_on: ::Value<String>,
    }

    impl ::codec::SerializeValue for RuntimeManagementConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref runtime_version_arn) = self.runtime_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeVersionArn", runtime_version_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateRuntimeOn", &self.update_runtime_on)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuntimeManagementConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuntimeManagementConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuntimeManagementConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuntimeManagementConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut runtime_version_arn: Option<::Value<String>> = None;
                    let mut update_runtime_on: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuntimeVersionArn" => {
                                runtime_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateRuntimeOn" => {
                                update_runtime_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuntimeManagementConfig {
                        runtime_version_arn: runtime_version_arn,
                        update_runtime_on: update_runtime_on.ok_or(::serde::de::Error::missing_field("UpdateRuntimeOn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.SnapStart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstart.html) property type.
    #[derive(Debug, Default)]
    pub struct SnapStart {
        /// Property [`ApplyOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstart.html#cfn-lambda-function-snapstart-applyon).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_on: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnapStart {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyOn", &self.apply_on)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnapStart {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnapStart, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnapStart;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnapStart")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apply_on: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplyOn" => {
                                apply_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnapStart {
                        apply_on: apply_on.ok_or(::serde::de::Error::missing_field("ApplyOn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.SnapStartResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstartresponse.html) property type.
    #[derive(Debug, Default)]
    pub struct SnapStartResponse {
        /// Property [`ApplyOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstartresponse.html#cfn-lambda-function-snapstartresponse-applyon).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_on: Option<::Value<String>>,
        /// Property [`OptimizationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-snapstartresponse.html#cfn-lambda-function-snapstartresponse-optimizationstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub optimization_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SnapStartResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref apply_on) = self.apply_on {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyOn", apply_on)?;
            }
            if let Some(ref optimization_status) = self.optimization_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptimizationStatus", optimization_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnapStartResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnapStartResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnapStartResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnapStartResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apply_on: Option<::Value<String>> = None;
                    let mut optimization_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplyOn" => {
                                apply_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptimizationStatus" => {
                                optimization_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnapStartResponse {
                        apply_on: apply_on,
                        optimization_status: optimization_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Debug, Default)]
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
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`Ipv6AllowedForDualStack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-ipv6allowedfordualstack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6_allowed_for_dual_stack: Option<::Value<bool>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html#cfn-lambda-function-vpcconfig-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ipv6_allowed_for_dual_stack) = self.ipv6_allowed_for_dual_stack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6AllowedForDualStack", ipv6_allowed_for_dual_stack)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
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
                    let mut ipv6_allowed_for_dual_stack: Option<::Value<bool>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ipv6AllowedForDualStack" => {
                                ipv6_allowed_for_dual_stack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
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
                        ipv6_allowed_for_dual_stack: ipv6_allowed_for_dual_stack,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod layer_version {
    //! Property types for the `LayerVersion` resource.

    /// The [`AWS::Lambda::LayerVersion.Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html) property type.
    #[derive(Debug, Default)]
    pub struct Content {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_key: ::Value<String>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-layerversion-content.html#cfn-lambda-layerversion-content-s3objectversion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_object_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Content {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Key", &self.s3_key)?;
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Content {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Content, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Content;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Content")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_key: Option<::Value<String>> = None;
                    let mut s3_object_version: Option<::Value<String>> = None;

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
                            _ => {}
                        }
                    }

                    Ok(Content {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_key: s3_key.ok_or(::serde::de::Error::missing_field("S3Key"))?,
                        s3_object_version: s3_object_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod url {
    //! Property types for the `Url` resource.

    /// The [`AWS::Lambda::Url.Cors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html) property type.
    #[derive(Debug, Default)]
    pub struct Cors {
        /// Property [`AllowCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-allowcredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_credentials: Option<::Value<bool>>,
        /// Property [`AllowHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-allowheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_headers: Option<::ValueList<String>>,
        /// Property [`AllowMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-allowmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_methods: Option<::ValueList<String>>,
        /// Property [`AllowOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-alloworigins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_origins: Option<::ValueList<String>>,
        /// Property [`ExposeHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-exposeheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expose_headers: Option<::ValueList<String>>,
        /// Property [`MaxAge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-url-cors.html#cfn-lambda-url-cors-maxage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_age: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Cors {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_credentials) = self.allow_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowCredentials", allow_credentials)?;
            }
            if let Some(ref allow_headers) = self.allow_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowHeaders", allow_headers)?;
            }
            if let Some(ref allow_methods) = self.allow_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMethods", allow_methods)?;
            }
            if let Some(ref allow_origins) = self.allow_origins {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowOrigins", allow_origins)?;
            }
            if let Some(ref expose_headers) = self.expose_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExposeHeaders", expose_headers)?;
            }
            if let Some(ref max_age) = self.max_age {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAge", max_age)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Cors {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Cors, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Cors;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Cors")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_credentials: Option<::Value<bool>> = None;
                    let mut allow_headers: Option<::ValueList<String>> = None;
                    let mut allow_methods: Option<::ValueList<String>> = None;
                    let mut allow_origins: Option<::ValueList<String>> = None;
                    let mut expose_headers: Option<::ValueList<String>> = None;
                    let mut max_age: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowCredentials" => {
                                allow_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowHeaders" => {
                                allow_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowMethods" => {
                                allow_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowOrigins" => {
                                allow_origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExposeHeaders" => {
                                expose_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxAge" => {
                                max_age = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Cors {
                        allow_credentials: allow_credentials,
                        allow_headers: allow_headers,
                        allow_methods: allow_methods,
                        allow_origins: allow_origins,
                        expose_headers: expose_headers,
                        max_age: max_age,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod version {
    //! Property types for the `Version` resource.

    /// The [`AWS::Lambda::Version.ProvisionedConcurrencyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-provisionedconcurrencyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedConcurrencyConfiguration {
        /// Property [`ProvisionedConcurrentExecutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-provisionedconcurrencyconfiguration.html#cfn-lambda-version-provisionedconcurrencyconfiguration-provisionedconcurrentexecutions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub provisioned_concurrent_executions: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ProvisionedConcurrencyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedConcurrentExecutions", &self.provisioned_concurrent_executions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionedConcurrencyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedConcurrencyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedConcurrencyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedConcurrencyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provisioned_concurrent_executions: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProvisionedConcurrentExecutions" => {
                                provisioned_concurrent_executions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedConcurrencyConfiguration {
                        provisioned_concurrent_executions: provisioned_concurrent_executions.ok_or(::serde::de::Error::missing_field("ProvisionedConcurrentExecutions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lambda::Version.RuntimePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-runtimepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct RuntimePolicy {
        /// Property [`RuntimeVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-runtimepolicy.html#cfn-lambda-version-runtimepolicy-runtimeversionarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub runtime_version_arn: Option<::Value<String>>,
        /// Property [`UpdateRuntimeOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-version-runtimepolicy.html#cfn-lambda-version-runtimepolicy-updateruntimeon).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub update_runtime_on: ::Value<String>,
    }

    impl ::codec::SerializeValue for RuntimePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref runtime_version_arn) = self.runtime_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeVersionArn", runtime_version_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateRuntimeOn", &self.update_runtime_on)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuntimePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuntimePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuntimePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuntimePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut runtime_version_arn: Option<::Value<String>> = None;
                    let mut update_runtime_on: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuntimeVersionArn" => {
                                runtime_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateRuntimeOn" => {
                                update_runtime_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuntimePolicy {
                        runtime_version_arn: runtime_version_arn,
                        update_runtime_on: update_runtime_on.ok_or(::serde::de::Error::missing_field("UpdateRuntimeOn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
