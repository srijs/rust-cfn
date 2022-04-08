//! Types for the `RefactorSpaces` service.

/// The [`AWS::RefactorSpaces::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`ApiGatewayProxy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-apigatewayproxy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_gateway_proxy: Option<::Value<self::application::ApiGatewayProxyInput>>,
    /// Property [`EnvironmentIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-environmentidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_identifier: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ProxyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-proxytype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub proxy_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-application.html#cfn-refactorspaces-application-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_gateway_proxy) = self.api_gateway_proxy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiGatewayProxy", api_gateway_proxy)?;
        }
        if let Some(ref environment_identifier) = self.environment_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentIdentifier", environment_identifier)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref proxy_type) = self.proxy_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProxyType", proxy_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_id) = self.vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_gateway_proxy: Option<::Value<self::application::ApiGatewayProxyInput>> = None;
                let mut environment_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut proxy_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiGatewayProxy" => {
                            api_gateway_proxy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentIdentifier" => {
                            environment_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProxyType" => {
                            proxy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    api_gateway_proxy: api_gateway_proxy,
                    environment_identifier: environment_identifier,
                    name: name,
                    proxy_type: proxy_type,
                    tags: tags,
                    vpc_id: vpc_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::RefactorSpaces::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::RefactorSpaces::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html) resource type.
#[derive(Debug, Default)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html#cfn-refactorspaces-environment-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html#cfn-refactorspaces-environment-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NetworkFabricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html#cfn-refactorspaces-environment-networkfabrictype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_fabric_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-environment.html#cfn-refactorspaces-environment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref network_fabric_type) = self.network_fabric_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkFabricType", network_fabric_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_fabric_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkFabricType" => {
                            network_fabric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    description: description,
                    name: name,
                    network_fabric_type: network_fabric_type,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::RefactorSpaces::Environment";
    fn properties(&self) -> &EnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Environment {}

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

/// The [`AWS::RefactorSpaces::Route`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html) resource type.
#[derive(Debug, Default)]
pub struct Route {
    properties: RouteProperties
}

/// Properties for the `Route` resource.
#[derive(Debug, Default)]
pub struct RouteProperties {
    /// Property [`ApplicationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-applicationidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_identifier: ::Value<String>,
    /// Property [`EnvironmentIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-environmentidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_identifier: ::Value<String>,
    /// Property [`RouteType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-routetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub route_type: Option<::Value<String>>,
    /// Property [`ServiceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-serviceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_identifier: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UriPathRoute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-route.html#cfn-refactorspaces-route-uripathroute).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub uri_path_route: Option<::Value<self::route::UriPathRouteInput>>,
}

