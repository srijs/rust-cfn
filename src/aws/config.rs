/// The [`AWS::Config::ConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigRule {
    properties: ConfigRuleProperties
}

/// Properties for the `ConfigRule` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigRuleProperties {
    #[serde(rename="ConfigRuleName")]
    pub config_rule_name: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="InputParameters")]
    pub input_parameters: ::serde_json::Value,
    #[serde(rename="MaximumExecutionFrequency")]
    pub maximum_execution_frequency: String,
    #[serde(rename="Scope")]
    pub scope: (),
    #[serde(rename="Source")]
    pub source: (),
}

impl<'a> ::Resource<'a> for ConfigRule {
    type Properties = ConfigRuleProperties;
    const TYPE: &'static str = "AWS::Config::ConfigRule";
    fn properties(&self) -> &ConfigRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigRuleProperties {
        &mut self.properties
    }
}

impl From<ConfigRuleProperties> for ConfigRule {
    fn from(properties: ConfigRuleProperties) -> ConfigRule {
        ConfigRule { properties }
    }
}

/// The [`AWS::Config::ConfigurationRecorder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationRecorder {
    properties: ConfigurationRecorderProperties
}

/// Properties for the `ConfigurationRecorder` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationRecorderProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RecordingGroup")]
    pub recording_group: (),
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

impl<'a> ::Resource<'a> for ConfigurationRecorder {
    type Properties = ConfigurationRecorderProperties;
    const TYPE: &'static str = "AWS::Config::ConfigurationRecorder";
    fn properties(&self) -> &ConfigurationRecorderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationRecorderProperties {
        &mut self.properties
    }
}

impl From<ConfigurationRecorderProperties> for ConfigurationRecorder {
    fn from(properties: ConfigurationRecorderProperties) -> ConfigurationRecorder {
        ConfigurationRecorder { properties }
    }
}

/// The [`AWS::Config::DeliveryChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryChannel {
    properties: DeliveryChannelProperties
}

/// Properties for the `DeliveryChannel` resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryChannelProperties {
    #[serde(rename="ConfigSnapshotDeliveryProperties")]
    pub config_snapshot_delivery_properties: (),
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="S3BucketName")]
    pub s3_bucket_name: String,
    #[serde(rename="S3KeyPrefix")]
    pub s3_key_prefix: String,
    #[serde(rename="SnsTopicARN")]
    pub sns_topic_arn: String,
}

impl<'a> ::Resource<'a> for DeliveryChannel {
    type Properties = DeliveryChannelProperties;
    const TYPE: &'static str = "AWS::Config::DeliveryChannel";
    fn properties(&self) -> &DeliveryChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeliveryChannelProperties {
        &mut self.properties
    }
}

impl From<DeliveryChannelProperties> for DeliveryChannel {
    fn from(properties: DeliveryChannelProperties) -> DeliveryChannel {
        DeliveryChannel { properties }
    }
}

