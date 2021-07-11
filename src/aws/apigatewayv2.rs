//! Types for the `ApiGatewayV2` service.

/// The [`AWS::ApiGatewayV2::Api`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html) resource type.
#[derive(Debug, Default)]
pub struct Api {
    properties: ApiProperties
}

/// Properties for the `Api` resource.
#[derive(Debug, Default)]
pub struct ApiProperties {
    /// Property [`ApiKeySelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-apikeyselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key_selection_expression: Option<::Value<String>>,
    /// Property [`BasePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-basepath).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub base_path: Option<::Value<String>>,
    /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-body).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub body: Option<::Value<::json::Value>>,
    /// Property [`BodyS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-bodys3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub body_s3_location: Option<::Value<self::api::BodyS3Location>>,
    /// Property [`CorsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-corsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cors_configuration: Option<::Value<self::api::Cors>>,
    /// Property [`CredentialsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-credentialsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub credentials_arn: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisableExecuteApiEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-disableexecuteapiendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disable_execute_api_endpoint: Option<::Value<bool>>,
    /// Property [`DisableSchemaValidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-disableschemavalidation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disable_schema_validation: Option<::Value<bool>>,
    /// Property [`FailOnWarnings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-failonwarnings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fail_on_warnings: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ProtocolType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-protocoltype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol_type: Option<::Value<String>>,
    /// Property [`RouteKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-routekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_key: Option<::Value<String>>,
    /// Property [`RouteSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-routeselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_selection_expression: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-target).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target: Option<::Value<String>>,
    /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html#cfn-apigatewayv2-api-version).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version: Option<::Value<String>>,
}

impl ::serde::Serialize for ApiProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_key_selection_expression) = self.api_key_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeySelectionExpression", api_key_selection_expression)?;
        }
        if let Some(ref base_path) = self.base_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasePath", base_path)?;
        }
        if let Some(ref body) = self.body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
        }
        if let Some(ref body_s3_location) = self.body_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyS3Location", body_s3_location)?;
        }
        if let Some(ref cors_configuration) = self.cors_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorsConfiguration", cors_configuration)?;
        }
        if let Some(ref credentials_arn) = self.credentials_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialsArn", credentials_arn)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref disable_execute_api_endpoint) = self.disable_execute_api_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableExecuteApiEndpoint", disable_execute_api_endpoint)?;
        }
        if let Some(ref disable_schema_validation) = self.disable_schema_validation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableSchemaValidation", disable_schema_validation)?;
        }
        if let Some(ref fail_on_warnings) = self.fail_on_warnings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailOnWarnings", fail_on_warnings)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref protocol_type) = self.protocol_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolType", protocol_type)?;
        }
        if let Some(ref route_key) = self.route_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteKey", route_key)?;
        }
        if let Some(ref route_selection_expression) = self.route_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteSelectionExpression", route_selection_expression)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target) = self.target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
        }
        if let Some(ref version) = self.version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_key_selection_expression: Option<::Value<String>> = None;
                let mut base_path: Option<::Value<String>> = None;
                let mut body: Option<::Value<::json::Value>> = None;
                let mut body_s3_location: Option<::Value<self::api::BodyS3Location>> = None;
                let mut cors_configuration: Option<::Value<self::api::Cors>> = None;
                let mut credentials_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut disable_execute_api_endpoint: Option<::Value<bool>> = None;
                let mut disable_schema_validation: Option<::Value<bool>> = None;
                let mut fail_on_warnings: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut protocol_type: Option<::Value<String>> = None;
                let mut route_key: Option<::Value<String>> = None;
                let mut route_selection_expression: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut target: Option<::Value<String>> = None;
                let mut version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKeySelectionExpression" => {
                            api_key_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BasePath" => {
                            base_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Body" => {
                            body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BodyS3Location" => {
                            body_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CorsConfiguration" => {
                            cors_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CredentialsArn" => {
                            credentials_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisableExecuteApiEndpoint" => {
                            disable_execute_api_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisableSchemaValidation" => {
                            disable_schema_validation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FailOnWarnings" => {
                            fail_on_warnings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtocolType" => {
                            protocol_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteKey" => {
                            route_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteSelectionExpression" => {
                            route_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Target" => {
                            target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Version" => {
                            version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiProperties {
                    api_key_selection_expression: api_key_selection_expression,
                    base_path: base_path,
                    body: body,
                    body_s3_location: body_s3_location,
                    cors_configuration: cors_configuration,
                    credentials_arn: credentials_arn,
                    description: description,
                    disable_execute_api_endpoint: disable_execute_api_endpoint,
                    disable_schema_validation: disable_schema_validation,
                    fail_on_warnings: fail_on_warnings,
                    name: name,
                    protocol_type: protocol_type,
                    route_key: route_key,
                    route_selection_expression: route_selection_expression,
                    tags: tags,
                    target: target,
                    version: version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Api {
    type Properties = ApiProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Api";
    fn properties(&self) -> &ApiProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Api {}

impl From<ApiProperties> for Api {
    fn from(properties: ApiProperties) -> Api {
        Api { properties }
    }
}

/// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html) resource type.
#[derive(Debug, Default)]
pub struct ApiGatewayManagedOverrides {
    properties: ApiGatewayManagedOverridesProperties
}

/// Properties for the `ApiGatewayManagedOverrides` resource.
#[derive(Debug, Default)]
pub struct ApiGatewayManagedOverridesProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-integration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration: Option<::Value<self::api_gateway_managed_overrides::IntegrationOverrides>>,
    /// Property [`Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-route).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route: Option<::Value<self::api_gateway_managed_overrides::RouteOverrides>>,
    /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage: Option<::Value<self::api_gateway_managed_overrides::StageOverrides>>,
}

impl ::serde::Serialize for ApiGatewayManagedOverridesProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref integration) = self.integration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Integration", integration)?;
        }
        if let Some(ref route) = self.route {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Route", route)?;
        }
        if let Some(ref stage) = self.stage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", stage)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiGatewayManagedOverridesProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiGatewayManagedOverridesProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiGatewayManagedOverridesProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiGatewayManagedOverridesProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut integration: Option<::Value<self::api_gateway_managed_overrides::IntegrationOverrides>> = None;
                let mut route: Option<::Value<self::api_gateway_managed_overrides::RouteOverrides>> = None;
                let mut stage: Option<::Value<self::api_gateway_managed_overrides::StageOverrides>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Integration" => {
                            integration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Route" => {
                            route = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stage" => {
                            stage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiGatewayManagedOverridesProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    integration: integration,
                    route: route,
                    stage: stage,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiGatewayManagedOverrides {
    type Properties = ApiGatewayManagedOverridesProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::ApiGatewayManagedOverrides";
    fn properties(&self) -> &ApiGatewayManagedOverridesProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiGatewayManagedOverridesProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiGatewayManagedOverrides {}

impl From<ApiGatewayManagedOverridesProperties> for ApiGatewayManagedOverrides {
    fn from(properties: ApiGatewayManagedOverridesProperties) -> ApiGatewayManagedOverrides {
        ApiGatewayManagedOverrides { properties }
    }
}

/// The [`AWS::ApiGatewayV2::ApiMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html) resource type.
#[derive(Debug, Default)]
pub struct ApiMapping {
    properties: ApiMappingProperties
}

/// Properties for the `ApiMapping` resource.
#[derive(Debug, Default)]
pub struct ApiMappingProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html#cfn-apigatewayv2-apimapping-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ApiMappingKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html#cfn-apigatewayv2-apimapping-apimappingkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_mapping_key: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html#cfn-apigatewayv2-apimapping-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html#cfn-apigatewayv2-apimapping-stage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage: ::Value<String>,
}

