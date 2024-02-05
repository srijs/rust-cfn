//! Types for the `MediaTailor` service.

/// The [`AWS::MediaTailor::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: ::Value<String>,
    /// Property [`FillerSlate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-fillerslate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filler_slate: Option<::Value<self::channel::SlateSource>>,
    /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-logconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_configuration: Option<::Value<self::channel::LogConfigurationForChannel>>,
    /// Property [`Outputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-outputs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub outputs: ::ValueList<self::channel::RequestOutputItem>,
    /// Property [`PlaybackMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-playbackmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub playback_mode: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-tier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tier: Option<::Value<String>>,
    /// Property [`TimeShiftConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channel.html#cfn-mediatailor-channel-timeshiftconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_shift_configuration: Option<::Value<self::channel::TimeShiftConfiguration>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", &self.channel_name)?;
        if let Some(ref filler_slate) = self.filler_slate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FillerSlate", filler_slate)?;
        }
        if let Some(ref log_configuration) = self.log_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Outputs", &self.outputs)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlaybackMode", &self.playback_mode)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tier) = self.tier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", tier)?;
        }
        if let Some(ref time_shift_configuration) = self.time_shift_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeShiftConfiguration", time_shift_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_name: Option<::Value<String>> = None;
                let mut filler_slate: Option<::Value<self::channel::SlateSource>> = None;
                let mut log_configuration: Option<::Value<self::channel::LogConfigurationForChannel>> = None;
                let mut outputs: Option<::ValueList<self::channel::RequestOutputItem>> = None;
                let mut playback_mode: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tier: Option<::Value<String>> = None;
                let mut time_shift_configuration: Option<::Value<self::channel::TimeShiftConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FillerSlate" => {
                            filler_slate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogConfiguration" => {
                            log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Outputs" => {
                            outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlaybackMode" => {
                            playback_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tier" => {
                            tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeShiftConfiguration" => {
                            time_shift_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    channel_name: channel_name.ok_or(::serde::de::Error::missing_field("ChannelName"))?,
                    filler_slate: filler_slate,
                    log_configuration: log_configuration,
                    outputs: outputs.ok_or(::serde::de::Error::missing_field("Outputs"))?,
                    playback_mode: playback_mode.ok_or(::serde::de::Error::missing_field("PlaybackMode"))?,
                    tags: tags,
                    tier: tier,
                    time_shift_configuration: time_shift_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::MediaTailor::Channel";
    fn properties(&self) -> &ChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Channel {}

impl From<ChannelProperties> for Channel {
    fn from(properties: ChannelProperties) -> Channel {
        Channel { properties }
    }
}

/// The [`AWS::MediaTailor::ChannelPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channelpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ChannelPolicy {
    properties: ChannelPolicyProperties
}

/// Properties for the `ChannelPolicy` resource.
#[derive(Debug, Default)]
pub struct ChannelPolicyProperties {
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channelpolicy.html#cfn-mediatailor-channelpolicy-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: ::Value<String>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-channelpolicy.html#cfn-mediatailor-channelpolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::Value<::json::Value>,
}

