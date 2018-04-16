//! Types for the `ApiGateway` service.

/// The [`AWS::ApiGateway::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html) resource type.
#[derive(Debug)]
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Debug, Default)]
pub struct AccountProperties {
    /// Property [`CloudWatchRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html#cfn-apigateway-account-cloudwatchrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for AccountProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cloud_watch_role_arn) = self.cloud_watch_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchRoleArn", cloud_watch_role_arn)?;
        }
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
                let mut cloud_watch_role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudWatchRoleArn" => {
                            cloud_watch_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct ApiKeyProperties {
    /// Property [`CustomerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-customerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub customer_id: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`GenerateDistinctId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-generatedistinctid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub generate_distinct_id: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`StageKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html#cfn-apigateway-apikey-stagekeys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage_keys: Option<::ValueList<self::api_key::StageKey>>,
}

impl ::serde::Serialize for ApiKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref customer_id) = self.customer_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerId", customer_id)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref generate_distinct_id) = self.generate_distinct_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateDistinctId", generate_distinct_id)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref stage_keys) = self.stage_keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageKeys", stage_keys)?;
        }
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
                let mut customer_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut generate_distinct_id: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut stage_keys: Option<::ValueList<self::api_key::StageKey>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomerId" => {
                            customer_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GenerateDistinctId" => {
                            generate_distinct_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageKeys" => {
                            stage_keys = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct AuthorizerProperties {
    /// Property [`AuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-authtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth_type: Option<::Value<String>>,
    /// Property [`AuthorizerCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-authorizercredentials).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_credentials: Option<::Value<String>>,
    /// Property [`AuthorizerResultTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-authorizerresultttlinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_result_ttl_in_seconds: Option<::Value<u32>>,
    /// Property [`AuthorizerUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-authorizeruri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_uri: Option<::Value<String>>,
    /// Property [`IdentitySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-identitysource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_source: Option<::Value<String>>,
    /// Property [`IdentityValidationExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-identityvalidationexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_validation_expression: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ProviderARNs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-providerarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provider_ar_ns: Option<::ValueList<String>>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html#cfn-apigateway-authorizer-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_: Option<::Value<String>>,
}

impl ::serde::Serialize for AuthorizerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auth_type) = self.auth_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", auth_type)?;
        }
        if let Some(ref authorizer_credentials) = self.authorizer_credentials {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerCredentials", authorizer_credentials)?;
        }
        if let Some(ref authorizer_result_ttl_in_seconds) = self.authorizer_result_ttl_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerResultTtlInSeconds", authorizer_result_ttl_in_seconds)?;
        }
        if let Some(ref authorizer_uri) = self.authorizer_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerUri", authorizer_uri)?;
        }
        if let Some(ref identity_source) = self.identity_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentitySource", identity_source)?;
        }
        if let Some(ref identity_validation_expression) = self.identity_validation_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityValidationExpression", identity_validation_expression)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref provider_ar_ns) = self.provider_ar_ns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderARNs", provider_ar_ns)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref type_) = self.type_ {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
        }
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
                let mut auth_type: Option<::Value<String>> = None;
                let mut authorizer_credentials: Option<::Value<String>> = None;
                let mut authorizer_result_ttl_in_seconds: Option<::Value<u32>> = None;
                let mut authorizer_uri: Option<::Value<String>> = None;
                let mut identity_source: Option<::Value<String>> = None;
                let mut identity_validation_expression: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut provider_ar_ns: Option<::ValueList<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut type_: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthType" => {
                            auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerCredentials" => {
                            authorizer_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerResultTtlInSeconds" => {
                            authorizer_result_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerUri" => {
                            authorizer_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentitySource" => {
                            identity_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityValidationExpression" => {
                            identity_validation_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderARNs" => {
                            provider_ar_ns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct BasePathMappingProperties {
    /// Property [`BasePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html#cfn-apigateway-basepathmapping-basepath).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub base_path: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html#cfn-apigateway-basepathmapping-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html#cfn-apigateway-basepathmapping-restapiid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rest_api_id: Option<::Value<String>>,
    /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html#cfn-apigateway-basepathmapping-stage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage: Option<::Value<String>>,
}

impl ::serde::Serialize for BasePathMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref base_path) = self.base_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasePath", base_path)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref rest_api_id) = self.rest_api_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", rest_api_id)?;
        }
        if let Some(ref stage) = self.stage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", stage)?;
        }
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
                let mut base_path: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut stage: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BasePath" => {
                            base_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stage" => {
                            stage = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct ClientCertificateProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-clientcertificate.html#cfn-apigateway-clientcertificate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
}

