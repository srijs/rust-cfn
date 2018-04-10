//! Types for the `Config` service.

/// The [`AWS::Config::ConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html) resource type.
#[derive(Debug)]
pub struct ConfigRule {
    properties: ConfigRuleProperties
}

/// Properties for the `ConfigRule` resource.
#[derive(Debug)]
pub struct ConfigRuleProperties {
    /// Property `ConfigRuleName`.
    pub config_rule_name: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `InputParameters`.
    pub input_parameters: Option<::Value<::json::Value>>,
    /// Property `MaximumExecutionFrequency`.
    pub maximum_execution_frequency: Option<::Value<String>>,
    /// Property `Scope`.
    pub scope: Option<::Value<self::config_rule::Scope>>,
    /// Property `Source`.
    pub source: ::Value<self::config_rule::Source>,
}

impl ::serde::Serialize for ConfigRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref config_rule_name) = self.config_rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigRuleName", config_rule_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref input_parameters) = self.input_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParameters", input_parameters)?;
        }
        if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
        }
        if let Some(ref scope) = self.scope {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config_rule_name = None;
                let mut description = None;
                let mut input_parameters = None;
                let mut maximum_execution_frequency = None;
                let mut scope = None;
                let mut source = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigRuleName" => {
                            config_rule_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InputParameters" => {
                            input_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaximumExecutionFrequency" => {
                            maximum_execution_frequency = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Scope" => {
                            scope = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Source" => {
                            source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ConfigRuleProperties {
                    config_rule_name: config_rule_name,
                    description: description,
                    input_parameters: input_parameters,
                    maximum_execution_frequency: maximum_execution_frequency,
                    scope: scope,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigRule {
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
#[derive(Debug)]
pub struct ConfigurationRecorderProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `RecordingGroup`.
    pub recording_group: Option<::Value<self::configuration_recorder::RecordingGroup>>,
    /// Property `RoleARN`.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for ConfigurationRecorderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref recording_group) = self.recording_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordingGroup", recording_group)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationRecorderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationRecorderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationRecorderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationRecorderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;
                let mut recording_group = None;
                let mut role_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RecordingGroup" => {
                            recording_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleARN" => {
                            role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationRecorderProperties {
                    name: name,
                    recording_group: recording_group,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationRecorder {
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
#[derive(Debug)]
pub struct DeliveryChannelProperties {
    /// Property `ConfigSnapshotDeliveryProperties`.
    pub config_snapshot_delivery_properties: Option<::Value<self::delivery_channel::ConfigSnapshotDeliveryProperties>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `S3BucketName`.
    pub s3_bucket_name: ::Value<String>,
    /// Property `S3KeyPrefix`.
    pub s3_key_prefix: Option<::Value<String>>,
    /// Property `SnsTopicARN`.
    pub sns_topic_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DeliveryChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref config_snapshot_delivery_properties) = self.config_snapshot_delivery_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigSnapshotDeliveryProperties", config_snapshot_delivery_properties)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
        if let Some(ref s3_key_prefix) = self.s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
        }
        if let Some(ref sns_topic_arn) = self.sns_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicARN", sns_topic_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeliveryChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeliveryChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeliveryChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeliveryChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config_snapshot_delivery_properties = None;
                let mut name = None;
                let mut s3_bucket_name = None;
                let mut s3_key_prefix = None;
                let mut sns_topic_arn = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigSnapshotDeliveryProperties" => {
                            config_snapshot_delivery_properties = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3BucketName" => {
                            s3_bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "S3KeyPrefix" => {
                            s3_key_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnsTopicARN" => {
                            sns_topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DeliveryChannelProperties {
                    config_snapshot_delivery_properties: config_snapshot_delivery_properties,
                    name: name,
                    s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                    s3_key_prefix: s3_key_prefix,
                    sns_topic_arn: sns_topic_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeliveryChannel {
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
    #[derive(Debug)]
    pub struct Scope {
        /// Property `ComplianceResourceId`.
        pub compliance_resource_id: Option<::Value<String>>,
        /// Property `ComplianceResourceTypes`.
        pub compliance_resource_types: Option<::ValueList<String>>,
        /// Property `TagKey`.
        pub tag_key: Option<::Value<String>>,
        /// Property `TagValue`.
        pub tag_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Scope {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compliance_resource_id) = self.compliance_resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceId", compliance_resource_id)?;
            }
            if let Some(ref compliance_resource_types) = self.compliance_resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceTypes", compliance_resource_types)?;
            }
            if let Some(ref tag_key) = self.tag_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", tag_key)?;
            }
            if let Some(ref tag_value) = self.tag_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValue", tag_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scope {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scope, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scope;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scope")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compliance_resource_id = None;
                    let mut compliance_resource_types = None;
                    let mut tag_key = None;
                    let mut tag_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComplianceResourceId" => {
                                compliance_resource_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ComplianceResourceTypes" => {
                                compliance_resource_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TagKey" => {
                                tag_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TagValue" => {
                                tag_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Scope {
                        compliance_resource_id: compliance_resource_id,
                        compliance_resource_types: compliance_resource_types,
                        tag_key: tag_key,
                        tag_value: tag_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::ConfigRule.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html) property type.
    #[derive(Debug)]
    pub struct Source {
        /// Property `Owner`.
        pub owner: ::Value<String>,
        /// Property `SourceDetails`.
        pub source_details: Option<::ValueList<SourceDetail>>,
        /// Property `SourceIdentifier`.
        pub source_identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", &self.owner)?;
            if let Some(ref source_details) = self.source_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDetails", source_details)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIdentifier", &self.source_identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut owner = None;
                    let mut source_details = None;
                    let mut source_identifier = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Owner" => {
                                owner = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceDetails" => {
                                source_details = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceIdentifier" => {
                                source_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        owner: owner.ok_or(::serde::de::Error::missing_field("Owner"))?,
                        source_details: source_details,
                        source_identifier: source_identifier.ok_or(::serde::de::Error::missing_field("SourceIdentifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::ConfigRule.SourceDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html) property type.
    #[derive(Debug)]
    pub struct SourceDetail {
        /// Property `EventSource`.
        pub event_source: ::Value<String>,
        /// Property `MaximumExecutionFrequency`.
        pub maximum_execution_frequency: Option<::Value<String>>,
        /// Property `MessageType`.
        pub message_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceDetail {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSource", &self.event_source)?;
            if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageType", &self.message_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceDetail {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceDetail, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceDetail;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceDetail")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_source = None;
                    let mut maximum_execution_frequency = None;
                    let mut message_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventSource" => {
                                event_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaximumExecutionFrequency" => {
                                maximum_execution_frequency = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MessageType" => {
                                message_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceDetail {
                        event_source: event_source.ok_or(::serde::de::Error::missing_field("EventSource"))?,
                        maximum_execution_frequency: maximum_execution_frequency,
                        message_type: message_type.ok_or(::serde::de::Error::missing_field("MessageType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_recorder {
    //! Property types for the `ConfigurationRecorder` resource.

    /// The [`AWS::Config::ConfigurationRecorder.RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html) property type.
    #[derive(Debug)]
    pub struct RecordingGroup {
        /// Property `AllSupported`.
        pub all_supported: Option<::Value<bool>>,
        /// Property `IncludeGlobalResourceTypes`.
        pub include_global_resource_types: Option<::Value<bool>>,
        /// Property `ResourceTypes`.
        pub resource_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RecordingGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_supported) = self.all_supported {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllSupported", all_supported)?;
            }
            if let Some(ref include_global_resource_types) = self.include_global_resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeGlobalResourceTypes", include_global_resource_types)?;
            }
            if let Some(ref resource_types) = self.resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypes", resource_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordingGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordingGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordingGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordingGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_supported = None;
                    let mut include_global_resource_types = None;
                    let mut resource_types = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllSupported" => {
                                all_supported = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IncludeGlobalResourceTypes" => {
                                include_global_resource_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ResourceTypes" => {
                                resource_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordingGroup {
                        all_supported: all_supported,
                        include_global_resource_types: include_global_resource_types,
                        resource_types: resource_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod delivery_channel {
    //! Property types for the `DeliveryChannel` resource.

    /// The [`AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html) property type.
    #[derive(Debug)]
    pub struct ConfigSnapshotDeliveryProperties {
        /// Property `DeliveryFrequency`.
        pub delivery_frequency: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigSnapshotDeliveryProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delivery_frequency) = self.delivery_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryFrequency", delivery_frequency)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigSnapshotDeliveryProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigSnapshotDeliveryProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigSnapshotDeliveryProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigSnapshotDeliveryProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_frequency = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryFrequency" => {
                                delivery_frequency = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigSnapshotDeliveryProperties {
                        delivery_frequency: delivery_frequency,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
