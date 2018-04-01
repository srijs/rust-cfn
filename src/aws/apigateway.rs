//! Types for the `ApiGateway` service.

/// The [`AWS::ApiGateway::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html) resource type.
#[derive(Debug)]
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProperties {
    /// Property `CloudWatchRoleArn`.
    #[serde(rename = "CloudWatchRoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_watch_role_arn: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Account {
    type Properties = AccountProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Account";
    fn properties(&self) -> &AccountProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccountProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Account {}

impl From<AccountProperties> for Account {
    fn from(properties: AccountProperties) -> Account {
        Account { properties }
    }
}

/// The [`AWS::ApiGateway::ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html) resource type.
#[derive(Debug)]
pub struct ApiKey {
    properties: ApiKeyProperties
}

/// Properties for the `ApiKey` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyProperties {
    /// Property `CustomerId`.
    #[serde(rename = "CustomerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Enabled`.
    #[serde(rename = "Enabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<::Value<bool>>,
    /// Property `GenerateDistinctId`.
    #[serde(rename = "GenerateDistinctId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_distinct_id: Option<::Value<bool>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `StageKeys`.
    #[serde(rename = "StageKeys")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<::ValueList<self::api_key::StageKey>>,
}

impl<'a> ::Resource<'a> for ApiKey {
    type Properties = ApiKeyProperties;
    const TYPE: &'static str = "AWS::ApiGateway::ApiKey";
    fn properties(&self) -> &ApiKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiKey {}

impl From<ApiKeyProperties> for ApiKey {
    fn from(properties: ApiKeyProperties) -> ApiKey {
        ApiKey { properties }
    }
}

/// The [`AWS::ApiGateway::Authorizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html) resource type.
#[derive(Debug)]
pub struct Authorizer {
    properties: AuthorizerProperties
}

/// Properties for the `Authorizer` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizerProperties {
    /// Property `AuthType`.
    #[serde(rename = "AuthType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<::Value<String>>,
    /// Property `AuthorizerCredentials`.
    #[serde(rename = "AuthorizerCredentials")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<::Value<String>>,
    /// Property `AuthorizerResultTtlInSeconds`.
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<::Value<u32>>,
    /// Property `AuthorizerUri`.
    #[serde(rename = "AuthorizerUri")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<::Value<String>>,
    /// Property `IdentitySource`.
    #[serde(rename = "IdentitySource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<::Value<String>>,
    /// Property `IdentityValidationExpression`.
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `ProviderARNs`.
    #[serde(rename = "ProviderARNs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_ar_ns: Option<::ValueList<String>>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Authorizer {
    type Properties = AuthorizerProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Authorizer";
    fn properties(&self) -> &AuthorizerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AuthorizerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Authorizer {}

impl From<AuthorizerProperties> for Authorizer {
    fn from(properties: AuthorizerProperties) -> Authorizer {
        Authorizer { properties }
    }
}

/// The [`AWS::ApiGateway::BasePathMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html) resource type.
#[derive(Debug)]
pub struct BasePathMapping {
    properties: BasePathMappingProperties
}

/// Properties for the `BasePathMapping` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePathMappingProperties {
    /// Property `BasePath`.
    #[serde(rename = "BasePath")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_path: Option<::Value<String>>,
    /// Property `DomainName`.
    #[serde(rename = "DomainName")]
    pub domain_name: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<::Value<String>>,
    /// Property `Stage`.
    #[serde(rename = "Stage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for BasePathMapping {
    type Properties = BasePathMappingProperties;
    const TYPE: &'static str = "AWS::ApiGateway::BasePathMapping";
    fn properties(&self) -> &BasePathMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BasePathMappingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BasePathMapping {}

impl From<BasePathMappingProperties> for BasePathMapping {
    fn from(properties: BasePathMappingProperties) -> BasePathMapping {
        BasePathMapping { properties }
    }
}

/// The [`AWS::ApiGateway::ClientCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-clientcertificate.html) resource type.
#[derive(Debug)]
pub struct ClientCertificate {
    properties: ClientCertificateProperties
}

/// Properties for the `ClientCertificate` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCertificateProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for ClientCertificate {
    type Properties = ClientCertificateProperties;
    const TYPE: &'static str = "AWS::ApiGateway::ClientCertificate";
    fn properties(&self) -> &ClientCertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClientCertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClientCertificate {}

impl From<ClientCertificateProperties> for ClientCertificate {
    fn from(properties: ClientCertificateProperties) -> ClientCertificate {
        ClientCertificate { properties }
    }
}

/// The [`AWS::ApiGateway::Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html) resource type.
#[derive(Debug)]
pub struct Deployment {
    properties: DeploymentProperties
}

/// Properties for the `Deployment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `StageDescription`.
    #[serde(rename = "StageDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_description: Option<::Value<self::deployment::StageDescription>>,
    /// Property `StageName`.
    #[serde(rename = "StageName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Deployment {
    type Properties = DeploymentProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Deployment";
    fn properties(&self) -> &DeploymentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Deployment {}

impl From<DeploymentProperties> for Deployment {
    fn from(properties: DeploymentProperties) -> Deployment {
        Deployment { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html) resource type.
#[derive(Debug)]
pub struct DocumentationPart {
    properties: DocumentationPartProperties
}

/// Properties for the `DocumentationPart` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationPartProperties {
    /// Property `Location`.
    #[serde(rename = "Location")]
    pub location: ::Value<self::documentation_part::Location>,
    /// Property `Properties`.
    #[serde(rename = "Properties")]
    pub properties: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for DocumentationPart {
    type Properties = DocumentationPartProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DocumentationPart";
    fn properties(&self) -> &DocumentationPartProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentationPartProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DocumentationPart {}

impl From<DocumentationPartProperties> for DocumentationPart {
    fn from(properties: DocumentationPartProperties) -> DocumentationPart {
        DocumentationPart { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html) resource type.
#[derive(Debug)]
pub struct DocumentationVersion {
    properties: DocumentationVersionProperties
}

/// Properties for the `DocumentationVersion` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationVersionProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `DocumentationVersion`.
    #[serde(rename = "DocumentationVersion")]
    pub documentation_version: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for DocumentationVersion {
    type Properties = DocumentationVersionProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DocumentationVersion";
    fn properties(&self) -> &DocumentationVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentationVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DocumentationVersion {}

impl From<DocumentationVersionProperties> for DocumentationVersion {
    fn from(properties: DocumentationVersionProperties) -> DocumentationVersion {
        DocumentationVersion { properties }
    }
}

/// The [`AWS::ApiGateway::DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html) resource type.
#[derive(Debug)]
pub struct DomainName {
    properties: DomainNameProperties
}

/// Properties for the `DomainName` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainNameProperties {
    /// Property `CertificateArn`.
    #[serde(rename = "CertificateArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<::Value<String>>,
    /// Property `DomainName`.
    #[serde(rename = "DomainName")]
    pub domain_name: ::Value<String>,
    /// Property `EndpointConfiguration`.
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<::Value<self::domain_name::EndpointConfiguration>>,
    /// Property `RegionalCertificateArn`.
    #[serde(rename = "RegionalCertificateArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for DomainName {
    type Properties = DomainNameProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DomainName";
    fn properties(&self) -> &DomainNameProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainNameProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DomainName {}

impl From<DomainNameProperties> for DomainName {
    fn from(properties: DomainNameProperties) -> DomainName {
        DomainName { properties }
    }
}

/// The [`AWS::ApiGateway::GatewayResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html) resource type.
#[derive(Debug)]
pub struct GatewayResponse {
    properties: GatewayResponseProperties
}

/// Properties for the `GatewayResponse` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayResponseProperties {
    /// Property `ResponseParameters`.
    #[serde(rename = "ResponseParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
    /// Property `ResponseTemplates`.
    #[serde(rename = "ResponseTemplates")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, ::Value<String>>>,
    /// Property `ResponseType`.
    #[serde(rename = "ResponseType")]
    pub response_type: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `StatusCode`.
    #[serde(rename = "StatusCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for GatewayResponse {
    type Properties = GatewayResponseProperties;
    const TYPE: &'static str = "AWS::ApiGateway::GatewayResponse";
    fn properties(&self) -> &GatewayResponseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GatewayResponseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GatewayResponse {}

impl From<GatewayResponseProperties> for GatewayResponse {
    fn from(properties: GatewayResponseProperties) -> GatewayResponse {
        GatewayResponse { properties }
    }
}

/// The [`AWS::ApiGateway::Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html) resource type.
#[derive(Debug)]
pub struct Method {
    properties: MethodProperties
}

/// Properties for the `Method` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MethodProperties {
    /// Property `ApiKeyRequired`.
    #[serde(rename = "ApiKeyRequired")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<::Value<bool>>,
    /// Property `AuthorizationType`.
    #[serde(rename = "AuthorizationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<::Value<String>>,
    /// Property `AuthorizerId`.
    #[serde(rename = "AuthorizerId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<::Value<String>>,
    /// Property `HttpMethod`.
    #[serde(rename = "HttpMethod")]
    pub http_method: ::Value<String>,
    /// Property `Integration`.
    #[serde(rename = "Integration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration: Option<::Value<self::method::Integration>>,
    /// Property `MethodResponses`.
    #[serde(rename = "MethodResponses")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_responses: Option<::ValueList<self::method::MethodResponse>>,
    /// Property `OperationName`.
    #[serde(rename = "OperationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<::Value<String>>,
    /// Property `RequestModels`.
    #[serde(rename = "RequestModels")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, ::Value<String>>>,
    /// Property `RequestParameters`.
    #[serde(rename = "RequestParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, ::Value<bool>>>,
    /// Property `RequestValidatorId`.
    #[serde(rename = "RequestValidatorId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<::Value<String>>,
    /// Property `ResourceId`.
    #[serde(rename = "ResourceId")]
    pub resource_id: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for Method {
    type Properties = MethodProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Method";
    fn properties(&self) -> &MethodProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MethodProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Method {}

impl From<MethodProperties> for Method {
    fn from(properties: MethodProperties) -> Method {
        Method { properties }
    }
}

/// The [`AWS::ApiGateway::Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html) resource type.
#[derive(Debug)]
pub struct Model {
    properties: ModelProperties
}

/// Properties for the `Model` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelProperties {
    /// Property `ContentType`.
    #[serde(rename = "ContentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `Schema`.
    #[serde(rename = "Schema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<::Value<::json::Value>>,
}

impl<'a> ::Resource<'a> for Model {
    type Properties = ModelProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Model";
    fn properties(&self) -> &ModelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Model {}

impl From<ModelProperties> for Model {
    fn from(properties: ModelProperties) -> Model {
        Model { properties }
    }
}

/// The [`AWS::ApiGateway::RequestValidator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html) resource type.
#[derive(Debug)]
pub struct RequestValidator {
    properties: RequestValidatorProperties
}

/// Properties for the `RequestValidator` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestValidatorProperties {
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `ValidateRequestBody`.
    #[serde(rename = "ValidateRequestBody")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<::Value<bool>>,
    /// Property `ValidateRequestParameters`.
    #[serde(rename = "ValidateRequestParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<::Value<bool>>,
}

impl<'a> ::Resource<'a> for RequestValidator {
    type Properties = RequestValidatorProperties;
    const TYPE: &'static str = "AWS::ApiGateway::RequestValidator";
    fn properties(&self) -> &RequestValidatorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RequestValidatorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RequestValidator {}

impl From<RequestValidatorProperties> for RequestValidator {
    fn from(properties: RequestValidatorProperties) -> RequestValidator {
        RequestValidator { properties }
    }
}

/// The [`AWS::ApiGateway::Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html) resource type.
#[derive(Debug)]
pub struct Resource {
    properties: ResourceProperties
}

/// Properties for the `Resource` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceProperties {
    /// Property `ParentId`.
    #[serde(rename = "ParentId")]
    pub parent_id: ::Value<String>,
    /// Property `PathPart`.
    #[serde(rename = "PathPart")]
    pub path_part: ::Value<String>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for Resource {
    type Properties = ResourceProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Resource";
    fn properties(&self) -> &ResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Resource {}

impl From<ResourceProperties> for Resource {
    fn from(properties: ResourceProperties) -> Resource {
        Resource { properties }
    }
}

/// The [`AWS::ApiGateway::RestApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html) resource type.
#[derive(Debug)]
pub struct RestApi {
    properties: RestApiProperties
}

/// Properties for the `RestApi` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestApiProperties {
    /// Property `ApiKeySourceType`.
    #[serde(rename = "ApiKeySourceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key_source_type: Option<::Value<String>>,
    /// Property `BinaryMediaTypes`.
    #[serde(rename = "BinaryMediaTypes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<::ValueList<String>>,
    /// Property `Body`.
    #[serde(rename = "Body")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<::Value<::json::Value>>,
    /// Property `BodyS3Location`.
    #[serde(rename = "BodyS3Location")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_s3_location: Option<::Value<self::rest_api::S3Location>>,
    /// Property `CloneFrom`.
    #[serde(rename = "CloneFrom")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_from: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `EndpointConfiguration`.
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<::Value<self::rest_api::EndpointConfiguration>>,
    /// Property `FailOnWarnings`.
    #[serde(rename = "FailOnWarnings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<::Value<bool>>,
    /// Property `MinimumCompressionSize`.
    #[serde(rename = "MinimumCompressionSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<::Value<u32>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
}

impl<'a> ::Resource<'a> for RestApi {
    type Properties = RestApiProperties;
    const TYPE: &'static str = "AWS::ApiGateway::RestApi";
    fn properties(&self) -> &RestApiProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RestApiProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RestApi {}

impl From<RestApiProperties> for RestApi {
    fn from(properties: RestApiProperties) -> RestApi {
        RestApi { properties }
    }
}

/// The [`AWS::ApiGateway::Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html) resource type.
#[derive(Debug)]
pub struct Stage {
    properties: StageProperties
}

/// Properties for the `Stage` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct StageProperties {
    /// Property `CacheClusterEnabled`.
    #[serde(rename = "CacheClusterEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<::Value<bool>>,
    /// Property `CacheClusterSize`.
    #[serde(rename = "CacheClusterSize")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<::Value<String>>,
    /// Property `ClientCertificateId`.
    #[serde(rename = "ClientCertificateId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<::Value<String>>,
    /// Property `DeploymentId`.
    #[serde(rename = "DeploymentId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `DocumentationVersion`.
    #[serde(rename = "DocumentationVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<::Value<String>>,
    /// Property `MethodSettings`.
    #[serde(rename = "MethodSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<::ValueList<self::stage::MethodSetting>>,
    /// Property `RestApiId`.
    #[serde(rename = "RestApiId")]
    pub rest_api_id: ::Value<String>,
    /// Property `StageName`.
    #[serde(rename = "StageName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<::Value<String>>,
    /// Property `Variables`.
    #[serde(rename = "Variables")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, ::Value<String>>>,
}

impl<'a> ::Resource<'a> for Stage {
    type Properties = StageProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Stage";
    fn properties(&self) -> &StageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stage {}

impl From<StageProperties> for Stage {
    fn from(properties: StageProperties) -> Stage {
        Stage { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html) resource type.
#[derive(Debug)]
pub struct UsagePlan {
    properties: UsagePlanProperties
}

/// Properties for the `UsagePlan` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UsagePlanProperties {
    /// Property `ApiStages`.
    #[serde(rename = "ApiStages")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<::ValueList<self::usage_plan::ApiStage>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Quota`.
    #[serde(rename = "Quota")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<::Value<self::usage_plan::QuotaSettings>>,
    /// Property `Throttle`.
    #[serde(rename = "Throttle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttle: Option<::Value<self::usage_plan::ThrottleSettings>>,
    /// Property `UsagePlanName`.
    #[serde(rename = "UsagePlanName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_plan_name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for UsagePlan {
    type Properties = UsagePlanProperties;
    const TYPE: &'static str = "AWS::ApiGateway::UsagePlan";
    fn properties(&self) -> &UsagePlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UsagePlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UsagePlan {}

impl From<UsagePlanProperties> for UsagePlan {
    fn from(properties: UsagePlanProperties) -> UsagePlan {
        UsagePlan { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlanKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html) resource type.
#[derive(Debug)]
pub struct UsagePlanKey {
    properties: UsagePlanKeyProperties
}

/// Properties for the `UsagePlanKey` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UsagePlanKeyProperties {
    /// Property `KeyId`.
    #[serde(rename = "KeyId")]
    pub key_id: ::Value<String>,
    /// Property `KeyType`.
    #[serde(rename = "KeyType")]
    pub key_type: ::Value<String>,
    /// Property `UsagePlanId`.
    #[serde(rename = "UsagePlanId")]
    pub usage_plan_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for UsagePlanKey {
    type Properties = UsagePlanKeyProperties;
    const TYPE: &'static str = "AWS::ApiGateway::UsagePlanKey";
    fn properties(&self) -> &UsagePlanKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UsagePlanKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UsagePlanKey {}

impl From<UsagePlanKeyProperties> for UsagePlanKey {
    fn from(properties: UsagePlanKeyProperties) -> UsagePlanKey {
        UsagePlanKey { properties }
    }
}

/// The [`AWS::ApiGateway::VpcLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html) resource type.
#[derive(Debug)]
pub struct VpcLink {
    properties: VpcLinkProperties
}

/// Properties for the `VpcLink` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VpcLinkProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `TargetArns`.
    #[serde(rename = "TargetArns")]
    pub target_arns: ::ValueList<String>,
}

impl<'a> ::Resource<'a> for VpcLink {
    type Properties = VpcLinkProperties;
    const TYPE: &'static str = "AWS::ApiGateway::VpcLink";
    fn properties(&self) -> &VpcLinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VpcLinkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VpcLink {}

impl From<VpcLinkProperties> for VpcLink {
    fn from(properties: VpcLinkProperties) -> VpcLink {
        VpcLink { properties }
    }
}

pub mod api_key {
    //! Property types for the `ApiKey` resource.

    /// The [`AWS::ApiGateway::ApiKey.StageKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-apikey-stagekey.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StageKey {
        /// Property `RestApiId`.
        #[serde(rename = "RestApiId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rest_api_id: Option<::Value<String>>,
        /// Property `StageName`.
        #[serde(rename = "StageName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stage_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(StageKey);
}

pub mod deployment {
    //! Property types for the `Deployment` resource.

    /// The [`AWS::ApiGateway::Deployment.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        #[serde(rename = "CacheDataEncrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename = "CacheTtlInSeconds")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        #[serde(rename = "CachingEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `DataTraceEnabled`.
        #[serde(rename = "DataTraceEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `HttpMethod`.
        #[serde(rename = "HttpMethod")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_method: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        #[serde(rename = "LoggingLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logging_level: Option<::Value<String>>,
        /// Property `MetricsEnabled`.
        #[serde(rename = "MetricsEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ResourcePath`.
        #[serde(rename = "ResourcePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_path: Option<::Value<String>>,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename = "ThrottlingBurstLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename = "ThrottlingRateLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(MethodSetting);

    /// The [`AWS::ApiGateway::Deployment.StageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StageDescription {
        /// Property `CacheClusterEnabled`.
        #[serde(rename = "CacheClusterEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_cluster_enabled: Option<::Value<bool>>,
        /// Property `CacheClusterSize`.
        #[serde(rename = "CacheClusterSize")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_cluster_size: Option<::Value<String>>,
        /// Property `CacheDataEncrypted`.
        #[serde(rename = "CacheDataEncrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename = "CacheTtlInSeconds")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        #[serde(rename = "CachingEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `ClientCertificateId`.
        #[serde(rename = "ClientCertificateId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client_certificate_id: Option<::Value<String>>,
        /// Property `DataTraceEnabled`.
        #[serde(rename = "DataTraceEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `DocumentationVersion`.
        #[serde(rename = "DocumentationVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub documentation_version: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        #[serde(rename = "LoggingLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logging_level: Option<::Value<String>>,
        /// Property `MethodSettings`.
        #[serde(rename = "MethodSettings")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub method_settings: Option<::ValueList<MethodSetting>>,
        /// Property `MetricsEnabled`.
        #[serde(rename = "MetricsEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename = "ThrottlingBurstLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename = "ThrottlingRateLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_rate_limit: Option<::Value<f64>>,
        /// Property `Variables`.
        #[serde(rename = "Variables")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variables: Option<::std::collections::HashMap<String, ::Value<String>>>,
    }

    cfn_internal__inherit_codec_impls!(StageDescription);
}

pub mod documentation_part {
    //! Property types for the `DocumentationPart` resource.

    /// The [`AWS::ApiGateway::DocumentationPart.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Location {
        /// Property `Method`.
        #[serde(rename = "Method")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub method: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Path`.
        #[serde(rename = "Path")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<::Value<String>>,
        /// Property `StatusCode`.
        #[serde(rename = "StatusCode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status_code: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Location);
}

pub mod domain_name {
    //! Property types for the `DomainName` resource.

    /// The [`AWS::ApiGateway::DomainName.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        #[serde(rename = "Types")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub types: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(EndpointConfiguration);
}

pub mod method {
    //! Property types for the `Method` resource.

    /// The [`AWS::ApiGateway::Method.Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Integration {
        /// Property `CacheKeyParameters`.
        #[serde(rename = "CacheKeyParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_key_parameters: Option<::ValueList<String>>,
        /// Property `CacheNamespace`.
        #[serde(rename = "CacheNamespace")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_namespace: Option<::Value<String>>,
        /// Property `ContentHandling`.
        #[serde(rename = "ContentHandling")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content_handling: Option<::Value<String>>,
        /// Property `Credentials`.
        #[serde(rename = "Credentials")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub credentials: Option<::Value<String>>,
        /// Property `IntegrationHttpMethod`.
        #[serde(rename = "IntegrationHttpMethod")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub integration_http_method: Option<::Value<String>>,
        /// Property `IntegrationResponses`.
        #[serde(rename = "IntegrationResponses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub integration_responses: Option<::ValueList<IntegrationResponse>>,
        /// Property `PassthroughBehavior`.
        #[serde(rename = "PassthroughBehavior")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub passthrough_behavior: Option<::Value<String>>,
        /// Property `RequestParameters`.
        #[serde(rename = "RequestParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub request_parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `RequestTemplates`.
        #[serde(rename = "RequestTemplates")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub request_templates: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Uri`.
        #[serde(rename = "Uri")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uri: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Integration);

    /// The [`AWS::ApiGateway::Method.IntegrationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IntegrationResponse {
        /// Property `ContentHandling`.
        #[serde(rename = "ContentHandling")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content_handling: Option<::Value<String>>,
        /// Property `ResponseParameters`.
        #[serde(rename = "ResponseParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `ResponseTemplates`.
        #[serde(rename = "ResponseTemplates")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_templates: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `SelectionPattern`.
        #[serde(rename = "SelectionPattern")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub selection_pattern: Option<::Value<String>>,
        /// Property `StatusCode`.
        #[serde(rename = "StatusCode")]
        pub status_code: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(IntegrationResponse);

    /// The [`AWS::ApiGateway::Method.MethodResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodResponse {
        /// Property `ResponseModels`.
        #[serde(rename = "ResponseModels")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_models: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `ResponseParameters`.
        #[serde(rename = "ResponseParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_parameters: Option<::std::collections::HashMap<String, ::Value<bool>>>,
        /// Property `StatusCode`.
        #[serde(rename = "StatusCode")]
        pub status_code: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MethodResponse);
}

pub mod rest_api {
    //! Property types for the `RestApi` resource.

    /// The [`AWS::ApiGateway::RestApi.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        #[serde(rename = "Types")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub types: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(EndpointConfiguration);

    /// The [`AWS::ApiGateway::RestApi.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Location {
        /// Property `Bucket`.
        #[serde(rename = "Bucket")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket: Option<::Value<String>>,
        /// Property `ETag`.
        #[serde(rename = "ETag")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub e_tag: Option<::Value<String>>,
        /// Property `Key`.
        #[serde(rename = "Key")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<::Value<String>>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(S3Location);
}

pub mod stage {
    //! Property types for the `Stage` resource.

    /// The [`AWS::ApiGateway::Stage.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        #[serde(rename = "CacheDataEncrypted")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename = "CacheTtlInSeconds")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        #[serde(rename = "CachingEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `DataTraceEnabled`.
        #[serde(rename = "DataTraceEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `HttpMethod`.
        #[serde(rename = "HttpMethod")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_method: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        #[serde(rename = "LoggingLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logging_level: Option<::Value<String>>,
        /// Property `MetricsEnabled`.
        #[serde(rename = "MetricsEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ResourcePath`.
        #[serde(rename = "ResourcePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_path: Option<::Value<String>>,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename = "ThrottlingBurstLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename = "ThrottlingRateLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(MethodSetting);
}

pub mod usage_plan {
    //! Property types for the `UsagePlan` resource.

    /// The [`AWS::ApiGateway::UsagePlan.ApiStage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiStage {
        /// Property `ApiId`.
        #[serde(rename = "ApiId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub api_id: Option<::Value<String>>,
        /// Property `Stage`.
        #[serde(rename = "Stage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stage: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ApiStage);

    /// The [`AWS::ApiGateway::UsagePlan.QuotaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct QuotaSettings {
        /// Property `Limit`.
        #[serde(rename = "Limit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: Option<::Value<u32>>,
        /// Property `Offset`.
        #[serde(rename = "Offset")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: Option<::Value<u32>>,
        /// Property `Period`.
        #[serde(rename = "Period")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub period: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(QuotaSettings);

    /// The [`AWS::ApiGateway::UsagePlan.ThrottleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ThrottleSettings {
        /// Property `BurstLimit`.
        #[serde(rename = "BurstLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub burst_limit: Option<::Value<u32>>,
        /// Property `RateLimit`.
        #[serde(rename = "RateLimit")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rate_limit: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(ThrottleSettings);
}