impl ::serde::Serialize for ChannelPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", &self.channel_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ChannelPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChannelPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ChannelPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_name: Option<::Value<String>> = None;
                let mut policy: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelPolicyProperties {
                    channel_name: channel_name.ok_or(::serde::de::Error::missing_field("ChannelName"))?,
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ChannelPolicy {
    type Properties = ChannelPolicyProperties;
    const TYPE: &'static str = "AWS::MediaTailor::ChannelPolicy";
    fn properties(&self) -> &ChannelPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ChannelPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ChannelPolicy {}

impl From<ChannelPolicyProperties> for ChannelPolicy {
    fn from(properties: ChannelPolicyProperties) -> ChannelPolicy {
        ChannelPolicy { properties }
    }
}

/// The [`AWS::MediaTailor::LiveSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html) resource type.
#[derive(Debug, Default)]
pub struct LiveSource {
    properties: LiveSourceProperties
}

/// Properties for the `LiveSource` resource.
#[derive(Debug, Default)]
pub struct LiveSourceProperties {
    /// Property [`HttpPackageConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html#cfn-mediatailor-livesource-httppackageconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_package_configurations: ::ValueList<self::live_source::HttpPackageConfiguration>,
    /// Property [`LiveSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html#cfn-mediatailor-livesource-livesourcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub live_source_name: ::Value<String>,
    /// Property [`SourceLocationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html#cfn-mediatailor-livesource-sourcelocationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_location_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-livesource.html#cfn-mediatailor-livesource-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LiveSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpPackageConfigurations", &self.http_package_configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LiveSourceName", &self.live_source_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLocationName", &self.source_location_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LiveSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LiveSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LiveSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LiveSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut http_package_configurations: Option<::ValueList<self::live_source::HttpPackageConfiguration>> = None;
                let mut live_source_name: Option<::Value<String>> = None;
                let mut source_location_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HttpPackageConfigurations" => {
                            http_package_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LiveSourceName" => {
                            live_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceLocationName" => {
                            source_location_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LiveSourceProperties {
                    http_package_configurations: http_package_configurations.ok_or(::serde::de::Error::missing_field("HttpPackageConfigurations"))?,
                    live_source_name: live_source_name.ok_or(::serde::de::Error::missing_field("LiveSourceName"))?,
                    source_location_name: source_location_name.ok_or(::serde::de::Error::missing_field("SourceLocationName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LiveSource {
    type Properties = LiveSourceProperties;
    const TYPE: &'static str = "AWS::MediaTailor::LiveSource";
    fn properties(&self) -> &LiveSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LiveSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LiveSource {}

impl From<LiveSourceProperties> for LiveSource {
    fn from(properties: LiveSourceProperties) -> LiveSource {
        LiveSource { properties }
    }
}

/// The [`AWS::MediaTailor::PlaybackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct PlaybackConfiguration {
    properties: PlaybackConfigurationProperties
}

/// Properties for the `PlaybackConfiguration` resource.
#[derive(Debug, Default)]
pub struct PlaybackConfigurationProperties {
    /// Property [`AdDecisionServerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-addecisionserverurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ad_decision_server_url: ::Value<String>,
    /// Property [`AvailSuppression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-availsuppression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub avail_suppression: Option<::Value<self::playback_configuration::AvailSuppression>>,
    /// Property [`Bumper`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-bumper).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bumper: Option<::Value<self::playback_configuration::Bumper>>,
    /// Property [`CdnConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-cdnconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cdn_configuration: Option<::Value<self::playback_configuration::CdnConfiguration>>,
    /// Property [`ConfigurationAliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-configurationaliases).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration_aliases: Option<::ValueMap<::json::Value>>,
    /// Property [`DashConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-dashconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dash_configuration: Option<::Value<self::playback_configuration::DashConfiguration>>,
    /// Property [`HlsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-hlsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hls_configuration: Option<::Value<self::playback_configuration::HlsConfiguration>>,
    /// Property [`LivePreRollConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-liveprerollconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub live_pre_roll_configuration: Option<::Value<self::playback_configuration::LivePreRollConfiguration>>,
    /// Property [`ManifestProcessingRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-manifestprocessingrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manifest_processing_rules: Option<::Value<self::playback_configuration::ManifestProcessingRules>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PersonalizationThresholdSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-personalizationthresholdseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub personalization_threshold_seconds: Option<::Value<u32>>,
    /// Property [`SlateAdUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-slateadurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub slate_ad_url: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TranscodeProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-transcodeprofilename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transcode_profile_name: Option<::Value<String>>,
    /// Property [`VideoContentSourceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-videocontentsourceurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub video_content_source_url: ::Value<String>,
}

impl ::serde::Serialize for PlaybackConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdDecisionServerUrl", &self.ad_decision_server_url)?;
        if let Some(ref avail_suppression) = self.avail_suppression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailSuppression", avail_suppression)?;
        }
        if let Some(ref bumper) = self.bumper {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bumper", bumper)?;
        }
        if let Some(ref cdn_configuration) = self.cdn_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdnConfiguration", cdn_configuration)?;
        }
        if let Some(ref configuration_aliases) = self.configuration_aliases {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationAliases", configuration_aliases)?;
        }
        if let Some(ref dash_configuration) = self.dash_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashConfiguration", dash_configuration)?;
        }
        if let Some(ref hls_configuration) = self.hls_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsConfiguration", hls_configuration)?;
        }
        if let Some(ref live_pre_roll_configuration) = self.live_pre_roll_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LivePreRollConfiguration", live_pre_roll_configuration)?;
        }
        if let Some(ref manifest_processing_rules) = self.manifest_processing_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestProcessingRules", manifest_processing_rules)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref personalization_threshold_seconds) = self.personalization_threshold_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PersonalizationThresholdSeconds", personalization_threshold_seconds)?;
        }
        if let Some(ref slate_ad_url) = self.slate_ad_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlateAdUrl", slate_ad_url)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref transcode_profile_name) = self.transcode_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TranscodeProfileName", transcode_profile_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoContentSourceUrl", &self.video_content_source_url)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PlaybackConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PlaybackConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PlaybackConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PlaybackConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ad_decision_server_url: Option<::Value<String>> = None;
                let mut avail_suppression: Option<::Value<self::playback_configuration::AvailSuppression>> = None;
                let mut bumper: Option<::Value<self::playback_configuration::Bumper>> = None;
                let mut cdn_configuration: Option<::Value<self::playback_configuration::CdnConfiguration>> = None;
                let mut configuration_aliases: Option<::ValueMap<::json::Value>> = None;
                let mut dash_configuration: Option<::Value<self::playback_configuration::DashConfiguration>> = None;
                let mut hls_configuration: Option<::Value<self::playback_configuration::HlsConfiguration>> = None;
                let mut live_pre_roll_configuration: Option<::Value<self::playback_configuration::LivePreRollConfiguration>> = None;
                let mut manifest_processing_rules: Option<::Value<self::playback_configuration::ManifestProcessingRules>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut personalization_threshold_seconds: Option<::Value<u32>> = None;
                let mut slate_ad_url: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transcode_profile_name: Option<::Value<String>> = None;
                let mut video_content_source_url: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdDecisionServerUrl" => {
                            ad_decision_server_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailSuppression" => {
                            avail_suppression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Bumper" => {
                            bumper = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CdnConfiguration" => {
                            cdn_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationAliases" => {
                            configuration_aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashConfiguration" => {
                            dash_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HlsConfiguration" => {
                            hls_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LivePreRollConfiguration" => {
                            live_pre_roll_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManifestProcessingRules" => {
                            manifest_processing_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PersonalizationThresholdSeconds" => {
                            personalization_threshold_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SlateAdUrl" => {
                            slate_ad_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TranscodeProfileName" => {
                            transcode_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VideoContentSourceUrl" => {
                            video_content_source_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PlaybackConfigurationProperties {
                    ad_decision_server_url: ad_decision_server_url.ok_or(::serde::de::Error::missing_field("AdDecisionServerUrl"))?,
                    avail_suppression: avail_suppression,
                    bumper: bumper,
                    cdn_configuration: cdn_configuration,
                    configuration_aliases: configuration_aliases,
                    dash_configuration: dash_configuration,
                    hls_configuration: hls_configuration,
                    live_pre_roll_configuration: live_pre_roll_configuration,
                    manifest_processing_rules: manifest_processing_rules,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    personalization_threshold_seconds: personalization_threshold_seconds,
                    slate_ad_url: slate_ad_url,
                    tags: tags,
                    transcode_profile_name: transcode_profile_name,
                    video_content_source_url: video_content_source_url.ok_or(::serde::de::Error::missing_field("VideoContentSourceUrl"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PlaybackConfiguration {
    type Properties = PlaybackConfigurationProperties;
    const TYPE: &'static str = "AWS::MediaTailor::PlaybackConfiguration";
    fn properties(&self) -> &PlaybackConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlaybackConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PlaybackConfiguration {}

impl From<PlaybackConfigurationProperties> for PlaybackConfiguration {
    fn from(properties: PlaybackConfigurationProperties) -> PlaybackConfiguration {
        PlaybackConfiguration { properties }
    }
}

/// The [`AWS::MediaTailor::SourceLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html) resource type.
#[derive(Debug, Default)]
pub struct SourceLocation {
    properties: SourceLocationProperties
}

/// Properties for the `SourceLocation` resource.
#[derive(Debug, Default)]
pub struct SourceLocationProperties {
    /// Property [`AccessConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-accessconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_configuration: Option<::Value<self::source_location::AccessConfiguration>>,
    /// Property [`DefaultSegmentDeliveryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-defaultsegmentdeliveryconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_segment_delivery_configuration: Option<::Value<self::source_location::DefaultSegmentDeliveryConfiguration>>,
    /// Property [`HttpConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-httpconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_configuration: ::Value<self::source_location::HttpConfiguration>,
    /// Property [`SegmentDeliveryConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-segmentdeliveryconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment_delivery_configurations: Option<::ValueList<self::source_location::SegmentDeliveryConfiguration>>,
    /// Property [`SourceLocationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-sourcelocationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_location_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-sourcelocation.html#cfn-mediatailor-sourcelocation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SourceLocationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_configuration) = self.access_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessConfiguration", access_configuration)?;
        }
        if let Some(ref default_segment_delivery_configuration) = self.default_segment_delivery_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSegmentDeliveryConfiguration", default_segment_delivery_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpConfiguration", &self.http_configuration)?;
        if let Some(ref segment_delivery_configurations) = self.segment_delivery_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDeliveryConfigurations", segment_delivery_configurations)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLocationName", &self.source_location_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SourceLocationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceLocationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SourceLocationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SourceLocationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_configuration: Option<::Value<self::source_location::AccessConfiguration>> = None;
                let mut default_segment_delivery_configuration: Option<::Value<self::source_location::DefaultSegmentDeliveryConfiguration>> = None;
                let mut http_configuration: Option<::Value<self::source_location::HttpConfiguration>> = None;
                let mut segment_delivery_configurations: Option<::ValueList<self::source_location::SegmentDeliveryConfiguration>> = None;
                let mut source_location_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessConfiguration" => {
                            access_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultSegmentDeliveryConfiguration" => {
                            default_segment_delivery_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HttpConfiguration" => {
                            http_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SegmentDeliveryConfigurations" => {
                            segment_delivery_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceLocationName" => {
                            source_location_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SourceLocationProperties {
                    access_configuration: access_configuration,
                    default_segment_delivery_configuration: default_segment_delivery_configuration,
                    http_configuration: http_configuration.ok_or(::serde::de::Error::missing_field("HttpConfiguration"))?,
                    segment_delivery_configurations: segment_delivery_configurations,
                    source_location_name: source_location_name.ok_or(::serde::de::Error::missing_field("SourceLocationName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SourceLocation {
    type Properties = SourceLocationProperties;
    const TYPE: &'static str = "AWS::MediaTailor::SourceLocation";
    fn properties(&self) -> &SourceLocationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SourceLocationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SourceLocation {}

impl From<SourceLocationProperties> for SourceLocation {
    fn from(properties: SourceLocationProperties) -> SourceLocation {
        SourceLocation { properties }
    }
}

/// The [`AWS::MediaTailor::VodSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html) resource type.
#[derive(Debug, Default)]
pub struct VodSource {
    properties: VodSourceProperties
}

/// Properties for the `VodSource` resource.
#[derive(Debug, Default)]
pub struct VodSourceProperties {
    /// Property [`HttpPackageConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html#cfn-mediatailor-vodsource-httppackageconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_package_configurations: ::ValueList<self::vod_source::HttpPackageConfiguration>,
    /// Property [`SourceLocationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html#cfn-mediatailor-vodsource-sourcelocationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_location_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html#cfn-mediatailor-vodsource-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VodSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-vodsource.html#cfn-mediatailor-vodsource-vodsourcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vod_source_name: ::Value<String>,
}

impl ::serde::Serialize for VodSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpPackageConfigurations", &self.http_package_configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLocationName", &self.source_location_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VodSourceName", &self.vod_source_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VodSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VodSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VodSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VodSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut http_package_configurations: Option<::ValueList<self::vod_source::HttpPackageConfiguration>> = None;
                let mut source_location_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vod_source_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HttpPackageConfigurations" => {
                            http_package_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceLocationName" => {
                            source_location_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VodSourceName" => {
                            vod_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VodSourceProperties {
                    http_package_configurations: http_package_configurations.ok_or(::serde::de::Error::missing_field("HttpPackageConfigurations"))?,
                    source_location_name: source_location_name.ok_or(::serde::de::Error::missing_field("SourceLocationName"))?,
                    tags: tags,
                    vod_source_name: vod_source_name.ok_or(::serde::de::Error::missing_field("VodSourceName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VodSource {
    type Properties = VodSourceProperties;
    const TYPE: &'static str = "AWS::MediaTailor::VodSource";
    fn properties(&self) -> &VodSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VodSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VodSource {}

impl From<VodSourceProperties> for VodSource {
    fn from(properties: VodSourceProperties) -> VodSource {
        VodSource { properties }
    }
}

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::MediaTailor::Channel.DashPlaylistSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DashPlaylistSettings {
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html#cfn-mediatailor-channel-dashplaylistsettings-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<f64>>,
        /// Property [`MinBufferTimeSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html#cfn-mediatailor-channel-dashplaylistsettings-minbuffertimeseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_buffer_time_seconds: Option<::Value<f64>>,
        /// Property [`MinUpdatePeriodSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html#cfn-mediatailor-channel-dashplaylistsettings-minupdateperiodseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_update_period_seconds: Option<::Value<f64>>,
        /// Property [`SuggestedPresentationDelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-dashplaylistsettings.html#cfn-mediatailor-channel-dashplaylistsettings-suggestedpresentationdelayseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suggested_presentation_delay_seconds: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for DashPlaylistSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            if let Some(ref min_buffer_time_seconds) = self.min_buffer_time_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinBufferTimeSeconds", min_buffer_time_seconds)?;
            }
            if let Some(ref min_update_period_seconds) = self.min_update_period_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinUpdatePeriodSeconds", min_update_period_seconds)?;
            }
            if let Some(ref suggested_presentation_delay_seconds) = self.suggested_presentation_delay_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuggestedPresentationDelaySeconds", suggested_presentation_delay_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashPlaylistSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashPlaylistSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashPlaylistSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashPlaylistSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_window_seconds: Option<::Value<f64>> = None;
                    let mut min_buffer_time_seconds: Option<::Value<f64>> = None;
                    let mut min_update_period_seconds: Option<::Value<f64>> = None;
                    let mut suggested_presentation_delay_seconds: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinBufferTimeSeconds" => {
                                min_buffer_time_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinUpdatePeriodSeconds" => {
                                min_update_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuggestedPresentationDelaySeconds" => {
                                suggested_presentation_delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashPlaylistSettings {
                        manifest_window_seconds: manifest_window_seconds,
                        min_buffer_time_seconds: min_buffer_time_seconds,
                        min_update_period_seconds: min_update_period_seconds,
                        suggested_presentation_delay_seconds: suggested_presentation_delay_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::Channel.HlsPlaylistSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-hlsplaylistsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsPlaylistSettings {
        /// Property [`AdMarkupType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-hlsplaylistsettings.html#cfn-mediatailor-channel-hlsplaylistsettings-admarkuptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markup_type: Option<::ValueList<String>>,
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-hlsplaylistsettings.html#cfn-mediatailor-channel-hlsplaylistsettings-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for HlsPlaylistSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markup_type) = self.ad_markup_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkupType", ad_markup_type)?;
            }
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsPlaylistSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsPlaylistSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsPlaylistSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsPlaylistSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markup_type: Option<::ValueList<String>> = None;
                    let mut manifest_window_seconds: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkupType" => {
                                ad_markup_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsPlaylistSettings {
                        ad_markup_type: ad_markup_type,
                        manifest_window_seconds: manifest_window_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::Channel.LogConfigurationForChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-logconfigurationforchannel.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfigurationForChannel {
        /// Property [`LogTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-logconfigurationforchannel.html#cfn-mediatailor-channel-logconfigurationforchannel-logtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for LogConfigurationForChannel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_types) = self.log_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogTypes", log_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogConfigurationForChannel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfigurationForChannel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfigurationForChannel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfigurationForChannel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogTypes" => {
                                log_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfigurationForChannel {
                        log_types: log_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::Channel.RequestOutputItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html) property type.
    #[derive(Debug, Default)]
    pub struct RequestOutputItem {
        /// Property [`DashPlaylistSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html#cfn-mediatailor-channel-requestoutputitem-dashplaylistsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dash_playlist_settings: Option<::Value<DashPlaylistSettings>>,
        /// Property [`HlsPlaylistSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html#cfn-mediatailor-channel-requestoutputitem-hlsplaylistsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_playlist_settings: Option<::Value<HlsPlaylistSettings>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html#cfn-mediatailor-channel-requestoutputitem-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: ::Value<String>,
        /// Property [`SourceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-requestoutputitem.html#cfn-mediatailor-channel-requestoutputitem-sourcegroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_group: ::Value<String>,
    }

    impl ::codec::SerializeValue for RequestOutputItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dash_playlist_settings) = self.dash_playlist_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashPlaylistSettings", dash_playlist_settings)?;
            }
            if let Some(ref hls_playlist_settings) = self.hls_playlist_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsPlaylistSettings", hls_playlist_settings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", &self.manifest_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceGroup", &self.source_group)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequestOutputItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequestOutputItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequestOutputItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequestOutputItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dash_playlist_settings: Option<::Value<DashPlaylistSettings>> = None;
                    let mut hls_playlist_settings: Option<::Value<HlsPlaylistSettings>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut source_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DashPlaylistSettings" => {
                                dash_playlist_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsPlaylistSettings" => {
                                hls_playlist_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceGroup" => {
                                source_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RequestOutputItem {
                        dash_playlist_settings: dash_playlist_settings,
                        hls_playlist_settings: hls_playlist_settings,
                        manifest_name: manifest_name.ok_or(::serde::de::Error::missing_field("ManifestName"))?,
                        source_group: source_group.ok_or(::serde::de::Error::missing_field("SourceGroup"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::Channel.SlateSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-slatesource.html) property type.
    #[derive(Debug, Default)]
    pub struct SlateSource {
        /// Property [`SourceLocationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-slatesource.html#cfn-mediatailor-channel-slatesource-sourcelocationname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_location_name: Option<::Value<String>>,
        /// Property [`VodSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-slatesource.html#cfn-mediatailor-channel-slatesource-vodsourcename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vod_source_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SlateSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_location_name) = self.source_location_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLocationName", source_location_name)?;
            }
            if let Some(ref vod_source_name) = self.vod_source_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VodSourceName", vod_source_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlateSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlateSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlateSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlateSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_location_name: Option<::Value<String>> = None;
                    let mut vod_source_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceLocationName" => {
                                source_location_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VodSourceName" => {
                                vod_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlateSource {
                        source_location_name: source_location_name,
                        vod_source_name: vod_source_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::Channel.TimeShiftConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-timeshiftconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeShiftConfiguration {
        /// Property [`MaxTimeDelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-channel-timeshiftconfiguration.html#cfn-mediatailor-channel-timeshiftconfiguration-maxtimedelayseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_time_delay_seconds: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TimeShiftConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTimeDelaySeconds", &self.max_time_delay_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeShiftConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeShiftConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeShiftConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeShiftConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_time_delay_seconds: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxTimeDelaySeconds" => {
                                max_time_delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeShiftConfiguration {
                        max_time_delay_seconds: max_time_delay_seconds.ok_or(::serde::de::Error::missing_field("MaxTimeDelaySeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod live_source {
    //! Property types for the `LiveSource` resource.

    /// The [`AWS::MediaTailor::LiveSource.HttpPackageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-livesource-httppackageconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpPackageConfiguration {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-livesource-httppackageconfiguration.html#cfn-mediatailor-livesource-httppackageconfiguration-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
        /// Property [`SourceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-livesource-httppackageconfiguration.html#cfn-mediatailor-livesource-httppackageconfiguration-sourcegroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_group: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-livesource-httppackageconfiguration.html#cfn-mediatailor-livesource-httppackageconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpPackageConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceGroup", &self.source_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpPackageConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpPackageConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpPackageConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpPackageConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;
                    let mut source_group: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceGroup" => {
                                source_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpPackageConfiguration {
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                        source_group: source_group.ok_or(::serde::de::Error::missing_field("SourceGroup"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod playback_configuration {
    //! Property types for the `PlaybackConfiguration` resource.

    /// The [`AWS::MediaTailor::PlaybackConfiguration.AdMarkerPassthrough`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-admarkerpassthrough.html) property type.
    #[derive(Debug, Default)]
    pub struct AdMarkerPassthrough {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-admarkerpassthrough.html#cfn-mediatailor-playbackconfiguration-admarkerpassthrough-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AdMarkerPassthrough {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdMarkerPassthrough {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdMarkerPassthrough, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdMarkerPassthrough;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdMarkerPassthrough")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdMarkerPassthrough {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.AvailSuppression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-availsuppression.html) property type.
    #[derive(Debug, Default)]
    pub struct AvailSuppression {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-availsuppression.html#cfn-mediatailor-playbackconfiguration-availsuppression-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-availsuppression.html#cfn-mediatailor-playbackconfiguration-availsuppression-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AvailSuppression {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AvailSuppression {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AvailSuppression, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AvailSuppression;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AvailSuppression")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AvailSuppression {
                        mode: mode,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.Bumper`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-bumper.html) property type.
    #[derive(Debug, Default)]
    pub struct Bumper {
        /// Property [`EndUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-bumper.html#cfn-mediatailor-playbackconfiguration-bumper-endurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_url: Option<::Value<String>>,
        /// Property [`StartUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-bumper.html#cfn-mediatailor-playbackconfiguration-bumper-starturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Bumper {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_url) = self.end_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndUrl", end_url)?;
            }
            if let Some(ref start_url) = self.start_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartUrl", start_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Bumper {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Bumper, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Bumper;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Bumper")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_url: Option<::Value<String>> = None;
                    let mut start_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndUrl" => {
                                end_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartUrl" => {
                                start_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Bumper {
                        end_url: end_url,
                        start_url: start_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.CdnConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-cdnconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CdnConfiguration {
        /// Property [`AdSegmentUrlPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-cdnconfiguration.html#cfn-mediatailor-playbackconfiguration-cdnconfiguration-adsegmenturlprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_segment_url_prefix: Option<::Value<String>>,
        /// Property [`ContentSegmentUrlPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-cdnconfiguration.html#cfn-mediatailor-playbackconfiguration-cdnconfiguration-contentsegmenturlprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_segment_url_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CdnConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_segment_url_prefix) = self.ad_segment_url_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdSegmentUrlPrefix", ad_segment_url_prefix)?;
            }
            if let Some(ref content_segment_url_prefix) = self.content_segment_url_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentSegmentUrlPrefix", content_segment_url_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CdnConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CdnConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CdnConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CdnConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_segment_url_prefix: Option<::Value<String>> = None;
                    let mut content_segment_url_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdSegmentUrlPrefix" => {
                                ad_segment_url_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentSegmentUrlPrefix" => {
                                content_segment_url_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CdnConfiguration {
                        ad_segment_url_prefix: ad_segment_url_prefix,
                        content_segment_url_prefix: content_segment_url_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.DashConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DashConfiguration {
        /// Property [`ManifestEndpointPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfiguration.html#cfn-mediatailor-playbackconfiguration-dashconfiguration-manifestendpointprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_endpoint_prefix: Option<::Value<String>>,
        /// Property [`MpdLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfiguration.html#cfn-mediatailor-playbackconfiguration-dashconfiguration-mpdlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mpd_location: Option<::Value<String>>,
        /// Property [`OriginManifestType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfiguration.html#cfn-mediatailor-playbackconfiguration-dashconfiguration-originmanifesttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_manifest_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DashConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref manifest_endpoint_prefix) = self.manifest_endpoint_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestEndpointPrefix", manifest_endpoint_prefix)?;
            }
            if let Some(ref mpd_location) = self.mpd_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MpdLocation", mpd_location)?;
            }
            if let Some(ref origin_manifest_type) = self.origin_manifest_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginManifestType", origin_manifest_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_endpoint_prefix: Option<::Value<String>> = None;
                    let mut mpd_location: Option<::Value<String>> = None;
                    let mut origin_manifest_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestEndpointPrefix" => {
                                manifest_endpoint_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MpdLocation" => {
                                mpd_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginManifestType" => {
                                origin_manifest_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashConfiguration {
                        manifest_endpoint_prefix: manifest_endpoint_prefix,
                        mpd_location: mpd_location,
                        origin_manifest_type: origin_manifest_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.HlsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-hlsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsConfiguration {
        /// Property [`ManifestEndpointPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-hlsconfiguration.html#cfn-mediatailor-playbackconfiguration-hlsconfiguration-manifestendpointprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_endpoint_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref manifest_endpoint_prefix) = self.manifest_endpoint_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestEndpointPrefix", manifest_endpoint_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_endpoint_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestEndpointPrefix" => {
                                manifest_endpoint_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsConfiguration {
                        manifest_endpoint_prefix: manifest_endpoint_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.LivePreRollConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-liveprerollconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LivePreRollConfiguration {
        /// Property [`AdDecisionServerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-liveprerollconfiguration.html#cfn-mediatailor-playbackconfiguration-liveprerollconfiguration-addecisionserverurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_decision_server_url: Option<::Value<String>>,
        /// Property [`MaxDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-liveprerollconfiguration.html#cfn-mediatailor-playbackconfiguration-liveprerollconfiguration-maxdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_duration_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for LivePreRollConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_decision_server_url) = self.ad_decision_server_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdDecisionServerUrl", ad_decision_server_url)?;
            }
            if let Some(ref max_duration_seconds) = self.max_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxDurationSeconds", max_duration_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LivePreRollConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LivePreRollConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LivePreRollConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LivePreRollConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_decision_server_url: Option<::Value<String>> = None;
                    let mut max_duration_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdDecisionServerUrl" => {
                                ad_decision_server_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxDurationSeconds" => {
                                max_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LivePreRollConfiguration {
                        ad_decision_server_url: ad_decision_server_url,
                        max_duration_seconds: max_duration_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::PlaybackConfiguration.ManifestProcessingRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-manifestprocessingrules.html) property type.
    #[derive(Debug, Default)]
    pub struct ManifestProcessingRules {
        /// Property [`AdMarkerPassthrough`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-manifestprocessingrules.html#cfn-mediatailor-playbackconfiguration-manifestprocessingrules-admarkerpassthrough).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_marker_passthrough: Option<::Value<AdMarkerPassthrough>>,
    }

    impl ::codec::SerializeValue for ManifestProcessingRules {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_marker_passthrough) = self.ad_marker_passthrough {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkerPassthrough", ad_marker_passthrough)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManifestProcessingRules {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManifestProcessingRules, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManifestProcessingRules;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManifestProcessingRules")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_marker_passthrough: Option<::Value<AdMarkerPassthrough>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkerPassthrough" => {
                                ad_marker_passthrough = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManifestProcessingRules {
                        ad_marker_passthrough: ad_marker_passthrough,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod source_location {
    //! Property types for the `SourceLocation` resource.

    /// The [`AWS::MediaTailor::SourceLocation.AccessConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-accessconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessConfiguration {
        /// Property [`AccessType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-accessconfiguration.html#cfn-mediatailor-sourcelocation-accessconfiguration-accesstype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_type: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessTokenConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-accessconfiguration.html#cfn-mediatailor-sourcelocation-accessconfiguration-secretsmanageraccesstokenconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_token_configuration: Option<::Value<SecretsManagerAccessTokenConfiguration>>,
    }

    impl ::codec::SerializeValue for AccessConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_type) = self.access_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessType", access_type)?;
            }
            if let Some(ref secrets_manager_access_token_configuration) = self.secrets_manager_access_token_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessTokenConfiguration", secrets_manager_access_token_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_type: Option<::Value<String>> = None;
                    let mut secrets_manager_access_token_configuration: Option<::Value<SecretsManagerAccessTokenConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessType" => {
                                access_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessTokenConfiguration" => {
                                secrets_manager_access_token_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessConfiguration {
                        access_type: access_type,
                        secrets_manager_access_token_configuration: secrets_manager_access_token_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::SourceLocation.DefaultSegmentDeliveryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-defaultsegmentdeliveryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultSegmentDeliveryConfiguration {
        /// Property [`BaseUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-defaultsegmentdeliveryconfiguration.html#cfn-mediatailor-sourcelocation-defaultsegmentdeliveryconfiguration-baseurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DefaultSegmentDeliveryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base_url) = self.base_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrl", base_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultSegmentDeliveryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultSegmentDeliveryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultSegmentDeliveryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultSegmentDeliveryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseUrl" => {
                                base_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultSegmentDeliveryConfiguration {
                        base_url: base_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::SourceLocation.HttpConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-httpconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpConfiguration {
        /// Property [`BaseUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-httpconfiguration.html#cfn-mediatailor-sourcelocation-httpconfiguration-baseurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrl", &self.base_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseUrl" => {
                                base_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpConfiguration {
                        base_url: base_url.ok_or(::serde::de::Error::missing_field("BaseUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::SourceLocation.SecretsManagerAccessTokenConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SecretsManagerAccessTokenConfiguration {
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration.html#cfn-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: Option<::Value<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration.html#cfn-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
        /// Property [`SecretStringKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration.html#cfn-mediatailor-sourcelocation-secretsmanageraccesstokenconfiguration-secretstringkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_string_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SecretsManagerAccessTokenConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref header_name) = self.header_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", header_name)?;
            }
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            if let Some(ref secret_string_key) = self.secret_string_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretStringKey", secret_string_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecretsManagerAccessTokenConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecretsManagerAccessTokenConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecretsManagerAccessTokenConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecretsManagerAccessTokenConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_name: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut secret_string_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretStringKey" => {
                                secret_string_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecretsManagerAccessTokenConfiguration {
                        header_name: header_name,
                        secret_arn: secret_arn,
                        secret_string_key: secret_string_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaTailor::SourceLocation.SegmentDeliveryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-segmentdeliveryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SegmentDeliveryConfiguration {
        /// Property [`BaseUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-segmentdeliveryconfiguration.html#cfn-mediatailor-sourcelocation-segmentdeliveryconfiguration-baseurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-sourcelocation-segmentdeliveryconfiguration.html#cfn-mediatailor-sourcelocation-segmentdeliveryconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SegmentDeliveryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base_url) = self.base_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrl", base_url)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SegmentDeliveryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentDeliveryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SegmentDeliveryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SegmentDeliveryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_url: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseUrl" => {
                                base_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SegmentDeliveryConfiguration {
                        base_url: base_url,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod vod_source {
    //! Property types for the `VodSource` resource.

    /// The [`AWS::MediaTailor::VodSource.HttpPackageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-vodsource-httppackageconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpPackageConfiguration {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-vodsource-httppackageconfiguration.html#cfn-mediatailor-vodsource-httppackageconfiguration-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: ::Value<String>,
        /// Property [`SourceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-vodsource-httppackageconfiguration.html#cfn-mediatailor-vodsource-httppackageconfiguration-sourcegroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_group: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-vodsource-httppackageconfiguration.html#cfn-mediatailor-vodsource-httppackageconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpPackageConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", &self.path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceGroup", &self.source_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpPackageConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpPackageConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpPackageConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpPackageConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;
                    let mut source_group: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceGroup" => {
                                source_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpPackageConfiguration {
                        path: path.ok_or(::serde::de::Error::missing_field("Path"))?,
                        source_group: source_group.ok_or(::serde::de::Error::missing_field("SourceGroup"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
