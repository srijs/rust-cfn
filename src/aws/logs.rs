//! Types for the `Logs` service.

/// The [`AWS::Logs::Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html) resource type.
#[derive(Debug)]
pub struct Destination {
    properties: DestinationProperties
}

/// Properties for the `Destination` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationProperties {
    /// Property `DestinationName`.
    #[serde(rename = "DestinationName")]
    pub destination_name: ::Value<String>,
    /// Property `DestinationPolicy`.
    #[serde(rename = "DestinationPolicy")]
    pub destination_policy: ::Value<String>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    pub role_arn: ::Value<String>,
    /// Property `TargetArn`.
    #[serde(rename = "TargetArn")]
    pub target_arn: ::Value<String>,
}

impl<'a> ::Resource<'a> for Destination {
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
#[derive(Debug)]
pub struct LogGroup {
    properties: LogGroupProperties
}

/// Properties for the `LogGroup` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogGroupProperties {
    /// Property `LogGroupName`.
    #[serde(rename = "LogGroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<::Value<String>>,
    /// Property `RetentionInDays`.
    #[serde(rename = "RetentionInDays")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<::Value<u32>>,
}

impl<'a> ::Resource<'a> for LogGroup {
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
#[derive(Debug)]
pub struct LogStream {
    properties: LogStreamProperties
}

/// Properties for the `LogStream` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogStreamProperties {
    /// Property `LogGroupName`.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: ::Value<String>,
    /// Property `LogStreamName`.
    #[serde(rename = "LogStreamName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for LogStream {
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
#[derive(Debug)]
pub struct MetricFilter {
    properties: MetricFilterProperties
}

/// Properties for the `MetricFilter` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricFilterProperties {
    /// Property `FilterPattern`.
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: ::Value<String>,
    /// Property `LogGroupName`.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: ::Value<String>,
    /// Property `MetricTransformations`.
    #[serde(rename = "MetricTransformations")]
    pub metric_transformations: ::ValueList<self::metric_filter::MetricTransformation>,
}

impl<'a> ::Resource<'a> for MetricFilter {
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

/// The [`AWS::Logs::SubscriptionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html) resource type.
#[derive(Debug)]
pub struct SubscriptionFilter {
    properties: SubscriptionFilterProperties
}

/// Properties for the `SubscriptionFilter` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionFilterProperties {
    /// Property `DestinationArn`.
    #[serde(rename = "DestinationArn")]
    pub destination_arn: ::Value<String>,
    /// Property `FilterPattern`.
    #[serde(rename = "FilterPattern")]
    pub filter_pattern: ::Value<String>,
    /// Property `LogGroupName`.
    #[serde(rename = "LogGroupName")]
    pub log_group_name: ::Value<String>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for SubscriptionFilter {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MetricTransformation {
        /// Property `MetricName`.
        #[serde(rename = "MetricName")]
        pub metric_name: ::Value<String>,
        /// Property `MetricNamespace`.
        #[serde(rename = "MetricNamespace")]
        pub metric_namespace: ::Value<String>,
        /// Property `MetricValue`.
        #[serde(rename = "MetricValue")]
        pub metric_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MetricTransformation);
}