impl ::serde::Serialize for RouteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationIdentifier", &self.application_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentIdentifier", &self.environment_identifier)?;
        if let Some(ref route_type) = self.route_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteType", route_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceIdentifier", &self.service_identifier)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref uri_path_route) = self.uri_path_route {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPathRoute", uri_path_route)?;
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
                let mut application_identifier: Option<::Value<String>> = None;
                let mut environment_identifier: Option<::Value<String>> = None;
                let mut route_type: Option<::Value<String>> = None;
                let mut service_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut uri_path_route: Option<::Value<self::route::UriPathRouteInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationIdentifier" => {
                            application_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentIdentifier" => {
                            environment_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RouteType" => {
                            route_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceIdentifier" => {
                            service_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UriPathRoute" => {
                            uri_path_route = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RouteProperties {
                    application_identifier: application_identifier.ok_or(::serde::de::Error::missing_field("ApplicationIdentifier"))?,
                    environment_identifier: environment_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentIdentifier"))?,
                    route_type: route_type,
                    service_identifier: service_identifier.ok_or(::serde::de::Error::missing_field("ServiceIdentifier"))?,
                    tags: tags,
                    uri_path_route: uri_path_route,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Route {
    type Properties = RouteProperties;
    const TYPE: &'static str = "AWS::RefactorSpaces::Route";
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

/// The [`AWS::RefactorSpaces::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html) resource type.
#[derive(Debug, Default)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Debug, Default)]
pub struct ServiceProperties {
    /// Property [`ApplicationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-applicationidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_identifier: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-endpointtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_type: Option<::Value<String>>,
    /// Property [`EnvironmentIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-environmentidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub environment_identifier: ::Value<String>,
    /// Property [`LambdaEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-lambdaendpoint).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub lambda_endpoint: Option<::Value<self::service::LambdaEndpointInput>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UrlEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-urlendpoint).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub url_endpoint: Option<::Value<self::service::UrlEndpointInput>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-refactorspaces-service.html#cfn-refactorspaces-service-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationIdentifier", &self.application_identifier)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref endpoint_type) = self.endpoint_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", endpoint_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentIdentifier", &self.environment_identifier)?;
        if let Some(ref lambda_endpoint) = self.lambda_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaEndpoint", lambda_endpoint)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref url_endpoint) = self.url_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UrlEndpoint", url_endpoint)?;
        }
        if let Some(ref vpc_id) = self.vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_identifier: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut endpoint_type: Option<::Value<String>> = None;
                let mut environment_identifier: Option<::Value<String>> = None;
                let mut lambda_endpoint: Option<::Value<self::service::LambdaEndpointInput>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut url_endpoint: Option<::Value<self::service::UrlEndpointInput>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationIdentifier" => {
                            application_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointType" => {
                            endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentIdentifier" => {
                            environment_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaEndpoint" => {
                            lambda_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UrlEndpoint" => {
                            url_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceProperties {
                    application_identifier: application_identifier.ok_or(::serde::de::Error::missing_field("ApplicationIdentifier"))?,
                    description: description,
                    endpoint_type: endpoint_type,
                    environment_identifier: environment_identifier.ok_or(::serde::de::Error::missing_field("EnvironmentIdentifier"))?,
                    lambda_endpoint: lambda_endpoint,
                    name: name,
                    tags: tags,
                    url_endpoint: url_endpoint,
                    vpc_id: vpc_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Service {
    type Properties = ServiceProperties;
    const TYPE: &'static str = "AWS::RefactorSpaces::Service";
    fn properties(&self) -> &ServiceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Service {}

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::RefactorSpaces::Application.ApiGatewayProxyInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-application-apigatewayproxyinput.html) property type.
    #[derive(Debug, Default)]
    pub struct ApiGatewayProxyInput {
        /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-application-apigatewayproxyinput.html#cfn-refactorspaces-application-apigatewayproxyinput-endpointtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_type: Option<::Value<String>>,
        /// Property [`StageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-application-apigatewayproxyinput.html#cfn-refactorspaces-application-apigatewayproxyinput-stagename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub stage_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApiGatewayProxyInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint_type) = self.endpoint_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", endpoint_type)?;
            }
            if let Some(ref stage_name) = self.stage_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StageName", stage_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApiGatewayProxyInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiGatewayProxyInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApiGatewayProxyInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApiGatewayProxyInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_type: Option<::Value<String>> = None;
                    let mut stage_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointType" => {
                                endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StageName" => {
                                stage_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApiGatewayProxyInput {
                        endpoint_type: endpoint_type,
                        stage_name: stage_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod route {
    //! Property types for the `Route` resource.

    /// The [`AWS::RefactorSpaces::Route.UriPathRouteInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html) property type.
    #[derive(Debug, Default)]
    pub struct UriPathRouteInput {
        /// Property [`ActivationState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html#cfn-refactorspaces-route-uripathrouteinput-activationstate).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub activation_state: ::Value<String>,
        /// Property [`IncludeChildPaths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html#cfn-refactorspaces-route-uripathrouteinput-includechildpaths).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub include_child_paths: Option<::Value<bool>>,
        /// Property [`Methods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html#cfn-refactorspaces-route-uripathrouteinput-methods).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub methods: Option<::ValueList<String>>,
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-route-uripathrouteinput.html#cfn-refactorspaces-route-uripathrouteinput-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UriPathRouteInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActivationState", &self.activation_state)?;
            if let Some(ref include_child_paths) = self.include_child_paths {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeChildPaths", include_child_paths)?;
            }
            if let Some(ref methods) = self.methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Methods", methods)?;
            }
            if let Some(ref source_path) = self.source_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", source_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UriPathRouteInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UriPathRouteInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UriPathRouteInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UriPathRouteInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut activation_state: Option<::Value<String>> = None;
                    let mut include_child_paths: Option<::Value<bool>> = None;
                    let mut methods: Option<::ValueList<String>> = None;
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActivationState" => {
                                activation_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeChildPaths" => {
                                include_child_paths = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Methods" => {
                                methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UriPathRouteInput {
                        activation_state: activation_state.ok_or(::serde::de::Error::missing_field("ActivationState"))?,
                        include_child_paths: include_child_paths,
                        methods: methods,
                        source_path: source_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod service {
    //! Property types for the `Service` resource.

    /// The [`AWS::RefactorSpaces::Service.LambdaEndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-lambdaendpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaEndpointInput {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-lambdaendpointinput.html#cfn-refactorspaces-service-lambdaendpointinput-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaEndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaEndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaEndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaEndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaEndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaEndpointInput {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RefactorSpaces::Service.UrlEndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-urlendpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct UrlEndpointInput {
        /// Property [`HealthUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-urlendpointinput.html#cfn-refactorspaces-service-urlendpointinput-healthurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub health_url: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-refactorspaces-service-urlendpointinput.html#cfn-refactorspaces-service-urlendpointinput-url).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for UrlEndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref health_url) = self.health_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthUrl", health_url)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UrlEndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UrlEndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UrlEndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UrlEndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut health_url: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthUrl" => {
                                health_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UrlEndpointInput {
                        health_url: health_url,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
