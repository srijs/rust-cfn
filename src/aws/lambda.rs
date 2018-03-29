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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `FunctionName`.
    #[serde(rename="FunctionName")]
    pub function_name: String,
    /// Property `FunctionVersion`.
    #[serde(rename="FunctionVersion")]
    pub function_version: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RoutingConfig`.
    #[serde(rename="RoutingConfig")]
    pub routing_config: self::alias::AliasRoutingConfiguration,
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
    #[serde(rename="BatchSize")]
    pub batch_size: u32,
    /// Property `Enabled`.
    #[serde(rename="Enabled")]
    pub enabled: bool,
    /// Property `EventSourceArn`.
    #[serde(rename="EventSourceArn")]
    pub event_source_arn: String,
    /// Property `FunctionName`.
    #[serde(rename="FunctionName")]
    pub function_name: String,
    /// Property `StartingPosition`.
    #[serde(rename="StartingPosition")]
    pub starting_position: String,
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
    #[serde(rename="Code")]
    pub code: self::function::Code,
    /// Property `DeadLetterConfig`.
    #[serde(rename="DeadLetterConfig")]
    pub dead_letter_config: self::function::DeadLetterConfig,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Environment`.
    #[serde(rename="Environment")]
    pub environment: self::function::Environment,
    /// Property `FunctionName`.
    #[serde(rename="FunctionName")]
    pub function_name: String,
    /// Property `Handler`.
    #[serde(rename="Handler")]
    pub handler: String,
    /// Property `KmsKeyArn`.
    #[serde(rename="KmsKeyArn")]
    pub kms_key_arn: String,
    /// Property `MemorySize`.
    #[serde(rename="MemorySize")]
    pub memory_size: u32,
    /// Property `ReservedConcurrentExecutions`.
    #[serde(rename="ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: u32,
    /// Property `Role`.
    #[serde(rename="Role")]
    pub role: String,
    /// Property `Runtime`.
    #[serde(rename="Runtime")]
    pub runtime: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `Timeout`.
    #[serde(rename="Timeout")]
    pub timeout: u32,
    /// Property `TracingConfig`.
    #[serde(rename="TracingConfig")]
    pub tracing_config: self::function::TracingConfig,
    /// Property `VpcConfig`.
    #[serde(rename="VpcConfig")]
    pub vpc_config: self::function::VpcConfig,
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
    #[serde(rename="Action")]
    pub action: String,
    /// Property `EventSourceToken`.
    #[serde(rename="EventSourceToken")]
    pub event_source_token: String,
    /// Property `FunctionName`.
    #[serde(rename="FunctionName")]
    pub function_name: String,
    /// Property `Principal`.
    #[serde(rename="Principal")]
    pub principal: String,
    /// Property `SourceAccount`.
    #[serde(rename="SourceAccount")]
    pub source_account: String,
    /// Property `SourceArn`.
    #[serde(rename="SourceArn")]
    pub source_arn: String,
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
    #[serde(rename="CodeSha256")]
    pub code_sha256: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `FunctionName`.
    #[serde(rename="FunctionName")]
    pub function_name: String,
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
        #[serde(rename="AdditionalVersionWeights")]
        pub additional_version_weights: Vec<VersionWeight>,
    }

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VersionWeight {
        /// Property `FunctionVersion`.
        #[serde(rename="FunctionVersion")]
        pub function_version: String,
        /// Property `FunctionWeight`.
        #[serde(rename="FunctionWeight")]
        pub function_weight: f64,
    }
}

pub mod function {
    //! Property types for the `Function` resource.

    /// The [`AWS::Lambda::Function.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Code {
        /// Property `S3Bucket`.
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        /// Property `S3Key`.
        #[serde(rename="S3Key")]
        pub s3_key: String,
        /// Property `S3ObjectVersion`.
        #[serde(rename="S3ObjectVersion")]
        pub s3_object_version: String,
        /// Property `ZipFile`.
        #[serde(rename="ZipFile")]
        pub zip_file: String,
    }

    /// The [`AWS::Lambda::Function.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeadLetterConfig {
        /// Property `TargetArn`.
        #[serde(rename="TargetArn")]
        pub target_arn: String,
    }

    /// The [`AWS::Lambda::Function.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `Variables`.
        #[serde(rename="Variables")]
        pub variables: ::std::collections::HashMap<String, String>,
    }

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TracingConfig {
        /// Property `Mode`.
        #[serde(rename="Mode")]
        pub mode: String,
    }

    /// The [`AWS::Lambda::Function.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcConfig {
        /// Property `SecurityGroupIds`.
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        /// Property `SubnetIds`.
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
    }
}
