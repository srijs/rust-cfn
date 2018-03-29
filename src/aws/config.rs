//! Types for the `Config` service.

/// The [`AWS::Config::ConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html) resource type.
#[derive(Debug)]
pub struct ConfigRule {
    properties: ConfigRuleProperties
}

/// Properties for the `ConfigRule` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRuleProperties {
    /// Property `ConfigRuleName`.
    #[serde(rename="ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `InputParameters`.
    #[serde(rename="InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<::json::Value>,
    /// Property `MaximumExecutionFrequency`.
    #[serde(rename="MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// Property `Scope`.
    #[serde(rename="Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<self::config_rule::Scope>,
    /// Property `Source`.
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
#[derive(Debug)]
pub struct ConfigurationRecorder {
    properties: ConfigurationRecorderProperties
}

/// Properties for the `ConfigurationRecorder` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationRecorderProperties {
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `RecordingGroup`.
    #[serde(rename="RecordingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<self::configuration_recorder::RecordingGroup>,
    /// Property `RoleARN`.
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
#[derive(Debug)]
pub struct DeliveryChannel {
    properties: DeliveryChannelProperties
}

/// Properties for the `DeliveryChannel` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryChannelProperties {
    /// Property `ConfigSnapshotDeliveryProperties`.
    #[serde(rename="ConfigSnapshotDeliveryProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_properties: Option<self::delivery_channel::ConfigSnapshotDeliveryProperties>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `S3BucketName`.
    #[serde(rename="S3BucketName")]
    pub s3_bucket_name: String,
    /// Property `S3KeyPrefix`.
    #[serde(rename="S3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    /// Property `SnsTopicARN`.
    #[serde(rename="SnsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
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
    //! Property types for the `ConfigRule` resource.

    /// The [`AWS::Config::ConfigRule.Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Scope {
        /// Property `ComplianceResourceId`.
        #[serde(rename="ComplianceResourceId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compliance_resource_id: Option<String>,
        /// Property `ComplianceResourceTypes`.
        #[serde(rename="ComplianceResourceTypes")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compliance_resource_types: Option<Vec<String>>,
        /// Property `TagKey`.
        #[serde(rename="TagKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag_key: Option<String>,
        /// Property `TagValue`.
        #[serde(rename="TagValue")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag_value: Option<String>,
    }

    /// The [`AWS::Config::ConfigRule.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Owner`.
        #[serde(rename="Owner")]
        pub owner: String,
        /// Property `SourceDetails`.
        #[serde(rename="SourceDetails")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_details: Option<Vec<SourceDetail>>,
        /// Property `SourceIdentifier`.
        #[serde(rename="SourceIdentifier")]
        pub source_identifier: String,
    }

    /// The [`AWS::Config::ConfigRule.SourceDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceDetail {
        /// Property `EventSource`.
        #[serde(rename="EventSource")]
        pub event_source: String,
        /// Property `MaximumExecutionFrequency`.
        #[serde(rename="MaximumExecutionFrequency")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maximum_execution_frequency: Option<String>,
        /// Property `MessageType`.
        #[serde(rename="MessageType")]
        pub message_type: String,
    }
}

pub mod configuration_recorder {
    //! Property types for the `ConfigurationRecorder` resource.

    /// The [`AWS::Config::ConfigurationRecorder.RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordingGroup {
        /// Property `AllSupported`.
        #[serde(rename="AllSupported")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub all_supported: Option<bool>,
        /// Property `IncludeGlobalResourceTypes`.
        #[serde(rename="IncludeGlobalResourceTypes")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include_global_resource_types: Option<bool>,
        /// Property `ResourceTypes`.
        #[serde(rename="ResourceTypes")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_types: Option<Vec<String>>,
    }
}

pub mod delivery_channel {
    //! Property types for the `DeliveryChannel` resource.

    /// The [`AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigSnapshotDeliveryProperties {
        /// Property `DeliveryFrequency`.
        #[serde(rename="DeliveryFrequency")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delivery_frequency: Option<String>,
    }
}
