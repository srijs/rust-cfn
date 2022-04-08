//! Types for the `MediaTailor` service.

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
    pub dash_configuration: Option<::Value<self::playback_configuration::DashConfigurationForPut>>,
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
    /// Property [`SessionInitializationEndpointPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediatailor-playbackconfiguration.html#cfn-mediatailor-playbackconfiguration-sessioninitializationendpointprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_initialization_endpoint_prefix: Option<::Value<String>>,
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
        if let Some(ref session_initialization_endpoint_prefix) = self.session_initialization_endpoint_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionInitializationEndpointPrefix", session_initialization_endpoint_prefix)?;
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
                let mut dash_configuration: Option<::Value<self::playback_configuration::DashConfigurationForPut>> = None;
                let mut live_pre_roll_configuration: Option<::Value<self::playback_configuration::LivePreRollConfiguration>> = None;
                let mut manifest_processing_rules: Option<::Value<self::playback_configuration::ManifestProcessingRules>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut personalization_threshold_seconds: Option<::Value<u32>> = None;
                let mut session_initialization_endpoint_prefix: Option<::Value<String>> = None;
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
                        "SessionInitializationEndpointPrefix" => {
                            session_initialization_endpoint_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    live_pre_roll_configuration: live_pre_roll_configuration,
                    manifest_processing_rules: manifest_processing_rules,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    personalization_threshold_seconds: personalization_threshold_seconds,
                    session_initialization_endpoint_prefix: session_initialization_endpoint_prefix,
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

    /// The [`AWS::MediaTailor::PlaybackConfiguration.DashConfigurationForPut`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfigurationforput.html) property type.
    #[derive(Debug, Default)]
    pub struct DashConfigurationForPut {
        /// Property [`MpdLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfigurationforput.html#cfn-mediatailor-playbackconfiguration-dashconfigurationforput-mpdlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mpd_location: Option<::Value<String>>,
        /// Property [`OriginManifestType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediatailor-playbackconfiguration-dashconfigurationforput.html#cfn-mediatailor-playbackconfiguration-dashconfigurationforput-originmanifesttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origin_manifest_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DashConfigurationForPut {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mpd_location) = self.mpd_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MpdLocation", mpd_location)?;
            }
            if let Some(ref origin_manifest_type) = self.origin_manifest_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginManifestType", origin_manifest_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashConfigurationForPut {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashConfigurationForPut, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashConfigurationForPut;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashConfigurationForPut")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mpd_location: Option<::Value<String>> = None;
                    let mut origin_manifest_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MpdLocation" => {
                                mpd_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginManifestType" => {
                                origin_manifest_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashConfigurationForPut {
                        mpd_location: mpd_location,
                        origin_manifest_type: origin_manifest_type,
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
