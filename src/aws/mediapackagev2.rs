//! Types for the `MediaPackageV2` service.

/// The [`AWS::MediaPackageV2::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`ChannelGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html#cfn-mediapackagev2-channel-channelgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_group_name: Option<::Value<String>>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html#cfn-mediapackagev2-channel-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html#cfn-mediapackagev2-channel-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channel.html#cfn-mediapackagev2-channel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_group_name) = self.channel_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelGroupName", channel_group_name)?;
        }
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut channel_group_name: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelGroupName" => {
                            channel_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    channel_group_name: channel_group_name,
                    channel_name: channel_name,
                    description: description,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::MediaPackageV2::Channel";
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

/// The [`AWS::MediaPackageV2::ChannelGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelgroup.html) resource type.
#[derive(Debug, Default)]
pub struct ChannelGroup {
    properties: ChannelGroupProperties
}

/// Properties for the `ChannelGroup` resource.
#[derive(Debug, Default)]
pub struct ChannelGroupProperties {
    /// Property [`ChannelGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelgroup.html#cfn-mediapackagev2-channelgroup-channelgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_group_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelgroup.html#cfn-mediapackagev2-channelgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelgroup.html#cfn-mediapackagev2-channelgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ChannelGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_group_name) = self.channel_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelGroupName", channel_group_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ChannelGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ChannelGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ChannelGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_group_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelGroupName" => {
                            channel_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelGroupProperties {
                    channel_group_name: channel_group_name,
                    description: description,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ChannelGroup {
    type Properties = ChannelGroupProperties;
    const TYPE: &'static str = "AWS::MediaPackageV2::ChannelGroup";
    fn properties(&self) -> &ChannelGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ChannelGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ChannelGroup {}

impl From<ChannelGroupProperties> for ChannelGroup {
    fn from(properties: ChannelGroupProperties) -> ChannelGroup {
        ChannelGroup { properties }
    }
}

/// The [`AWS::MediaPackageV2::ChannelPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ChannelPolicy {
    properties: ChannelPolicyProperties
}

/// Properties for the `ChannelPolicy` resource.
#[derive(Debug, Default)]
pub struct ChannelPolicyProperties {
    /// Property [`ChannelGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelpolicy.html#cfn-mediapackagev2-channelpolicy-channelgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_group_name: Option<::Value<String>>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelpolicy.html#cfn-mediapackagev2-channelpolicy-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-channelpolicy.html#cfn-mediapackagev2-channelpolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::Value<::json::Value>,
}

impl ::serde::Serialize for ChannelPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_group_name) = self.channel_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelGroupName", channel_group_name)?;
        }
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
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
                let mut channel_group_name: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut policy: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelGroupName" => {
                            channel_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
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
                    channel_group_name: channel_group_name,
                    channel_name: channel_name,
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ChannelPolicy {
    type Properties = ChannelPolicyProperties;
    const TYPE: &'static str = "AWS::MediaPackageV2::ChannelPolicy";
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

/// The [`AWS::MediaPackageV2::OriginEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct OriginEndpoint {
    properties: OriginEndpointProperties
}

