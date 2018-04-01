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
    #[serde(rename = "ConfigRuleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<::Value<String>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `InputParameters`.
    #[serde(rename = "InputParameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<::Value<::json::Value>>,
    /// Property `MaximumExecutionFrequency`.
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<::Value<String>>,
    /// Property `Scope`.
    #[serde(rename = "Scope")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<::Value<self::config_rule::Scope>>,
    /// Property `Source`.
    #[serde(rename = "Source")]
    pub source: ::Value<self::config_rule::Source>,
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
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `RecordingGroup`.
    #[serde(rename = "RecordingGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<::Value<self::configuration_recorder::RecordingGroup>>,
    /// Property `RoleARN`.
    #[serde(rename = "RoleARN")]
    pub role_arn: ::Value<String>,
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
    #[serde(rename = "ConfigSnapshotDeliveryProperties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_properties: Option<::Value<self::delivery_channel::ConfigSnapshotDeliveryProperties>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `S3BucketName`.
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: ::Value<String>,
    /// Property `S3KeyPrefix`.
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<::Value<String>>,
    /// Property `SnsTopicARN`.
    #[serde(rename = "SnsTopicARN")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<::Value<String>>,
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
        #[serde(rename = "ComplianceResourceId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compliance_resource_id: Option<::Value<String>>,
        /// Property `ComplianceResourceTypes`.
        #[serde(rename = "ComplianceResourceTypes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compliance_resource_types: Option<::ValueList<String>>,
        /// Property `TagKey`.
        #[serde(rename = "TagKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_key: Option<::Value<String>>,
        /// Property `TagValue`.
        #[serde(rename = "TagValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag_value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Scope);

    /// The [`AWS::Config::ConfigRule.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Owner`.
        #[serde(rename = "Owner")]
        pub owner: ::Value<String>,
        /// Property `SourceDetails`.
        #[serde(rename = "SourceDetails")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_details: Option<::ValueList<SourceDetail>>,
        /// Property `SourceIdentifier`.
        #[serde(rename = "SourceIdentifier")]
        pub source_identifier: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Source);

    /// The [`AWS::Config::ConfigRule.SourceDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceDetail {
        /// Property `EventSource`.
        #[serde(rename = "EventSource")]
        pub event_source: ::Value<String>,
        /// Property `MaximumExecutionFrequency`.
        #[serde(rename = "MaximumExecutionFrequency")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maximum_execution_frequency: Option<::Value<String>>,
        /// Property `MessageType`.
        #[serde(rename = "MessageType")]
        pub message_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SourceDetail);
}

pub mod configuration_recorder {
    //! Property types for the `ConfigurationRecorder` resource.

    /// The [`AWS::Config::ConfigurationRecorder.RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordingGroup {
        /// Property `AllSupported`.
        #[serde(rename = "AllSupported")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub all_supported: Option<::Value<bool>>,
        /// Property `IncludeGlobalResourceTypes`.
        #[serde(rename = "IncludeGlobalResourceTypes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub include_global_resource_types: Option<::Value<bool>>,
        /// Property `ResourceTypes`.
        #[serde(rename = "ResourceTypes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_types: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(RecordingGroup);
}

pub mod delivery_channel {
    //! Property types for the `DeliveryChannel` resource.

    /// The [`AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigSnapshotDeliveryProperties {
        /// Property `DeliveryFrequency`.
        #[serde(rename = "DeliveryFrequency")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delivery_frequency: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ConfigSnapshotDeliveryProperties);
}
