/// The [`AWS::Lambda::Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-alias.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Alias {
    properties: AliasProperties
}

/// Properties for the `Alias` resource.
#[derive(Serialize, Deserialize)]
pub struct AliasProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="FunctionName")]
    pub function_name: (),
    #[serde(rename="FunctionVersion")]
    pub function_version: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RoutingConfig")]
    pub routing_config: (),
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

/// The [`AWS::Lambda::EventSourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-eventsourcemapping.html) resource.
#[derive(Serialize, Deserialize)]
pub struct EventSourceMapping {
    properties: EventSourceMappingProperties
}

/// Properties for the `EventSourceMapping` resource.
#[derive(Serialize, Deserialize)]
pub struct EventSourceMappingProperties {
    #[serde(rename="BatchSize")]
    pub batch_size: (),
    #[serde(rename="Enabled")]
    pub enabled: (),
    #[serde(rename="EventSourceArn")]
    pub event_source_arn: (),
    #[serde(rename="FunctionName")]
    pub function_name: (),
    #[serde(rename="StartingPosition")]
    pub starting_position: (),
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

/// The [`AWS::Lambda::Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-function.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Function {
    properties: FunctionProperties
}

/// Properties for the `Function` resource.
#[derive(Serialize, Deserialize)]
pub struct FunctionProperties {
    #[serde(rename="Code")]
    pub code: (),
    #[serde(rename="DeadLetterConfig")]
    pub dead_letter_config: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Environment")]
    pub environment: (),
    #[serde(rename="FunctionName")]
    pub function_name: (),
    #[serde(rename="Handler")]
    pub handler: (),
    #[serde(rename="KmsKeyArn")]
    pub kms_key_arn: (),
    #[serde(rename="MemorySize")]
    pub memory_size: (),
    #[serde(rename="ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: (),
    #[serde(rename="Role")]
    pub role: (),
    #[serde(rename="Runtime")]
    pub runtime: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="Timeout")]
    pub timeout: (),
    #[serde(rename="TracingConfig")]
    pub tracing_config: (),
    #[serde(rename="VpcConfig")]
    pub vpc_config: (),
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

/// The [`AWS::Lambda::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-permission.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Permission {
    properties: PermissionProperties
}

/// Properties for the `Permission` resource.
#[derive(Serialize, Deserialize)]
pub struct PermissionProperties {
    #[serde(rename="Action")]
    pub action: (),
    #[serde(rename="EventSourceToken")]
    pub event_source_token: (),
    #[serde(rename="FunctionName")]
    pub function_name: (),
    #[serde(rename="Principal")]
    pub principal: (),
    #[serde(rename="SourceAccount")]
    pub source_account: (),
    #[serde(rename="SourceArn")]
    pub source_arn: (),
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

/// The [`AWS::Lambda::Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lambda-version.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Version {
    properties: VersionProperties
}

/// Properties for the `Version` resource.
#[derive(Serialize, Deserialize)]
pub struct VersionProperties {
    #[serde(rename="CodeSha256")]
    pub code_sha256: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="FunctionName")]
    pub function_name: (),
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

