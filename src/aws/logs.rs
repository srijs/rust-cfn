//! Types for the `Logs` service.

/// The [`AWS::Logs::Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html) resource type.
#[derive(Debug, Default)]
pub struct Destination {
    properties: DestinationProperties
}

/// Properties for the `Destination` resource.
#[derive(Debug, Default)]
pub struct DestinationProperties {
    /// Property [`DestinationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html#cfn-logs-destination-destinationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub destination_name: ::Value<String>,
    /// Property [`DestinationPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html#cfn-logs-destination-destinationpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_policy: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html#cfn-logs-destination-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html#cfn-logs-destination-targetarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_arn: ::Value<String>,
}

impl ::serde::Serialize for DestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationName", &self.destination_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPolicy", &self.destination_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_name: Option<::Value<String>> = None;
                let mut destination_policy: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut target_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationName" => {
                            destination_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationPolicy" => {
                            destination_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArn" => {
                            target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DestinationProperties {
                    destination_name: destination_name.ok_or(::serde::de::Error::missing_field("DestinationName"))?,
                    destination_policy: destination_policy.ok_or(::serde::de::Error::missing_field("DestinationPolicy"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Destination {
    type Properties = DestinationProperties;
    const TYPE: &'static str = "AWS::Logs::Destination";
    fn properties(&self) -> &DestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Destination {}

impl From<DestinationProperties> for Destination {
    fn from(properties: DestinationProperties) -> Destination {
        Destination { properties }
    }
}

/// The [`AWS::Logs::LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html) resource type.
#[derive(Debug, Default)]
pub struct LogGroup {
    properties: LogGroupProperties
}

/// Properties for the `LogGroup` resource.
#[derive(Debug, Default)]
pub struct LogGroupProperties {
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html#cfn-logs-loggroup-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html#cfn-logs-loggroup-loggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_group_name: Option<::Value<String>>,
    /// Property [`RetentionInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html#cfn-logs-loggroup-retentionindays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retention_in_days: Option<::Value<u32>>,
}

impl ::serde::Serialize for LogGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref log_group_name) = self.log_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
        }
        if let Some(ref retention_in_days) = self.retention_in_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionInDays", retention_in_days)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LogGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LogGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LogGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LogGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut log_group_name: Option<::Value<String>> = None;
                let mut retention_in_days: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogGroupName" => {
                            log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetentionInDays" => {
                            retention_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LogGroupProperties {
                    kms_key_id: kms_key_id,
                    log_group_name: log_group_name,
                    retention_in_days: retention_in_days,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LogGroup {
    type Properties = LogGroupProperties;
    const TYPE: &'static str = "AWS::Logs::LogGroup";
    fn properties(&self) -> &LogGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LogGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LogGroup {}

impl From<LogGroupProperties> for LogGroup {
    fn from(properties: LogGroupProperties) -> LogGroup {
        LogGroup { properties }
    }
}

/// The [`AWS::Logs::LogStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-logstream.html) resource type.
#[derive(Debug, Default)]
pub struct LogStream {
    properties: LogStreamProperties
}

/// Properties for the `LogStream` resource.
#[derive(Debug, Default)]
pub struct LogStreamProperties {
    /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-logstream.html#cfn-logs-logstream-loggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_group_name: ::Value<String>,
    /// Property [`LogStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-logstream.html#cfn-logs-logstream-logstreamname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_stream_name: Option<::Value<String>>,
}

impl ::serde::Serialize for LogStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
        if let Some(ref log_stream_name) = self.log_stream_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamName", log_stream_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LogStreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LogStreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LogStreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LogStreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut log_group_name: Option<::Value<String>> = None;
                let mut log_stream_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LogGroupName" => {
                            log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogStreamName" => {
                            log_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LogStreamProperties {
                    log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                    log_stream_name: log_stream_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LogStream {
    type Properties = LogStreamProperties;
    const TYPE: &'static str = "AWS::Logs::LogStream";
    fn properties(&self) -> &LogStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LogStreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LogStream {}

impl From<LogStreamProperties> for LogStream {
    fn from(properties: LogStreamProperties) -> LogStream {
        LogStream { properties }
    }
}

/// The [`AWS::Logs::MetricFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html) resource type.
#[derive(Debug, Default)]
pub struct MetricFilter {
    properties: MetricFilterProperties
}

/// Properties for the `MetricFilter` resource.
#[derive(Debug, Default)]
pub struct MetricFilterProperties {
    /// Property [`FilterPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html#cfn-cwl-metricfilter-filterpattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_pattern: ::Value<String>,
    /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html#cfn-cwl-metricfilter-loggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_group_name: ::Value<String>,
    /// Property [`MetricTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html#cfn-cwl-metricfilter-metrictransformations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_transformations: ::ValueList<self::metric_filter::MetricTransformation>,
}

impl ::serde::Serialize for MetricFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterPattern", &self.filter_pattern)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricTransformations", &self.metric_transformations)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MetricFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MetricFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MetricFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter_pattern: Option<::Value<String>> = None;
                let mut log_group_name: Option<::Value<String>> = None;
                let mut metric_transformations: Option<::ValueList<self::metric_filter::MetricTransformation>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FilterPattern" => {
                            filter_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogGroupName" => {
                            log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricTransformations" => {
                            metric_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MetricFilterProperties {
                    filter_pattern: filter_pattern.ok_or(::serde::de::Error::missing_field("FilterPattern"))?,
                    log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                    metric_transformations: metric_transformations.ok_or(::serde::de::Error::missing_field("MetricTransformations"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MetricFilter {
    type Properties = MetricFilterProperties;
    const TYPE: &'static str = "AWS::Logs::MetricFilter";
    fn properties(&self) -> &MetricFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MetricFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MetricFilter {}

impl From<MetricFilterProperties> for MetricFilter {
    fn from(properties: MetricFilterProperties) -> MetricFilter {
        MetricFilter { properties }
    }
}

/// The [`AWS::Logs::QueryDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-querydefinition.html) resource type.
#[derive(Debug, Default)]
pub struct QueryDefinition {
    properties: QueryDefinitionProperties
}

/// Properties for the `QueryDefinition` resource.
#[derive(Debug, Default)]
pub struct QueryDefinitionProperties {
    /// Property [`LogGroupNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-querydefinition.html#cfn-logs-querydefinition-loggroupnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_group_names: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-querydefinition.html#cfn-logs-querydefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-querydefinition.html#cfn-logs-querydefinition-querystring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_string: ::Value<String>,
}

impl ::serde::Serialize for QueryDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref log_group_names) = self.log_group_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupNames", log_group_names)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", &self.query_string)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueryDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueryDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueryDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut log_group_names: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut query_string: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LogGroupNames" => {
                            log_group_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryString" => {
                            query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QueryDefinitionProperties {
                    log_group_names: log_group_names,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    query_string: query_string.ok_or(::serde::de::Error::missing_field("QueryString"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for QueryDefinition {
    type Properties = QueryDefinitionProperties;
    const TYPE: &'static str = "AWS::Logs::QueryDefinition";
    fn properties(&self) -> &QueryDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueryDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for QueryDefinition {}

impl From<QueryDefinitionProperties> for QueryDefinition {
    fn from(properties: QueryDefinitionProperties) -> QueryDefinition {
        QueryDefinition { properties }
    }
}

/// The [`AWS::Logs::SubscriptionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html) resource type.
#[derive(Debug, Default)]
pub struct SubscriptionFilter {
    properties: SubscriptionFilterProperties
}

/// Properties for the `SubscriptionFilter` resource.
#[derive(Debug, Default)]
pub struct SubscriptionFilterProperties {
    /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html#cfn-cwl-subscriptionfilter-destinationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub destination_arn: ::Value<String>,
    /// Property [`FilterPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html#cfn-cwl-subscriptionfilter-filterpattern).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub filter_pattern: ::Value<String>,
    /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html#cfn-cwl-subscriptionfilter-loggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_group_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html#cfn-cwl-subscriptionfilter-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for SubscriptionFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", &self.destination_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterPattern", &self.filter_pattern)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SubscriptionFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SubscriptionFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SubscriptionFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_arn: Option<::Value<String>> = None;
                let mut filter_pattern: Option<::Value<String>> = None;
                let mut log_group_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationArn" => {
                            destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FilterPattern" => {
                            filter_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogGroupName" => {
                            log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SubscriptionFilterProperties {
                    destination_arn: destination_arn.ok_or(::serde::de::Error::missing_field("DestinationArn"))?,
                    filter_pattern: filter_pattern.ok_or(::serde::de::Error::missing_field("FilterPattern"))?,
                    log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SubscriptionFilter {
    type Properties = SubscriptionFilterProperties;
    const TYPE: &'static str = "AWS::Logs::SubscriptionFilter";
    fn properties(&self) -> &SubscriptionFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SubscriptionFilter {}

impl From<SubscriptionFilterProperties> for SubscriptionFilter {
    fn from(properties: SubscriptionFilterProperties) -> SubscriptionFilter {
        SubscriptionFilter { properties }
    }
}

pub mod metric_filter {
    //! Property types for the `MetricFilter` resource.

    /// The [`AWS::Logs::MetricFilter.MetricTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricTransformation {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html#cfn-cwl-metricfilter-metrictransformation-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<f64>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html#cfn-cwl-metricfilter-metrictransformation-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`MetricNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html#cfn-cwl-metricfilter-metrictransformation-metricnamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_namespace: ::Value<String>,
        /// Property [`MetricValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-logs-metricfilter-metrictransformation.html#cfn-cwl-metricfilter-metrictransformation-metricvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricTransformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricNamespace", &self.metric_namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricValue", &self.metric_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricTransformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricTransformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricTransformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricTransformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<f64>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut metric_namespace: Option<::Value<String>> = None;
                    let mut metric_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricNamespace" => {
                                metric_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricValue" => {
                                metric_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricTransformation {
                        default_value: default_value,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        metric_namespace: metric_namespace.ok_or(::serde::de::Error::missing_field("MetricNamespace"))?,
                        metric_value: metric_value.ok_or(::serde::de::Error::missing_field("MetricValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