impl ::serde::Serialize for ApiMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref api_mapping_key) = self.api_mapping_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiMappingKey", api_mapping_key)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", &self.stage)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiMappingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiMappingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiMappingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiMappingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut api_mapping_key: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut stage: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApiMappingKey" => {
                            api_mapping_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stage" => {
                            stage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiMappingProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    api_mapping_key: api_mapping_key,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    stage: stage.ok_or(::serde::de::Error::missing_field("Stage"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiMapping {
    type Properties = ApiMappingProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::ApiMapping";
    fn properties(&self) -> &ApiMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiMappingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiMapping {}

impl From<ApiMappingProperties> for ApiMapping {
    fn from(properties: ApiMappingProperties) -> ApiMapping {
        ApiMapping { properties }
    }
}

/// The [`AWS::ApiGatewayV2::Authorizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html) resource type.
#[derive(Debug, Default)]
pub struct Authorizer {
    properties: AuthorizerProperties
}

/// Properties for the `Authorizer` resource.
#[derive(Debug, Default)]
pub struct AuthorizerProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`AuthorizerCredentialsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-authorizercredentialsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_credentials_arn: Option<::Value<String>>,
    /// Property [`AuthorizerPayloadFormatVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-authorizerpayloadformatversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_payload_format_version: Option<::Value<String>>,
    /// Property [`AuthorizerResultTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-authorizerresultttlinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_result_ttl_in_seconds: Option<::Value<u32>>,
    /// Property [`AuthorizerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-authorizertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_type: ::Value<String>,
    /// Property [`AuthorizerUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-authorizeruri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_uri: Option<::Value<String>>,
    /// Property [`EnableSimpleResponses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-enablesimpleresponses).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_simple_responses: Option<::Value<bool>>,
    /// Property [`IdentitySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-identitysource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_source: Option<::ValueList<String>>,
    /// Property [`IdentityValidationExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-identityvalidationexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_validation_expression: Option<::Value<String>>,
    /// Property [`JwtConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-jwtconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub jwt_configuration: Option<::Value<self::authorizer::JWTConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html#cfn-apigatewayv2-authorizer-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for AuthorizerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref authorizer_credentials_arn) = self.authorizer_credentials_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerCredentialsArn", authorizer_credentials_arn)?;
        }
        if let Some(ref authorizer_payload_format_version) = self.authorizer_payload_format_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerPayloadFormatVersion", authorizer_payload_format_version)?;
        }
        if let Some(ref authorizer_result_ttl_in_seconds) = self.authorizer_result_ttl_in_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerResultTtlInSeconds", authorizer_result_ttl_in_seconds)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerType", &self.authorizer_type)?;
        if let Some(ref authorizer_uri) = self.authorizer_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerUri", authorizer_uri)?;
        }
        if let Some(ref enable_simple_responses) = self.enable_simple_responses {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSimpleResponses", enable_simple_responses)?;
        }
        if let Some(ref identity_source) = self.identity_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentitySource", identity_source)?;
        }
        if let Some(ref identity_validation_expression) = self.identity_validation_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityValidationExpression", identity_validation_expression)?;
        }
        if let Some(ref jwt_configuration) = self.jwt_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JwtConfiguration", jwt_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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
                let mut api_id: Option<::Value<String>> = None;
                let mut authorizer_credentials_arn: Option<::Value<String>> = None;
                let mut authorizer_payload_format_version: Option<::Value<String>> = None;
                let mut authorizer_result_ttl_in_seconds: Option<::Value<u32>> = None;
                let mut authorizer_type: Option<::Value<String>> = None;
                let mut authorizer_uri: Option<::Value<String>> = None;
                let mut enable_simple_responses: Option<::Value<bool>> = None;
                let mut identity_source: Option<::ValueList<String>> = None;
                let mut identity_validation_expression: Option<::Value<String>> = None;
                let mut jwt_configuration: Option<::Value<self::authorizer::JWTConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerCredentialsArn" => {
                            authorizer_credentials_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerPayloadFormatVersion" => {
                            authorizer_payload_format_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerResultTtlInSeconds" => {
                            authorizer_result_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerType" => {
                            authorizer_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerUri" => {
                            authorizer_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableSimpleResponses" => {
                            enable_simple_responses = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentitySource" => {
                            identity_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityValidationExpression" => {
                            identity_validation_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JwtConfiguration" => {
                            jwt_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AuthorizerProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    authorizer_credentials_arn: authorizer_credentials_arn,
                    authorizer_payload_format_version: authorizer_payload_format_version,
                    authorizer_result_ttl_in_seconds: authorizer_result_ttl_in_seconds,
                    authorizer_type: authorizer_type.ok_or(::serde::de::Error::missing_field("AuthorizerType"))?,
                    authorizer_uri: authorizer_uri,
                    enable_simple_responses: enable_simple_responses,
                    identity_source: identity_source,
                    identity_validation_expression: identity_validation_expression,
                    jwt_configuration: jwt_configuration,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Authorizer {
    type Properties = AuthorizerProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Authorizer";
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

/// The [`AWS::ApiGatewayV2::Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-deployment.html) resource type.
#[derive(Debug, Default)]
pub struct Deployment {
    properties: DeploymentProperties
}

/// Properties for the `Deployment` resource.
#[derive(Debug, Default)]
pub struct DeploymentProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-deployment.html#cfn-apigatewayv2-deployment-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-deployment.html#cfn-apigatewayv2-deployment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-deployment.html#cfn-apigatewayv2-deployment-stagename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage_name: Option<::Value<String>>,
}

