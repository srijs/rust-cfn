//! Types for the `ApiGateway` service.

/// The [`AWS::ApiGateway::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html) resource type.
#[derive(Debug)]
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Debug)]
pub struct AccountProperties {
    /// Property `CloudWatchRoleArn`.
    pub cloud_watch_role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for AccountProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchRoleArn", &self.cloud_watch_role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccountProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccountProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccountProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_watch_role_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudWatchRoleArn" => {
                            cloud_watch_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AccountProperties {
                    cloud_watch_role_arn: cloud_watch_role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Account {
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
#[derive(Debug)]
pub struct ApiKeyProperties {
    /// Property `CustomerId`.
    pub customer_id: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Enabled`.
    pub enabled: Option<::Value<bool>>,
    /// Property `GenerateDistinctId`.
    pub generate_distinct_id: Option<::Value<bool>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `StageKeys`.
    pub stage_keys: Option<::ValueList<self::api_key::StageKey>>,
}

impl ::serde::Serialize for ApiKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerId", &self.customer_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateDistinctId", &self.generate_distinct_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageKeys", &self.stage_keys)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut customer_id = None;
                let mut description = None;
                let mut enabled = None;
                let mut generate_distinct_id = None;
                let mut name = None;
                let mut stage_keys = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomerId" => {
                            customer_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Enabled" => {
                            enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GenerateDistinctId" => {
                            generate_distinct_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StageKeys" => {
                            stage_keys = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ApiKeyProperties {
                    customer_id: customer_id,
                    description: description,
                    enabled: enabled,
                    generate_distinct_id: generate_distinct_id,
                    name: name,
                    stage_keys: stage_keys,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiKey {
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
#[derive(Debug)]
pub struct AuthorizerProperties {
    /// Property `AuthType`.
    pub auth_type: Option<::Value<String>>,
    /// Property `AuthorizerCredentials`.
    pub authorizer_credentials: Option<::Value<String>>,
    /// Property `AuthorizerResultTtlInSeconds`.
    pub authorizer_result_ttl_in_seconds: Option<::Value<u32>>,
    /// Property `AuthorizerUri`.
    pub authorizer_uri: Option<::Value<String>>,
    /// Property `IdentitySource`.
    pub identity_source: Option<::Value<String>>,
    /// Property `IdentityValidationExpression`.
    pub identity_validation_expression: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `ProviderARNs`.
    pub provider_ar_ns: Option<::ValueList<String>>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `Type`.
    pub type_: Option<::Value<String>>,
}

impl ::serde::Serialize for AuthorizerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", &self.auth_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerCredentials", &self.authorizer_credentials)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerResultTtlInSeconds", &self.authorizer_result_ttl_in_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerUri", &self.authorizer_uri)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentitySource", &self.identity_source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityValidationExpression", &self.identity_validation_expression)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderARNs", &self.provider_ar_ns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AuthorizerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AuthorizerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AuthorizerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth_type = None;
                let mut authorizer_credentials = None;
                let mut authorizer_result_ttl_in_seconds = None;
                let mut authorizer_uri = None;
                let mut identity_source = None;
                let mut identity_validation_expression = None;
                let mut name = None;
                let mut provider_ar_ns = None;
                let mut rest_api_id = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthType" => {
                            auth_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthorizerCredentials" => {
                            authorizer_credentials = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthorizerResultTtlInSeconds" => {
                            authorizer_result_ttl_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthorizerUri" => {
                            authorizer_uri = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IdentitySource" => {
                            identity_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IdentityValidationExpression" => {
                            identity_validation_expression = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ProviderARNs" => {
                            provider_ar_ns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AuthorizerProperties {
                    auth_type: auth_type,
                    authorizer_credentials: authorizer_credentials,
                    authorizer_result_ttl_in_seconds: authorizer_result_ttl_in_seconds,
                    authorizer_uri: authorizer_uri,
                    identity_source: identity_source,
                    identity_validation_expression: identity_validation_expression,
                    name: name,
                    provider_ar_ns: provider_ar_ns,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    type_: type_,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Authorizer {
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
#[derive(Debug)]
pub struct BasePathMappingProperties {
    /// Property `BasePath`.
    pub base_path: Option<::Value<String>>,
    /// Property `DomainName`.
    pub domain_name: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: Option<::Value<String>>,
    /// Property `Stage`.
    pub stage: Option<::Value<String>>,
}

impl ::serde::Serialize for BasePathMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasePath", &self.base_path)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", &self.stage)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BasePathMappingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BasePathMappingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BasePathMappingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BasePathMappingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut base_path = None;
                let mut domain_name = None;
                let mut rest_api_id = None;
                let mut stage = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BasePath" => {
                            base_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DomainName" => {
                            domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Stage" => {
                            stage = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(BasePathMappingProperties {
                    base_path: base_path,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    rest_api_id: rest_api_id,
                    stage: stage,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BasePathMapping {
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
#[derive(Debug)]
pub struct ClientCertificateProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
}

impl ::serde::Serialize for ClientCertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClientCertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientCertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClientCertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClientCertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClientCertificateProperties {
                    description: description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClientCertificate {
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
#[derive(Debug)]
pub struct DeploymentProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `StageDescription`.
    pub stage_description: Option<::Value<self::deployment::StageDescription>>,
    /// Property `StageName`.
    pub stage_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DeploymentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageDescription", &self.stage_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", &self.stage_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeploymentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeploymentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut rest_api_id = None;
                let mut stage_description = None;
                let mut stage_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StageDescription" => {
                            stage_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StageName" => {
                            stage_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DeploymentProperties {
                    description: description,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    stage_description: stage_description,
                    stage_name: stage_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Deployment {
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
#[derive(Debug)]
pub struct DocumentationPartProperties {
    /// Property `Location`.
    pub location: ::Value<self::documentation_part::Location>,
    /// Property `Properties`.
    pub properties: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for DocumentationPartProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", &self.properties)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DocumentationPartProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentationPartProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DocumentationPartProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DocumentationPartProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut location = None;
                let mut properties = None;
                let mut rest_api_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Location" => {
                            location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Properties" => {
                            properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DocumentationPartProperties {
                    location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                    properties: properties.ok_or(::serde::de::Error::missing_field("Properties"))?,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DocumentationPart {
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
#[derive(Debug)]
pub struct DocumentationVersionProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `DocumentationVersion`.
    pub documentation_version: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for DocumentationVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentationVersion", &self.documentation_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DocumentationVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentationVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DocumentationVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DocumentationVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut documentation_version = None;
                let mut rest_api_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DocumentationVersion" => {
                            documentation_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DocumentationVersionProperties {
                    description: description,
                    documentation_version: documentation_version.ok_or(::serde::de::Error::missing_field("DocumentationVersion"))?,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DocumentationVersion {
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
#[derive(Debug)]
pub struct DomainNameProperties {
    /// Property `CertificateArn`.
    pub certificate_arn: Option<::Value<String>>,
    /// Property `DomainName`.
    pub domain_name: ::Value<String>,
    /// Property `EndpointConfiguration`.
    pub endpoint_configuration: Option<::Value<self::domain_name::EndpointConfiguration>>,
    /// Property `RegionalCertificateArn`.
    pub regional_certificate_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DomainNameProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfiguration", &self.endpoint_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionalCertificateArn", &self.regional_certificate_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainNameProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainNameProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainNameProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainNameProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_arn = None;
                let mut domain_name = None;
                let mut endpoint_configuration = None;
                let mut regional_certificate_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateArn" => {
                            certificate_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DomainName" => {
                            domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EndpointConfiguration" => {
                            endpoint_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RegionalCertificateArn" => {
                            regional_certificate_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DomainNameProperties {
                    certificate_arn: certificate_arn,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    endpoint_configuration: endpoint_configuration,
                    regional_certificate_arn: regional_certificate_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DomainName {
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
#[derive(Debug)]
pub struct GatewayResponseProperties {
    /// Property `ResponseParameters`.
    pub response_parameters: Option<::ValueMap<String>>,
    /// Property `ResponseTemplates`.
    pub response_templates: Option<::ValueMap<String>>,
    /// Property `ResponseType`.
    pub response_type: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `StatusCode`.
    pub status_code: Option<::Value<String>>,
}

impl ::serde::Serialize for GatewayResponseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", &self.response_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTemplates", &self.response_templates)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseType", &self.response_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GatewayResponseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayResponseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GatewayResponseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GatewayResponseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut response_parameters = None;
                let mut response_templates = None;
                let mut response_type = None;
                let mut rest_api_id = None;
                let mut status_code = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResponseParameters" => {
                            response_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResponseTemplates" => {
                            response_templates = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResponseType" => {
                            response_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StatusCode" => {
                            status_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(GatewayResponseProperties {
                    response_parameters: response_parameters,
                    response_templates: response_templates,
                    response_type: response_type.ok_or(::serde::de::Error::missing_field("ResponseType"))?,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    status_code: status_code,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GatewayResponse {
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
#[derive(Debug)]
pub struct MethodProperties {
    /// Property `ApiKeyRequired`.
    pub api_key_required: Option<::Value<bool>>,
    /// Property `AuthorizationType`.
    pub authorization_type: Option<::Value<String>>,
    /// Property `AuthorizerId`.
    pub authorizer_id: Option<::Value<String>>,
    /// Property `HttpMethod`.
    pub http_method: ::Value<String>,
    /// Property `Integration`.
    pub integration: Option<::Value<self::method::Integration>>,
    /// Property `MethodResponses`.
    pub method_responses: Option<::ValueList<self::method::MethodResponse>>,
    /// Property `OperationName`.
    pub operation_name: Option<::Value<String>>,
    /// Property `RequestModels`.
    pub request_models: Option<::ValueMap<String>>,
    /// Property `RequestParameters`.
    pub request_parameters: Option<::ValueMap<bool>>,
    /// Property `RequestValidatorId`.
    pub request_validator_id: Option<::Value<String>>,
    /// Property `ResourceId`.
    pub resource_id: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for MethodProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeyRequired", &self.api_key_required)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", &self.authorization_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerId", &self.authorizer_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", &self.http_method)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Integration", &self.integration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodResponses", &self.method_responses)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperationName", &self.operation_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestModels", &self.request_models)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestParameters", &self.request_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestValidatorId", &self.request_validator_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MethodProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MethodProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MethodProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MethodProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_key_required = None;
                let mut authorization_type = None;
                let mut authorizer_id = None;
                let mut http_method = None;
                let mut integration = None;
                let mut method_responses = None;
                let mut operation_name = None;
                let mut request_models = None;
                let mut request_parameters = None;
                let mut request_validator_id = None;
                let mut resource_id = None;
                let mut rest_api_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKeyRequired" => {
                            api_key_required = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthorizationType" => {
                            authorization_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AuthorizerId" => {
                            authorizer_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HttpMethod" => {
                            http_method = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Integration" => {
                            integration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MethodResponses" => {
                            method_responses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OperationName" => {
                            operation_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RequestModels" => {
                            request_models = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RequestParameters" => {
                            request_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RequestValidatorId" => {
                            request_validator_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ResourceId" => {
                            resource_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(MethodProperties {
                    api_key_required: api_key_required,
                    authorization_type: authorization_type,
                    authorizer_id: authorizer_id,
                    http_method: http_method.ok_or(::serde::de::Error::missing_field("HttpMethod"))?,
                    integration: integration,
                    method_responses: method_responses,
                    operation_name: operation_name,
                    request_models: request_models,
                    request_parameters: request_parameters,
                    request_validator_id: request_validator_id,
                    resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Method {
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
#[derive(Debug)]
pub struct ModelProperties {
    /// Property `ContentType`.
    pub content_type: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `Schema`.
    pub schema: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", &self.content_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content_type = None;
                let mut description = None;
                let mut name = None;
                let mut rest_api_id = None;
                let mut schema = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContentType" => {
                            content_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Schema" => {
                            schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ModelProperties {
                    content_type: content_type,
                    description: description,
                    name: name,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    schema: schema,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Model {
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
#[derive(Debug)]
pub struct RequestValidatorProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `ValidateRequestBody`.
    pub validate_request_body: Option<::Value<bool>>,
    /// Property `ValidateRequestParameters`.
    pub validate_request_parameters: Option<::Value<bool>>,
}

impl ::serde::Serialize for RequestValidatorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidateRequestBody", &self.validate_request_body)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidateRequestParameters", &self.validate_request_parameters)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RequestValidatorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RequestValidatorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RequestValidatorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RequestValidatorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;
                let mut rest_api_id = None;
                let mut validate_request_body = None;
                let mut validate_request_parameters = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ValidateRequestBody" => {
                            validate_request_body = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ValidateRequestParameters" => {
                            validate_request_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RequestValidatorProperties {
                    name: name,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    validate_request_body: validate_request_body,
                    validate_request_parameters: validate_request_parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RequestValidator {
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
#[derive(Debug)]
pub struct ResourceProperties {
    /// Property `ParentId`.
    pub parent_id: ::Value<String>,
    /// Property `PathPart`.
    pub path_part: ::Value<String>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for ResourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentId", &self.parent_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPart", &self.path_part)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut parent_id = None;
                let mut path_part = None;
                let mut rest_api_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ParentId" => {
                            parent_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PathPart" => {
                            path_part = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ResourceProperties {
                    parent_id: parent_id.ok_or(::serde::de::Error::missing_field("ParentId"))?,
                    path_part: path_part.ok_or(::serde::de::Error::missing_field("PathPart"))?,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Resource {
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
#[derive(Debug)]
pub struct RestApiProperties {
    /// Property `ApiKeySourceType`.
    pub api_key_source_type: Option<::Value<String>>,
    /// Property `BinaryMediaTypes`.
    pub binary_media_types: Option<::ValueList<String>>,
    /// Property `Body`.
    pub body: Option<::Value<::json::Value>>,
    /// Property `BodyS3Location`.
    pub body_s3_location: Option<::Value<self::rest_api::S3Location>>,
    /// Property `CloneFrom`.
    pub clone_from: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `EndpointConfiguration`.
    pub endpoint_configuration: Option<::Value<self::rest_api::EndpointConfiguration>>,
    /// Property `FailOnWarnings`.
    pub fail_on_warnings: Option<::Value<bool>>,
    /// Property `MinimumCompressionSize`.
    pub minimum_compression_size: Option<::Value<u32>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `Parameters`.
    pub parameters: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for RestApiProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeySourceType", &self.api_key_source_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BinaryMediaTypes", &self.binary_media_types)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", &self.body)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyS3Location", &self.body_s3_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloneFrom", &self.clone_from)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfiguration", &self.endpoint_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailOnWarnings", &self.fail_on_warnings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumCompressionSize", &self.minimum_compression_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RestApiProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RestApiProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RestApiProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RestApiProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_key_source_type = None;
                let mut binary_media_types = None;
                let mut body = None;
                let mut body_s3_location = None;
                let mut clone_from = None;
                let mut description = None;
                let mut endpoint_configuration = None;
                let mut fail_on_warnings = None;
                let mut minimum_compression_size = None;
                let mut name = None;
                let mut parameters = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKeySourceType" => {
                            api_key_source_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BinaryMediaTypes" => {
                            binary_media_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Body" => {
                            body = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BodyS3Location" => {
                            body_s3_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CloneFrom" => {
                            clone_from = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EndpointConfiguration" => {
                            endpoint_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "FailOnWarnings" => {
                            fail_on_warnings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinimumCompressionSize" => {
                            minimum_compression_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(RestApiProperties {
                    api_key_source_type: api_key_source_type,
                    binary_media_types: binary_media_types,
                    body: body,
                    body_s3_location: body_s3_location,
                    clone_from: clone_from,
                    description: description,
                    endpoint_configuration: endpoint_configuration,
                    fail_on_warnings: fail_on_warnings,
                    minimum_compression_size: minimum_compression_size,
                    name: name,
                    parameters: parameters,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RestApi {
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
#[derive(Debug)]
pub struct StageProperties {
    /// Property `CacheClusterEnabled`.
    pub cache_cluster_enabled: Option<::Value<bool>>,
    /// Property `CacheClusterSize`.
    pub cache_cluster_size: Option<::Value<String>>,
    /// Property `ClientCertificateId`.
    pub client_certificate_id: Option<::Value<String>>,
    /// Property `DeploymentId`.
    pub deployment_id: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `DocumentationVersion`.
    pub documentation_version: Option<::Value<String>>,
    /// Property `MethodSettings`.
    pub method_settings: Option<::ValueList<self::stage::MethodSetting>>,
    /// Property `RestApiId`.
    pub rest_api_id: ::Value<String>,
    /// Property `StageName`.
    pub stage_name: Option<::Value<String>>,
    /// Property `Variables`.
    pub variables: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for StageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterEnabled", &self.cache_cluster_enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterSize", &self.cache_cluster_size)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateId", &self.client_certificate_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentId", &self.deployment_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentationVersion", &self.documentation_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodSettings", &self.method_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", &self.stage_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", &self.variables)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cache_cluster_enabled = None;
                let mut cache_cluster_size = None;
                let mut client_certificate_id = None;
                let mut deployment_id = None;
                let mut description = None;
                let mut documentation_version = None;
                let mut method_settings = None;
                let mut rest_api_id = None;
                let mut stage_name = None;
                let mut variables = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheClusterEnabled" => {
                            cache_cluster_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CacheClusterSize" => {
                            cache_cluster_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClientCertificateId" => {
                            client_certificate_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeploymentId" => {
                            deployment_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DocumentationVersion" => {
                            documentation_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MethodSettings" => {
                            method_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RestApiId" => {
                            rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StageName" => {
                            stage_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Variables" => {
                            variables = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(StageProperties {
                    cache_cluster_enabled: cache_cluster_enabled,
                    cache_cluster_size: cache_cluster_size,
                    client_certificate_id: client_certificate_id,
                    deployment_id: deployment_id,
                    description: description,
                    documentation_version: documentation_version,
                    method_settings: method_settings,
                    rest_api_id: rest_api_id.ok_or(::serde::de::Error::missing_field("RestApiId"))?,
                    stage_name: stage_name,
                    variables: variables,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Stage {
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
#[derive(Debug)]
pub struct UsagePlanProperties {
    /// Property `ApiStages`.
    pub api_stages: Option<::ValueList<self::usage_plan::ApiStage>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Quota`.
    pub quota: Option<::Value<self::usage_plan::QuotaSettings>>,
    /// Property `Throttle`.
    pub throttle: Option<::Value<self::usage_plan::ThrottleSettings>>,
    /// Property `UsagePlanName`.
    pub usage_plan_name: Option<::Value<String>>,
}

impl ::serde::Serialize for UsagePlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiStages", &self.api_stages)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quota", &self.quota)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throttle", &self.throttle)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsagePlanName", &self.usage_plan_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UsagePlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UsagePlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UsagePlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UsagePlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_stages = None;
                let mut description = None;
                let mut quota = None;
                let mut throttle = None;
                let mut usage_plan_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiStages" => {
                            api_stages = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Quota" => {
                            quota = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Throttle" => {
                            throttle = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UsagePlanName" => {
                            usage_plan_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UsagePlanProperties {
                    api_stages: api_stages,
                    description: description,
                    quota: quota,
                    throttle: throttle,
                    usage_plan_name: usage_plan_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UsagePlan {
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
#[derive(Debug)]
pub struct UsagePlanKeyProperties {
    /// Property `KeyId`.
    pub key_id: ::Value<String>,
    /// Property `KeyType`.
    pub key_type: ::Value<String>,
    /// Property `UsagePlanId`.
    pub usage_plan_id: ::Value<String>,
}

impl ::serde::Serialize for UsagePlanKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyId", &self.key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", &self.key_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsagePlanId", &self.usage_plan_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UsagePlanKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UsagePlanKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UsagePlanKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UsagePlanKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut key_id = None;
                let mut key_type = None;
                let mut usage_plan_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KeyId" => {
                            key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeyType" => {
                            key_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UsagePlanId" => {
                            usage_plan_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UsagePlanKeyProperties {
                    key_id: key_id.ok_or(::serde::de::Error::missing_field("KeyId"))?,
                    key_type: key_type.ok_or(::serde::de::Error::missing_field("KeyType"))?,
                    usage_plan_id: usage_plan_id.ok_or(::serde::de::Error::missing_field("UsagePlanId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UsagePlanKey {
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
#[derive(Debug)]
pub struct VpcLinkProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `TargetArns`.
    pub target_arns: ::ValueList<String>,
}

impl ::serde::Serialize for VpcLinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArns", &self.target_arns)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VpcLinkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcLinkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VpcLinkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VpcLinkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut name = None;
                let mut target_arns = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TargetArns" => {
                            target_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VpcLinkProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    target_arns: target_arns.ok_or(::serde::de::Error::missing_field("TargetArns"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VpcLink {
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
    #[derive(Debug)]
    pub struct StageKey {
        /// Property `RestApiId`.
        pub rest_api_id: Option<::Value<String>>,
        /// Property `StageName`.
        pub stage_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StageKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", &self.stage_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StageKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StageKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StageKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StageKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rest_api_id = None;
                    let mut stage_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RestApiId" => {
                                rest_api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StageName" => {
                                stage_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StageKey {
                        rest_api_id: rest_api_id,
                        stage_name: stage_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod deployment {
    //! Property types for the `Deployment` resource.

    /// The [`AWS::ApiGateway::Deployment.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html) property type.
    #[derive(Debug)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `DataTraceEnabled`.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `HttpMethod`.
        pub http_method: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        pub logging_level: Option<::Value<String>>,
        /// Property `MetricsEnabled`.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ResourcePath`.
        pub resource_path: Option<::Value<String>>,
        /// Property `ThrottlingBurstLimit`.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for MethodSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", &self.cache_data_encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", &self.cache_ttl_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", &self.caching_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", &self.data_trace_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", &self.http_method)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", &self.logging_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", &self.metrics_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", &self.resource_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", &self.throttling_burst_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", &self.throttling_rate_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MethodSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MethodSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MethodSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MethodSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cache_data_encrypted = None;
                    let mut cache_ttl_in_seconds = None;
                    let mut caching_enabled = None;
                    let mut data_trace_enabled = None;
                    let mut http_method = None;
                    let mut logging_level = None;
                    let mut metrics_enabled = None;
                    let mut resource_path = None;
                    let mut throttling_burst_limit = None;
                    let mut throttling_rate_limit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CachingEnabled" => {
                                caching_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HttpMethod" => {
                                http_method = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoggingLevel" => {
                                logging_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourcePath" => {
                                resource_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MethodSetting {
                        cache_data_encrypted: cache_data_encrypted,
                        cache_ttl_in_seconds: cache_ttl_in_seconds,
                        caching_enabled: caching_enabled,
                        data_trace_enabled: data_trace_enabled,
                        http_method: http_method,
                        logging_level: logging_level,
                        metrics_enabled: metrics_enabled,
                        resource_path: resource_path,
                        throttling_burst_limit: throttling_burst_limit,
                        throttling_rate_limit: throttling_rate_limit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::Deployment.StageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html) property type.
    #[derive(Debug)]
    pub struct StageDescription {
        /// Property `CacheClusterEnabled`.
        pub cache_cluster_enabled: Option<::Value<bool>>,
        /// Property `CacheClusterSize`.
        pub cache_cluster_size: Option<::Value<String>>,
        /// Property `CacheDataEncrypted`.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `ClientCertificateId`.
        pub client_certificate_id: Option<::Value<String>>,
        /// Property `DataTraceEnabled`.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `DocumentationVersion`.
        pub documentation_version: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        pub logging_level: Option<::Value<String>>,
        /// Property `MethodSettings`.
        pub method_settings: Option<::ValueList<MethodSetting>>,
        /// Property `MetricsEnabled`.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ThrottlingBurstLimit`.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        pub throttling_rate_limit: Option<::Value<f64>>,
        /// Property `Variables`.
        pub variables: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for StageDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterEnabled", &self.cache_cluster_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterSize", &self.cache_cluster_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", &self.cache_data_encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", &self.cache_ttl_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", &self.caching_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateId", &self.client_certificate_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", &self.data_trace_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentationVersion", &self.documentation_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", &self.logging_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodSettings", &self.method_settings)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", &self.metrics_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", &self.throttling_burst_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", &self.throttling_rate_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", &self.variables)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StageDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StageDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StageDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StageDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cache_cluster_enabled = None;
                    let mut cache_cluster_size = None;
                    let mut cache_data_encrypted = None;
                    let mut cache_ttl_in_seconds = None;
                    let mut caching_enabled = None;
                    let mut client_certificate_id = None;
                    let mut data_trace_enabled = None;
                    let mut description = None;
                    let mut documentation_version = None;
                    let mut logging_level = None;
                    let mut method_settings = None;
                    let mut metrics_enabled = None;
                    let mut throttling_burst_limit = None;
                    let mut throttling_rate_limit = None;
                    let mut variables = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheClusterEnabled" => {
                                cache_cluster_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheClusterSize" => {
                                cache_cluster_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CachingEnabled" => {
                                caching_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ClientCertificateId" => {
                                client_certificate_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DocumentationVersion" => {
                                documentation_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoggingLevel" => {
                                logging_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MethodSettings" => {
                                method_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Variables" => {
                                variables = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StageDescription {
                        cache_cluster_enabled: cache_cluster_enabled,
                        cache_cluster_size: cache_cluster_size,
                        cache_data_encrypted: cache_data_encrypted,
                        cache_ttl_in_seconds: cache_ttl_in_seconds,
                        caching_enabled: caching_enabled,
                        client_certificate_id: client_certificate_id,
                        data_trace_enabled: data_trace_enabled,
                        description: description,
                        documentation_version: documentation_version,
                        logging_level: logging_level,
                        method_settings: method_settings,
                        metrics_enabled: metrics_enabled,
                        throttling_burst_limit: throttling_burst_limit,
                        throttling_rate_limit: throttling_rate_limit,
                        variables: variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod documentation_part {
    //! Property types for the `DocumentationPart` resource.

    /// The [`AWS::ApiGateway::DocumentationPart.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html) property type.
    #[derive(Debug)]
    pub struct Location {
        /// Property `Method`.
        pub method: Option<::Value<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Path`.
        pub path: Option<::Value<String>>,
        /// Property `StatusCode`.
        pub status_code: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", &self.method)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut method = None;
                    let mut name = None;
                    let mut path = None;
                    let mut status_code = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Method" => {
                                method = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Path" => {
                                path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StatusCode" => {
                                status_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        method: method,
                        name: name,
                        path: path,
                        status_code: status_code,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain_name {
    //! Property types for the `DomainName` resource.

    /// The [`AWS::ApiGateway::DomainName.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        pub types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for EndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Types", &self.types)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut types = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Types" => {
                                types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointConfiguration {
                        types: types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod method {
    //! Property types for the `Method` resource.

    /// The [`AWS::ApiGateway::Method.Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html) property type.
    #[derive(Debug)]
    pub struct Integration {
        /// Property `CacheKeyParameters`.
        pub cache_key_parameters: Option<::ValueList<String>>,
        /// Property `CacheNamespace`.
        pub cache_namespace: Option<::Value<String>>,
        /// Property `ContentHandling`.
        pub content_handling: Option<::Value<String>>,
        /// Property `Credentials`.
        pub credentials: Option<::Value<String>>,
        /// Property `IntegrationHttpMethod`.
        pub integration_http_method: Option<::Value<String>>,
        /// Property `IntegrationResponses`.
        pub integration_responses: Option<::ValueList<IntegrationResponse>>,
        /// Property `PassthroughBehavior`.
        pub passthrough_behavior: Option<::Value<String>>,
        /// Property `RequestParameters`.
        pub request_parameters: Option<::ValueMap<String>>,
        /// Property `RequestTemplates`.
        pub request_templates: Option<::ValueMap<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Uri`.
        pub uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Integration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheKeyParameters", &self.cache_key_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNamespace", &self.cache_namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandling", &self.content_handling)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", &self.credentials)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationHttpMethod", &self.integration_http_method)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationResponses", &self.integration_responses)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassthroughBehavior", &self.passthrough_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestParameters", &self.request_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestTemplates", &self.request_templates)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", &self.uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Integration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Integration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Integration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Integration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cache_key_parameters = None;
                    let mut cache_namespace = None;
                    let mut content_handling = None;
                    let mut credentials = None;
                    let mut integration_http_method = None;
                    let mut integration_responses = None;
                    let mut passthrough_behavior = None;
                    let mut request_parameters = None;
                    let mut request_templates = None;
                    let mut type_ = None;
                    let mut uri = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheKeyParameters" => {
                                cache_key_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheNamespace" => {
                                cache_namespace = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ContentHandling" => {
                                content_handling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Credentials" => {
                                credentials = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IntegrationHttpMethod" => {
                                integration_http_method = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IntegrationResponses" => {
                                integration_responses = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PassthroughBehavior" => {
                                passthrough_behavior = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequestParameters" => {
                                request_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequestTemplates" => {
                                request_templates = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Uri" => {
                                uri = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Integration {
                        cache_key_parameters: cache_key_parameters,
                        cache_namespace: cache_namespace,
                        content_handling: content_handling,
                        credentials: credentials,
                        integration_http_method: integration_http_method,
                        integration_responses: integration_responses,
                        passthrough_behavior: passthrough_behavior,
                        request_parameters: request_parameters,
                        request_templates: request_templates,
                        type_: type_,
                        uri: uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::Method.IntegrationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html) property type.
    #[derive(Debug)]
    pub struct IntegrationResponse {
        /// Property `ContentHandling`.
        pub content_handling: Option<::Value<String>>,
        /// Property `ResponseParameters`.
        pub response_parameters: Option<::ValueMap<String>>,
        /// Property `ResponseTemplates`.
        pub response_templates: Option<::ValueMap<String>>,
        /// Property `SelectionPattern`.
        pub selection_pattern: Option<::Value<String>>,
        /// Property `StatusCode`.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for IntegrationResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandling", &self.content_handling)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", &self.response_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTemplates", &self.response_templates)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectionPattern", &self.selection_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntegrationResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntegrationResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntegrationResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_handling = None;
                    let mut response_parameters = None;
                    let mut response_templates = None;
                    let mut selection_pattern = None;
                    let mut status_code = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentHandling" => {
                                content_handling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResponseParameters" => {
                                response_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResponseTemplates" => {
                                response_templates = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SelectionPattern" => {
                                selection_pattern = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StatusCode" => {
                                status_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(IntegrationResponse {
                        content_handling: content_handling,
                        response_parameters: response_parameters,
                        response_templates: response_templates,
                        selection_pattern: selection_pattern,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::Method.MethodResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html) property type.
    #[derive(Debug)]
    pub struct MethodResponse {
        /// Property `ResponseModels`.
        pub response_models: Option<::ValueMap<String>>,
        /// Property `ResponseParameters`.
        pub response_parameters: Option<::ValueMap<bool>>,
        /// Property `StatusCode`.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for MethodResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseModels", &self.response_models)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", &self.response_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MethodResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MethodResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MethodResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MethodResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut response_models = None;
                    let mut response_parameters = None;
                    let mut status_code = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResponseModels" => {
                                response_models = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResponseParameters" => {
                                response_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StatusCode" => {
                                status_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MethodResponse {
                        response_models: response_models,
                        response_parameters: response_parameters,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rest_api {
    //! Property types for the `RestApi` resource.

    /// The [`AWS::ApiGateway::RestApi.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html) property type.
    #[derive(Debug)]
    pub struct EndpointConfiguration {
        /// Property `Types`.
        pub types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for EndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Types", &self.types)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut types = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Types" => {
                                types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointConfiguration {
                        types: types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::RestApi.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html) property type.
    #[derive(Debug)]
    pub struct S3Location {
        /// Property `Bucket`.
        pub bucket: Option<::Value<String>>,
        /// Property `ETag`.
        pub e_tag: Option<::Value<String>>,
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Version`.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ETag", &self.e_tag)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket = None;
                    let mut e_tag = None;
                    let mut key = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ETag" => {
                                e_tag = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket,
                        e_tag: e_tag,
                        key: key,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod stage {
    //! Property types for the `Stage` resource.

    /// The [`AWS::ApiGateway::Stage.MethodSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html) property type.
    #[derive(Debug)]
    pub struct MethodSetting {
        /// Property `CacheDataEncrypted`.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property `CacheTtlInSeconds`.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property `CachingEnabled`.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property `DataTraceEnabled`.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property `HttpMethod`.
        pub http_method: Option<::Value<String>>,
        /// Property `LoggingLevel`.
        pub logging_level: Option<::Value<String>>,
        /// Property `MetricsEnabled`.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property `ResourcePath`.
        pub resource_path: Option<::Value<String>>,
        /// Property `ThrottlingBurstLimit`.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property `ThrottlingRateLimit`.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for MethodSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", &self.cache_data_encrypted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", &self.cache_ttl_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", &self.caching_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", &self.data_trace_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", &self.http_method)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", &self.logging_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", &self.metrics_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", &self.resource_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", &self.throttling_burst_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", &self.throttling_rate_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MethodSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MethodSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MethodSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MethodSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cache_data_encrypted = None;
                    let mut cache_ttl_in_seconds = None;
                    let mut caching_enabled = None;
                    let mut data_trace_enabled = None;
                    let mut http_method = None;
                    let mut logging_level = None;
                    let mut metrics_enabled = None;
                    let mut resource_path = None;
                    let mut throttling_burst_limit = None;
                    let mut throttling_rate_limit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CachingEnabled" => {
                                caching_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HttpMethod" => {
                                http_method = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoggingLevel" => {
                                logging_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourcePath" => {
                                resource_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MethodSetting {
                        cache_data_encrypted: cache_data_encrypted,
                        cache_ttl_in_seconds: cache_ttl_in_seconds,
                        caching_enabled: caching_enabled,
                        data_trace_enabled: data_trace_enabled,
                        http_method: http_method,
                        logging_level: logging_level,
                        metrics_enabled: metrics_enabled,
                        resource_path: resource_path,
                        throttling_burst_limit: throttling_burst_limit,
                        throttling_rate_limit: throttling_rate_limit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod usage_plan {
    //! Property types for the `UsagePlan` resource.

    /// The [`AWS::ApiGateway::UsagePlan.ApiStage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html) property type.
    #[derive(Debug)]
    pub struct ApiStage {
        /// Property `ApiId`.
        pub api_id: Option<::Value<String>>,
        /// Property `Stage`.
        pub stage: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApiStage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", &self.stage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApiStage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiStage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApiStage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApiStage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_id = None;
                    let mut stage = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiId" => {
                                api_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Stage" => {
                                stage = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ApiStage {
                        api_id: api_id,
                        stage: stage,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::UsagePlan.QuotaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html) property type.
    #[derive(Debug)]
    pub struct QuotaSettings {
        /// Property `Limit`.
        pub limit: Option<::Value<u32>>,
        /// Property `Offset`.
        pub offset: Option<::Value<u32>>,
        /// Property `Period`.
        pub period: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for QuotaSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limit", &self.limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", &self.offset)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuotaSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuotaSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuotaSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuotaSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut limit = None;
                    let mut offset = None;
                    let mut period = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Limit" => {
                                limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Offset" => {
                                offset = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Period" => {
                                period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(QuotaSettings {
                        limit: limit,
                        offset: offset,
                        period: period,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGateway::UsagePlan.ThrottleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html) property type.
    #[derive(Debug)]
    pub struct ThrottleSettings {
        /// Property `BurstLimit`.
        pub burst_limit: Option<::Value<u32>>,
        /// Property `RateLimit`.
        pub rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ThrottleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BurstLimit", &self.burst_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateLimit", &self.rate_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThrottleSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThrottleSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThrottleSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThrottleSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut burst_limit = None;
                    let mut rate_limit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BurstLimit" => {
                                burst_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RateLimit" => {
                                rate_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ThrottleSettings {
                        burst_limit: burst_limit,
                        rate_limit: rate_limit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