impl ::serde::Serialize for ClientCertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
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
                let mut description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct DeploymentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html#cfn-apigateway-deployment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html#cfn-apigateway-deployment-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`StageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html#cfn-apigateway-deployment-stagedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage_description: Option<::Value<self::deployment::StageDescription>>,
    /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html#cfn-apigateway-deployment-stagename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DeploymentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref stage_description) = self.stage_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageDescription", stage_description)?;
        }
        if let Some(ref stage_name) = self.stage_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", stage_name)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut stage_description: Option<::Value<self::deployment::StageDescription>> = None;
                let mut stage_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageDescription" => {
                            stage_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageName" => {
                            stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct DocumentationPartProperties {
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html#cfn-apigateway-documentationpart-location).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub location: ::Value<self::documentation_part::Location>,
    /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html#cfn-apigateway-documentationpart-properties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub properties: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html#cfn-apigateway-documentationpart-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
                let mut location: Option<::Value<self::documentation_part::Location>> = None;
                let mut properties: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Properties" => {
                            properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct DocumentationVersionProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html#cfn-apigateway-documentationversion-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html#cfn-apigateway-documentationversion-documentationversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub documentation_version: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html#cfn-apigateway-documentationversion-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for DocumentationVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut documentation_version: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentationVersion" => {
                            documentation_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct DomainNameProperties {
    /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html#cfn-apigateway-domainname-certificatearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_arn: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html#cfn-apigateway-domainname-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html#cfn-apigateway-domainname-endpointconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_configuration: Option<::Value<self::domain_name::EndpointConfiguration>>,
    /// Property [`RegionalCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html#cfn-apigateway-domainname-regionalcertificatearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regional_certificate_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DomainNameProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate_arn) = self.certificate_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref endpoint_configuration) = self.endpoint_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfiguration", endpoint_configuration)?;
        }
        if let Some(ref regional_certificate_arn) = self.regional_certificate_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionalCertificateArn", regional_certificate_arn)?;
        }
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
                let mut certificate_arn: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut endpoint_configuration: Option<::Value<self::domain_name::EndpointConfiguration>> = None;
                let mut regional_certificate_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateArn" => {
                            certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointConfiguration" => {
                            endpoint_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegionalCertificateArn" => {
                            regional_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct GatewayResponseProperties {
    /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html#cfn-apigateway-gatewayresponse-responseparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_parameters: Option<::ValueMap<String>>,
    /// Property [`ResponseTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html#cfn-apigateway-gatewayresponse-responsetemplates).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_templates: Option<::ValueMap<String>>,
    /// Property [`ResponseType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html#cfn-apigateway-gatewayresponse-responsetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub response_type: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html#cfn-apigateway-gatewayresponse-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html#cfn-apigateway-gatewayresponse-statuscode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status_code: Option<::Value<String>>,
}

impl ::serde::Serialize for GatewayResponseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref response_parameters) = self.response_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
        }
        if let Some(ref response_templates) = self.response_templates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTemplates", response_templates)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseType", &self.response_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref status_code) = self.status_code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", status_code)?;
        }
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
                let mut response_parameters: Option<::ValueMap<String>> = None;
                let mut response_templates: Option<::ValueMap<String>> = None;
                let mut response_type: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut status_code: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResponseParameters" => {
                            response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseTemplates" => {
                            response_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseType" => {
                            response_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StatusCode" => {
                            status_code = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct MethodProperties {
    /// Property [`ApiKeyRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-apikeyrequired).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key_required: Option<::Value<bool>>,
    /// Property [`AuthorizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-authorizationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization_type: Option<::Value<String>>,
    /// Property [`AuthorizerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-authorizerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_id: Option<::Value<String>>,
    /// Property [`HttpMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-httpmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_method: ::Value<String>,
    /// Property [`Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-integration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration: Option<::Value<self::method::Integration>>,
    /// Property [`MethodResponses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-methodresponses).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub method_responses: Option<::ValueList<self::method::MethodResponse>>,
    /// Property [`OperationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-operationname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub operation_name: Option<::Value<String>>,
    /// Property [`RequestModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-requestmodels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_models: Option<::ValueMap<String>>,
    /// Property [`RequestParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-requestparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_parameters: Option<::ValueMap<bool>>,
    /// Property [`RequestValidatorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-requestvalidatorid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_validator_id: Option<::Value<String>>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-resourceid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_id: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html#cfn-apigateway-method-restapiid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rest_api_id: ::Value<String>,
}