impl ::serde::Serialize for DeploymentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
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
                let mut api_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut stage_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageName" => {
                            stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeploymentProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    description: description,
                    stage_name: stage_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Deployment {
    type Properties = DeploymentProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Deployment";
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

/// The [`AWS::ApiGatewayV2::DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html) resource type.
#[derive(Debug, Default)]
pub struct DomainName {
    properties: DomainNameProperties
}

/// Properties for the `DomainName` resource.
#[derive(Debug, Default)]
pub struct DomainNameProperties {
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html#cfn-apigatewayv2-domainname-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`DomainNameConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html#cfn-apigatewayv2-domainname-domainnameconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_name_configurations: Option<::ValueList<self::domain_name::DomainNameConfiguration>>,
    /// Property [`MutualTlsAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html#cfn-apigatewayv2-domainname-mutualtlsauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mutual_tls_authentication: Option<::Value<self::domain_name::MutualTlsAuthentication>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html#cfn-apigatewayv2-domainname-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for DomainNameProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref domain_name_configurations) = self.domain_name_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainNameConfigurations", domain_name_configurations)?;
        }
        if let Some(ref mutual_tls_authentication) = self.mutual_tls_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MutualTlsAuthentication", mutual_tls_authentication)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut domain_name: Option<::Value<String>> = None;
                let mut domain_name_configurations: Option<::ValueList<self::domain_name::DomainNameConfiguration>> = None;
                let mut mutual_tls_authentication: Option<::Value<self::domain_name::MutualTlsAuthentication>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainNameConfigurations" => {
                            domain_name_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MutualTlsAuthentication" => {
                            mutual_tls_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainNameProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    domain_name_configurations: domain_name_configurations,
                    mutual_tls_authentication: mutual_tls_authentication,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DomainName {
    type Properties = DomainNameProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::DomainName";
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

/// The [`AWS::ApiGatewayV2::Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html) resource type.
#[derive(Debug, Default)]
pub struct Integration {
    properties: IntegrationProperties
}

/// Properties for the `Integration` resource.
#[derive(Debug, Default)]
pub struct IntegrationProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ConnectionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-connectionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_id: Option<::Value<String>>,
    /// Property [`ConnectionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-connectiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_type: Option<::Value<String>>,
    /// Property [`ContentHandlingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-contenthandlingstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_handling_strategy: Option<::Value<String>>,
    /// Property [`CredentialsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-credentialsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub credentials_arn: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IntegrationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-integrationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_method: Option<::Value<String>>,
    /// Property [`IntegrationSubtype`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-integrationsubtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_subtype: Option<::Value<String>>,
    /// Property [`IntegrationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-integrationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_type: ::Value<String>,
    /// Property [`IntegrationUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-integrationuri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_uri: Option<::Value<String>>,
    /// Property [`PassthroughBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-passthroughbehavior).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub passthrough_behavior: Option<::Value<String>>,
    /// Property [`PayloadFormatVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-payloadformatversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub payload_format_version: Option<::Value<String>>,
    /// Property [`RequestParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-requestparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_parameters: Option<::Value<::json::Value>>,
    /// Property [`RequestTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-requesttemplates).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_templates: Option<::Value<::json::Value>>,
    /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-responseparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_parameters: Option<::Value<::json::Value>>,
    /// Property [`TemplateSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-templateselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_selection_expression: Option<::Value<String>>,
    /// Property [`TimeoutInMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-timeoutinmillis).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout_in_millis: Option<::Value<u32>>,
    /// Property [`TlsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html#cfn-apigatewayv2-integration-tlsconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tls_config: Option<::Value<self::integration::TlsConfig>>,
}

impl ::serde::Serialize for IntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref connection_id) = self.connection_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionId", connection_id)?;
        }
        if let Some(ref connection_type) = self.connection_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionType", connection_type)?;
        }
        if let Some(ref content_handling_strategy) = self.content_handling_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandlingStrategy", content_handling_strategy)?;
        }
        if let Some(ref credentials_arn) = self.credentials_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialsArn", credentials_arn)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref integration_method) = self.integration_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationMethod", integration_method)?;
        }
        if let Some(ref integration_subtype) = self.integration_subtype {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationSubtype", integration_subtype)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationType", &self.integration_type)?;
        if let Some(ref integration_uri) = self.integration_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationUri", integration_uri)?;
        }
        if let Some(ref passthrough_behavior) = self.passthrough_behavior {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassthroughBehavior", passthrough_behavior)?;
        }
        if let Some(ref payload_format_version) = self.payload_format_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadFormatVersion", payload_format_version)?;
        }
        if let Some(ref request_parameters) = self.request_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestParameters", request_parameters)?;
        }
        if let Some(ref request_templates) = self.request_templates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestTemplates", request_templates)?;
        }
        if let Some(ref response_parameters) = self.response_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
        }
        if let Some(ref template_selection_expression) = self.template_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateSelectionExpression", template_selection_expression)?;
        }
        if let Some(ref timeout_in_millis) = self.timeout_in_millis {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMillis", timeout_in_millis)?;
        }
        if let Some(ref tls_config) = self.tls_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsConfig", tls_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut connection_id: Option<::Value<String>> = None;
                let mut connection_type: Option<::Value<String>> = None;
                let mut content_handling_strategy: Option<::Value<String>> = None;
                let mut credentials_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut integration_method: Option<::Value<String>> = None;
                let mut integration_subtype: Option<::Value<String>> = None;
                let mut integration_type: Option<::Value<String>> = None;
                let mut integration_uri: Option<::Value<String>> = None;
                let mut passthrough_behavior: Option<::Value<String>> = None;
                let mut payload_format_version: Option<::Value<String>> = None;
                let mut request_parameters: Option<::Value<::json::Value>> = None;
                let mut request_templates: Option<::Value<::json::Value>> = None;
                let mut response_parameters: Option<::Value<::json::Value>> = None;
                let mut template_selection_expression: Option<::Value<String>> = None;
                let mut timeout_in_millis: Option<::Value<u32>> = None;
                let mut tls_config: Option<::Value<self::integration::TlsConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionId" => {
                            connection_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionType" => {
                            connection_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContentHandlingStrategy" => {
                            content_handling_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CredentialsArn" => {
                            credentials_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationMethod" => {
                            integration_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationSubtype" => {
                            integration_subtype = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationType" => {
                            integration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationUri" => {
                            integration_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PassthroughBehavior" => {
                            passthrough_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PayloadFormatVersion" => {
                            payload_format_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestParameters" => {
                            request_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestTemplates" => {
                            request_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseParameters" => {
                            response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateSelectionExpression" => {
                            template_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeoutInMillis" => {
                            timeout_in_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TlsConfig" => {
                            tls_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    connection_id: connection_id,
                    connection_type: connection_type,
                    content_handling_strategy: content_handling_strategy,
                    credentials_arn: credentials_arn,
                    description: description,
                    integration_method: integration_method,
                    integration_subtype: integration_subtype,
                    integration_type: integration_type.ok_or(::serde::de::Error::missing_field("IntegrationType"))?,
                    integration_uri: integration_uri,
                    passthrough_behavior: passthrough_behavior,
                    payload_format_version: payload_format_version,
                    request_parameters: request_parameters,
                    request_templates: request_templates,
                    response_parameters: response_parameters,
                    template_selection_expression: template_selection_expression,
                    timeout_in_millis: timeout_in_millis,
                    tls_config: tls_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Integration {
    type Properties = IntegrationProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Integration";
    fn properties(&self) -> &IntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Integration {}

impl From<IntegrationProperties> for Integration {
    fn from(properties: IntegrationProperties) -> Integration {
        Integration { properties }
    }
}

/// The [`AWS::ApiGatewayV2::IntegrationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html) resource type.
#[derive(Debug, Default)]
pub struct IntegrationResponse {
    properties: IntegrationResponseProperties
}

/// Properties for the `IntegrationResponse` resource.
#[derive(Debug, Default)]
pub struct IntegrationResponseProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-apiid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ContentHandlingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-contenthandlingstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_handling_strategy: Option<::Value<String>>,
    /// Property [`IntegrationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-integrationid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_id: ::Value<String>,
    /// Property [`IntegrationResponseKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-integrationresponsekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub integration_response_key: ::Value<String>,
    /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-responseparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_parameters: Option<::Value<::json::Value>>,
    /// Property [`ResponseTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-responsetemplates).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_templates: Option<::Value<::json::Value>>,
    /// Property [`TemplateSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html#cfn-apigatewayv2-integrationresponse-templateselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_selection_expression: Option<::Value<String>>,
}