/// Properties for the `OriginEndpoint` resource.
#[derive(Debug, Default)]
pub struct OriginEndpointProperties {
    /// Property [`ChannelGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-channelgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_group_name: Option<::Value<String>>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`ContainerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-containertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub container_type: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`HlsManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-hlsmanifests).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hls_manifests: Option<::ValueList<self::origin_endpoint::HlsManifestConfiguration>>,
    /// Property [`LowLatencyHlsManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifests).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub low_latency_hls_manifests: Option<::ValueList<self::origin_endpoint::LowLatencyHlsManifestConfiguration>>,
    /// Property [`OriginEndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-originendpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub origin_endpoint_name: Option<::Value<String>>,
    /// Property [`Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-segment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment: Option<::Value<self::origin_endpoint::Segment>>,
    /// Property [`StartoverWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-startoverwindowseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub startover_window_seconds: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpoint.html#cfn-mediapackagev2-originendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for OriginEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_group_name) = self.channel_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelGroupName", channel_group_name)?;
        }
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerType", &self.container_type)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref hls_manifests) = self.hls_manifests {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsManifests", hls_manifests)?;
        }
        if let Some(ref low_latency_hls_manifests) = self.low_latency_hls_manifests {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LowLatencyHlsManifests", low_latency_hls_manifests)?;
        }
        if let Some(ref origin_endpoint_name) = self.origin_endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginEndpointName", origin_endpoint_name)?;
        }
        if let Some(ref segment) = self.segment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Segment", segment)?;
        }
        if let Some(ref startover_window_seconds) = self.startover_window_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartoverWindowSeconds", startover_window_seconds)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OriginEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OriginEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OriginEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_group_name: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut container_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut hls_manifests: Option<::ValueList<self::origin_endpoint::HlsManifestConfiguration>> = None;
                let mut low_latency_hls_manifests: Option<::ValueList<self::origin_endpoint::LowLatencyHlsManifestConfiguration>> = None;
                let mut origin_endpoint_name: Option<::Value<String>> = None;
                let mut segment: Option<::Value<self::origin_endpoint::Segment>> = None;
                let mut startover_window_seconds: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelGroupName" => {
                            channel_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContainerType" => {
                            container_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HlsManifests" => {
                            hls_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LowLatencyHlsManifests" => {
                            low_latency_hls_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OriginEndpointName" => {
                            origin_endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Segment" => {
                            segment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartoverWindowSeconds" => {
                            startover_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OriginEndpointProperties {
                    channel_group_name: channel_group_name,
                    channel_name: channel_name,
                    container_type: container_type.ok_or(::serde::de::Error::missing_field("ContainerType"))?,
                    description: description,
                    hls_manifests: hls_manifests,
                    low_latency_hls_manifests: low_latency_hls_manifests,
                    origin_endpoint_name: origin_endpoint_name,
                    segment: segment,
                    startover_window_seconds: startover_window_seconds,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OriginEndpoint {
    type Properties = OriginEndpointProperties;
    const TYPE: &'static str = "AWS::MediaPackageV2::OriginEndpoint";
    fn properties(&self) -> &OriginEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OriginEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OriginEndpoint {}

impl From<OriginEndpointProperties> for OriginEndpoint {
    fn from(properties: OriginEndpointProperties) -> OriginEndpoint {
        OriginEndpoint { properties }
    }
}

/// The [`AWS::MediaPackageV2::OriginEndpointPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct OriginEndpointPolicy {
    properties: OriginEndpointPolicyProperties
}

/// Properties for the `OriginEndpointPolicy` resource.
#[derive(Debug, Default)]
pub struct OriginEndpointPolicyProperties {
    /// Property [`ChannelGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html#cfn-mediapackagev2-originendpointpolicy-channelgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_group_name: Option<::Value<String>>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html#cfn-mediapackagev2-originendpointpolicy-channelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`OriginEndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html#cfn-mediapackagev2-originendpointpolicy-originendpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub origin_endpoint_name: Option<::Value<String>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackagev2-originendpointpolicy.html#cfn-mediapackagev2-originendpointpolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::Value<::json::Value>,
}

impl ::serde::Serialize for OriginEndpointPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref channel_group_name) = self.channel_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelGroupName", channel_group_name)?;
        }
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
        if let Some(ref origin_endpoint_name) = self.origin_endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginEndpointName", origin_endpoint_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OriginEndpointPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginEndpointPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OriginEndpointPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OriginEndpointPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_group_name: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut origin_endpoint_name: Option<::Value<String>> = None;
                let mut policy: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelGroupName" => {
                            channel_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OriginEndpointName" => {
                            origin_endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OriginEndpointPolicyProperties {
                    channel_group_name: channel_group_name,
                    channel_name: channel_name,
                    origin_endpoint_name: origin_endpoint_name,
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OriginEndpointPolicy {
    type Properties = OriginEndpointPolicyProperties;
    const TYPE: &'static str = "AWS::MediaPackageV2::OriginEndpointPolicy";
    fn properties(&self) -> &OriginEndpointPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OriginEndpointPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OriginEndpointPolicy {}

impl From<OriginEndpointPolicyProperties> for OriginEndpointPolicy {
    fn from(properties: OriginEndpointPolicyProperties) -> OriginEndpointPolicy {
        OriginEndpointPolicy { properties }
    }
}

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::MediaPackageV2::Channel.IngestEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-ingestendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct IngestEndpoint {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-ingestendpoint.html#cfn-mediapackagev2-channel-ingestendpoint-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-channel-ingestendpoint.html#cfn-mediapackagev2-channel-ingestendpoint-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IngestEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IngestEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IngestEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IngestEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IngestEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IngestEndpoint {
                        id: id,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod origin_endpoint {
    //! Property types for the `OriginEndpoint` resource.

    /// The [`AWS::MediaPackageV2::OriginEndpoint.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html#cfn-mediapackagev2-originendpoint-encryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`EncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html#cfn-mediapackagev2-originendpoint-encryption-encryptionmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_method: ::Value<EncryptionMethod>,
        /// Property [`KeyRotationIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html#cfn-mediapackagev2-originendpoint-encryption-keyrotationintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_rotation_interval_seconds: Option<::Value<u32>>,
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryption.html#cfn-mediapackagev2-originendpoint-encryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMethod", &self.encryption_method)?;
            if let Some(ref key_rotation_interval_seconds) = self.key_rotation_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyRotationIntervalSeconds", key_rotation_interval_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut encryption_method: Option<::Value<EncryptionMethod>> = None;
                    let mut key_rotation_interval_seconds: Option<::Value<u32>> = None;
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionMethod" => {
                                encryption_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyRotationIntervalSeconds" => {
                                key_rotation_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Encryption {
                        constant_initialization_vector: constant_initialization_vector,
                        encryption_method: encryption_method.ok_or(::serde::de::Error::missing_field("EncryptionMethod"))?,
                        key_rotation_interval_seconds: key_rotation_interval_seconds,
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.EncryptionContractConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptioncontractconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionContractConfiguration {
        /// Property [`PresetSpeke20Audio`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptioncontractconfiguration.html#cfn-mediapackagev2-originendpoint-encryptioncontractconfiguration-presetspeke20audio).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preset_speke20_audio: ::Value<String>,
        /// Property [`PresetSpeke20Video`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptioncontractconfiguration.html#cfn-mediapackagev2-originendpoint-encryptioncontractconfiguration-presetspeke20video).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preset_speke20_video: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionContractConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PresetSpeke20Audio", &self.preset_speke20_audio)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PresetSpeke20Video", &self.preset_speke20_video)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionContractConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionContractConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionContractConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionContractConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut preset_speke20_audio: Option<::Value<String>> = None;
                    let mut preset_speke20_video: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PresetSpeke20Audio" => {
                                preset_speke20_audio = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PresetSpeke20Video" => {
                                preset_speke20_video = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionContractConfiguration {
                        preset_speke20_audio: preset_speke20_audio.ok_or(::serde::de::Error::missing_field("PresetSpeke20Audio"))?,
                        preset_speke20_video: preset_speke20_video.ok_or(::serde::de::Error::missing_field("PresetSpeke20Video"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.EncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptionmethod.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionMethod {
        /// Property [`CmafEncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptionmethod.html#cfn-mediapackagev2-originendpoint-encryptionmethod-cmafencryptionmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cmaf_encryption_method: Option<::Value<String>>,
        /// Property [`TsEncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-encryptionmethod.html#cfn-mediapackagev2-originendpoint-encryptionmethod-tsencryptionmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ts_encryption_method: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionMethod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cmaf_encryption_method) = self.cmaf_encryption_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmafEncryptionMethod", cmaf_encryption_method)?;
            }
            if let Some(ref ts_encryption_method) = self.ts_encryption_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TsEncryptionMethod", ts_encryption_method)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionMethod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionMethod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionMethod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionMethod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cmaf_encryption_method: Option<::Value<String>> = None;
                    let mut ts_encryption_method: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CmafEncryptionMethod" => {
                                cmaf_encryption_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TsEncryptionMethod" => {
                                ts_encryption_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionMethod {
                        cmaf_encryption_method: cmaf_encryption_method,
                        ts_encryption_method: ts_encryption_method,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.FilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterConfiguration {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html#cfn-mediapackagev2-originendpoint-filterconfiguration-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: Option<::Value<String>>,
        /// Property [`ManifestFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html#cfn-mediapackagev2-originendpoint-filterconfiguration-manifestfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_filter: Option<::Value<String>>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html#cfn-mediapackagev2-originendpoint-filterconfiguration-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: Option<::Value<String>>,
        /// Property [`TimeDelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-filterconfiguration.html#cfn-mediapackagev2-originendpoint-filterconfiguration-timedelayseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_delay_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for FilterConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end) = self.end {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", end)?;
            }
            if let Some(ref manifest_filter) = self.manifest_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestFilter", manifest_filter)?;
            }
            if let Some(ref start) = self.start {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", start)?;
            }
            if let Some(ref time_delay_seconds) = self.time_delay_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeDelaySeconds", time_delay_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<String>> = None;
                    let mut manifest_filter: Option<::Value<String>> = None;
                    let mut start: Option<::Value<String>> = None;
                    let mut time_delay_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestFilter" => {
                                manifest_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeDelaySeconds" => {
                                time_delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterConfiguration {
                        end: end,
                        manifest_filter: manifest_filter,
                        start: start,
                        time_delay_seconds: time_delay_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.HlsManifestConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsManifestConfiguration {
        /// Property [`ChildManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-childmanifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_manifest_name: Option<::Value<String>>,
        /// Property [`FilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-filterconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_configuration: Option<::Value<FilterConfiguration>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: ::Value<String>,
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<u32>>,
        /// Property [`ProgramDateTimeIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-programdatetimeintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_interval_seconds: Option<::Value<u32>>,
        /// Property [`ScteHls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-sctehls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte_hls: Option<::Value<ScteHls>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-hlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-hlsmanifestconfiguration-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsManifestConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref child_manifest_name) = self.child_manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildManifestName", child_manifest_name)?;
            }
            if let Some(ref filter_configuration) = self.filter_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterConfiguration", filter_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", &self.manifest_name)?;
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            if let Some(ref program_date_time_interval_seconds) = self.program_date_time_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimeIntervalSeconds", program_date_time_interval_seconds)?;
            }
            if let Some(ref scte_hls) = self.scte_hls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScteHls", scte_hls)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsManifestConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsManifestConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsManifestConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsManifestConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_manifest_name: Option<::Value<String>> = None;
                    let mut filter_configuration: Option<::Value<FilterConfiguration>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut manifest_window_seconds: Option<::Value<u32>> = None;
                    let mut program_date_time_interval_seconds: Option<::Value<u32>> = None;
                    let mut scte_hls: Option<::Value<ScteHls>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildManifestName" => {
                                child_manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterConfiguration" => {
                                filter_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimeIntervalSeconds" => {
                                program_date_time_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScteHls" => {
                                scte_hls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsManifestConfiguration {
                        child_manifest_name: child_manifest_name,
                        filter_configuration: filter_configuration,
                        manifest_name: manifest_name.ok_or(::serde::de::Error::missing_field("ManifestName"))?,
                        manifest_window_seconds: manifest_window_seconds,
                        program_date_time_interval_seconds: program_date_time_interval_seconds,
                        scte_hls: scte_hls,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.LowLatencyHlsManifestConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LowLatencyHlsManifestConfiguration {
        /// Property [`ChildManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-childmanifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_manifest_name: Option<::Value<String>>,
        /// Property [`FilterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-filterconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_configuration: Option<::Value<FilterConfiguration>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: ::Value<String>,
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<u32>>,
        /// Property [`ProgramDateTimeIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-programdatetimeintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_interval_seconds: Option<::Value<u32>>,
        /// Property [`ScteHls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-sctehls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte_hls: Option<::Value<ScteHls>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration.html#cfn-mediapackagev2-originendpoint-lowlatencyhlsmanifestconfiguration-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LowLatencyHlsManifestConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref child_manifest_name) = self.child_manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildManifestName", child_manifest_name)?;
            }
            if let Some(ref filter_configuration) = self.filter_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterConfiguration", filter_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", &self.manifest_name)?;
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            if let Some(ref program_date_time_interval_seconds) = self.program_date_time_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimeIntervalSeconds", program_date_time_interval_seconds)?;
            }
            if let Some(ref scte_hls) = self.scte_hls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScteHls", scte_hls)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LowLatencyHlsManifestConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LowLatencyHlsManifestConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LowLatencyHlsManifestConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LowLatencyHlsManifestConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_manifest_name: Option<::Value<String>> = None;
                    let mut filter_configuration: Option<::Value<FilterConfiguration>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut manifest_window_seconds: Option<::Value<u32>> = None;
                    let mut program_date_time_interval_seconds: Option<::Value<u32>> = None;
                    let mut scte_hls: Option<::Value<ScteHls>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildManifestName" => {
                                child_manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterConfiguration" => {
                                filter_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimeIntervalSeconds" => {
                                program_date_time_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScteHls" => {
                                scte_hls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LowLatencyHlsManifestConfiguration {
                        child_manifest_name: child_manifest_name,
                        filter_configuration: filter_configuration,
                        manifest_name: manifest_name.ok_or(::serde::de::Error::missing_field("ManifestName"))?,
                        manifest_window_seconds: manifest_window_seconds,
                        program_date_time_interval_seconds: program_date_time_interval_seconds,
                        scte_hls: scte_hls,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.Scte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-scte.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte {
        /// Property [`ScteFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-scte.html#cfn-mediapackagev2-originendpoint-scte-sctefilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte_filter: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Scte {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref scte_filter) = self.scte_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScteFilter", scte_filter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scte_filter: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScteFilter" => {
                                scte_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scte {
                        scte_filter: scte_filter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.ScteHls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-sctehls.html) property type.
    #[derive(Debug, Default)]
    pub struct ScteHls {
        /// Property [`AdMarkerHls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-sctehls.html#cfn-mediapackagev2-originendpoint-sctehls-admarkerhls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_marker_hls: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScteHls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_marker_hls) = self.ad_marker_hls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkerHls", ad_marker_hls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScteHls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScteHls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScteHls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScteHls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_marker_hls: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkerHls" => {
                                ad_marker_hls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScteHls {
                        ad_marker_hls: ad_marker_hls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html) property type.
    #[derive(Debug, Default)]
    pub struct Segment {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<Encryption>>,
        /// Property [`IncludeIframeOnlyStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-includeiframeonlystreams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_iframe_only_streams: Option<::Value<bool>>,
        /// Property [`Scte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-scte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte: Option<::Value<Scte>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`SegmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-segmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_name: Option<::Value<String>>,
        /// Property [`TsIncludeDvbSubtitles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-tsincludedvbsubtitles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ts_include_dvb_subtitles: Option<::Value<bool>>,
        /// Property [`TsUseAudioRenditionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-segment.html#cfn-mediapackagev2-originendpoint-segment-tsuseaudiorenditiongroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ts_use_audio_rendition_group: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Segment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref include_iframe_only_streams) = self.include_iframe_only_streams {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeIframeOnlyStreams", include_iframe_only_streams)?;
            }
            if let Some(ref scte) = self.scte {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte", scte)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref segment_name) = self.segment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentName", segment_name)?;
            }
            if let Some(ref ts_include_dvb_subtitles) = self.ts_include_dvb_subtitles {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TsIncludeDvbSubtitles", ts_include_dvb_subtitles)?;
            }
            if let Some(ref ts_use_audio_rendition_group) = self.ts_use_audio_rendition_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TsUseAudioRenditionGroup", ts_use_audio_rendition_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Segment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Segment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Segment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Segment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<Encryption>> = None;
                    let mut include_iframe_only_streams: Option<::Value<bool>> = None;
                    let mut scte: Option<::Value<Scte>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut segment_name: Option<::Value<String>> = None;
                    let mut ts_include_dvb_subtitles: Option<::Value<bool>> = None;
                    let mut ts_use_audio_rendition_group: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeIframeOnlyStreams" => {
                                include_iframe_only_streams = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte" => {
                                scte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentName" => {
                                segment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TsIncludeDvbSubtitles" => {
                                ts_include_dvb_subtitles = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TsUseAudioRenditionGroup" => {
                                ts_use_audio_rendition_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Segment {
                        encryption: encryption,
                        include_iframe_only_streams: include_iframe_only_streams,
                        scte: scte,
                        segment_duration_seconds: segment_duration_seconds,
                        segment_name: segment_name,
                        ts_include_dvb_subtitles: ts_include_dvb_subtitles,
                        ts_use_audio_rendition_group: ts_use_audio_rendition_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackageV2::OriginEndpoint.SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct SpekeKeyProvider {
        /// Property [`DrmSystems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html#cfn-mediapackagev2-originendpoint-spekekeyprovider-drmsystems).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub drm_systems: ::ValueList<String>,
        /// Property [`EncryptionContractConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html#cfn-mediapackagev2-originendpoint-spekekeyprovider-encryptioncontractconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_contract_configuration: ::Value<EncryptionContractConfiguration>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html#cfn-mediapackagev2-originendpoint-spekekeyprovider-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html#cfn-mediapackagev2-originendpoint-spekekeyprovider-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackagev2-originendpoint-spekekeyprovider.html#cfn-mediapackagev2-originendpoint-spekekeyprovider-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for SpekeKeyProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrmSystems", &self.drm_systems)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionContractConfiguration", &self.encryption_contract_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpekeKeyProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpekeKeyProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpekeKeyProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpekeKeyProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut drm_systems: Option<::ValueList<String>> = None;
                    let mut encryption_contract_configuration: Option<::Value<EncryptionContractConfiguration>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DrmSystems" => {
                                drm_systems = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionContractConfiguration" => {
                                encryption_contract_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SpekeKeyProvider {
                        drm_systems: drm_systems.ok_or(::serde::de::Error::missing_field("DrmSystems"))?,
                        encryption_contract_configuration: encryption_contract_configuration.ok_or(::serde::de::Error::missing_field("EncryptionContractConfiguration"))?,
                        resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