impl ::serde::Serialize for MethodProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_key_required) = self.api_key_required {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeyRequired", api_key_required)?;
        }
        if let Some(ref authorization_type) = self.authorization_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", authorization_type)?;
        }
        if let Some(ref authorizer_id) = self.authorizer_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerId", authorizer_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", &self.http_method)?;
        if let Some(ref integration) = self.integration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Integration", integration)?;
        }
        if let Some(ref method_responses) = self.method_responses {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodResponses", method_responses)?;
        }
        if let Some(ref operation_name) = self.operation_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperationName", operation_name)?;
        }
        if let Some(ref request_models) = self.request_models {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestModels", request_models)?;
        }
        if let Some(ref request_parameters) = self.request_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestParameters", request_parameters)?;
        }
        if let Some(ref request_validator_id) = self.request_validator_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestValidatorId", request_validator_id)?;
        }
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
                let mut api_key_required: Option<::Value<bool>> = None;
                let mut authorization_type: Option<::Value<String>> = None;
                let mut authorizer_id: Option<::Value<String>> = None;
                let mut http_method: Option<::Value<String>> = None;
                let mut integration: Option<::Value<self::method::Integration>> = None;
                let mut method_responses: Option<::ValueList<self::method::MethodResponse>> = None;
                let mut operation_name: Option<::Value<String>> = None;
                let mut request_models: Option<::ValueMap<String>> = None;
                let mut request_parameters: Option<::ValueMap<bool>> = None;
                let mut request_validator_id: Option<::Value<String>> = None;
                let mut resource_id: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKeyRequired" => {
                            api_key_required = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizationType" => {
                            authorization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerId" => {
                            authorizer_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HttpMethod" => {
                            http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Integration" => {
                            integration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MethodResponses" => {
                            method_responses = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OperationName" => {
                            operation_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestModels" => {
                            request_models = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestParameters" => {
                            request_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestValidatorId" => {
                            request_validator_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct ModelProperties {
    /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html#cfn-apigateway-model-contenttype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub content_type: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html#cfn-apigateway-model-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html#cfn-apigateway-model-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html#cfn-apigateway-model-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html#cfn-apigateway-model-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for ModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref content_type) = self.content_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref schema) = self.schema {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
        }
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
                let mut content_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut schema: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContentType" => {
                            content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct RequestValidatorProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html#cfn-apigateway-requestvalidator-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html#cfn-apigateway-requestvalidator-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`ValidateRequestBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html#cfn-apigateway-requestvalidator-validaterequestbody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub validate_request_body: Option<::Value<bool>>,
    /// Property [`ValidateRequestParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html#cfn-apigateway-requestvalidator-validaterequestparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub validate_request_parameters: Option<::Value<bool>>,
}

