/// The [`AWS::ApiGateway::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html) resource type.
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Serialize, Deserialize)]
pub struct AccountProperties {
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

impl From<AccountProperties> for Account {
    fn from(properties: AccountProperties) -> Account {
        Account { properties }
    }
}

/// The [`AWS::ApiGateway::ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html) resource type.
pub struct ApiKey {
    properties: ApiKeyProperties
}

/// Properties for the `ApiKey` resource.
#[derive(Serialize, Deserialize)]
pub struct ApiKeyProperties {
    #[serde(rename="CustomerId")]
    pub customer_id: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Enabled")]
    pub enabled: bool,
    #[serde(rename="GenerateDistinctId")]
    pub generate_distinct_id: bool,
    #[serde(rename="Name")]
    pub name: String,
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

impl From<ApiKeyProperties> for ApiKey {
    fn from(properties: ApiKeyProperties) -> ApiKey {
        ApiKey { properties }
    }
}

/// The [`AWS::ApiGateway::Authorizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html) resource type.
pub struct Authorizer {
    properties: AuthorizerProperties
}

/// Properties for the `Authorizer` resource.
#[derive(Serialize, Deserialize)]
pub struct AuthorizerProperties {
    #[serde(rename="AuthType")]
    pub auth_type: String,
    #[serde(rename="AuthorizerCredentials")]
    pub authorizer_credentials: String,
    #[serde(rename="AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: u32,
    #[serde(rename="AuthorizerUri")]
    pub authorizer_uri: String,
    #[serde(rename="IdentitySource")]
    pub identity_source: String,
    #[serde(rename="IdentityValidationExpression")]
    pub identity_validation_expression: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ProviderARNs")]
    pub provider_ar_ns: Vec<String>,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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

impl From<AuthorizerProperties> for Authorizer {
    fn from(properties: AuthorizerProperties) -> Authorizer {
        Authorizer { properties }
    }
}

/// The [`AWS::ApiGateway::BasePathMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html) resource type.
pub struct BasePathMapping {
    properties: BasePathMappingProperties
}

/// Properties for the `BasePathMapping` resource.
#[derive(Serialize, Deserialize)]
pub struct BasePathMappingProperties {
    #[serde(rename="BasePath")]
    pub base_path: String,
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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

impl From<BasePathMappingProperties> for BasePathMapping {
    fn from(properties: BasePathMappingProperties) -> BasePathMapping {
        BasePathMapping { properties }
    }
}

/// The [`AWS::ApiGateway::ClientCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-clientcertificate.html) resource type.
pub struct ClientCertificate {
    properties: ClientCertificateProperties
}

/// Properties for the `ClientCertificate` resource.
#[derive(Serialize, Deserialize)]
pub struct ClientCertificateProperties {
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

impl From<ClientCertificateProperties> for ClientCertificate {
    fn from(properties: ClientCertificateProperties) -> ClientCertificate {
        ClientCertificate { properties }
    }
}

/// The [`AWS::ApiGateway::Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html) resource type.
pub struct Deployment {
    properties: DeploymentProperties
}

/// Properties for the `Deployment` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    #[serde(rename="StageDescription")]
    pub stage_description: self::deployment::StageDescription,
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

impl From<DeploymentProperties> for Deployment {
    fn from(properties: DeploymentProperties) -> Deployment {
        Deployment { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html) resource type.
pub struct DocumentationPart {
    properties: DocumentationPartProperties
}

/// Properties for the `DocumentationPart` resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationPartProperties {
    #[serde(rename="Location")]
    pub location: self::documentation_part::Location,
    #[serde(rename="Properties")]
    pub properties: String,
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

impl From<DocumentationPartProperties> for DocumentationPart {
    fn from(properties: DocumentationPartProperties) -> DocumentationPart {
        DocumentationPart { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html) resource type.
pub struct DocumentationVersion {
    properties: DocumentationVersionProperties
}

/// Properties for the `DocumentationVersion` resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationVersionProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: String,
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

impl From<DocumentationVersionProperties> for DocumentationVersion {
    fn from(properties: DocumentationVersionProperties) -> DocumentationVersion {
        DocumentationVersion { properties }
    }
}

/// The [`AWS::ApiGateway::DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html) resource type.
pub struct DomainName {
    properties: DomainNameProperties
}

/// Properties for the `DomainName` resource.
#[derive(Serialize, Deserialize)]
pub struct DomainNameProperties {
    #[serde(rename="CertificateArn")]
    pub certificate_arn: String,
    #[serde(rename="DomainName")]
    pub domain_name: String,
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: self::domain_name::EndpointConfiguration,
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

impl From<DomainNameProperties> for DomainName {
    fn from(properties: DomainNameProperties) -> DomainName {
        DomainName { properties }
    }
}

/// The [`AWS::ApiGateway::GatewayResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html) resource type.
pub struct GatewayResponse {
    properties: GatewayResponseProperties
}

/// Properties for the `GatewayResponse` resource.
#[derive(Serialize, Deserialize)]
pub struct GatewayResponseProperties {
    #[serde(rename="ResponseParameters")]
    pub response_parameters: ::std::collections::HashMap<String, String>,
    #[serde(rename="ResponseTemplates")]
    pub response_templates: ::std::collections::HashMap<String, String>,
    #[serde(rename="ResponseType")]
    pub response_type: String,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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

impl From<GatewayResponseProperties> for GatewayResponse {
    fn from(properties: GatewayResponseProperties) -> GatewayResponse {
        GatewayResponse { properties }
    }
}

/// The [`AWS::ApiGateway::Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html) resource type.
pub struct Method {
    properties: MethodProperties
}

/// Properties for the `Method` resource.
#[derive(Serialize, Deserialize)]
pub struct MethodProperties {
    #[serde(rename="ApiKeyRequired")]
    pub api_key_required: bool,
    #[serde(rename="AuthorizationType")]
    pub authorization_type: String,
    #[serde(rename="AuthorizerId")]
    pub authorizer_id: String,
    #[serde(rename="HttpMethod")]
    pub http_method: String,
    #[serde(rename="Integration")]
    pub integration: self::method::Integration,
    #[serde(rename="MethodResponses")]
    pub method_responses: Vec<self::method::MethodResponse>,
    #[serde(rename="OperationName")]
    pub operation_name: String,
    #[serde(rename="RequestModels")]
    pub request_models: ::std::collections::HashMap<String, String>,
    #[serde(rename="RequestParameters")]
    pub request_parameters: ::std::collections::HashMap<String, bool>,
    #[serde(rename="RequestValidatorId")]
    pub request_validator_id: String,
    #[serde(rename="ResourceId")]
    pub resource_id: String,
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

impl From<MethodProperties> for Method {
    fn from(properties: MethodProperties) -> Method {
        Method { properties }
    }
}

/// The [`AWS::ApiGateway::Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html) resource type.
pub struct Model {
    properties: ModelProperties
}

/// Properties for the `Model` resource.
#[derive(Serialize, Deserialize)]
pub struct ModelProperties {
    #[serde(rename="ContentType")]
    pub content_type: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
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

impl From<ModelProperties> for Model {
    fn from(properties: ModelProperties) -> Model {
        Model { properties }
    }
}

/// The [`AWS::ApiGateway::RequestValidator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html) resource type.
pub struct RequestValidator {
    properties: RequestValidatorProperties
}

/// Properties for the `RequestValidator` resource.
#[derive(Serialize, Deserialize)]
pub struct RequestValidatorProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    #[serde(rename="ValidateRequestBody")]
    pub validate_request_body: bool,
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

impl From<RequestValidatorProperties> for RequestValidator {
    fn from(properties: RequestValidatorProperties) -> RequestValidator {
        RequestValidator { properties }
    }
}

/// The [`AWS::ApiGateway::Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html) resource type.
pub struct Resource {
    properties: ResourceProperties
}

/// Properties for the `Resource` resource.
#[derive(Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename="ParentId")]
    pub parent_id: String,
    #[serde(rename="PathPart")]
    pub path_part: String,
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

impl From<ResourceProperties> for Resource {
    fn from(properties: ResourceProperties) -> Resource {
        Resource { properties }
    }
}

/// The [`AWS::ApiGateway::RestApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html) resource type.
pub struct RestApi {
    properties: RestApiProperties
}

/// Properties for the `RestApi` resource.
#[derive(Serialize, Deserialize)]
pub struct RestApiProperties {
    #[serde(rename="ApiKeySourceType")]
    pub api_key_source_type: String,
    #[serde(rename="BinaryMediaTypes")]
    pub binary_media_types: Vec<String>,
    #[serde(rename="Body")]
    pub body: ::json::Value,
    #[serde(rename="BodyS3Location")]
    pub body_s3_location: self::rest_api::S3Location,
    #[serde(rename="CloneFrom")]
    pub clone_from: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: self::rest_api::EndpointConfiguration,
    #[serde(rename="FailOnWarnings")]
    pub fail_on_warnings: bool,
    #[serde(rename="MinimumCompressionSize")]
    pub minimum_compression_size: u32,
    #[serde(rename="Name")]
    pub name: String,
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

impl From<RestApiProperties> for RestApi {
    fn from(properties: RestApiProperties) -> RestApi {
        RestApi { properties }
    }
}

/// The [`AWS::ApiGateway::Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html) resource type.
pub struct Stage {
    properties: StageProperties
}

/// Properties for the `Stage` resource.
#[derive(Serialize, Deserialize)]
pub struct StageProperties {
    #[serde(rename="CacheClusterEnabled")]
    pub cache_cluster_enabled: bool,
    #[serde(rename="CacheClusterSize")]
    pub cache_cluster_size: String,
    #[serde(rename="ClientCertificateId")]
    pub client_certificate_id: String,
    #[serde(rename="DeploymentId")]
    pub deployment_id: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: String,
    #[serde(rename="MethodSettings")]
    pub method_settings: Vec<self::stage::MethodSetting>,
    #[serde(rename="RestApiId")]
    pub rest_api_id: String,
    #[serde(rename="StageName")]
    pub stage_name: String,
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

impl From<StageProperties> for Stage {
    fn from(properties: StageProperties) -> Stage {
        Stage { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html) resource type.
pub struct UsagePlan {
    properties: UsagePlanProperties
}

/// Properties for the `UsagePlan` resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlanProperties {
    #[serde(rename="ApiStages")]
    pub api_stages: Vec<self::usage_plan::ApiStage>,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Quota")]
    pub quota: self::usage_plan::QuotaSettings,
    #[serde(rename="Throttle")]
    pub throttle: self::usage_plan::ThrottleSettings,
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

impl From<UsagePlanProperties> for UsagePlan {
    fn from(properties: UsagePlanProperties) -> UsagePlan {
        UsagePlan { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlanKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html) resource type.
pub struct UsagePlanKey {
    properties: UsagePlanKeyProperties
}

/// Properties for the `UsagePlanKey` resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlanKeyProperties {
    #[serde(rename="KeyId")]
    pub key_id: String,
    #[serde(rename="KeyType")]
    pub key_type: String,
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

impl From<UsagePlanKeyProperties> for UsagePlanKey {
    fn from(properties: UsagePlanKeyProperties) -> UsagePlanKey {
        UsagePlanKey { properties }
    }
}

/// The [`AWS::ApiGateway::VpcLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html) resource type.
pub struct VpcLink {
    properties: VpcLinkProperties
}

/// Properties for the `VpcLink` resource.
#[derive(Serialize, Deserialize)]
pub struct VpcLinkProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
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

impl From<VpcLinkProperties> for VpcLink {
    fn from(properties: VpcLinkProperties) -> VpcLink {
        VpcLink { properties }
    }
}

pub mod api_key {
    /// The [`AWS::ApiGateway::ApiKey.StageKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-apikey-stagekey.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StageKey {
        #[serde(rename="RestApiId")]
        pub rest_api_id: String,
        #[serde(rename="StageName")]
        pub stage_name: String,
    }

}

pub mod deployment {
    /// The [`AWS::ApiGateway::Deployment.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MethodSetting {
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        #[serde(rename="HttpMethod")]
        pub http_method: String,
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
    }

    /// The [`AWS::ApiGateway::Deployment.StageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StageDescription {
        #[serde(rename="CacheClusterEnabled")]
        pub cache_cluster_enabled: bool,
        #[serde(rename="CacheClusterSize")]
        pub cache_cluster_size: String,
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        #[serde(rename="ClientCertificateId")]
        pub client_certificate_id: String,
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="DocumentationVersion")]
        pub documentation_version: String,
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        #[serde(rename="MethodSettings")]
        pub method_settings: Vec<MethodSetting>,
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
        #[serde(rename="Variables")]
        pub variables: ::std::collections::HashMap<String, String>,
    }

}

pub mod documentation_part {
    /// The [`AWS::ApiGateway::DocumentationPart.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Location {
        #[serde(rename="Method")]
        pub method: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Path")]
        pub path: String,
        #[serde(rename="StatusCode")]
        pub status_code: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod domain_name {
    /// The [`AWS::ApiGateway::DomainName.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        #[serde(rename="Types")]
        pub types: Vec<String>,
    }

}

pub mod method {
    /// The [`AWS::ApiGateway::Method.Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Integration {
        #[serde(rename="CacheKeyParameters")]
        pub cache_key_parameters: Vec<String>,
        #[serde(rename="CacheNamespace")]
        pub cache_namespace: String,
        #[serde(rename="ContentHandling")]
        pub content_handling: String,
        #[serde(rename="Credentials")]
        pub credentials: String,
        #[serde(rename="IntegrationHttpMethod")]
        pub integration_http_method: String,
        #[serde(rename="IntegrationResponses")]
        pub integration_responses: Vec<IntegrationResponse>,
        #[serde(rename="PassthroughBehavior")]
        pub passthrough_behavior: String,
        #[serde(rename="RequestParameters")]
        pub request_parameters: ::std::collections::HashMap<String, String>,
        #[serde(rename="RequestTemplates")]
        pub request_templates: ::std::collections::HashMap<String, String>,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Uri")]
        pub uri: String,
    }

    /// The [`AWS::ApiGateway::Method.IntegrationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct IntegrationResponse {
        #[serde(rename="ContentHandling")]
        pub content_handling: String,
        #[serde(rename="ResponseParameters")]
        pub response_parameters: ::std::collections::HashMap<String, String>,
        #[serde(rename="ResponseTemplates")]
        pub response_templates: ::std::collections::HashMap<String, String>,
        #[serde(rename="SelectionPattern")]
        pub selection_pattern: String,
        #[serde(rename="StatusCode")]
        pub status_code: String,
    }

    /// The [`AWS::ApiGateway::Method.MethodResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MethodResponse {
        #[serde(rename="ResponseModels")]
        pub response_models: ::std::collections::HashMap<String, String>,
        #[serde(rename="ResponseParameters")]
        pub response_parameters: ::std::collections::HashMap<String, bool>,
        #[serde(rename="StatusCode")]
        pub status_code: String,
    }

}

pub mod rest_api {
    /// The [`AWS::ApiGateway::RestApi.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct EndpointConfiguration {
        #[serde(rename="Types")]
        pub types: Vec<String>,
    }

    /// The [`AWS::ApiGateway::RestApi.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3Location {
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="ETag")]
        pub e_tag: String,
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Version")]
        pub version: String,
    }

}

pub mod stage {
    /// The [`AWS::ApiGateway::Stage.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MethodSetting {
        #[serde(rename="CacheDataEncrypted")]
        pub cache_data_encrypted: bool,
        #[serde(rename="CacheTtlInSeconds")]
        pub cache_ttl_in_seconds: u32,
        #[serde(rename="CachingEnabled")]
        pub caching_enabled: bool,
        #[serde(rename="DataTraceEnabled")]
        pub data_trace_enabled: bool,
        #[serde(rename="HttpMethod")]
        pub http_method: String,
        #[serde(rename="LoggingLevel")]
        pub logging_level: String,
        #[serde(rename="MetricsEnabled")]
        pub metrics_enabled: bool,
        #[serde(rename="ResourcePath")]
        pub resource_path: String,
        #[serde(rename="ThrottlingBurstLimit")]
        pub throttling_burst_limit: u32,
        #[serde(rename="ThrottlingRateLimit")]
        pub throttling_rate_limit: f64,
    }

}

pub mod usage_plan {
    /// The [`AWS::ApiGateway::UsagePlan.ApiStage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ApiStage {
        #[serde(rename="ApiId")]
        pub api_id: String,
        #[serde(rename="Stage")]
        pub stage: String,
    }

    /// The [`AWS::ApiGateway::UsagePlan.QuotaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct QuotaSettings {
        #[serde(rename="Limit")]
        pub limit: u32,
        #[serde(rename="Offset")]
        pub offset: u32,
        #[serde(rename="Period")]
        pub period: String,
    }

    /// The [`AWS::ApiGateway::UsagePlan.ThrottleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ThrottleSettings {
        #[serde(rename="BurstLimit")]
        pub burst_limit: u32,
        #[serde(rename="RateLimit")]
        pub rate_limit: f64,
    }

}

