//! Types for the `Events` service.

/// The [`AWS::Events::ApiDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html) resource type.
#[derive(Debug, Default)]
pub struct ApiDestination {
    properties: ApiDestinationProperties
}

/// Properties for the `ApiDestination` resource.
#[derive(Debug, Default)]
pub struct ApiDestinationProperties {
    /// Property [`ConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-connectionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_arn: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`HttpMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-httpmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_method: ::Value<String>,
    /// Property [`InvocationEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-invocationendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub invocation_endpoint: ::Value<String>,
    /// Property [`InvocationRateLimitPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-invocationratelimitpersecond).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub invocation_rate_limit_per_second: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html#cfn-events-apidestination-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ApiDestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionArn", &self.connection_arn)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpMethod", &self.http_method)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationEndpoint", &self.invocation_endpoint)?;
        if let Some(ref invocation_rate_limit_per_second) = self.invocation_rate_limit_per_second {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationRateLimitPerSecond", invocation_rate_limit_per_second)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiDestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiDestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiDestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiDestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut http_method: Option<::Value<String>> = None;
                let mut invocation_endpoint: Option<::Value<String>> = None;
                let mut invocation_rate_limit_per_second: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionArn" => {
                            connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HttpMethod" => {
                            http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InvocationEndpoint" => {
                            invocation_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InvocationRateLimitPerSecond" => {
                            invocation_rate_limit_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiDestinationProperties {
                    connection_arn: connection_arn.ok_or(::serde::de::Error::missing_field("ConnectionArn"))?,
                    description: description,
                    http_method: http_method.ok_or(::serde::de::Error::missing_field("HttpMethod"))?,
                    invocation_endpoint: invocation_endpoint.ok_or(::serde::de::Error::missing_field("InvocationEndpoint"))?,
                    invocation_rate_limit_per_second: invocation_rate_limit_per_second,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiDestination {
    type Properties = ApiDestinationProperties;
    const TYPE: &'static str = "AWS::Events::ApiDestination";
    fn properties(&self) -> &ApiDestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiDestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiDestination {}

impl From<ApiDestinationProperties> for ApiDestination {
    fn from(properties: ApiDestinationProperties) -> ApiDestination {
        ApiDestination { properties }
    }
}

/// The [`AWS::Events::Archive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html) resource type.
#[derive(Debug, Default)]
pub struct Archive {
    properties: ArchiveProperties
}

/// Properties for the `Archive` resource.
#[derive(Debug, Default)]
pub struct ArchiveProperties {
    /// Property [`ArchiveName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html#cfn-events-archive-archivename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub archive_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html#cfn-events-archive-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html#cfn-events-archive-eventpattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_pattern: Option<::Value<::json::Value>>,
    /// Property [`RetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html#cfn-events-archive-retentiondays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_days: Option<::Value<u32>>,
    /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html#cfn-events-archive-sourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_arn: ::Value<String>,
}

impl ::serde::Serialize for ArchiveProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref archive_name) = self.archive_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveName", archive_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_pattern) = self.event_pattern {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventPattern", event_pattern)?;
        }
        if let Some(ref retention_days) = self.retention_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionDays", retention_days)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ArchiveProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ArchiveProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ArchiveProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut archive_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut event_pattern: Option<::Value<::json::Value>> = None;
                let mut retention_days: Option<::Value<u32>> = None;
                let mut source_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ArchiveName" => {
                            archive_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventPattern" => {
                            event_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionDays" => {
                            retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceArn" => {
                            source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ArchiveProperties {
                    archive_name: archive_name,
                    description: description,
                    event_pattern: event_pattern,
                    retention_days: retention_days,
                    source_arn: source_arn.ok_or(::serde::de::Error::missing_field("SourceArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Archive {
    type Properties = ArchiveProperties;
    const TYPE: &'static str = "AWS::Events::Archive";
    fn properties(&self) -> &ArchiveProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ArchiveProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Archive {}

impl From<ArchiveProperties> for Archive {
    fn from(properties: ArchiveProperties) -> Archive {
        Archive { properties }
    }
}

/// The [`AWS::Events::Connection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html) resource type.
#[derive(Debug, Default)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Debug, Default)]
pub struct ConnectionProperties {
    /// Property [`AuthParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html#cfn-events-connection-authparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth_parameters: ::Value<::json::Value>,
    /// Property [`AuthorizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html#cfn-events-connection-authorizationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization_type: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html#cfn-events-connection-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html#cfn-events-connection-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthParameters", &self.auth_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", &self.authorization_type)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth_parameters: Option<::Value<::json::Value>> = None;
                let mut authorization_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthParameters" => {
                            auth_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizationType" => {
                            authorization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectionProperties {
                    auth_parameters: auth_parameters.ok_or(::serde::de::Error::missing_field("AuthParameters"))?,
                    authorization_type: authorization_type.ok_or(::serde::de::Error::missing_field("AuthorizationType"))?,
                    description: description,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connection {
    type Properties = ConnectionProperties;
    const TYPE: &'static str = "AWS::Events::Connection";
    fn properties(&self) -> &ConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connection {}

impl From<ConnectionProperties> for Connection {
    fn from(properties: ConnectionProperties) -> Connection {
        Connection { properties }
    }
}

/// The [`AWS::Events::EventBus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbus.html) resource type.
#[derive(Debug, Default)]
pub struct EventBus {
    properties: EventBusProperties
}

/// Properties for the `EventBus` resource.
#[derive(Debug, Default)]
pub struct EventBusProperties {
    /// Property [`EventSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbus.html#cfn-events-eventbus-eventsourcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_source_name: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbus.html#cfn-events-eventbus-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for EventBusProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref event_source_name) = self.event_source_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceName", event_source_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventBusProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBusProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventBusProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventBusProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut event_source_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EventSourceName" => {
                            event_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventBusProperties {
                    event_source_name: event_source_name,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventBus {
    type Properties = EventBusProperties;
    const TYPE: &'static str = "AWS::Events::EventBus";
    fn properties(&self) -> &EventBusProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventBusProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventBus {}

impl From<EventBusProperties> for EventBus {
    fn from(properties: EventBusProperties) -> EventBus {
        EventBus { properties }
    }
}

/// The [`AWS::Events::EventBusPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html) resource type.
#[derive(Debug, Default)]
pub struct EventBusPolicy {
    properties: EventBusPolicyProperties
}

/// Properties for the `EventBusPolicy` resource.
#[derive(Debug, Default)]
pub struct EventBusPolicyProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-action).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action: Option<::Value<String>>,
    /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-condition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub condition: Option<::Value<self::event_bus_policy::Condition>>,
    /// Property [`EventBusName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-eventbusname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_bus_name: Option<::Value<String>>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-principal).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub principal: Option<::Value<String>>,
    /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-statement).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub statement: Option<::Value<::json::Value>>,
    /// Property [`StatementId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html#cfn-events-eventbuspolicy-statementid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub statement_id: ::Value<String>,
}

impl ::serde::Serialize for EventBusPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref action) = self.action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
        }
        if let Some(ref condition) = self.condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
        }
        if let Some(ref event_bus_name) = self.event_bus_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBusName", event_bus_name)?;
        }
        if let Some(ref principal) = self.principal {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", principal)?;
        }
        if let Some(ref statement) = self.statement {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", statement)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatementId", &self.statement_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventBusPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBusPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventBusPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventBusPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut condition: Option<::Value<self::event_bus_policy::Condition>> = None;
                let mut event_bus_name: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;
                let mut statement: Option<::Value<::json::Value>> = None;
                let mut statement_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Condition" => {
                            condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventBusName" => {
                            event_bus_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Statement" => {
                            statement = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StatementId" => {
                            statement_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventBusPolicyProperties {
                    action: action,
                    condition: condition,
                    event_bus_name: event_bus_name,
                    principal: principal,
                    statement: statement,
                    statement_id: statement_id.ok_or(::serde::de::Error::missing_field("StatementId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventBusPolicy {
    type Properties = EventBusPolicyProperties;
    const TYPE: &'static str = "AWS::Events::EventBusPolicy";
    fn properties(&self) -> &EventBusPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventBusPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventBusPolicy {}

impl From<EventBusPolicyProperties> for EventBusPolicy {
    fn from(properties: EventBusPolicyProperties) -> EventBusPolicy {
        EventBusPolicy { properties }
    }
}

/// The [`AWS::Events::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html) resource type.
#[derive(Debug, Default)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Default)]
pub struct RuleProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventBusName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-eventbusname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_bus_name: Option<::Value<String>>,
    /// Property [`EventPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-eventpattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_pattern: Option<::Value<::json::Value>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-scheduleexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression: Option<::Value<String>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html#cfn-events-rule-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: Option<::ValueList<self::rule::Target>>,
}

impl ::serde::Serialize for RuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_bus_name) = self.event_bus_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBusName", event_bus_name)?;
        }
        if let Some(ref event_pattern) = self.event_pattern {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventPattern", event_pattern)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref schedule_expression) = self.schedule_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref targets) = self.targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut event_bus_name: Option<::Value<String>> = None;
                let mut event_pattern: Option<::Value<::json::Value>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut schedule_expression: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::rule::Target>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventBusName" => {
                            event_bus_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventPattern" => {
                            event_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpression" => {
                            schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleProperties {
                    description: description,
                    event_bus_name: event_bus_name,
                    event_pattern: event_pattern,
                    name: name,
                    role_arn: role_arn,
                    schedule_expression: schedule_expression,
                    state: state,
                    targets: targets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::Events::Rule";
    fn properties(&self) -> &RuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Rule {}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

pub mod event_bus_policy {
    //! Property types for the `EventBusPolicy` resource.

    /// The [`AWS::Events::EventBusPolicy.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbuspolicy-condition.html) property type.
    #[derive(Debug, Default)]
    pub struct Condition {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbuspolicy-condition.html#cfn-events-eventbuspolicy-condition-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbuspolicy-condition.html#cfn-events-eventbuspolicy-condition-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbuspolicy-condition.html#cfn-events-eventbuspolicy-condition-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Condition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Condition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Condition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Condition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Condition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Condition {
                        key: key,
                        r#type: r#type,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::Events::Rule.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-awsvpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsVpcConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-awsvpcconfiguration.html#cfn-events-rule-awsvpcconfiguration-assignpublicip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-awsvpcconfiguration.html#cfn-events-rule-awsvpcconfiguration-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-awsvpcconfiguration.html#cfn-events-rule-awsvpcconfiguration-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_public_ip: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsVpcConfiguration {
                        assign_public_ip: assign_public_ip,
                        security_groups: security_groups,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.BatchArrayProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batcharrayproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchArrayProperties {
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batcharrayproperties.html#cfn-events-rule-batcharrayproperties-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BatchArrayProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref size) = self.size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchArrayProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchArrayProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchArrayProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchArrayProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchArrayProperties {
                        size: size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.BatchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchParameters {
        /// Property [`ArrayProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html#cfn-events-rule-batchparameters-arrayproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub array_properties: Option<::Value<BatchArrayProperties>>,
        /// Property [`JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html#cfn-events-rule-batchparameters-jobdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_definition: ::Value<String>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html#cfn-events-rule-batchparameters-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: ::Value<String>,
        /// Property [`RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html#cfn-events-rule-batchparameters-retrystrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_strategy: Option<::Value<BatchRetryStrategy>>,
    }

    impl ::codec::SerializeValue for BatchParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref array_properties) = self.array_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArrayProperties", array_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinition", &self.job_definition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", &self.job_name)?;
            if let Some(ref retry_strategy) = self.retry_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryStrategy", retry_strategy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut array_properties: Option<::Value<BatchArrayProperties>> = None;
                    let mut job_definition: Option<::Value<String>> = None;
                    let mut job_name: Option<::Value<String>> = None;
                    let mut retry_strategy: Option<::Value<BatchRetryStrategy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArrayProperties" => {
                                array_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobDefinition" => {
                                job_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryStrategy" => {
                                retry_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchParameters {
                        array_properties: array_properties,
                        job_definition: job_definition.ok_or(::serde::de::Error::missing_field("JobDefinition"))?,
                        job_name: job_name.ok_or(::serde::de::Error::missing_field("JobName"))?,
                        retry_strategy: retry_strategy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.BatchRetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchretrystrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct BatchRetryStrategy {
        /// Property [`Attempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchretrystrategy.html#cfn-events-rule-batchretrystrategy-attempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attempts: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BatchRetryStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attempts) = self.attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attempts", attempts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BatchRetryStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BatchRetryStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BatchRetryStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BatchRetryStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attempts: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attempts" => {
                                attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BatchRetryStrategy {
                        attempts: attempts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-deadletterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeadLetterConfig {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-deadletterconfig.html#cfn-events-rule-deadletterconfig-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeadLetterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeadLetterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeadLetterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeadLetterConfig")
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

                    Ok(DeadLetterConfig {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsParameters {
        /// Property [`Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-group).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group: Option<::Value<String>>,
        /// Property [`LaunchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-launchtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_type: Option<::Value<String>>,
        /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-networkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_configuration: Option<::Value<NetworkConfiguration>>,
        /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-platformversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform_version: Option<::Value<String>>,
        /// Property [`TaskCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-taskcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_count: Option<::Value<u32>>,
        /// Property [`TaskDefinitionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html#cfn-events-rule-ecsparameters-taskdefinitionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_definition_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for EcsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref group) = self.group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Group", group)?;
            }
            if let Some(ref launch_type) = self.launch_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", launch_type)?;
            }
            if let Some(ref network_configuration) = self.network_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
            }
            if let Some(ref platform_version) = self.platform_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
            }
            if let Some(ref task_count) = self.task_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskCount", task_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinitionArn", &self.task_definition_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group: Option<::Value<String>> = None;
                    let mut launch_type: Option<::Value<String>> = None;
                    let mut network_configuration: Option<::Value<NetworkConfiguration>> = None;
                    let mut platform_version: Option<::Value<String>> = None;
                    let mut task_count: Option<::Value<u32>> = None;
                    let mut task_definition_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Group" => {
                                group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchType" => {
                                launch_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkConfiguration" => {
                                network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlatformVersion" => {
                                platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskCount" => {
                                task_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskDefinitionArn" => {
                                task_definition_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsParameters {
                        group: group,
                        launch_type: launch_type,
                        network_configuration: network_configuration,
                        platform_version: platform_version,
                        task_count: task_count,
                        task_definition_arn: task_definition_arn.ok_or(::serde::de::Error::missing_field("TaskDefinitionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.HttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-httpparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpParameters {
        /// Property [`HeaderParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-httpparameters.html#cfn-events-rule-httpparameters-headerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_parameters: Option<::ValueMap<String>>,
        /// Property [`PathParameterValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-httpparameters.html#cfn-events-rule-httpparameters-pathparametervalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_parameter_values: Option<::ValueList<String>>,
        /// Property [`QueryStringParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-httpparameters.html#cfn-events-rule-httpparameters-querystringparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_parameters: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for HttpParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref header_parameters) = self.header_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderParameters", header_parameters)?;
            }
            if let Some(ref path_parameter_values) = self.path_parameter_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathParameterValues", path_parameter_values)?;
            }
            if let Some(ref query_string_parameters) = self.query_string_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringParameters", query_string_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_parameters: Option<::ValueMap<String>> = None;
                    let mut path_parameter_values: Option<::ValueList<String>> = None;
                    let mut query_string_parameters: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderParameters" => {
                                header_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathParameterValues" => {
                                path_parameter_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringParameters" => {
                                query_string_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpParameters {
                        header_parameters: header_parameters,
                        path_parameter_values: path_parameter_values,
                        query_string_parameters: query_string_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html) property type.
    #[derive(Debug, Default)]
    pub struct InputTransformer {
        /// Property [`InputPathsMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html#cfn-events-rule-inputtransformer-inputpathsmap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_paths_map: Option<::ValueMap<String>>,
        /// Property [`InputTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html#cfn-events-rule-inputtransformer-inputtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_template: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputTransformer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_paths_map) = self.input_paths_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPathsMap", input_paths_map)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTemplate", &self.input_template)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputTransformer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputTransformer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputTransformer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputTransformer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_paths_map: Option<::ValueMap<String>> = None;
                    let mut input_template: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputPathsMap" => {
                                input_paths_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTemplate" => {
                                input_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputTransformer {
                        input_paths_map: input_paths_map,
                        input_template: input_template.ok_or(::serde::de::Error::missing_field("InputTemplate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisParameters {
        /// Property [`PartitionKeyPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html#cfn-events-rule-kinesisparameters-partitionkeypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKeyPath", &self.partition_key_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKeyPath" => {
                                partition_key_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisParameters {
                        partition_key_path: partition_key_path.ok_or(::serde::de::Error::missing_field("PartitionKeyPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-networkconfiguration.html#cfn-events-rule-networkconfiguration-awsvpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_vpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_vpc_configuration) = self.aws_vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsVpcConfiguration", aws_vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_vpc_configuration: Option<::Value<AwsVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsVpcConfiguration" => {
                                aws_vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        aws_vpc_configuration: aws_vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RedshiftDataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftDataParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`DbUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-dbuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_user: Option<::Value<String>>,
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: Option<::Value<String>>,
        /// Property [`Sql`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-sql).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql: ::Value<String>,
        /// Property [`StatementName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-statementname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement_name: Option<::Value<String>>,
        /// Property [`WithEvent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html#cfn-events-rule-redshiftdataparameters-withevent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub with_event: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for RedshiftDataParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            if let Some(ref db_user) = self.db_user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbUser", db_user)?;
            }
            if let Some(ref secret_manager_arn) = self.secret_manager_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", secret_manager_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sql", &self.sql)?;
            if let Some(ref statement_name) = self.statement_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatementName", statement_name)?;
            }
            if let Some(ref with_event) = self.with_event {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WithEvent", with_event)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftDataParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftDataParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftDataParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftDataParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut db_user: Option<::Value<String>> = None;
                    let mut secret_manager_arn: Option<::Value<String>> = None;
                    let mut sql: Option<::Value<String>> = None;
                    let mut statement_name: Option<::Value<String>> = None;
                    let mut with_event: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DbUser" => {
                                db_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sql" => {
                                sql = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatementName" => {
                                statement_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WithEvent" => {
                                with_event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftDataParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        db_user: db_user,
                        secret_manager_arn: secret_manager_arn,
                        sql: sql.ok_or(::serde::de::Error::missing_field("Sql"))?,
                        statement_name: statement_name,
                        with_event: with_event,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-retrypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct RetryPolicy {
        /// Property [`MaximumEventAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-retrypolicy.html#cfn-events-rule-retrypolicy-maximumeventageinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_event_age_in_seconds: Option<::Value<u32>>,
        /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-retrypolicy.html#cfn-events-rule-retrypolicy-maximumretryattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_retry_attempts: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RetryPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_event_age_in_seconds) = self.maximum_event_age_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumEventAgeInSeconds", maximum_event_age_in_seconds)?;
            }
            if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetryPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_event_age_in_seconds: Option<::Value<u32>> = None;
                    let mut maximum_retry_attempts: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumEventAgeInSeconds" => {
                                maximum_event_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRetryAttempts" => {
                                maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetryPolicy {
                        maximum_event_age_in_seconds: maximum_event_age_in_seconds,
                        maximum_retry_attempts: maximum_retry_attempts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RunCommandParameters {
        /// Property [`RunCommandTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html#cfn-events-rule-runcommandparameters-runcommandtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub run_command_targets: ::ValueList<RunCommandTarget>,
    }

    impl ::codec::SerializeValue for RunCommandParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunCommandTargets", &self.run_command_targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunCommandParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunCommandParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunCommandParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunCommandParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut run_command_targets: Option<::ValueList<RunCommandTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RunCommandTargets" => {
                                run_command_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunCommandParameters {
                        run_command_targets: run_command_targets.ok_or(::serde::de::Error::missing_field("RunCommandTargets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.RunCommandTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct RunCommandTarget {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html#cfn-events-rule-runcommandtarget-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html#cfn-events-rule-runcommandtarget-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for RunCommandTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunCommandTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunCommandTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunCommandTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunCommandTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunCommandTarget {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.SqsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-sqsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SqsParameters {
        /// Property [`MessageGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-sqsparameters.html#cfn-events-rule-sqsparameters-messagegroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_group_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SqsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupId", &self.message_group_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageGroupId" => {
                                message_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqsParameters {
                        message_group_id: message_group_id.ok_or(::serde::de::Error::missing_field("MessageGroupId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Events::Rule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`BatchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-batchparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_parameters: Option<::Value<BatchParameters>>,
        /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-deadletterconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dead_letter_config: Option<::Value<DeadLetterConfig>>,
        /// Property [`EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-ecsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecs_parameters: Option<::Value<EcsParameters>>,
        /// Property [`HttpParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-httpparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_parameters: Option<::Value<HttpParameters>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-input).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input: Option<::Value<String>>,
        /// Property [`InputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-inputpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_path: Option<::Value<String>>,
        /// Property [`InputTransformer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-inputtransformer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_transformer: Option<::Value<InputTransformer>>,
        /// Property [`KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-kinesisparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_parameters: Option<::Value<KinesisParameters>>,
        /// Property [`RedshiftDataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-redshiftdataparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_data_parameters: Option<::Value<RedshiftDataParameters>>,
        /// Property [`RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-retrypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_policy: Option<::Value<RetryPolicy>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`RunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-runcommandparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub run_command_parameters: Option<::Value<RunCommandParameters>>,
        /// Property [`SqsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html#cfn-events-rule-target-sqsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs_parameters: Option<::Value<SqsParameters>>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref batch_parameters) = self.batch_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchParameters", batch_parameters)?;
            }
            if let Some(ref dead_letter_config) = self.dead_letter_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
            }
            if let Some(ref ecs_parameters) = self.ecs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsParameters", ecs_parameters)?;
            }
            if let Some(ref http_parameters) = self.http_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpParameters", http_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref input) = self.input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", input)?;
            }
            if let Some(ref input_path) = self.input_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPath", input_path)?;
            }
            if let Some(ref input_transformer) = self.input_transformer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTransformer", input_transformer)?;
            }
            if let Some(ref kinesis_parameters) = self.kinesis_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisParameters", kinesis_parameters)?;
            }
            if let Some(ref redshift_data_parameters) = self.redshift_data_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftDataParameters", redshift_data_parameters)?;
            }
            if let Some(ref retry_policy) = self.retry_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryPolicy", retry_policy)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref run_command_parameters) = self.run_command_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunCommandParameters", run_command_parameters)?;
            }
            if let Some(ref sqs_parameters) = self.sqs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqsParameters", sqs_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut batch_parameters: Option<::Value<BatchParameters>> = None;
                    let mut dead_letter_config: Option<::Value<DeadLetterConfig>> = None;
                    let mut ecs_parameters: Option<::Value<EcsParameters>> = None;
                    let mut http_parameters: Option<::Value<HttpParameters>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut input: Option<::Value<String>> = None;
                    let mut input_path: Option<::Value<String>> = None;
                    let mut input_transformer: Option<::Value<InputTransformer>> = None;
                    let mut kinesis_parameters: Option<::Value<KinesisParameters>> = None;
                    let mut redshift_data_parameters: Option<::Value<RedshiftDataParameters>> = None;
                    let mut retry_policy: Option<::Value<RetryPolicy>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut run_command_parameters: Option<::Value<RunCommandParameters>> = None;
                    let mut sqs_parameters: Option<::Value<SqsParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BatchParameters" => {
                                batch_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeadLetterConfig" => {
                                dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcsParameters" => {
                                ecs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpParameters" => {
                                http_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Input" => {
                                input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputPath" => {
                                input_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTransformer" => {
                                input_transformer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisParameters" => {
                                kinesis_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftDataParameters" => {
                                redshift_data_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryPolicy" => {
                                retry_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunCommandParameters" => {
                                run_command_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqsParameters" => {
                                sqs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        batch_parameters: batch_parameters,
                        dead_letter_config: dead_letter_config,
                        ecs_parameters: ecs_parameters,
                        http_parameters: http_parameters,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        input: input,
                        input_path: input_path,
                        input_transformer: input_transformer,
                        kinesis_parameters: kinesis_parameters,
                        redshift_data_parameters: redshift_data_parameters,
                        retry_policy: retry_policy,
                        role_arn: role_arn,
                        run_command_parameters: run_command_parameters,
                        sqs_parameters: sqs_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