impl ::serde::Serialize for RequestValidatorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref validate_request_body) = self.validate_request_body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidateRequestBody", validate_request_body)?;
        }
        if let Some(ref validate_request_parameters) = self.validate_request_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidateRequestParameters", validate_request_parameters)?;
        }
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
                let mut name: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut validate_request_body: Option<::Value<bool>> = None;
                let mut validate_request_parameters: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidateRequestBody" => {
                            validate_request_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidateRequestParameters" => {
                            validate_request_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct ResourceProperties {
    /// Property [`ParentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html#cfn-apigateway-resource-parentid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_id: ::Value<String>,
    /// Property [`PathPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html#cfn-apigateway-resource-pathpart).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub path_part: ::Value<String>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html#cfn-apigateway-resource-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
                let mut parent_id: Option<::Value<String>> = None;
                let mut path_part: Option<::Value<String>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ParentId" => {
                            parent_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PathPart" => {
                            path_part = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct RestApiProperties {
    /// Property [`ApiKeySourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-apikeysourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key_source_type: Option<::Value<String>>,
    /// Property [`BinaryMediaTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-binarymediatypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub binary_media_types: Option<::ValueList<String>>,
    /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-body).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub body: Option<::Value<::json::Value>>,
    /// Property [`BodyS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-bodys3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub body_s3_location: Option<::Value<self::rest_api::S3Location>>,
    /// Property [`CloneFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-clonefrom).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub clone_from: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-endpointconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_configuration: Option<::Value<self::rest_api::EndpointConfiguration>>,
    /// Property [`FailOnWarnings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-failonwarnings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fail_on_warnings: Option<::Value<bool>>,
    /// Property [`MinimumCompressionSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-minimumcompressionsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub minimum_compression_size: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html#cfn-apigateway-restapi-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for RestApiProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_key_source_type) = self.api_key_source_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeySourceType", api_key_source_type)?;
        }
        if let Some(ref binary_media_types) = self.binary_media_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BinaryMediaTypes", binary_media_types)?;
        }
        if let Some(ref body) = self.body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
        }
        if let Some(ref body_s3_location) = self.body_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyS3Location", body_s3_location)?;
        }
        if let Some(ref clone_from) = self.clone_from {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloneFrom", clone_from)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref endpoint_configuration) = self.endpoint_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfiguration", endpoint_configuration)?;
        }
        if let Some(ref fail_on_warnings) = self.fail_on_warnings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailOnWarnings", fail_on_warnings)?;
        }
        if let Some(ref minimum_compression_size) = self.minimum_compression_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumCompressionSize", minimum_compression_size)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
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
                let mut api_key_source_type: Option<::Value<String>> = None;
                let mut binary_media_types: Option<::ValueList<String>> = None;
                let mut body: Option<::Value<::json::Value>> = None;
                let mut body_s3_location: Option<::Value<self::rest_api::S3Location>> = None;
                let mut clone_from: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut endpoint_configuration: Option<::Value<self::rest_api::EndpointConfiguration>> = None;
                let mut fail_on_warnings: Option<::Value<bool>> = None;
                let mut minimum_compression_size: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameters: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKeySourceType" => {
                            api_key_source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BinaryMediaTypes" => {
                            binary_media_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Body" => {
                            body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BodyS3Location" => {
                            body_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloneFrom" => {
                            clone_from = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointConfiguration" => {
                            endpoint_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FailOnWarnings" => {
                            fail_on_warnings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinimumCompressionSize" => {
                            minimum_compression_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct StageProperties {
    /// Property [`CacheClusterEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-cacheclusterenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_cluster_enabled: Option<::Value<bool>>,
    /// Property [`CacheClusterSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-cacheclustersize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cache_cluster_size: Option<::Value<String>>,
    /// Property [`ClientCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-clientcertificateid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_certificate_id: Option<::Value<String>>,
    /// Property [`DeploymentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-deploymentid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deployment_id: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-documentationversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub documentation_version: Option<::Value<String>>,
    /// Property [`MethodSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-methodsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub method_settings: Option<::ValueList<self::stage::MethodSetting>>,
    /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-restapiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rest_api_id: ::Value<String>,
    /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-stagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stage_name: Option<::Value<String>>,
    /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html#cfn-apigateway-stage-variables).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub variables: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for StageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cache_cluster_enabled) = self.cache_cluster_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterEnabled", cache_cluster_enabled)?;
        }
        if let Some(ref cache_cluster_size) = self.cache_cluster_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterSize", cache_cluster_size)?;
        }
        if let Some(ref client_certificate_id) = self.client_certificate_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateId", client_certificate_id)?;
        }
        if let Some(ref deployment_id) = self.deployment_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentId", deployment_id)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref documentation_version) = self.documentation_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentationVersion", documentation_version)?;
        }
        if let Some(ref method_settings) = self.method_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodSettings", method_settings)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", &self.rest_api_id)?;
        if let Some(ref stage_name) = self.stage_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", stage_name)?;
        }
        if let Some(ref variables) = self.variables {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
        }
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
                let mut cache_cluster_enabled: Option<::Value<bool>> = None;
                let mut cache_cluster_size: Option<::Value<String>> = None;
                let mut client_certificate_id: Option<::Value<String>> = None;
                let mut deployment_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut documentation_version: Option<::Value<String>> = None;
                let mut method_settings: Option<::ValueList<self::stage::MethodSetting>> = None;
                let mut rest_api_id: Option<::Value<String>> = None;
                let mut stage_name: Option<::Value<String>> = None;
                let mut variables: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CacheClusterEnabled" => {
                            cache_cluster_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CacheClusterSize" => {
                            cache_cluster_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientCertificateId" => {
                            client_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeploymentId" => {
                            deployment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentationVersion" => {
                            documentation_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MethodSettings" => {
                            method_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestApiId" => {
                            rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageName" => {
                            stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Variables" => {
                            variables = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct UsagePlanProperties {
    /// Property [`ApiStages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html#cfn-apigateway-usageplan-apistages).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_stages: Option<::ValueList<self::usage_plan::ApiStage>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html#cfn-apigateway-usageplan-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Quota`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html#cfn-apigateway-usageplan-quota).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quota: Option<::Value<self::usage_plan::QuotaSettings>>,
    /// Property [`Throttle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html#cfn-apigateway-usageplan-throttle).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub throttle: Option<::Value<self::usage_plan::ThrottleSettings>>,
    /// Property [`UsagePlanName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html#cfn-apigateway-usageplan-usageplanname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub usage_plan_name: Option<::Value<String>>,
}

