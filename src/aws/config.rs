/// The [`AWS::Config::ConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html) resource type.
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
    pub input_parameters: ::json::Value,
    #[serde(rename="MaximumExecutionFrequency")]
    pub maximum_execution_frequency: String,
    #[serde(rename="Scope")]
    pub scope: self::config_rule::Scope,
    #[serde(rename="Source")]
    pub source: self::config_rule::Source,
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

impl ::private::Sealed for ConfigRule {}

impl From<ConfigRuleProperties> for ConfigRule {
    fn from(properties: ConfigRuleProperties) -> ConfigRule {
        ConfigRule { properties }
    }
}

/// The [`AWS::Config::ConfigurationRecorder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html) resource type.
pub struct ConfigurationRecorder {
    properties: ConfigurationRecorderProperties
}

/// Properties for the `ConfigurationRecorder` resource.
#[derive(Serialize, Deserialize)]
pub struct ConfigurationRecorderProperties {
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RecordingGroup")]
    pub recording_group: self::configuration_recorder::RecordingGroup,
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

impl ::private::Sealed for ConfigurationRecorder {}

impl From<ConfigurationRecorderProperties> for ConfigurationRecorder {
    fn from(properties: ConfigurationRecorderProperties) -> ConfigurationRecorder {
        ConfigurationRecorder { properties }
    }
}

/// The [`AWS::Config::DeliveryChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html) resource type.
pub struct DeliveryChannel {
    properties: DeliveryChannelProperties
}

/// Properties for the `DeliveryChannel` resource.
#[derive(Serialize, Deserialize)]
pub struct DeliveryChannelProperties {
    #[serde(rename="ConfigSnapshotDeliveryProperties")]
    pub config_snapshot_delivery_properties: self::delivery_channel::ConfigSnapshotDeliveryProperties,
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

impl ::private::Sealed for DeliveryChannel {}

impl From<DeliveryChannelProperties> for DeliveryChannel {
    fn from(properties: DeliveryChannelProperties) -> DeliveryChannel {
        DeliveryChannel { properties }
    }
}

pub mod config_rule {
    /// The [`AWS::Config::ConfigRule.Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Scope {
        #[serde(rename="ComplianceResourceId")]
        pub compliance_resource_id: String,
        #[serde(rename="ComplianceResourceTypes")]
        pub compliance_resource_types: Vec<String>,
        #[serde(rename="TagKey")]
        pub tag_key: String,
        #[serde(rename="TagValue")]
        pub tag_value: String,
    }

    /// The [`AWS::Config::ConfigRule.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Source {
        #[serde(rename="Owner")]
        pub owner: String,
        #[serde(rename="SourceDetails")]
        pub source_details: Vec<SourceDetail>,
        #[serde(rename="SourceIdentifier")]
        pub source_identifier: String,
    }

    /// The [`AWS::Config::ConfigRule.SourceDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SourceDetail {
        #[serde(rename="EventSource")]
        pub event_source: String,
        #[serde(rename="MaximumExecutionFrequency")]
        pub maximum_execution_frequency: String,
        #[serde(rename="MessageType")]
        pub message_type: String,
    }

}

pub mod configuration_recorder {
    /// The [`AWS::Config::ConfigurationRecorder.RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct RecordingGroup {
        #[serde(rename="AllSupported")]
        pub all_supported: bool,
        #[serde(rename="IncludeGlobalResourceTypes")]
        pub include_global_resource_types: bool,
        #[serde(rename="ResourceTypes")]
        pub resource_types: Vec<String>,
    }

}

pub mod delivery_channel {
    /// The [`AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ConfigSnapshotDeliveryProperties {
        #[serde(rename="DeliveryFrequency")]
        pub delivery_frequency: String,
    }

}

