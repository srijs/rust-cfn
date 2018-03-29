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
    #[serde(rename="CloudWatchRoleArn")]
    pub cloud_watch_role_arn: String,
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
    #[serde(rename="CustomerId")]
    pub customer_id: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Enabled`.
    #[serde(rename="Enabled")]
    pub enabled: bool,
    /// Property `GenerateDistinctId`.
    #[serde(rename="GenerateDistinctId")]
    pub generate_distinct_id: bool,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `StageKeys`.
    #[serde(rename="StageKeys")]
    pub stage_keys: Vec<self::api_key::StageKey>,
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
    #[serde(rename="AuthType")]
    pub auth_type: String,
    /// Property `AuthorizerCredentials`.
    #[serde(rename="AuthorizerCredentials")]
    pub authorizer_credentials: String,
    /// Property `AuthorizerResultTtlInSeconds`.
    #[serde(rename="AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: u32,
    /// Property `AuthorizerUri`.
    #[serde(rename="AuthorizerUri")]
    pub authorizer_uri: String,
    /// Property `IdentitySource`.
    #[serde(rename="IdentitySource")]
    pub identity_source: String,
    /// Property `IdentityValidationExpression`.
    #[serde(rename="IdentityValidationExpression")]
    pub identity_validation_expression: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `ProviderARNs`.
    #[serde(rename="ProviderARNs")]
    pub provider_ar_ns: Vec<String>,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
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
    #[serde(rename="BasePath")]
    pub base_path: String,
    /// Property `DomainName`.
    #[serde(rename="DomainName")]
    pub domain_name: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `Stage`.
    #[serde(rename="Stage")]
    pub stage: String,
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
    #[serde(rename="Description")]
    pub description: String,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `StageDescription`.
    #[serde(rename="StageDescription")]
    pub stage_description: self::deployment::StageDescription,
    /// Property `StageName`.
    #[serde(rename="StageName")]
    pub stage_name: String,
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
    #[serde(rename="Location")]
    pub location: self::documentation_part::Location,
    /// Property `Properties`.
    #[serde(rename="Properties")]
    pub properties: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `DocumentationVersion`.
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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
    #[serde(rename="CertificateArn")]
    pub certificate_arn: String,
    /// Property `DomainName`.
    #[serde(rename="DomainName")]
    pub domain_name: String,
    /// Property `EndpointConfiguration`.
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: self::domain_name::EndpointConfiguration,
    /// Property `RegionalCertificateArn`.
    #[serde(rename="RegionalCertificateArn")]
    pub regional_certificate_arn: String,
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
    #[serde(rename="ResponseParameters")]
    pub response_parameters: ::std::collections::HashMap<String, String>,
    /// Property `ResponseTemplates`.
    #[serde(rename="ResponseTemplates")]
    pub response_templates: ::std::collections::HashMap<String, String>,
    /// Property `ResponseType`.
    #[serde(rename="ResponseType")]
    pub response_type: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `StatusCode`.
    #[serde(rename="StatusCode")]
    pub status_code: String,
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
    #[serde(rename="ApiKeyRequired")]
    pub api_key_required: bool,
    /// Property `AuthorizationType`.
    #[serde(rename="AuthorizationType")]
    pub authorization_type: String,
    /// Property `AuthorizerId`.
    #[serde(rename="AuthorizerId")]
    pub authorizer_id: String,
    /// Property `HttpMethod`.
    #[serde(rename="HttpMethod")]
    pub http_method: String,
    /// Property `Integration`.
    #[serde(rename="Integration")]
    pub integration: self::method::Integration,
    /// Property `MethodResponses`.
    #[serde(rename="MethodResponses")]
    pub method_responses: Vec<self::method::MethodResponse>,
    /// Property `OperationName`.
    #[serde(rename="OperationName")]
    pub operation_name: String,
    /// Property `RequestModels`.
    #[serde(rename="RequestModels")]
    pub request_models: ::std::collections::HashMap<String, String>,
    /// Property `RequestParameters`.
    #[serde(rename="RequestParameters")]
    pub request_parameters: ::std::collections::HashMap<String, bool>,
    /// Property `RequestValidatorId`.
    #[serde(rename="RequestValidatorId")]
    pub request_validator_id: String,
    /// Property `ResourceId`.
    #[serde(rename="ResourceId")]
    pub resource_id: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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
    #[serde(rename="ContentType")]
    pub content_type: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `Schema`.
    #[serde(rename="Schema")]
    pub schema: ::json::Value,
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
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `ValidateRequestBody`.
    #[serde(rename="ValidateRequestBody")]
    pub validate_request_body: bool,
    /// Property `ValidateRequestParameters`.
    #[serde(rename="ValidateRequestParameters")]
    pub validate_request_parameters: bool,
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
    #[serde(rename="ParentId")]
    pub parent_id: String,
    /// Property `PathPart`.
    #[serde(rename="PathPart")]
    pub path_part: String,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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
    #[serde(rename="ApiKeySourceType")]
    pub api_key_source_type: String,
    /// Property `BinaryMediaTypes`.
    #[serde(rename="BinaryMediaTypes")]
    pub binary_media_types: Vec<String>,
    /// Property `Body`.
    #[serde(rename="Body")]
    pub body: ::json::Value,
    /// Property `BodyS3Location`.
    #[serde(rename="BodyS3Location")]
    pub body_s3_location: self::rest_api::S3Location,
    /// Property `CloneFrom`.
    #[serde(rename="CloneFrom")]
    pub clone_from: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `EndpointConfiguration`.
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: self::rest_api::EndpointConfiguration,
    /// Property `FailOnWarnings`.
    #[serde(rename="FailOnWarnings")]
    pub fail_on_warnings: bool,
    /// Property `MinimumCompressionSize`.
    #[serde(rename="MinimumCompressionSize")]
    pub minimum_compression_size: u32,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Parameters`.
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, String>,
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
    #[serde(rename="CacheClusterEnabled")]
    pub cache_cluster_enabled: bool,
    /// Property `CacheClusterSize`.
    #[serde(rename="CacheClusterSize")]
    pub cache_cluster_size: String,
    /// Property `ClientCertificateId`.
    #[serde(rename="ClientCertificateId")]
    pub client_certificate_id: String,
    /// Property `DeploymentId`.
    #[serde(rename="DeploymentId")]
    pub deployment_id: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `DocumentationVersion`.
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: String,
    /// Property `MethodSettings`.
    #[serde(rename="MethodSettings")]
    pub method_settings: Vec<self::stage::MethodSetting>,
    /// Property `RestApiId`.
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    /// Property `StageName`.
    #[serde(rename="StageName")]
    pub stage_name: String,
    /// Property `Variables`.
    #[serde(rename="Variables")]
    pub variables: ::std::collections::HashMap<String, String>,
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
    #[serde(rename="ApiStages")]
    pub api_stages: Vec<self::usage_plan::ApiStage>,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Quota`.
    #[serde(rename="Quota")]
    pub quota: self::usage_plan::QuotaSettings,
    /// Property `Throttle`.
    #[serde(rename="Throttle")]
    pub throttle: self::usage_plan::ThrottleSettings,
    /// Property `UsagePlanName`.
    #[serde(rename="UsagePlanName")]
    pub usage_plan_name: String,
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
    #[serde(rename="KeyId")]
    pub key_id: String,
    /// Property `KeyType`.
    #[serde(rename="KeyType")]
    pub key_type: String,
    /// Property `UsagePlanId`.
    #[serde(rename="UsagePlanId")]
    pub usage_plan_id: String,
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
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `TargetArns`.
    #[serde(rename="TargetArns")]
    pub target_arns: Vec<String>,
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
        #[serde(rename="RestApiId")]
        pub rest_api_id: String,
        /// Property `StageName`.
        #[serde(rename="StageName")]
        pub stage_name: String,
    }
}

pub mod deployment {
    //! Property types for the `Deployment` resource.

    /// The [`AWS::ApiGateway::Deployment.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        /// Property `CachingEnabled`.
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        /// Property `DataTraceEnabled`.
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        /// Property `HttpMethod`.
        #[serde(rename="HttpMethod")]
        pub http_method: String,
        /// Property `LoggingLevel`.
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        /// Property `MetricsEnabled`.
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        /// Property `ResourcePath`.
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
    }

    /// The [`AWS::ApiGateway::Deployment.StageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StageDescription {
        /// Property `CacheClusterEnabled`.
        #[serde(rename="CacheClusterEnabled")]
        pub cache_cluster_enabled: bool,
        /// Property `CacheClusterSize`.
        #[serde(rename="CacheClusterSize")]
        pub cache_cluster_size: String,
        /// Property `CacheDataEncrypted`.
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        /// Property `CachingEnabled`.
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        /// Property `ClientCertificateId`.
        #[serde(rename="ClientCertificateId")]
        pub client_certificate_id: String,
        /// Property `DataTraceEnabled`.
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `DocumentationVersion`.
        #[serde(rename="DocumentationVersion")]
        pub documentation_version: String,
        /// Property `LoggingLevel`.
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        /// Property `MethodSettings`.
        #[serde(rename="MethodSettings")]
        pub method_settings: Vec<MethodSetting>,
        /// Property `MetricsEnabled`.
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
        /// Property `Variables`.
        #[serde(rename="Variables")]
        pub variables: ::std::collections::HashMap<String, String>,
    }
}

