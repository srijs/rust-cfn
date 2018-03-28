/// The [`AWS::Logs::Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-destination.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Destination {
    properties: DestinationProperties
}

/// Properties for the `Destination` resource.
#[derive(Serialize, Deserialize)]
pub struct DestinationProperties {
    #[serde(rename="DestinationName")]
    pub destination_name: String,
    #[serde(rename="DestinationPolicy")]
    pub destination_policy: String,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[serde(rename="TargetArn")]
    pub target_arn: String,
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

impl From<DestinationProperties> for Destination {
    fn from(properties: DestinationProperties) -> Destination {
        Destination { properties }
    }
}

/// The [`AWS::Logs::LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-loggroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LogGroup {
    properties: LogGroupProperties
}

/// Properties for the `LogGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct LogGroupProperties {
    #[serde(rename="LogGroupName")]
    pub log_group_name: String,
    #[serde(rename="RetentionInDays")]
    pub retention_in_days: u32,
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

impl From<LogGroupProperties> for LogGroup {
    fn from(properties: LogGroupProperties) -> LogGroup {
        LogGroup { properties }
    }
}

/// The [`AWS::Logs::LogStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-logstream.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LogStream {
    properties: LogStreamProperties
}

/// Properties for the `LogStream` resource.
#[derive(Serialize, Deserialize)]
pub struct LogStreamProperties {
    #[serde(rename="LogGroupName")]
    pub log_group_name: String,
    #[serde(rename="LogStreamName")]
    pub log_stream_name: String,
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

impl From<LogStreamProperties> for LogStream {
    fn from(properties: LogStreamProperties) -> LogStream {
        LogStream { properties }
    }
}

/// The [`AWS::Logs::MetricFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-metricfilter.html) resource.
#[derive(Serialize, Deserialize)]
pub struct MetricFilter {
    properties: MetricFilterProperties
}

/// Properties for the `MetricFilter` resource.
#[derive(Serialize, Deserialize)]
pub struct MetricFilterProperties {
    #[serde(rename="FilterPattern")]
    pub filter_pattern: String,
    #[serde(rename="LogGroupName")]
    pub log_group_name: String,
    #[serde(rename="MetricTransformations")]
    pub metric_transformations: Vec<()>,
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

impl From<MetricFilterProperties> for MetricFilter {
    fn from(properties: MetricFilterProperties) -> MetricFilter {
        MetricFilter { properties }
    }
}

/// The [`AWS::Logs::SubscriptionFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-logs-subscriptionfilter.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SubscriptionFilter {
    properties: SubscriptionFilterProperties
}

/// Properties for the `SubscriptionFilter` resource.
#[derive(Serialize, Deserialize)]
pub struct SubscriptionFilterProperties {
    #[serde(rename="DestinationArn")]
    pub destination_arn: String,
    #[serde(rename="FilterPattern")]
    pub filter_pattern: String,
    #[serde(rename="LogGroupName")]
    pub log_group_name: String,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
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

impl From<SubscriptionFilterProperties> for SubscriptionFilter {
    fn from(properties: SubscriptionFilterProperties) -> SubscriptionFilter {
        SubscriptionFilter { properties }
    }
}

