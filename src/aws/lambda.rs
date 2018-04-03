//! Types for the `Lambda` service.

/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
#[derive(Debug)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AliasProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `FunctionName`.
    #[serde(rename = "FunctionName")]
    pub function_name: ::Value<String>,
    /// Property `FunctionVersion`.
    #[serde(rename = "FunctionVersion")]
    pub function_version: ::Value<String>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `RoutingConfig`.
    #[serde(rename = "RoutingConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<::Value<self::alias::AliasRoutingConfiguration>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct EventSourceMappingProperties {
    /// Property `BatchSize`.
    #[serde(rename = "BatchSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<::Value<u32>>,
    /// Property `Enabled`.
    #[serde(rename = "Enabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<::Value<bool>>,
    /// Property `EventSourceArn`.
    #[serde(rename = "EventSourceArn")]
    pub event_source_arn: ::Value<String>,
    /// Property `FunctionName`.
    #[serde(rename = "FunctionName")]
    pub function_name: ::Value<String>,
    /// Property `StartingPosition`.
    #[serde(rename = "StartingPosition")]
    pub starting_position: ::Value<String>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionProperties {
    /// Property `Code`.
    #[serde(rename = "Code")]
    pub code: ::Value<self::function::Code>,
    /// Property `DeadLetterConfig`.
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<::Value<self::function::DeadLetterConfig>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Environment`.
    #[serde(rename = "Environment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<::Value<self::function::Environment>>,
    /// Property `FunctionName`.
    #[serde(rename = "FunctionName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_name: Option<::Value<String>>,
    /// Property `Handler`.
    #[serde(rename = "Handler")]
    pub handler: ::Value<String>,
    /// Property `KmsKeyArn`.
    #[serde(rename = "KmsKeyArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<::Value<String>>,
    /// Property `MemorySize`.
    #[serde(rename = "MemorySize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<::Value<u32>>,
    /// Property `ReservedConcurrentExecutions`.
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<::Value<u32>>,
    /// Property `Role`.
    #[serde(rename = "Role")]
    pub role: ::Value<String>,
    /// Property `Runtime`.
    #[serde(rename = "Runtime")]
    pub runtime: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Timeout`.
    #[serde(rename = "Timeout")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<::Value<u32>>,
    /// Property `TracingConfig`.
    #[serde(rename = "TracingConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<::Value<self::function::TracingConfig>>,
    /// Property `VpcConfig`.
    #[serde(rename = "VpcConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<::Value<self::function::VpcConfig>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionProperties {
    /// Property `Action`.
    #[serde(rename = "Action")]
    pub action: ::Value<String>,
    /// Property `EventSourceToken`.
    #[serde(rename = "EventSourceToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<::Value<String>>,
    /// Property `FunctionName`.
    #[serde(rename = "FunctionName")]
    pub function_name: ::Value<String>,
    /// Property `Principal`.
    #[serde(rename = "Principal")]
    pub principal: ::Value<String>,
    /// Property `SourceAccount`.
    #[serde(rename = "SourceAccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_account: Option<::Value<String>>,
    /// Property `SourceArn`.
    #[serde(rename = "SourceArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<::Value<String>>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionProperties {
    /// Property `CodeSha256`.
    #[serde(rename = "CodeSha256")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `FunctionName`.
    #[serde(rename = "FunctionName")]
    pub function_name: ::Value<String>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AliasRoutingConfiguration {
        /// Property `AdditionalVersionWeights`.
        #[serde(rename = "AdditionalVersionWeights")]
        pub additional_version_weights: ::ValueList<VersionWeight>,
    }

    cfn_internal__inherit_codec_impls!(AliasRoutingConfiguration);

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersionWeight {
        /// Property `FunctionVersion`.
        #[serde(rename = "FunctionVersion")]
        pub function_version: ::Value<String>,
        /// Property `FunctionWeight`.
        #[serde(rename = "FunctionWeight")]
        pub function_weight: ::Value<f64>,
    }

    cfn_internal__inherit_codec_impls!(VersionWeight);
}

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::Lambda::Function.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Code {
        /// Property `S3Bucket`.
        #[serde(rename = "S3Bucket")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_bucket: Option<::Value<String>>,
        /// Property `S3Key`.
        #[serde(rename = "S3Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_key: Option<::Value<String>>,
        /// Property `S3ObjectVersion`.
        #[serde(rename = "S3ObjectVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_object_version: Option<::Value<String>>,
        /// Property `ZipFile`.
        #[serde(rename = "ZipFile")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zip_file: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Code);

    /// The [`AWS::Lambda::Function.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeadLetterConfig {
        /// Property `TargetArn`.
        #[serde(rename = "TargetArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DeadLetterConfig);

    /// The [`AWS::Lambda::Function.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `Variables`.
        #[serde(rename = "Variables")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variables: Option<::ValueMap<String>>,
    }

    cfn_internal__inherit_codec_impls!(Environment);

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TracingConfig {
        /// Property `Mode`.
        #[serde(rename = "Mode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(TracingConfig);

    /// The [`AWS::Lambda::Function.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcConfig {
        /// Property `SecurityGroupIds`.
        #[serde(rename = "SecurityGroupIds")]
        pub security_group_ids: ::ValueList<String>,
        /// Property `SubnetIds`.
        #[serde(rename = "SubnetIds")]
        pub subnet_ids: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(VpcConfig);
}