impl ::serde::Serialize for UsagePlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_stages) = self.api_stages {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiStages", api_stages)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref quota) = self.quota {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quota", quota)?;
        }
        if let Some(ref throttle) = self.throttle {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throttle", throttle)?;
        }
        if let Some(ref usage_plan_name) = self.usage_plan_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsagePlanName", usage_plan_name)?;
        }
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
                let mut api_stages: Option<::ValueList<self::usage_plan::ApiStage>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut quota: Option<::Value<self::usage_plan::QuotaSettings>> = None;
                let mut throttle: Option<::Value<self::usage_plan::ThrottleSettings>> = None;
                let mut usage_plan_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiStages" => {
                            api_stages = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Quota" => {
                            quota = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Throttle" => {
                            throttle = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsagePlanName" => {
                            usage_plan_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct UsagePlanKeyProperties {
    /// Property [`KeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html#cfn-apigateway-usageplankey-keyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_id: ::Value<String>,
    /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html#cfn-apigateway-usageplankey-keytype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_type: ::Value<String>,
    /// Property [`UsagePlanId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html#cfn-apigateway-usageplankey-usageplanid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
                let mut key_id: Option<::Value<String>> = None;
                let mut key_type: Option<::Value<String>> = None;
                let mut usage_plan_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KeyId" => {
                            key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyType" => {
                            key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsagePlanId" => {
                            usage_plan_id = ::serde::de::MapAccess::next_value(&mut map)?;
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
#[derive(Debug, Default)]
pub struct VpcLinkProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html#cfn-apigateway-vpclink-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html#cfn-apigateway-vpclink-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`TargetArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html#cfn-apigateway-vpclink-targetarns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_arns: ::ValueList<String>,
}