pub mod documentation_part {
    //! Property types for the `DocumentationPart` resource.

    /// The [`AWS::ApiGateway::DocumentationPart.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Location {
        /// Property `Method`.
        #[serde(rename="Method")]
        pub method: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Path`.
        #[serde(rename="Path")]
        pub path: String,
        /// Property `StatusCode`.
        #[serde(rename="StatusCode")]
        pub status_code: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod domain_name {
    //! Property types for the `DomainName` resource.

    /// The [`AWS::ApiGateway::DomainName.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        #[serde(rename="Types")]
        pub types: Vec<String>,
    }
}

pub mod method {
    //! Property types for the `Method` resource.

    /// The [`AWS::ApiGateway::Method.Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Integration {
        /// Property `CacheKeyParameters`.
        #[serde(rename="CacheKeyParameters")]
        pub cache_key_parameters: Vec<String>,
        /// Property `CacheNamespace`.
        #[serde(rename="CacheNamespace")]
        pub cache_namespace: String,
        /// Property `ContentHandling`.
        #[serde(rename="ContentHandling")]
        pub content_handling: String,
        /// Property `Credentials`.
        #[serde(rename="Credentials")]
        pub credentials: String,
        /// Property `IntegrationHttpMethod`.
        #[serde(rename="IntegrationHttpMethod")]
        pub integration_http_method: String,
        /// Property `IntegrationResponses`.
        #[serde(rename="IntegrationResponses")]
        pub integration_responses: Vec<IntegrationResponse>,
        /// Property `PassthroughBehavior`.
        #[serde(rename="PassthroughBehavior")]
        pub passthrough_behavior: String,
        /// Property `RequestParameters`.
        #[serde(rename="RequestParameters")]
        pub request_parameters: ::std::collections::HashMap<String, String>,
        /// Property `RequestTemplates`.
        #[serde(rename="RequestTemplates")]
        pub request_templates: ::std::collections::HashMap<String, String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
        /// Property `Uri`.
        #[serde(rename="Uri")]
        pub uri: String,
    }

    /// The [`AWS::ApiGateway::Method.IntegrationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct IntegrationResponse {
        /// Property `ContentHandling`.
        #[serde(rename="ContentHandling")]
        pub content_handling: String,
        /// Property `ResponseParameters`.
        #[serde(rename="ResponseParameters")]
        pub response_parameters: ::std::collections::HashMap<String, String>,
        /// Property `ResponseTemplates`.
        #[serde(rename="ResponseTemplates")]
        pub response_templates: ::std::collections::HashMap<String, String>,
        /// Property `SelectionPattern`.
        #[serde(rename="SelectionPattern")]
        pub selection_pattern: String,
        /// Property `StatusCode`.
        #[serde(rename="StatusCode")]
        pub status_code: String,
    }

    /// The [`AWS::ApiGateway::Method.MethodResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodResponse {
        /// Property `ResponseModels`.
        #[serde(rename="ResponseModels")]
        pub response_models: ::std::collections::HashMap<String, String>,
        /// Property `ResponseParameters`.
        #[serde(rename="ResponseParameters")]
        pub response_parameters: ::std::collections::HashMap<String, bool>,
        /// Property `StatusCode`.
        #[serde(rename="StatusCode")]
        pub status_code: String,
    }
}

