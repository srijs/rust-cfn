/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource type.
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="FunctionName")]
    pub function_name: String,
    #[serde(rename="FunctionVersion")]
    pub function_version: String,
    #[serde(rename="Name")]
    pub name: String,
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

impl From<AliasProperties> for Alias {
    fn from(properties: AliasProperties) -> Alias {
        Alias { properties }
    }
}

/// The [`AWS::Lambda::EventSourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html) resource type.
pub struct EventSourceMapping {
    properties: EventSourceMappingProperties
}

/// Properties for the `EventSourceMapping` resource.
#[derive(Serialize, Deserialize)]
pub struct EventSourceMappingProperties {
    #[serde(rename="BatchSize")]
    pub batch_size: u32,
    #[serde(rename="Enabled")]
    pub enabled: bool,
    #[serde(rename="EventSourceArn")]
    pub event_source_arn: String,
    #[serde(rename="FunctionName")]
    pub function_name: String,
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

impl From<EventSourceMappingProperties> for EventSourceMapping {
    fn from(properties: EventSourceMappingProperties) -> EventSourceMapping {
        EventSourceMapping { properties }
    }
}

/// The [`AWS::Lambda::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html) resource type.
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Serialize, Deserialize)]
pub struct FunctionProperties {
    #[serde(rename="Code")]
    pub code: self::function::Code,
    #[serde(rename="DeadLetterConfig")]
    pub dead_letter_config: self::function::DeadLetterConfig,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Environment")]
    pub environment: self::function::Environment,
    #[serde(rename="FunctionName")]
    pub function_name: String,
    #[serde(rename="Handler")]
    pub handler: String,
    #[serde(rename="KmsKeyArn")]
    pub kms_key_arn: String,
    #[serde(rename="MemorySize")]
    pub memory_size: u32,
    #[serde(rename="ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: u32,
    #[serde(rename="Role")]
    pub role: String,
    #[serde(rename="Runtime")]
    pub runtime: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="Timeout")]
    pub timeout: u32,
    #[serde(rename="TracingConfig")]
    pub tracing_config: self::function::TracingConfig,
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

impl From<FunctionProperties> for Function {
    fn from(properties: FunctionProperties) -> Function {
        Function { properties }
    }
}

/// The [`AWS::Lambda::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html) resource type.
pub struct Permission {
    properties: PermissionProperties
}

/// Properties for the `Permission` resource.
#[derive(Serialize, Deserialize)]
pub struct PermissionProperties {
    #[serde(rename="Action")]
    pub action: String,
    #[serde(rename="EventSourceToken")]
    pub event_source_token: String,
    #[serde(rename="FunctionName")]
    pub function_name: String,
    #[serde(rename="Principal")]
    pub principal: String,
    #[serde(rename="SourceAccount")]
    pub source_account: String,
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

impl From<PermissionProperties> for Permission {
    fn from(properties: PermissionProperties) -> Permission {
        Permission { properties }
    }
}

/// The [`AWS::Lambda::Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html) resource type.
pub struct Version {
    properties: VersionProperties
}

/// Properties for the `Version` resource.
#[derive(Serialize, Deserialize)]
pub struct VersionProperties {
    #[serde(rename="CodeSha256")]
    pub code_sha256: String,
    #[serde(rename="Description")]
    pub description: String,
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

impl From<VersionProperties> for Version {
    fn from(properties: VersionProperties) -> Version {
        Version { properties }
    }
}

pub mod alias {
    /// The [`AWS::Lambda::Alias.AliasRoutingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-aliasroutingconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AliasRoutingConfiguration {
        #[serde(rename="AdditionalVersionWeights")]
        pub additional_version_weights: Vec<VersionWeight>,
    }

    /// The [`AWS::Lambda::Alias.VersionWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-alias-versionweight.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct VersionWeight {
        #[serde(rename="FunctionVersion")]
        pub function_version: String,
        #[serde(rename="FunctionWeight")]
        pub function_weight: f64,
    }

}

pub mod function {
    /// The [`AWS::Lambda::Function.Code`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-code.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Code {
        #[serde(rename="S3Bucket")]
        pub s3_bucket: String,
        #[serde(rename="S3Key")]
        pub s3_key: String,
        #[serde(rename="S3ObjectVersion")]
        pub s3_object_version: String,
        #[serde(rename="ZipFile")]
        pub zip_file: String,
    }

    /// The [`AWS::Lambda::Function.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-deadletterconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DeadLetterConfig {
        #[serde(rename="TargetArn")]
        pub target_arn: String,
    }

    /// The [`AWS::Lambda::Function.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-environment.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Environment {
        #[serde(rename="Variables")]
        pub variables: ::std::collections::HashMap<String, String>,
    }

    /// The [`AWS::Lambda::Function.TracingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-tracingconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TracingConfig {
        #[serde(rename="Mode")]
        pub mode: String,
    }

    /// The [`AWS::Lambda::Function.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lambda-function-vpcconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct VpcConfig {
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        #[serde(rename="SubnetIds")]
        pub subnet_ids: Vec<String>,
    }

}