impl ::serde::Serialize for IntegrationResponseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref content_handling_strategy) = self.content_handling_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentHandlingStrategy", content_handling_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationId", &self.integration_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationResponseKey", &self.integration_response_key)?;
        if let Some(ref response_parameters) = self.response_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
        }
        if let Some(ref response_templates) = self.response_templates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTemplates", response_templates)?;
        }
        if let Some(ref template_selection_expression) = self.template_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateSelectionExpression", template_selection_expression)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationResponseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationResponseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationResponseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationResponseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut content_handling_strategy: Option<::Value<String>> = None;
                let mut integration_id: Option<::Value<String>> = None;
                let mut integration_response_key: Option<::Value<String>> = None;
                let mut response_parameters: Option<::Value<::json::Value>> = None;
                let mut response_templates: Option<::Value<::json::Value>> = None;
                let mut template_selection_expression: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContentHandlingStrategy" => {
                            content_handling_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationId" => {
                            integration_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationResponseKey" => {
                            integration_response_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseParameters" => {
                            response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseTemplates" => {
                            response_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateSelectionExpression" => {
                            template_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationResponseProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    content_handling_strategy: content_handling_strategy,
                    integration_id: integration_id.ok_or(::serde::de::Error::missing_field("IntegrationId"))?,
                    integration_response_key: integration_response_key.ok_or(::serde::de::Error::missing_field("IntegrationResponseKey"))?,
                    response_parameters: response_parameters,
                    response_templates: response_templates,
                    template_selection_expression: template_selection_expression,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IntegrationResponse {
    type Properties = IntegrationResponseProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::IntegrationResponse";
    fn properties(&self) -> &IntegrationResponseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationResponseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IntegrationResponse {}

impl From<IntegrationResponseProperties> for IntegrationResponse {
    fn from(properties: IntegrationResponseProperties) -> IntegrationResponse {
        IntegrationResponse { properties }
    }
}

/// The [`AWS::ApiGatewayV2::Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html) resource type.
#[derive(Debug, Default)]
pub struct Model {
    properties: ModelProperties
}

/// Properties for the `Model` resource.
#[derive(Debug, Default)]
pub struct ModelProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html#cfn-apigatewayv2-model-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html#cfn-apigatewayv2-model-contenttype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content_type: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html#cfn-apigatewayv2-model-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html#cfn-apigatewayv2-model-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html#cfn-apigatewayv2-model-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: ::Value<::json::Value>,
}

