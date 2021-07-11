//! Types for the `CodeGuruProfiler` service.

/// The [`AWS::CodeGuruProfiler::ProfilingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html) resource type.
#[derive(Debug, Default)]
pub struct ProfilingGroup {
    properties: ProfilingGroupProperties
}

/// Properties for the `ProfilingGroup` resource.
#[derive(Debug, Default)]
pub struct ProfilingGroupProperties {
    /// Property [`AgentPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html#cfn-codeguruprofiler-profilinggroup-agentpermissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_permissions: Option<::Value<::json::Value>>,
    /// Property [`AnomalyDetectionNotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html#cfn-codeguruprofiler-profilinggroup-anomalydetectionnotificationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub anomaly_detection_notification_configuration: Option<::ValueList<self::profiling_group::Channel>>,
    /// Property [`ComputePlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html#cfn-codeguruprofiler-profilinggroup-computeplatform).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compute_platform: Option<::Value<String>>,
    /// Property [`ProfilingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html#cfn-codeguruprofiler-profilinggroup-profilinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub profiling_group_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeguruprofiler-profilinggroup.html#cfn-codeguruprofiler-profilinggroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProfilingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref agent_permissions) = self.agent_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentPermissions", agent_permissions)?;
        }
        if let Some(ref anomaly_detection_notification_configuration) = self.anomaly_detection_notification_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnomalyDetectionNotificationConfiguration", anomaly_detection_notification_configuration)?;
        }
        if let Some(ref compute_platform) = self.compute_platform {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputePlatform", compute_platform)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfilingGroupName", &self.profiling_group_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProfilingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProfilingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProfilingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProfilingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_permissions: Option<::Value<::json::Value>> = None;
                let mut anomaly_detection_notification_configuration: Option<::ValueList<self::profiling_group::Channel>> = None;
                let mut compute_platform: Option<::Value<String>> = None;
                let mut profiling_group_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentPermissions" => {
                            agent_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnomalyDetectionNotificationConfiguration" => {
                            anomaly_detection_notification_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComputePlatform" => {
                            compute_platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProfilingGroupName" => {
                            profiling_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProfilingGroupProperties {
                    agent_permissions: agent_permissions,
                    anomaly_detection_notification_configuration: anomaly_detection_notification_configuration,
                    compute_platform: compute_platform,
                    profiling_group_name: profiling_group_name.ok_or(::serde::de::Error::missing_field("ProfilingGroupName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ProfilingGroup {
    type Properties = ProfilingGroupProperties;
    const TYPE: &'static str = "AWS::CodeGuruProfiler::ProfilingGroup";
    fn properties(&self) -> &ProfilingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProfilingGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ProfilingGroup {}

impl From<ProfilingGroupProperties> for ProfilingGroup {
    fn from(properties: ProfilingGroupProperties) -> ProfilingGroup {
        ProfilingGroup { properties }
    }
}

pub mod profiling_group {
    //! Property types for the `ProfilingGroup` resource.

    /// The [`AWS::CodeGuruProfiler::ProfilingGroup.Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeguruprofiler-profilinggroup-channel.html) property type.
    #[derive(Debug, Default)]
    pub struct Channel {
        /// Property [`channelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeguruprofiler-profilinggroup-channel.html#cfn-codeguruprofiler-profilinggroup-channel-channelid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_id: Option<::Value<String>>,
        /// Property [`channelUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeguruprofiler-profilinggroup-channel.html#cfn-codeguruprofiler-profilinggroup-channel-channeluri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for Channel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel_id) = self.channel_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "channelId", channel_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "channelUri", &self.channel_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Channel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Channel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Channel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Channel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_id: Option<::Value<String>> = None;
                    let mut channel_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "channelId" => {
                                channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "channelUri" => {
                                channel_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Channel {
                        channel_id: channel_id,
                        channel_uri: channel_uri.ok_or(::serde::de::Error::missing_field("channelUri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