pub mod rest_api {
    //! Property types for the `RestApi` resource.

    /// The [`AWS::ApiGateway::RestApi.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        #[serde(rename="Types")]
        pub types: Vec<String>,
    }

    /// The [`AWS::ApiGateway::RestApi.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Location {
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `ETag`.
        #[serde(rename="ETag")]
        pub e_tag: String,
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Version`.
        #[serde(rename="Version")]
        pub version: String,
    }
}

pub mod stage {
    //! Property types for the `Stage` resource.

    /// The [`AWS::ApiGateway::Stage.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        /// Property `CacheTtlInSeconds`.
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        /// Property `CachingEnabled`.
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        /// Property `DataTraceEnabled`.
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        /// Property `HttpMethod`.
        #[serde(rename="HttpMethod")]
        pub http_method: String,
        /// Property `LoggingLevel`.
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        /// Property `MetricsEnabled`.
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        /// Property `ResourcePath`.
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        /// Property `ThrottlingBurstLimit`.
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        /// Property `ThrottlingRateLimit`.
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
    }
}

pub mod usage_plan {
    //! Property types for the `UsagePlan` resource.

    /// The [`AWS::ApiGateway::UsagePlan.ApiStage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ApiStage {
        /// Property `ApiId`.
        #[serde(rename="ApiId")]
        pub api_id: String,
        /// Property `Stage`.
        #[serde(rename="Stage")]
        pub stage: String,
    }

    /// The [`AWS::ApiGateway::UsagePlan.QuotaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct QuotaSettings {
        /// Property `Limit`.
        #[serde(rename="Limit")]
        pub limit: u32,
        /// Property `Offset`.
        #[serde(rename="Offset")]
        pub offset: u32,
        /// Property `Period`.
        #[serde(rename="Period")]
        pub period: String,
    }

    /// The [`AWS::ApiGateway::UsagePlan.ThrottleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ThrottleSettings {
        /// Property `BurstLimit`.
        #[serde(rename="BurstLimit")]
        pub burst_limit: u32,
        /// Property `RateLimit`.
        #[serde(rename="RateLimit")]
        pub rate_limit: f64,
    }
}