impl ::serde::Serialize for ModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref content_type) = self.content_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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
                let mut api_id: Option<::Value<String>> = None;
                let mut content_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schema: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContentType" => {
                            content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    content_type: content_type,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schema: schema.ok_or(::serde::de::Error::missing_field("Schema"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Model {
    type Properties = ModelProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Model";
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

/// The [`AWS::ApiGatewayV2::Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html) resource type.
#[derive(Debug, Default)]
pub struct Route {
    properties: RouteProperties
}

/// Properties for the `Route` resource.
#[derive(Debug, Default)]
pub struct RouteProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ApiKeyRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-apikeyrequired).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key_required: Option<::Value<bool>>,
    /// Property [`AuthorizationScopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-authorizationscopes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization_scopes: Option<::ValueList<String>>,
    /// Property [`AuthorizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-authorizationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization_type: Option<::Value<String>>,
    /// Property [`AuthorizerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-authorizerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_id: Option<::Value<String>>,
    /// Property [`ModelSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-modelselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_selection_expression: Option<::Value<String>>,
    /// Property [`OperationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-operationname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub operation_name: Option<::Value<String>>,
    /// Property [`RequestModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-requestmodels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_models: Option<::Value<::json::Value>>,
    /// Property [`RequestParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-requestparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_parameters: Option<::Value<::json::Value>>,
    /// Property [`RouteKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-routekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_key: ::Value<String>,
    /// Property [`RouteResponseSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-routeresponseselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_response_selection_expression: Option<::Value<String>>,
    /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html#cfn-apigatewayv2-route-target).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target: Option<::Value<String>>,
}

impl ::serde::Serialize for RouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref api_key_required) = self.api_key_required {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeyRequired", api_key_required)?;
        }
        if let Some(ref authorization_scopes) = self.authorization_scopes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationScopes", authorization_scopes)?;
        }
        if let Some(ref authorization_type) = self.authorization_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", authorization_type)?;
        }
        if let Some(ref authorizer_id) = self.authorizer_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerId", authorizer_id)?;
        }
        if let Some(ref model_selection_expression) = self.model_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelSelectionExpression", model_selection_expression)?;
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
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteKey", &self.route_key)?;
        if let Some(ref route_response_selection_expression) = self.route_response_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteResponseSelectionExpression", route_response_selection_expression)?;
        }
        if let Some(ref target) = self.target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut api_key_required: Option<::Value<bool>> = None;
                let mut authorization_scopes: Option<::ValueList<String>> = None;
                let mut authorization_type: Option<::Value<String>> = None;
                let mut authorizer_id: Option<::Value<String>> = None;
                let mut model_selection_expression: Option<::Value<String>> = None;
                let mut operation_name: Option<::Value<String>> = None;
                let mut request_models: Option<::Value<::json::Value>> = None;
                let mut request_parameters: Option<::Value<::json::Value>> = None;
                let mut route_key: Option<::Value<String>> = None;
                let mut route_response_selection_expression: Option<::Value<String>> = None;
                let mut target: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApiKeyRequired" => {
                            api_key_required = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizationScopes" => {
                            authorization_scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizationType" => {
                            authorization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerId" => {
                            authorizer_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelSelectionExpression" => {
                            model_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "RouteKey" => {
                            route_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteResponseSelectionExpression" => {
                            route_response_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Target" => {
                            target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RouteProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    api_key_required: api_key_required,
                    authorization_scopes: authorization_scopes,
                    authorization_type: authorization_type,
                    authorizer_id: authorizer_id,
                    model_selection_expression: model_selection_expression,
                    operation_name: operation_name,
                    request_models: request_models,
                    request_parameters: request_parameters,
                    route_key: route_key.ok_or(::serde::de::Error::missing_field("RouteKey"))?,
                    route_response_selection_expression: route_response_selection_expression,
                    target: target,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Route {
    type Properties = RouteProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Route";
    fn properties(&self) -> &RouteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Route {}

impl From<RouteProperties> for Route {
    fn from(properties: RouteProperties) -> Route {
        Route { properties }
    }
}

/// The [`AWS::ApiGatewayV2::RouteResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html) resource type.
#[derive(Debug, Default)]
pub struct RouteResponse {
    properties: RouteResponseProperties
}

/// Properties for the `RouteResponse` resource.
#[derive(Debug, Default)]
pub struct RouteResponseProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ModelSelectionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-modelselectionexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_selection_expression: Option<::Value<String>>,
    /// Property [`ResponseModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-responsemodels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_models: Option<::Value<::json::Value>>,
    /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-responseparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_parameters: Option<::Value<::json::Value>>,
    /// Property [`RouteId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-routeid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub route_id: ::Value<String>,
    /// Property [`RouteResponseKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html#cfn-apigatewayv2-routeresponse-routeresponsekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_response_key: ::Value<String>,
}

impl ::serde::Serialize for RouteResponseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref model_selection_expression) = self.model_selection_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelSelectionExpression", model_selection_expression)?;
        }
        if let Some(ref response_models) = self.response_models {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseModels", response_models)?;
        }
        if let Some(ref response_parameters) = self.response_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteId", &self.route_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteResponseKey", &self.route_response_key)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RouteResponseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteResponseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RouteResponseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RouteResponseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut model_selection_expression: Option<::Value<String>> = None;
                let mut response_models: Option<::Value<::json::Value>> = None;
                let mut response_parameters: Option<::Value<::json::Value>> = None;
                let mut route_id: Option<::Value<String>> = None;
                let mut route_response_key: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelSelectionExpression" => {
                            model_selection_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseModels" => {
                            response_models = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseParameters" => {
                            response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteId" => {
                            route_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteResponseKey" => {
                            route_response_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RouteResponseProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    model_selection_expression: model_selection_expression,
                    response_models: response_models,
                    response_parameters: response_parameters,
                    route_id: route_id.ok_or(::serde::de::Error::missing_field("RouteId"))?,
                    route_response_key: route_response_key.ok_or(::serde::de::Error::missing_field("RouteResponseKey"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RouteResponse {
    type Properties = RouteResponseProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::RouteResponse";
    fn properties(&self) -> &RouteResponseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RouteResponseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RouteResponse {}

impl From<RouteResponseProperties> for RouteResponse {
    fn from(properties: RouteResponseProperties) -> RouteResponse {
        RouteResponse { properties }
    }
}

/// The [`AWS::ApiGatewayV2::Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html) resource type.
#[derive(Debug, Default)]
pub struct Stage {
    properties: StageProperties
}

/// Properties for the `Stage` resource.
#[derive(Debug, Default)]
pub struct StageProperties {
    /// Property [`AccessLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-accesslogsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_log_settings: Option<::Value<self::stage::AccessLogSettings>>,
    /// Property [`AccessPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-accesspolicyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_policy_id: Option<::Value<String>>,
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`AutoDeploy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-autodeploy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_deploy: Option<::Value<bool>>,
    /// Property [`ClientCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-clientcertificateid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_certificate_id: Option<::Value<String>>,
    /// Property [`DefaultRouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-defaultroutesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_route_settings: Option<::Value<self::stage::RouteSettings>>,
    /// Property [`DeploymentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-deploymentid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deployment_id: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`RouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-routesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub route_settings: Option<::Value<::json::Value>>,
    /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-stagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stage_name: ::Value<String>,
    /// Property [`StageVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-stagevariables).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stage_variables: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html#cfn-apigatewayv2-stage-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for StageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_log_settings) = self.access_log_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLogSettings", access_log_settings)?;
        }
        if let Some(ref access_policy_id) = self.access_policy_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPolicyId", access_policy_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref auto_deploy) = self.auto_deploy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoDeploy", auto_deploy)?;
        }
        if let Some(ref client_certificate_id) = self.client_certificate_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCertificateId", client_certificate_id)?;
        }
        if let Some(ref default_route_settings) = self.default_route_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRouteSettings", default_route_settings)?;
        }
        if let Some(ref deployment_id) = self.deployment_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentId", deployment_id)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref route_settings) = self.route_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteSettings", route_settings)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", &self.stage_name)?;
        if let Some(ref stage_variables) = self.stage_variables {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageVariables", stage_variables)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut access_log_settings: Option<::Value<self::stage::AccessLogSettings>> = None;
                let mut access_policy_id: Option<::Value<String>> = None;
                let mut api_id: Option<::Value<String>> = None;
                let mut auto_deploy: Option<::Value<bool>> = None;
                let mut client_certificate_id: Option<::Value<String>> = None;
                let mut default_route_settings: Option<::Value<self::stage::RouteSettings>> = None;
                let mut deployment_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut route_settings: Option<::Value<::json::Value>> = None;
                let mut stage_name: Option<::Value<String>> = None;
                let mut stage_variables: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessLogSettings" => {
                            access_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccessPolicyId" => {
                            access_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoDeploy" => {
                            auto_deploy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientCertificateId" => {
                            client_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultRouteSettings" => {
                            default_route_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeploymentId" => {
                            deployment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteSettings" => {
                            route_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageName" => {
                            stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StageVariables" => {
                            stage_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StageProperties {
                    access_log_settings: access_log_settings,
                    access_policy_id: access_policy_id,
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    auto_deploy: auto_deploy,
                    client_certificate_id: client_certificate_id,
                    default_route_settings: default_route_settings,
                    deployment_id: deployment_id,
                    description: description,
                    route_settings: route_settings,
                    stage_name: stage_name.ok_or(::serde::de::Error::missing_field("StageName"))?,
                    stage_variables: stage_variables,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Stage {
    type Properties = StageProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::Stage";
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

/// The [`AWS::ApiGatewayV2::VpcLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html) resource type.
#[derive(Debug, Default)]
pub struct VpcLink {
    properties: VpcLinkProperties
}

/// Properties for the `VpcLink` resource.
#[derive(Debug, Default)]
pub struct VpcLinkProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html#cfn-apigatewayv2-vpclink-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html#cfn-apigatewayv2-vpclink-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html#cfn-apigatewayv2-vpclink-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html#cfn-apigatewayv2-vpclink-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for VpcLinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut name: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VpcLinkProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    security_group_ids: security_group_ids,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VpcLink {
    type Properties = VpcLinkProperties;
    const TYPE: &'static str = "AWS::ApiGatewayV2::VpcLink";
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

pub mod api {
    //! Property types for the `Api` resource.

    /// The [`AWS::ApiGatewayV2::Api.BodyS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html) property type.
    #[derive(Debug, Default)]
    pub struct BodyS3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html#cfn-apigatewayv2-api-bodys3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`Etag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html#cfn-apigatewayv2-api-bodys3location-etag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub etag: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html#cfn-apigatewayv2-api-bodys3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html#cfn-apigatewayv2-api-bodys3location-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BodyS3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            if let Some(ref etag) = self.etag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Etag", etag)?;
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

    impl ::codec::DeserializeValue for BodyS3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BodyS3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BodyS3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BodyS3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut etag: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Etag" => {
                                etag = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(BodyS3Location {
                        bucket: bucket,
                        etag: etag,
                        key: key,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::Api.Cors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html) property type.
    #[derive(Debug, Default)]
    pub struct Cors {
        /// Property [`AllowCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-allowcredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_credentials: Option<::Value<bool>>,
        /// Property [`AllowHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-allowheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_headers: Option<::ValueList<String>>,
        /// Property [`AllowMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-allowmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_methods: Option<::ValueList<String>>,
        /// Property [`AllowOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-alloworigins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_origins: Option<::ValueList<String>>,
        /// Property [`ExposeHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-exposeheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expose_headers: Option<::ValueList<String>>,
        /// Property [`MaxAge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html#cfn-apigatewayv2-api-cors-maxage).
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

pub mod api_gateway_managed_overrides {
    //! Property types for the `ApiGatewayManagedOverrides` resource.

    /// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides.AccessLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessLogSettings {
        /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings-destinationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_arn: Option<::Value<String>>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessLogSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_arn) = self.destination_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", destination_arn)?;
            }
            if let Some(ref format) = self.format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessLogSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessLogSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessLogSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessLogSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_arn: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationArn" => {
                                destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessLogSettings {
                        destination_arn: destination_arn,
                        format: format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides.IntegrationOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct IntegrationOverrides {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`IntegrationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides-integrationmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integration_method: Option<::Value<String>>,
        /// Property [`PayloadFormatVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides-payloadformatversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_format_version: Option<::Value<String>>,
        /// Property [`TimeoutInMillis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides-timeoutinmillis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_millis: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for IntegrationOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref integration_method) = self.integration_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationMethod", integration_method)?;
            }
            if let Some(ref payload_format_version) = self.payload_format_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadFormatVersion", payload_format_version)?;
            }
            if let Some(ref timeout_in_millis) = self.timeout_in_millis {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMillis", timeout_in_millis)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntegrationOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntegrationOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntegrationOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut integration_method: Option<::Value<String>> = None;
                    let mut payload_format_version: Option<::Value<String>> = None;
                    let mut timeout_in_millis: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegrationMethod" => {
                                integration_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadFormatVersion" => {
                                payload_format_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInMillis" => {
                                timeout_in_millis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntegrationOverrides {
                        description: description,
                        integration_method: integration_method,
                        payload_format_version: payload_format_version,
                        timeout_in_millis: timeout_in_millis,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides.RouteOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct RouteOverrides {
        /// Property [`AuthorizationScopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routeoverrides-authorizationscopes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_scopes: Option<::ValueList<String>>,
        /// Property [`AuthorizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routeoverrides-authorizationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_type: Option<::Value<String>>,
        /// Property [`AuthorizerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routeoverrides-authorizerid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorizer_id: Option<::Value<String>>,
        /// Property [`OperationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routeoverrides-operationname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operation_name: Option<::Value<String>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routeoverrides-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RouteOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authorization_scopes) = self.authorization_scopes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationScopes", authorization_scopes)?;
            }
            if let Some(ref authorization_type) = self.authorization_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", authorization_type)?;
            }
            if let Some(ref authorizer_id) = self.authorizer_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerId", authorizer_id)?;
            }
            if let Some(ref operation_name) = self.operation_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperationName", operation_name)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RouteOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RouteOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RouteOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorization_scopes: Option<::ValueList<String>> = None;
                    let mut authorization_type: Option<::Value<String>> = None;
                    let mut authorizer_id: Option<::Value<String>> = None;
                    let mut operation_name: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizationScopes" => {
                                authorization_scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthorizationType" => {
                                authorization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthorizerId" => {
                                authorizer_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OperationName" => {
                                operation_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RouteOverrides {
                        authorization_scopes: authorization_scopes,
                        authorization_type: authorization_type,
                        authorizer_id: authorizer_id,
                        operation_name: operation_name,
                        target: target,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides.RouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RouteSettings {
        /// Property [`DataTraceEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routesettings-datatraceenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property [`DetailedMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routesettings-detailedmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detailed_metrics_enabled: Option<::Value<bool>>,
        /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routesettings-logginglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_level: Option<::Value<String>>,
        /// Property [`ThrottlingBurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routesettings-throttlingburstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property [`ThrottlingRateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html#cfn-apigatewayv2-apigatewaymanagedoverrides-routesettings-throttlingratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for RouteSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_trace_enabled) = self.data_trace_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", data_trace_enabled)?;
            }
            if let Some(ref detailed_metrics_enabled) = self.detailed_metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailedMetricsEnabled", detailed_metrics_enabled)?;
            }
            if let Some(ref logging_level) = self.logging_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
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

    impl ::codec::DeserializeValue for RouteSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RouteSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RouteSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_trace_enabled: Option<::Value<bool>> = None;
                    let mut detailed_metrics_enabled: Option<::Value<bool>> = None;
                    let mut logging_level: Option<::Value<String>> = None;
                    let mut throttling_burst_limit: Option<::Value<u32>> = None;
                    let mut throttling_rate_limit: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataTraceEnabled" => {
                                data_trace_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DetailedMetricsEnabled" => {
                                detailed_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingLevel" => {
                                logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(RouteSettings {
                        data_trace_enabled: data_trace_enabled,
                        detailed_metrics_enabled: detailed_metrics_enabled,
                        logging_level: logging_level,
                        throttling_burst_limit: throttling_burst_limit,
                        throttling_rate_limit: throttling_rate_limit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::ApiGatewayManagedOverrides.StageOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html) property type.
    #[derive(Debug, Default)]
    pub struct StageOverrides {
        /// Property [`AccessLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-accesslogsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_log_settings: Option<::Value<AccessLogSettings>>,
        /// Property [`AutoDeploy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-autodeploy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_deploy: Option<::Value<bool>>,
        /// Property [`DefaultRouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-defaultroutesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_route_settings: Option<::Value<RouteSettings>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`RouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-routesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub route_settings: Option<::Value<::json::Value>>,
        /// Property [`StageVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html#cfn-apigatewayv2-apigatewaymanagedoverrides-stageoverrides-stagevariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stage_variables: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for StageOverrides {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_log_settings) = self.access_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLogSettings", access_log_settings)?;
            }
            if let Some(ref auto_deploy) = self.auto_deploy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoDeploy", auto_deploy)?;
            }
            if let Some(ref default_route_settings) = self.default_route_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRouteSettings", default_route_settings)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref route_settings) = self.route_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteSettings", route_settings)?;
            }
            if let Some(ref stage_variables) = self.stage_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageVariables", stage_variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StageOverrides {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StageOverrides, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StageOverrides;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StageOverrides")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_log_settings: Option<::Value<AccessLogSettings>> = None;
                    let mut auto_deploy: Option<::Value<bool>> = None;
                    let mut default_route_settings: Option<::Value<RouteSettings>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut route_settings: Option<::Value<::json::Value>> = None;
                    let mut stage_variables: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessLogSettings" => {
                                access_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoDeploy" => {
                                auto_deploy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultRouteSettings" => {
                                default_route_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RouteSettings" => {
                                route_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StageVariables" => {
                                stage_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StageOverrides {
                        access_log_settings: access_log_settings,
                        auto_deploy: auto_deploy,
                        default_route_settings: default_route_settings,
                        description: description,
                        route_settings: route_settings,
                        stage_variables: stage_variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod authorizer {
    //! Property types for the `Authorizer` resource.

    /// The [`AWS::ApiGatewayV2::Authorizer.JWTConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-authorizer-jwtconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct JWTConfiguration {
        /// Property [`Audience`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-authorizer-jwtconfiguration.html#cfn-apigatewayv2-authorizer-jwtconfiguration-audience).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audience: Option<::ValueList<String>>,
        /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-authorizer-jwtconfiguration.html#cfn-apigatewayv2-authorizer-jwtconfiguration-issuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JWTConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audience) = self.audience {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Audience", audience)?;
            }
            if let Some(ref issuer) = self.issuer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", issuer)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JWTConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JWTConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JWTConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JWTConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audience: Option<::ValueList<String>> = None;
                    let mut issuer: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Audience" => {
                                audience = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Issuer" => {
                                issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JWTConfiguration {
                        audience: audience,
                        issuer: issuer,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain_name {
    //! Property types for the `DomainName` resource.

    /// The [`AWS::ApiGatewayV2::DomainName.DomainNameConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainNameConfiguration {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html#cfn-apigatewayv2-domainname-domainnameconfiguration-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`CertificateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html#cfn-apigatewayv2-domainname-domainnameconfiguration-certificatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_name: Option<::Value<String>>,
        /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html#cfn-apigatewayv2-domainname-domainnameconfiguration-endpointtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_type: Option<::Value<String>>,
        /// Property [`SecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html#cfn-apigatewayv2-domainname-domainnameconfiguration-securitypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DomainNameConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref certificate_name) = self.certificate_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateName", certificate_name)?;
            }
            if let Some(ref endpoint_type) = self.endpoint_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", endpoint_type)?;
            }
            if let Some(ref security_policy) = self.security_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityPolicy", security_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainNameConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainNameConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainNameConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainNameConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut certificate_name: Option<::Value<String>> = None;
                    let mut endpoint_type: Option<::Value<String>> = None;
                    let mut security_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CertificateName" => {
                                certificate_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointType" => {
                                endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityPolicy" => {
                                security_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainNameConfiguration {
                        certificate_arn: certificate_arn,
                        certificate_name: certificate_name,
                        endpoint_type: endpoint_type,
                        security_policy: security_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::DomainName.MutualTlsAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-mutualtlsauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct MutualTlsAuthentication {
        /// Property [`TruststoreUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-mutualtlsauthentication.html#cfn-apigatewayv2-domainname-mutualtlsauthentication-truststoreuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub truststore_uri: Option<::Value<String>>,
        /// Property [`TruststoreVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-mutualtlsauthentication.html#cfn-apigatewayv2-domainname-mutualtlsauthentication-truststoreversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub truststore_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MutualTlsAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref truststore_uri) = self.truststore_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TruststoreUri", truststore_uri)?;
            }
            if let Some(ref truststore_version) = self.truststore_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TruststoreVersion", truststore_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MutualTlsAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MutualTlsAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MutualTlsAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MutualTlsAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut truststore_uri: Option<::Value<String>> = None;
                    let mut truststore_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TruststoreUri" => {
                                truststore_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TruststoreVersion" => {
                                truststore_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MutualTlsAuthentication {
                        truststore_uri: truststore_uri,
                        truststore_version: truststore_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod integration {
    //! Property types for the `Integration` resource.

    /// The [`AWS::ApiGatewayV2::Integration.ResponseParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseParameter {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameter.html#cfn-apigatewayv2-integration-responseparameter-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<String>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameter.html#cfn-apigatewayv2-integration-responseparameter-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResponseParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseParameter {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::Integration.ResponseParameterList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameterlist.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseParameterList {
        /// Property [`ResponseParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameterlist.html#cfn-apigatewayv2-integration-responseparameterlist-responseparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_parameters: Option<::ValueList<ResponseParameter>>,
    }

    impl ::codec::SerializeValue for ResponseParameterList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref response_parameters) = self.response_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseParameters", response_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseParameterList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseParameterList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseParameterList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseParameterList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut response_parameters: Option<::ValueList<ResponseParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResponseParameters" => {
                                response_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseParameterList {
                        response_parameters: response_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::Integration.TlsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-tlsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsConfig {
        /// Property [`ServerNameToVerify`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-tlsconfig.html#cfn-apigatewayv2-integration-tlsconfig-servernametoverify).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name_to_verify: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TlsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref server_name_to_verify) = self.server_name_to_verify {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerNameToVerify", server_name_to_verify)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_name_to_verify: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerNameToVerify" => {
                                server_name_to_verify = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsConfig {
                        server_name_to_verify: server_name_to_verify,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod route {
    //! Property types for the `Route` resource.

    /// The [`AWS::ApiGatewayV2::Route.ParameterConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-route-parameterconstraints.html) property type.
    #[derive(Debug, Default)]
    pub struct ParameterConstraints {
        /// Property [`Required`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-route-parameterconstraints.html#cfn-apigatewayv2-route-parameterconstraints-required).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ParameterConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", &self.required)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Required" => {
                                required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterConstraints {
                        required: required.ok_or(::serde::de::Error::missing_field("Required"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod route_response {
    //! Property types for the `RouteResponse` resource.

    /// The [`AWS::ApiGatewayV2::RouteResponse.ParameterConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routeresponse-parameterconstraints.html) property type.
    #[derive(Debug, Default)]
    pub struct ParameterConstraints {
        /// Property [`Required`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routeresponse-parameterconstraints.html#cfn-apigatewayv2-routeresponse-parameterconstraints-required).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ParameterConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", &self.required)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Required" => {
                                required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterConstraints {
                        required: required.ok_or(::serde::de::Error::missing_field("Required"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod stage {
    //! Property types for the `Stage` resource.

    /// The [`AWS::ApiGatewayV2::Stage.AccessLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-accesslogsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessLogSettings {
        /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-accesslogsettings.html#cfn-apigatewayv2-stage-accesslogsettings-destinationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_arn: Option<::Value<String>>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-accesslogsettings.html#cfn-apigatewayv2-stage-accesslogsettings-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessLogSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_arn) = self.destination_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", destination_arn)?;
            }
            if let Some(ref format) = self.format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessLogSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessLogSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessLogSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessLogSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_arn: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationArn" => {
                                destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessLogSettings {
                        destination_arn: destination_arn,
                        format: format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApiGatewayV2::Stage.RouteSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RouteSettings {
        /// Property [`DataTraceEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html#cfn-apigatewayv2-stage-routesettings-datatraceenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_trace_enabled: Option<::Value<bool>>,
        /// Property [`DetailedMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html#cfn-apigatewayv2-stage-routesettings-detailedmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detailed_metrics_enabled: Option<::Value<bool>>,
        /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html#cfn-apigatewayv2-stage-routesettings-logginglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_level: Option<::Value<String>>,
        /// Property [`ThrottlingBurstLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html#cfn-apigatewayv2-stage-routesettings-throttlingburstlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_burst_limit: Option<::Value<u32>>,
        /// Property [`ThrottlingRateLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html#cfn-apigatewayv2-stage-routesettings-throttlingratelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throttling_rate_limit: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for RouteSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_trace_enabled) = self.data_trace_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTraceEnabled", data_trace_enabled)?;
            }
            if let Some(ref detailed_metrics_enabled) = self.detailed_metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailedMetricsEnabled", detailed_metrics_enabled)?;
            }
            if let Some(ref logging_level) = self.logging_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
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

    impl ::codec::DeserializeValue for RouteSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RouteSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RouteSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RouteSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_trace_enabled: Option<::Value<bool>> = None;
                    let mut detailed_metrics_enabled: Option<::Value<bool>> = None;
                    let mut logging_level: Option<::Value<String>> = None;
                    let mut throttling_burst_limit: Option<::Value<u32>> = None;
                    let mut throttling_rate_limit: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataTraceEnabled" => {
                                data_trace_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DetailedMetricsEnabled" => {
                                detailed_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingLevel" => {
                                logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(RouteSettings {
                        data_trace_enabled: data_trace_enabled,
                        detailed_metrics_enabled: detailed_metrics_enabled,
                        logging_level: logging_level,
                        throttling_burst_limit: throttling_burst_limit,
                        throttling_rate_limit: throttling_rate_limit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