impl ::serde::Serialize for VpcLinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
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
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut target_arns: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArns" => {
                            target_arns = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct StageKey {
        /// Property [`RestApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-apikey-stagekey.html#cfn-apigateway-apikey-stagekey-restapiid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rest_api_id: Option<::Value<String>>,
        /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-apikey-stagekey.html#cfn-apigateway-apikey-stagekey-stagename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stage_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StageKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rest_api_id) = self.rest_api_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestApiId", rest_api_id)?;
            }
            if let Some(ref stage_name) = self.stage_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", stage_name)?;
            }
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
                    let mut rest_api_id: Option<::Value<String>> = None;
                    let mut stage_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RestApiId" => {
                                rest_api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StageName" => {
                                stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct MethodSetting {
        /// Property [`CacheDataEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-cachedataencrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property [`CacheTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-cachettlinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property [`CachingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-cachingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property [`DataTraceEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-datatraceenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property [`HttpMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<String>>,
        /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-logginglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_level: Option<::Value<String>>,
        /// Property [`MetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-metricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property [`ResourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-resourcepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_path: Option<::Value<String>>,
        /// Property [`ThrottlingBurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-throttlingburstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property [`ThrottlingRateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription-methodsetting.html#cfn-apigateway-deployment-stagedescription-methodsetting-throttlingratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for MethodSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cache_data_encrypted) = self.cache_data_encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", cache_data_encrypted)?;
            }
            if let Some(ref cache_ttl_in_seconds) = self.cache_ttl_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", cache_ttl_in_seconds)?;
            }
            if let Some(ref caching_enabled) = self.caching_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", caching_enabled)?;
            }
            if let Some(ref data_trace_enabled) = self.data_trace_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", data_trace_enabled)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", http_method)?;
            }
            if let Some(ref logging_level) = self.logging_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
            }
            if let Some(ref metrics_enabled) = self.metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", metrics_enabled)?;
            }
            if let Some(ref resource_path) = self.resource_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", resource_path)?;
            }
            if let Some(ref throttling_burst_limit) = self.throttling_burst_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", throttling_burst_limit)?;
            }
            if let Some(ref throttling_rate_limit) = self.throttling_rate_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", throttling_rate_limit)?;
            }
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
                    let mut cache_data_encrypted: Option<::Value<bool>> = None;
                    let mut cache_ttl_in_seconds: Option<::Value<u32>> = None;
                    let mut caching_enabled: Option<::Value<bool>> = None;
                    let mut data_trace_enabled: Option<::Value<bool>> = None;
                    let mut http_method: Option<::Value<String>> = None;
                    let mut logging_level: Option<::Value<String>> = None;
                    let mut metrics_enabled: Option<::Value<bool>> = None;
                    let mut resource_path: Option<::Value<String>> = None;
                    let mut throttling_burst_limit: Option<::Value<u32>> = None;
                    let mut throttling_rate_limit: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachingEnabled" => {
                                caching_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingLevel" => {
                                logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourcePath" => {
                                resource_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct StageDescription {
        /// Property [`CacheClusterEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-cacheclusterenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_cluster_enabled: Option<::Value<bool>>,
        /// Property [`CacheClusterSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-cacheclustersize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_cluster_size: Option<::Value<String>>,
        /// Property [`CacheDataEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-cachedataencrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property [`CacheTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-cachettlinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property [`CachingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-cachingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property [`ClientCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-clientcertificateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_certificate_id: Option<::Value<String>>,
        /// Property [`DataTraceEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-datatraceenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-documentationversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub documentation_version: Option<::Value<String>>,
        /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-logginglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_level: Option<::Value<String>>,
        /// Property [`MethodSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-methodsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method_settings: Option<::ValueList<MethodSetting>>,
        /// Property [`MetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-metricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property [`ThrottlingBurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-throttlingburstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property [`ThrottlingRateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-throttlingratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_rate_limit: Option<::Value<f64>>,
        /// Property [`Variables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html#cfn-apigateway-deployment-stagedescription-variables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variables: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for StageDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cache_cluster_enabled) = self.cache_cluster_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterEnabled", cache_cluster_enabled)?;
            }
            if let Some(ref cache_cluster_size) = self.cache_cluster_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheClusterSize", cache_cluster_size)?;
            }
            if let Some(ref cache_data_encrypted) = self.cache_data_encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", cache_data_encrypted)?;
            }
            if let Some(ref cache_ttl_in_seconds) = self.cache_ttl_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", cache_ttl_in_seconds)?;
            }
            if let Some(ref caching_enabled) = self.caching_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", caching_enabled)?;
            }
            if let Some(ref client_certificate_id) = self.client_certificate_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateId", client_certificate_id)?;
            }
            if let Some(ref data_trace_enabled) = self.data_trace_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", data_trace_enabled)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref documentation_version) = self.documentation_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentationVersion", documentation_version)?;
            }
            if let Some(ref logging_level) = self.logging_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
            }
            if let Some(ref method_settings) = self.method_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MethodSettings", method_settings)?;
            }
            if let Some(ref metrics_enabled) = self.metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", metrics_enabled)?;
            }
            if let Some(ref throttling_burst_limit) = self.throttling_burst_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", throttling_burst_limit)?;
            }
            if let Some(ref throttling_rate_limit) = self.throttling_rate_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", throttling_rate_limit)?;
            }
            if let Some(ref variables) = self.variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variables", variables)?;
            }
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
                    let mut cache_cluster_enabled: Option<::Value<bool>> = None;
                    let mut cache_cluster_size: Option<::Value<String>> = None;
                    let mut cache_data_encrypted: Option<::Value<bool>> = None;
                    let mut cache_ttl_in_seconds: Option<::Value<u32>> = None;
                    let mut caching_enabled: Option<::Value<bool>> = None;
                    let mut client_certificate_id: Option<::Value<String>> = None;
                    let mut data_trace_enabled: Option<::Value<bool>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut documentation_version: Option<::Value<String>> = None;
                    let mut logging_level: Option<::Value<String>> = None;
                    let mut method_settings: Option<::ValueList<MethodSetting>> = None;
                    let mut metrics_enabled: Option<::Value<bool>> = None;
                    let mut throttling_burst_limit: Option<::Value<u32>> = None;
                    let mut throttling_rate_limit: Option<::Value<f64>> = None;
                    let mut variables: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheClusterEnabled" => {
                                cache_cluster_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheClusterSize" => {
                                cache_cluster_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachingEnabled" => {
                                caching_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientCertificateId" => {
                                client_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentationVersion" => {
                                documentation_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingLevel" => {
                                logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MethodSettings" => {
                                method_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variables" => {
                                variables = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html#cfn-apigateway-documentationpart-location-method).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub method: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html#cfn-apigateway-documentationpart-location-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html#cfn-apigateway-documentationpart-location-path).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html#cfn-apigateway-documentationpart-location-statuscode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub status_code: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html#cfn-apigateway-documentationpart-location-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref status_code) = self.status_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", status_code)?;
            }
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
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
                    let mut method: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EndpointConfiguration {
        /// Property [`Types`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html#cfn-apigateway-domainname-endpointconfiguration-types).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for EndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref types) = self.types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Types", types)?;
            }
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
                    let mut types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Types" => {
                                types = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Integration {
        /// Property [`CacheKeyParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-cachekeyparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_key_parameters: Option<::ValueList<String>>,
        /// Property [`CacheNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-cachenamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_namespace: Option<::Value<String>>,
        /// Property [`ContentHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-contenthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_handling: Option<::Value<String>>,
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: Option<::Value<String>>,
        /// Property [`IntegrationHttpMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-integrationhttpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integration_http_method: Option<::Value<String>>,
        /// Property [`IntegrationResponses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-integrationresponses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integration_responses: Option<::ValueList<IntegrationResponse>>,
        /// Property [`PassthroughBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-passthroughbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub passthrough_behavior: Option<::Value<String>>,
        /// Property [`RequestParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-requestparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_parameters: Option<::ValueMap<String>>,
        /// Property [`RequestTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-requesttemplates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_templates: Option<::ValueMap<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: Option<::Value<String>>,
        /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration.html#cfn-apigateway-method-integration-uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Integration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cache_key_parameters) = self.cache_key_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheKeyParameters", cache_key_parameters)?;
            }
            if let Some(ref cache_namespace) = self.cache_namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheNamespace", cache_namespace)?;
            }
            if let Some(ref content_handling) = self.content_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandling", content_handling)?;
            }
            if let Some(ref credentials) = self.credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
            }
            if let Some(ref integration_http_method) = self.integration_http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationHttpMethod", integration_http_method)?;
            }
            if let Some(ref integration_responses) = self.integration_responses {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationResponses", integration_responses)?;
            }
            if let Some(ref passthrough_behavior) = self.passthrough_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassthroughBehavior", passthrough_behavior)?;
            }
            if let Some(ref request_parameters) = self.request_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestParameters", request_parameters)?;
            }
            if let Some(ref request_templates) = self.request_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestTemplates", request_templates)?;
            }
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
            if let Some(ref uri) = self.uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", uri)?;
            }
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
                    let mut cache_key_parameters: Option<::ValueList<String>> = None;
                    let mut cache_namespace: Option<::Value<String>> = None;
                    let mut content_handling: Option<::Value<String>> = None;
                    let mut credentials: Option<::Value<String>> = None;
                    let mut integration_http_method: Option<::Value<String>> = None;
                    let mut integration_responses: Option<::ValueList<IntegrationResponse>> = None;
                    let mut passthrough_behavior: Option<::Value<String>> = None;
                    let mut request_parameters: Option<::ValueMap<String>> = None;
                    let mut request_templates: Option<::ValueMap<String>> = None;
                    let mut type_: Option<::Value<String>> = None;
                    let mut uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheKeyParameters" => {
                                cache_key_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheNamespace" => {
                                cache_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentHandling" => {
                                content_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegrationHttpMethod" => {
                                integration_http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegrationResponses" => {
                                integration_responses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PassthroughBehavior" => {
                                passthrough_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestParameters" => {
                                request_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestTemplates" => {
                                request_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uri" => {
                                uri = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct IntegrationResponse {
        /// Property [`ContentHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html#cfn-apigateway-method-integrationresponse-contenthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_handling: Option<::Value<String>>,
        /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html#cfn-apigateway-method-integration-integrationresponse-responseparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_parameters: Option<::ValueMap<String>>,
        /// Property [`ResponseTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html#cfn-apigateway-method-integration-integrationresponse-responsetemplates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_templates: Option<::ValueMap<String>>,
        /// Property [`SelectionPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html#cfn-apigateway-method-integration-integrationresponse-selectionpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selection_pattern: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-integration-integrationresponse.html#cfn-apigateway-method-integration-integrationresponse-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for IntegrationResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_handling) = self.content_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandling", content_handling)?;
            }
            if let Some(ref response_parameters) = self.response_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
            }
            if let Some(ref response_templates) = self.response_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTemplates", response_templates)?;
            }
            if let Some(ref selection_pattern) = self.selection_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectionPattern", selection_pattern)?;
            }
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
                    let mut content_handling: Option<::Value<String>> = None;
                    let mut response_parameters: Option<::ValueMap<String>> = None;
                    let mut response_templates: Option<::ValueMap<String>> = None;
                    let mut selection_pattern: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentHandling" => {
                                content_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseParameters" => {
                                response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseTemplates" => {
                                response_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectionPattern" => {
                                selection_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct MethodResponse {
        /// Property [`ResponseModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html#cfn-apigateway-method-methodresponse-responsemodels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_models: Option<::ValueMap<String>>,
        /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html#cfn-apigateway-method-methodresponse-responseparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_parameters: Option<::ValueMap<bool>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-method-methodresponse.html#cfn-apigateway-method-methodresponse-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for MethodResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref response_models) = self.response_models {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseModels", response_models)?;
            }
            if let Some(ref response_parameters) = self.response_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
            }
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
                    let mut response_models: Option<::ValueMap<String>> = None;
                    let mut response_parameters: Option<::ValueMap<bool>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResponseModels" => {
                                response_models = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseParameters" => {
                                response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EndpointConfiguration {
        /// Property [`Types`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html#cfn-apigateway-restapi-endpointconfiguration-types).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for EndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref types) = self.types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Types", types)?;
            }
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
                    let mut types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Types" => {
                                types = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html#cfn-apigateway-restapi-s3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`ETag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html#cfn-apigateway-restapi-s3location-etag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub e_tag: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html#cfn-apigateway-restapi-s3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html#cfn-apigateway-restapi-s3location-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            if let Some(ref e_tag) = self.e_tag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ETag", e_tag)?;
            }
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
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
                    let mut bucket: Option<::Value<String>> = None;
                    let mut e_tag: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ETag" => {
                                e_tag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct MethodSetting {
        /// Property [`CacheDataEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-cachedataencrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_data_encrypted: Option<::Value<bool>>,
        /// Property [`CacheTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-cachettlinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_ttl_in_seconds: Option<::Value<u32>>,
        /// Property [`CachingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-cachingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caching_enabled: Option<::Value<bool>>,
        /// Property [`DataTraceEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-datatraceenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property [`HttpMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<String>>,
        /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-logginglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_level: Option<::Value<String>>,
        /// Property [`MetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-metricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics_enabled: Option<::Value<bool>>,
        /// Property [`ResourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-resourcepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_path: Option<::Value<String>>,
        /// Property [`ThrottlingBurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-throttlingburstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property [`ThrottlingRateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apitgateway-stage-methodsetting.html#cfn-apigateway-stage-methodsetting-throttlingratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for MethodSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cache_data_encrypted) = self.cache_data_encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheDataEncrypted", cache_data_encrypted)?;
            }
            if let Some(ref cache_ttl_in_seconds) = self.cache_ttl_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheTtlInSeconds", cache_ttl_in_seconds)?;
            }
            if let Some(ref caching_enabled) = self.caching_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingEnabled", caching_enabled)?;
            }
            if let Some(ref data_trace_enabled) = self.data_trace_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", data_trace_enabled)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", http_method)?;
            }
            if let Some(ref logging_level) = self.logging_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
            }
            if let Some(ref metrics_enabled) = self.metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsEnabled", metrics_enabled)?;
            }
            if let Some(ref resource_path) = self.resource_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", resource_path)?;
            }
            if let Some(ref throttling_burst_limit) = self.throttling_burst_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingBurstLimit", throttling_burst_limit)?;
            }
            if let Some(ref throttling_rate_limit) = self.throttling_rate_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThrottlingRateLimit", throttling_rate_limit)?;
            }
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
                    let mut cache_data_encrypted: Option<::Value<bool>> = None;
                    let mut cache_ttl_in_seconds: Option<::Value<u32>> = None;
                    let mut caching_enabled: Option<::Value<bool>> = None;
                    let mut data_trace_enabled: Option<::Value<bool>> = None;
                    let mut http_method: Option<::Value<String>> = None;
                    let mut logging_level: Option<::Value<String>> = None;
                    let mut metrics_enabled: Option<::Value<bool>> = None;
                    let mut resource_path: Option<::Value<String>> = None;
                    let mut throttling_burst_limit: Option<::Value<u32>> = None;
                    let mut throttling_rate_limit: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CacheDataEncrypted" => {
                                cache_data_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheTtlInSeconds" => {
                                cache_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachingEnabled" => {
                                caching_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTraceEnabled" => {
                                data_trace_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingLevel" => {
                                logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricsEnabled" => {
                                metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourcePath" => {
                                resource_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingBurstLimit" => {
                                throttling_burst_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThrottlingRateLimit" => {
                                throttling_rate_limit = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ApiStage {
        /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html#cfn-apigateway-usageplan-apistage-apiid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_id: Option<::Value<String>>,
        /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html#cfn-apigateway-usageplan-apistage-stage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stage: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApiStage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref api_id) = self.api_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", api_id)?;
            }
            if let Some(ref stage) = self.stage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", stage)?;
            }
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
                    let mut api_id: Option<::Value<String>> = None;
                    let mut stage: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiId" => {
                                api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Stage" => {
                                stage = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct QuotaSettings {
        /// Property [`Limit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html#cfn-apigateway-usageplan-quotasettings-limit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub limit: Option<::Value<u32>>,
        /// Property [`Offset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html#cfn-apigateway-usageplan-quotasettings-offset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset: Option<::Value<u32>>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html#cfn-apigateway-usageplan-quotasettings-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for QuotaSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref limit) = self.limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limit", limit)?;
            }
            if let Some(ref offset) = self.offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", offset)?;
            }
            if let Some(ref period) = self.period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", period)?;
            }
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
                    let mut limit: Option<::Value<u32>> = None;
                    let mut offset: Option<::Value<u32>> = None;
                    let mut period: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Limit" => {
                                limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Offset" => {
                                offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ThrottleSettings {
        /// Property [`BurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html#cfn-apigateway-usageplan-throttlesettings-burstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub burst_limit: Option<::Value<u32>>,
        /// Property [`RateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html#cfn-apigateway-usageplan-throttlesettings-ratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ThrottleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref burst_limit) = self.burst_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BurstLimit", burst_limit)?;
            }
            if let Some(ref rate_limit) = self.rate_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateLimit", rate_limit)?;
            }
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
                    let mut burst_limit: Option<::Value<u32>> = None;
                    let mut rate_limit: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BurstLimit" => {
                                burst_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateLimit" => {
                                rate_limit = ::serde::de::MapAccess::next_value(&mut map)?;
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
