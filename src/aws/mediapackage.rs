//! Types for the `MediaPackage` service.

/// The [`AWS::MediaPackage::Asset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html) resource type.
#[derive(Debug, Default)]
pub struct Asset {
    properties: AssetProperties
}

/// Properties for the `Asset` resource.
#[derive(Debug, Default)]
pub struct AssetProperties {
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-id).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`PackagingGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-packaginggroupid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub packaging_group_id: ::Value<String>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-resourceid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_id: Option<::Value<String>>,
    /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-sourcearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_arn: ::Value<String>,
    /// Property [`SourceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-sourcerolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-asset.html#cfn-mediapackage-asset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackagingGroupId", &self.packaging_group_id)?;
        if let Some(ref resource_id) = self.resource_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRoleArn", &self.source_role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut id: Option<::Value<String>> = None;
                let mut packaging_group_id: Option<::Value<String>> = None;
                let mut resource_id: Option<::Value<String>> = None;
                let mut source_arn: Option<::Value<String>> = None;
                let mut source_role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackagingGroupId" => {
                            packaging_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceArn" => {
                            source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceRoleArn" => {
                            source_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssetProperties {
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    packaging_group_id: packaging_group_id.ok_or(::serde::de::Error::missing_field("PackagingGroupId"))?,
                    resource_id: resource_id,
                    source_arn: source_arn.ok_or(::serde::de::Error::missing_field("SourceArn"))?,
                    source_role_arn: source_role_arn.ok_or(::serde::de::Error::missing_field("SourceRoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Asset {
    type Properties = AssetProperties;
    const TYPE: &'static str = "AWS::MediaPackage::Asset";
    fn properties(&self) -> &AssetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Asset {}

impl From<AssetProperties> for Asset {
    fn from(properties: AssetProperties) -> Asset {
        Asset { properties }
    }
}

/// The [`AWS::MediaPackage::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html#cfn-mediapackage-channel-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EgressAccessLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html#cfn-mediapackage-channel-egressaccesslogs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub egress_access_logs: Option<::Value<self::channel::LogConfiguration>>,
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html#cfn-mediapackage-channel-id).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`IngressAccessLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html#cfn-mediapackage-channel-ingressaccesslogs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ingress_access_logs: Option<::Value<self::channel::LogConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-channel.html#cfn-mediapackage-channel-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref egress_access_logs) = self.egress_access_logs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressAccessLogs", egress_access_logs)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        if let Some(ref ingress_access_logs) = self.ingress_access_logs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngressAccessLogs", ingress_access_logs)?;
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
                let mut description: Option<::Value<String>> = None;
                let mut egress_access_logs: Option<::Value<self::channel::LogConfiguration>> = None;
                let mut id: Option<::Value<String>> = None;
                let mut ingress_access_logs: Option<::Value<self::channel::LogConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EgressAccessLogs" => {
                            egress_access_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IngressAccessLogs" => {
                            ingress_access_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    description: description,
                    egress_access_logs: egress_access_logs,
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    ingress_access_logs: ingress_access_logs,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::MediaPackage::Channel";
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

/// The [`AWS::MediaPackage::OriginEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct OriginEndpoint {
    properties: OriginEndpointProperties
}

/// Properties for the `OriginEndpoint` resource.
#[derive(Debug, Default)]
pub struct OriginEndpointProperties {
    /// Property [`Authorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-authorization).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization: Option<::Value<self::origin_endpoint::Authorization>>,
    /// Property [`ChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-channelid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_id: ::Value<String>,
    /// Property [`CmafPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-cmafpackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cmaf_package: Option<::Value<self::origin_endpoint::CmafPackage>>,
    /// Property [`DashPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-dashpackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dash_package: Option<::Value<self::origin_endpoint::DashPackage>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`HlsPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-hlspackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hls_package: Option<::Value<self::origin_endpoint::HlsPackage>>,
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-id).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-manifestname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manifest_name: Option<::Value<String>>,
    /// Property [`MssPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-msspackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mss_package: Option<::Value<self::origin_endpoint::MssPackage>>,
    /// Property [`Origination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-origination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub origination: Option<::Value<String>>,
    /// Property [`StartoverWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-startoverwindowseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub startover_window_seconds: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeDelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-timedelayseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_delay_seconds: Option<::Value<u32>>,
    /// Property [`Whitelist`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-originendpoint.html#cfn-mediapackage-originendpoint-whitelist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub whitelist: Option<::ValueList<String>>,
}

impl ::serde::Serialize for OriginEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref authorization) = self.authorization {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Authorization", authorization)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelId", &self.channel_id)?;
        if let Some(ref cmaf_package) = self.cmaf_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmafPackage", cmaf_package)?;
        }
        if let Some(ref dash_package) = self.dash_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashPackage", dash_package)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref hls_package) = self.hls_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsPackage", hls_package)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        if let Some(ref manifest_name) = self.manifest_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", manifest_name)?;
        }
        if let Some(ref mss_package) = self.mss_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MssPackage", mss_package)?;
        }
        if let Some(ref origination) = self.origination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Origination", origination)?;
        }
        if let Some(ref startover_window_seconds) = self.startover_window_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartoverWindowSeconds", startover_window_seconds)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref time_delay_seconds) = self.time_delay_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeDelaySeconds", time_delay_seconds)?;
        }
        if let Some(ref whitelist) = self.whitelist {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Whitelist", whitelist)?;
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
                let mut authorization: Option<::Value<self::origin_endpoint::Authorization>> = None;
                let mut channel_id: Option<::Value<String>> = None;
                let mut cmaf_package: Option<::Value<self::origin_endpoint::CmafPackage>> = None;
                let mut dash_package: Option<::Value<self::origin_endpoint::DashPackage>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut hls_package: Option<::Value<self::origin_endpoint::HlsPackage>> = None;
                let mut id: Option<::Value<String>> = None;
                let mut manifest_name: Option<::Value<String>> = None;
                let mut mss_package: Option<::Value<self::origin_endpoint::MssPackage>> = None;
                let mut origination: Option<::Value<String>> = None;
                let mut startover_window_seconds: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_delay_seconds: Option<::Value<u32>> = None;
                let mut whitelist: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Authorization" => {
                            authorization = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelId" => {
                            channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CmafPackage" => {
                            cmaf_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashPackage" => {
                            dash_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HlsPackage" => {
                            hls_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManifestName" => {
                            manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MssPackage" => {
                            mss_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Origination" => {
                            origination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartoverWindowSeconds" => {
                            startover_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeDelaySeconds" => {
                            time_delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Whitelist" => {
                            whitelist = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OriginEndpointProperties {
                    authorization: authorization,
                    channel_id: channel_id.ok_or(::serde::de::Error::missing_field("ChannelId"))?,
                    cmaf_package: cmaf_package,
                    dash_package: dash_package,
                    description: description,
                    hls_package: hls_package,
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    manifest_name: manifest_name,
                    mss_package: mss_package,
                    origination: origination,
                    startover_window_seconds: startover_window_seconds,
                    tags: tags,
                    time_delay_seconds: time_delay_seconds,
                    whitelist: whitelist,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OriginEndpoint {
    type Properties = OriginEndpointProperties;
    const TYPE: &'static str = "AWS::MediaPackage::OriginEndpoint";
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

/// The [`AWS::MediaPackage::PackagingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct PackagingConfiguration {
    properties: PackagingConfigurationProperties
}

/// Properties for the `PackagingConfiguration` resource.
#[derive(Debug, Default)]
pub struct PackagingConfigurationProperties {
    /// Property [`CmafPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-cmafpackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cmaf_package: Option<::Value<self::packaging_configuration::CmafPackage>>,
    /// Property [`DashPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-dashpackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dash_package: Option<::Value<self::packaging_configuration::DashPackage>>,
    /// Property [`HlsPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-hlspackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hls_package: Option<::Value<self::packaging_configuration::HlsPackage>>,
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-id).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`MssPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-msspackage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mss_package: Option<::Value<self::packaging_configuration::MssPackage>>,
    /// Property [`PackagingGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-packaginggroupid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub packaging_group_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packagingconfiguration.html#cfn-mediapackage-packagingconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PackagingConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cmaf_package) = self.cmaf_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmafPackage", cmaf_package)?;
        }
        if let Some(ref dash_package) = self.dash_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashPackage", dash_package)?;
        }
        if let Some(ref hls_package) = self.hls_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsPackage", hls_package)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        if let Some(ref mss_package) = self.mss_package {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MssPackage", mss_package)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackagingGroupId", &self.packaging_group_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PackagingConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PackagingConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PackagingConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PackagingConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cmaf_package: Option<::Value<self::packaging_configuration::CmafPackage>> = None;
                let mut dash_package: Option<::Value<self::packaging_configuration::DashPackage>> = None;
                let mut hls_package: Option<::Value<self::packaging_configuration::HlsPackage>> = None;
                let mut id: Option<::Value<String>> = None;
                let mut mss_package: Option<::Value<self::packaging_configuration::MssPackage>> = None;
                let mut packaging_group_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CmafPackage" => {
                            cmaf_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashPackage" => {
                            dash_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HlsPackage" => {
                            hls_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MssPackage" => {
                            mss_package = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackagingGroupId" => {
                            packaging_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PackagingConfigurationProperties {
                    cmaf_package: cmaf_package,
                    dash_package: dash_package,
                    hls_package: hls_package,
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    mss_package: mss_package,
                    packaging_group_id: packaging_group_id.ok_or(::serde::de::Error::missing_field("PackagingGroupId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PackagingConfiguration {
    type Properties = PackagingConfigurationProperties;
    const TYPE: &'static str = "AWS::MediaPackage::PackagingConfiguration";
    fn properties(&self) -> &PackagingConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PackagingConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PackagingConfiguration {}

impl From<PackagingConfigurationProperties> for PackagingConfiguration {
    fn from(properties: PackagingConfigurationProperties) -> PackagingConfiguration {
        PackagingConfiguration { properties }
    }
}

/// The [`AWS::MediaPackage::PackagingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html) resource type.
#[derive(Debug, Default)]
pub struct PackagingGroup {
    properties: PackagingGroupProperties
}

/// Properties for the `PackagingGroup` resource.
#[derive(Debug, Default)]
pub struct PackagingGroupProperties {
    /// Property [`Authorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html#cfn-mediapackage-packaginggroup-authorization).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorization: Option<::Value<self::packaging_group::Authorization>>,
    /// Property [`EgressAccessLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html#cfn-mediapackage-packaginggroup-egressaccesslogs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub egress_access_logs: Option<::Value<self::packaging_group::LogConfiguration>>,
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html#cfn-mediapackage-packaginggroup-id).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediapackage-packaginggroup.html#cfn-mediapackage-packaginggroup-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PackagingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref authorization) = self.authorization {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Authorization", authorization)?;
        }
        if let Some(ref egress_access_logs) = self.egress_access_logs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressAccessLogs", egress_access_logs)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PackagingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PackagingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PackagingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PackagingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authorization: Option<::Value<self::packaging_group::Authorization>> = None;
                let mut egress_access_logs: Option<::Value<self::packaging_group::LogConfiguration>> = None;
                let mut id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Authorization" => {
                            authorization = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EgressAccessLogs" => {
                            egress_access_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PackagingGroupProperties {
                    authorization: authorization,
                    egress_access_logs: egress_access_logs,
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PackagingGroup {
    type Properties = PackagingGroupProperties;
    const TYPE: &'static str = "AWS::MediaPackage::PackagingGroup";
    fn properties(&self) -> &PackagingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PackagingGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PackagingGroup {}

impl From<PackagingGroupProperties> for PackagingGroup {
    fn from(properties: PackagingGroupProperties) -> PackagingGroup {
        PackagingGroup { properties }
    }
}

pub mod asset {
    //! Property types for the `Asset` resource.

    /// The [`AWS::MediaPackage::Asset.EgressEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-asset-egressendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct EgressEndpoint {
        /// Property [`PackagingConfigurationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-asset-egressendpoint.html#cfn-mediapackage-asset-egressendpoint-packagingconfigurationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub packaging_configuration_id: ::Value<String>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-asset-egressendpoint.html#cfn-mediapackage-asset-egressendpoint-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for EgressEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackagingConfigurationId", &self.packaging_configuration_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EgressEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EgressEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EgressEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EgressEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut packaging_configuration_id: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PackagingConfigurationId" => {
                                packaging_configuration_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EgressEndpoint {
                        packaging_configuration_id: packaging_configuration_id.ok_or(::serde::de::Error::missing_field("PackagingConfigurationId"))?,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::MediaPackage::Channel.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-channel-logconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfiguration {
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-channel-logconfiguration.html#cfn-mediapackage-channel-logconfiguration-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_group_name: log_group_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod origin_endpoint {
    //! Property types for the `OriginEndpoint` resource.

    /// The [`AWS::MediaPackage::OriginEndpoint.Authorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-authorization.html) property type.
    #[derive(Debug, Default)]
    pub struct Authorization {
        /// Property [`CdnIdentifierSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-authorization.html#cfn-mediapackage-originendpoint-authorization-cdnidentifiersecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdn_identifier_secret: ::Value<String>,
        /// Property [`SecretsRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-authorization.html#cfn-mediapackage-originendpoint-authorization-secretsrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Authorization {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdnIdentifierSecret", &self.cdn_identifier_secret)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsRoleArn", &self.secrets_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Authorization {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Authorization, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Authorization;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Authorization")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cdn_identifier_secret: Option<::Value<String>> = None;
                    let mut secrets_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CdnIdentifierSecret" => {
                                cdn_identifier_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsRoleArn" => {
                                secrets_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Authorization {
                        cdn_identifier_secret: cdn_identifier_secret.ok_or(::serde::de::Error::missing_field("CdnIdentifierSecret"))?,
                        secrets_role_arn: secrets_role_arn.ok_or(::serde::de::Error::missing_field("SecretsRoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.CmafEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct CmafEncryption {
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafencryption.html#cfn-mediapackage-originendpoint-cmafencryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`KeyRotationIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafencryption.html#cfn-mediapackage-originendpoint-cmafencryption-keyrotationintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_rotation_interval_seconds: Option<::Value<u32>>,
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafencryption.html#cfn-mediapackage-originendpoint-cmafencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for CmafEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref key_rotation_interval_seconds) = self.key_rotation_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyRotationIntervalSeconds", key_rotation_interval_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CmafEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CmafEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CmafEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CmafEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut key_rotation_interval_seconds: Option<::Value<u32>> = None;
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(CmafEncryption {
                        constant_initialization_vector: constant_initialization_vector,
                        key_rotation_interval_seconds: key_rotation_interval_seconds,
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.CmafPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html) property type.
    #[derive(Debug, Default)]
    pub struct CmafPackage {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html#cfn-mediapackage-originendpoint-cmafpackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<CmafEncryption>>,
        /// Property [`HlsManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html#cfn-mediapackage-originendpoint-cmafpackage-hlsmanifests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_manifests: Option<::ValueList<HlsManifest>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html#cfn-mediapackage-originendpoint-cmafpackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`SegmentPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html#cfn-mediapackage-originendpoint-cmafpackage-segmentprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_prefix: Option<::Value<String>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-cmafpackage.html#cfn-mediapackage-originendpoint-cmafpackage-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
    }

    impl ::codec::SerializeValue for CmafPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref hls_manifests) = self.hls_manifests {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsManifests", hls_manifests)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref segment_prefix) = self.segment_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentPrefix", segment_prefix)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CmafPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CmafPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CmafPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CmafPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<CmafEncryption>> = None;
                    let mut hls_manifests: Option<::ValueList<HlsManifest>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut segment_prefix: Option<::Value<String>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsManifests" => {
                                hls_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentPrefix" => {
                                segment_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CmafPackage {
                        encryption: encryption,
                        hls_manifests: hls_manifests,
                        segment_duration_seconds: segment_duration_seconds,
                        segment_prefix: segment_prefix,
                        stream_selection: stream_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.DashEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct DashEncryption {
        /// Property [`KeyRotationIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashencryption.html#cfn-mediapackage-originendpoint-dashencryption-keyrotationintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_rotation_interval_seconds: Option<::Value<u32>>,
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashencryption.html#cfn-mediapackage-originendpoint-dashencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for DashEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_rotation_interval_seconds) = self.key_rotation_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyRotationIntervalSeconds", key_rotation_interval_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_rotation_interval_seconds: Option<::Value<u32>> = None;
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyRotationIntervalSeconds" => {
                                key_rotation_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashEncryption {
                        key_rotation_interval_seconds: key_rotation_interval_seconds,
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.DashPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html) property type.
    #[derive(Debug, Default)]
    pub struct DashPackage {
        /// Property [`AdTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-adtriggers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_triggers: Option<::ValueList<String>>,
        /// Property [`AdsOnDeliveryRestrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-adsondeliveryrestrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ads_on_delivery_restrictions: Option<::Value<String>>,
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<DashEncryption>>,
        /// Property [`ManifestLayout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-manifestlayout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_layout: Option<::Value<String>>,
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<u32>>,
        /// Property [`MinBufferTimeSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-minbuffertimeseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_buffer_time_seconds: Option<::Value<u32>>,
        /// Property [`MinUpdatePeriodSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-minupdateperiodseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_update_period_seconds: Option<::Value<u32>>,
        /// Property [`PeriodTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-periodtriggers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period_triggers: Option<::ValueList<String>>,
        /// Property [`Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-profile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile: Option<::Value<String>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`SegmentTemplateFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-segmenttemplateformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_template_format: Option<::Value<String>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
        /// Property [`SuggestedPresentationDelaySeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-suggestedpresentationdelayseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suggested_presentation_delay_seconds: Option<::Value<u32>>,
        /// Property [`UtcTiming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-utctiming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub utc_timing: Option<::Value<String>>,
        /// Property [`UtcTimingUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-dashpackage.html#cfn-mediapackage-originendpoint-dashpackage-utctiminguri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub utc_timing_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DashPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_triggers) = self.ad_triggers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdTriggers", ad_triggers)?;
            }
            if let Some(ref ads_on_delivery_restrictions) = self.ads_on_delivery_restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdsOnDeliveryRestrictions", ads_on_delivery_restrictions)?;
            }
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref manifest_layout) = self.manifest_layout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestLayout", manifest_layout)?;
            }
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            if let Some(ref min_buffer_time_seconds) = self.min_buffer_time_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinBufferTimeSeconds", min_buffer_time_seconds)?;
            }
            if let Some(ref min_update_period_seconds) = self.min_update_period_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinUpdatePeriodSeconds", min_update_period_seconds)?;
            }
            if let Some(ref period_triggers) = self.period_triggers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeriodTriggers", period_triggers)?;
            }
            if let Some(ref profile) = self.profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Profile", profile)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref segment_template_format) = self.segment_template_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentTemplateFormat", segment_template_format)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            if let Some(ref suggested_presentation_delay_seconds) = self.suggested_presentation_delay_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuggestedPresentationDelaySeconds", suggested_presentation_delay_seconds)?;
            }
            if let Some(ref utc_timing) = self.utc_timing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UtcTiming", utc_timing)?;
            }
            if let Some(ref utc_timing_uri) = self.utc_timing_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UtcTimingUri", utc_timing_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_triggers: Option<::ValueList<String>> = None;
                    let mut ads_on_delivery_restrictions: Option<::Value<String>> = None;
                    let mut encryption: Option<::Value<DashEncryption>> = None;
                    let mut manifest_layout: Option<::Value<String>> = None;
                    let mut manifest_window_seconds: Option<::Value<u32>> = None;
                    let mut min_buffer_time_seconds: Option<::Value<u32>> = None;
                    let mut min_update_period_seconds: Option<::Value<u32>> = None;
                    let mut period_triggers: Option<::ValueList<String>> = None;
                    let mut profile: Option<::Value<String>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut segment_template_format: Option<::Value<String>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;
                    let mut suggested_presentation_delay_seconds: Option<::Value<u32>> = None;
                    let mut utc_timing: Option<::Value<String>> = None;
                    let mut utc_timing_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdTriggers" => {
                                ad_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdsOnDeliveryRestrictions" => {
                                ads_on_delivery_restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestLayout" => {
                                manifest_layout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinBufferTimeSeconds" => {
                                min_buffer_time_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinUpdatePeriodSeconds" => {
                                min_update_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeriodTriggers" => {
                                period_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Profile" => {
                                profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentTemplateFormat" => {
                                segment_template_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuggestedPresentationDelaySeconds" => {
                                suggested_presentation_delay_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UtcTiming" => {
                                utc_timing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UtcTimingUri" => {
                                utc_timing_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashPackage {
                        ad_triggers: ad_triggers,
                        ads_on_delivery_restrictions: ads_on_delivery_restrictions,
                        encryption: encryption,
                        manifest_layout: manifest_layout,
                        manifest_window_seconds: manifest_window_seconds,
                        min_buffer_time_seconds: min_buffer_time_seconds,
                        min_update_period_seconds: min_update_period_seconds,
                        period_triggers: period_triggers,
                        profile: profile,
                        segment_duration_seconds: segment_duration_seconds,
                        segment_template_format: segment_template_format,
                        stream_selection: stream_selection,
                        suggested_presentation_delay_seconds: suggested_presentation_delay_seconds,
                        utc_timing: utc_timing,
                        utc_timing_uri: utc_timing_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.HlsEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsEncryption {
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html#cfn-mediapackage-originendpoint-hlsencryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`EncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html#cfn-mediapackage-originendpoint-hlsencryption-encryptionmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_method: Option<::Value<String>>,
        /// Property [`KeyRotationIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html#cfn-mediapackage-originendpoint-hlsencryption-keyrotationintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_rotation_interval_seconds: Option<::Value<u32>>,
        /// Property [`RepeatExtXKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html#cfn-mediapackage-originendpoint-hlsencryption-repeatextxkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub repeat_ext_x_key: Option<::Value<bool>>,
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsencryption.html#cfn-mediapackage-originendpoint-hlsencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for HlsEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref encryption_method) = self.encryption_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMethod", encryption_method)?;
            }
            if let Some(ref key_rotation_interval_seconds) = self.key_rotation_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyRotationIntervalSeconds", key_rotation_interval_seconds)?;
            }
            if let Some(ref repeat_ext_x_key) = self.repeat_ext_x_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepeatExtXKey", repeat_ext_x_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut encryption_method: Option<::Value<String>> = None;
                    let mut key_rotation_interval_seconds: Option<::Value<u32>> = None;
                    let mut repeat_ext_x_key: Option<::Value<bool>> = None;
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
                            "RepeatExtXKey" => {
                                repeat_ext_x_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsEncryption {
                        constant_initialization_vector: constant_initialization_vector,
                        encryption_method: encryption_method,
                        key_rotation_interval_seconds: key_rotation_interval_seconds,
                        repeat_ext_x_key: repeat_ext_x_key,
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.HlsManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsManifest {
        /// Property [`AdMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-admarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markers: Option<::Value<String>>,
        /// Property [`AdTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-adtriggers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_triggers: Option<::ValueList<String>>,
        /// Property [`AdsOnDeliveryRestrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-adsondeliveryrestrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ads_on_delivery_restrictions: Option<::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`IncludeIframeOnlyStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-includeiframeonlystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_iframe_only_stream: Option<::Value<bool>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: Option<::Value<String>>,
        /// Property [`PlaylistType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-playlisttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub playlist_type: Option<::Value<String>>,
        /// Property [`PlaylistWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-playlistwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub playlist_window_seconds: Option<::Value<u32>>,
        /// Property [`ProgramDateTimeIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-programdatetimeintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_interval_seconds: Option<::Value<u32>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlsmanifest.html#cfn-mediapackage-originendpoint-hlsmanifest-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsManifest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markers) = self.ad_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkers", ad_markers)?;
            }
            if let Some(ref ad_triggers) = self.ad_triggers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdTriggers", ad_triggers)?;
            }
            if let Some(ref ads_on_delivery_restrictions) = self.ads_on_delivery_restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdsOnDeliveryRestrictions", ads_on_delivery_restrictions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref include_iframe_only_stream) = self.include_iframe_only_stream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeIframeOnlyStream", include_iframe_only_stream)?;
            }
            if let Some(ref manifest_name) = self.manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", manifest_name)?;
            }
            if let Some(ref playlist_type) = self.playlist_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlaylistType", playlist_type)?;
            }
            if let Some(ref playlist_window_seconds) = self.playlist_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlaylistWindowSeconds", playlist_window_seconds)?;
            }
            if let Some(ref program_date_time_interval_seconds) = self.program_date_time_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimeIntervalSeconds", program_date_time_interval_seconds)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsManifest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsManifest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsManifest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsManifest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markers: Option<::Value<String>> = None;
                    let mut ad_triggers: Option<::ValueList<String>> = None;
                    let mut ads_on_delivery_restrictions: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut include_iframe_only_stream: Option<::Value<bool>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut playlist_type: Option<::Value<String>> = None;
                    let mut playlist_window_seconds: Option<::Value<u32>> = None;
                    let mut program_date_time_interval_seconds: Option<::Value<u32>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkers" => {
                                ad_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdTriggers" => {
                                ad_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdsOnDeliveryRestrictions" => {
                                ads_on_delivery_restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeIframeOnlyStream" => {
                                include_iframe_only_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlaylistType" => {
                                playlist_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlaylistWindowSeconds" => {
                                playlist_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimeIntervalSeconds" => {
                                program_date_time_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsManifest {
                        ad_markers: ad_markers,
                        ad_triggers: ad_triggers,
                        ads_on_delivery_restrictions: ads_on_delivery_restrictions,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        include_iframe_only_stream: include_iframe_only_stream,
                        manifest_name: manifest_name,
                        playlist_type: playlist_type,
                        playlist_window_seconds: playlist_window_seconds,
                        program_date_time_interval_seconds: program_date_time_interval_seconds,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.HlsPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsPackage {
        /// Property [`AdMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-admarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markers: Option<::Value<String>>,
        /// Property [`AdTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-adtriggers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_triggers: Option<::ValueList<String>>,
        /// Property [`AdsOnDeliveryRestrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-adsondeliveryrestrictions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ads_on_delivery_restrictions: Option<::Value<String>>,
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<HlsEncryption>>,
        /// Property [`IncludeIframeOnlyStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-includeiframeonlystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_iframe_only_stream: Option<::Value<bool>>,
        /// Property [`PlaylistType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-playlisttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub playlist_type: Option<::Value<String>>,
        /// Property [`PlaylistWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-playlistwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub playlist_window_seconds: Option<::Value<u32>>,
        /// Property [`ProgramDateTimeIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-programdatetimeintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_interval_seconds: Option<::Value<u32>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
        /// Property [`UseAudioRenditionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-hlspackage.html#cfn-mediapackage-originendpoint-hlspackage-useaudiorenditiongroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_audio_rendition_group: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for HlsPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markers) = self.ad_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkers", ad_markers)?;
            }
            if let Some(ref ad_triggers) = self.ad_triggers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdTriggers", ad_triggers)?;
            }
            if let Some(ref ads_on_delivery_restrictions) = self.ads_on_delivery_restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdsOnDeliveryRestrictions", ads_on_delivery_restrictions)?;
            }
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref include_iframe_only_stream) = self.include_iframe_only_stream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeIframeOnlyStream", include_iframe_only_stream)?;
            }
            if let Some(ref playlist_type) = self.playlist_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlaylistType", playlist_type)?;
            }
            if let Some(ref playlist_window_seconds) = self.playlist_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlaylistWindowSeconds", playlist_window_seconds)?;
            }
            if let Some(ref program_date_time_interval_seconds) = self.program_date_time_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimeIntervalSeconds", program_date_time_interval_seconds)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            if let Some(ref use_audio_rendition_group) = self.use_audio_rendition_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseAudioRenditionGroup", use_audio_rendition_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markers: Option<::Value<String>> = None;
                    let mut ad_triggers: Option<::ValueList<String>> = None;
                    let mut ads_on_delivery_restrictions: Option<::Value<String>> = None;
                    let mut encryption: Option<::Value<HlsEncryption>> = None;
                    let mut include_iframe_only_stream: Option<::Value<bool>> = None;
                    let mut playlist_type: Option<::Value<String>> = None;
                    let mut playlist_window_seconds: Option<::Value<u32>> = None;
                    let mut program_date_time_interval_seconds: Option<::Value<u32>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;
                    let mut use_audio_rendition_group: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkers" => {
                                ad_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdTriggers" => {
                                ad_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdsOnDeliveryRestrictions" => {
                                ads_on_delivery_restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeIframeOnlyStream" => {
                                include_iframe_only_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlaylistType" => {
                                playlist_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlaylistWindowSeconds" => {
                                playlist_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimeIntervalSeconds" => {
                                program_date_time_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseAudioRenditionGroup" => {
                                use_audio_rendition_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsPackage {
                        ad_markers: ad_markers,
                        ad_triggers: ad_triggers,
                        ads_on_delivery_restrictions: ads_on_delivery_restrictions,
                        encryption: encryption,
                        include_iframe_only_stream: include_iframe_only_stream,
                        playlist_type: playlist_type,
                        playlist_window_seconds: playlist_window_seconds,
                        program_date_time_interval_seconds: program_date_time_interval_seconds,
                        segment_duration_seconds: segment_duration_seconds,
                        stream_selection: stream_selection,
                        use_audio_rendition_group: use_audio_rendition_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.MssEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-mssencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct MssEncryption {
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-mssencryption.html#cfn-mediapackage-originendpoint-mssencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for MssEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MssEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MssEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MssEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MssEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MssEncryption {
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.MssPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html) property type.
    #[derive(Debug, Default)]
    pub struct MssPackage {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html#cfn-mediapackage-originendpoint-msspackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<MssEncryption>>,
        /// Property [`ManifestWindowSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html#cfn-mediapackage-originendpoint-msspackage-manifestwindowseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_window_seconds: Option<::Value<u32>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html#cfn-mediapackage-originendpoint-msspackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-msspackage.html#cfn-mediapackage-originendpoint-msspackage-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
    }

    impl ::codec::SerializeValue for MssPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref manifest_window_seconds) = self.manifest_window_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestWindowSeconds", manifest_window_seconds)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MssPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MssPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MssPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MssPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<MssEncryption>> = None;
                    let mut manifest_window_seconds: Option<::Value<u32>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestWindowSeconds" => {
                                manifest_window_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MssPackage {
                        encryption: encryption,
                        manifest_window_seconds: manifest_window_seconds,
                        segment_duration_seconds: segment_duration_seconds,
                        stream_selection: stream_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct SpekeKeyProvider {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html#cfn-mediapackage-originendpoint-spekekeyprovider-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html#cfn-mediapackage-originendpoint-spekekeyprovider-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html#cfn-mediapackage-originendpoint-spekekeyprovider-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SystemIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html#cfn-mediapackage-originendpoint-spekekeyprovider-systemids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub system_ids: ::ValueList<String>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-spekekeyprovider.html#cfn-mediapackage-originendpoint-spekekeyprovider-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for SpekeKeyProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemIds", &self.system_ids)?;
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
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut system_ids: Option<::ValueList<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SystemIds" => {
                                system_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SpekeKeyProvider {
                        certificate_arn: certificate_arn,
                        resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        system_ids: system_ids.ok_or(::serde::de::Error::missing_field("SystemIds"))?,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::OriginEndpoint.StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-streamselection.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamSelection {
        /// Property [`MaxVideoBitsPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-streamselection.html#cfn-mediapackage-originendpoint-streamselection-maxvideobitspersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_video_bits_per_second: Option<::Value<u32>>,
        /// Property [`MinVideoBitsPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-streamselection.html#cfn-mediapackage-originendpoint-streamselection-minvideobitspersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_video_bits_per_second: Option<::Value<u32>>,
        /// Property [`StreamOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-originendpoint-streamselection.html#cfn-mediapackage-originendpoint-streamselection-streamorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_order: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StreamSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_video_bits_per_second) = self.max_video_bits_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxVideoBitsPerSecond", max_video_bits_per_second)?;
            }
            if let Some(ref min_video_bits_per_second) = self.min_video_bits_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinVideoBitsPerSecond", min_video_bits_per_second)?;
            }
            if let Some(ref stream_order) = self.stream_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamOrder", stream_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_video_bits_per_second: Option<::Value<u32>> = None;
                    let mut min_video_bits_per_second: Option<::Value<u32>> = None;
                    let mut stream_order: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxVideoBitsPerSecond" => {
                                max_video_bits_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinVideoBitsPerSecond" => {
                                min_video_bits_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamOrder" => {
                                stream_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamSelection {
                        max_video_bits_per_second: max_video_bits_per_second,
                        min_video_bits_per_second: min_video_bits_per_second,
                        stream_order: stream_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod packaging_configuration {
    //! Property types for the `PackagingConfiguration` resource.

    /// The [`AWS::MediaPackage::PackagingConfiguration.CmafEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct CmafEncryption {
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafencryption.html#cfn-mediapackage-packagingconfiguration-cmafencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for CmafEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CmafEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CmafEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CmafEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CmafEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CmafEncryption {
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.CmafPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html) property type.
    #[derive(Debug, Default)]
    pub struct CmafPackage {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html#cfn-mediapackage-packagingconfiguration-cmafpackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<CmafEncryption>>,
        /// Property [`HlsManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html#cfn-mediapackage-packagingconfiguration-cmafpackage-hlsmanifests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_manifests: ::ValueList<HlsManifest>,
        /// Property [`IncludeEncoderConfigurationInSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html#cfn-mediapackage-packagingconfiguration-cmafpackage-includeencoderconfigurationinsegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_encoder_configuration_in_segments: Option<::Value<bool>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-cmafpackage.html#cfn-mediapackage-packagingconfiguration-cmafpackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CmafPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsManifests", &self.hls_manifests)?;
            if let Some(ref include_encoder_configuration_in_segments) = self.include_encoder_configuration_in_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeEncoderConfigurationInSegments", include_encoder_configuration_in_segments)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CmafPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CmafPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CmafPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CmafPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<CmafEncryption>> = None;
                    let mut hls_manifests: Option<::ValueList<HlsManifest>> = None;
                    let mut include_encoder_configuration_in_segments: Option<::Value<bool>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsManifests" => {
                                hls_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeEncoderConfigurationInSegments" => {
                                include_encoder_configuration_in_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CmafPackage {
                        encryption: encryption,
                        hls_manifests: hls_manifests.ok_or(::serde::de::Error::missing_field("HlsManifests"))?,
                        include_encoder_configuration_in_segments: include_encoder_configuration_in_segments,
                        segment_duration_seconds: segment_duration_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.DashEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct DashEncryption {
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashencryption.html#cfn-mediapackage-packagingconfiguration-dashencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for DashEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashEncryption {
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.DashManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html) property type.
    #[derive(Debug, Default)]
    pub struct DashManifest {
        /// Property [`ManifestLayout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html#cfn-mediapackage-packagingconfiguration-dashmanifest-manifestlayout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_layout: Option<::Value<String>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html#cfn-mediapackage-packagingconfiguration-dashmanifest-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: Option<::Value<String>>,
        /// Property [`MinBufferTimeSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html#cfn-mediapackage-packagingconfiguration-dashmanifest-minbuffertimeseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_buffer_time_seconds: Option<::Value<u32>>,
        /// Property [`Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html#cfn-mediapackage-packagingconfiguration-dashmanifest-profile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile: Option<::Value<String>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashmanifest.html#cfn-mediapackage-packagingconfiguration-dashmanifest-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
    }

    impl ::codec::SerializeValue for DashManifest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref manifest_layout) = self.manifest_layout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestLayout", manifest_layout)?;
            }
            if let Some(ref manifest_name) = self.manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", manifest_name)?;
            }
            if let Some(ref min_buffer_time_seconds) = self.min_buffer_time_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinBufferTimeSeconds", min_buffer_time_seconds)?;
            }
            if let Some(ref profile) = self.profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Profile", profile)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashManifest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashManifest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashManifest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashManifest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_layout: Option<::Value<String>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut min_buffer_time_seconds: Option<::Value<u32>> = None;
                    let mut profile: Option<::Value<String>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestLayout" => {
                                manifest_layout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinBufferTimeSeconds" => {
                                min_buffer_time_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Profile" => {
                                profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashManifest {
                        manifest_layout: manifest_layout,
                        manifest_name: manifest_name,
                        min_buffer_time_seconds: min_buffer_time_seconds,
                        profile: profile,
                        stream_selection: stream_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.DashPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html) property type.
    #[derive(Debug, Default)]
    pub struct DashPackage {
        /// Property [`DashManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-dashmanifests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dash_manifests: ::ValueList<DashManifest>,
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<DashEncryption>>,
        /// Property [`IncludeEncoderConfigurationInSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-includeencoderconfigurationinsegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_encoder_configuration_in_segments: Option<::Value<bool>>,
        /// Property [`PeriodTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-periodtriggers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period_triggers: Option<::ValueList<String>>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`SegmentTemplateFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-dashpackage.html#cfn-mediapackage-packagingconfiguration-dashpackage-segmenttemplateformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_template_format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DashPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashManifests", &self.dash_manifests)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            if let Some(ref include_encoder_configuration_in_segments) = self.include_encoder_configuration_in_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeEncoderConfigurationInSegments", include_encoder_configuration_in_segments)?;
            }
            if let Some(ref period_triggers) = self.period_triggers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeriodTriggers", period_triggers)?;
            }
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref segment_template_format) = self.segment_template_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentTemplateFormat", segment_template_format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dash_manifests: Option<::ValueList<DashManifest>> = None;
                    let mut encryption: Option<::Value<DashEncryption>> = None;
                    let mut include_encoder_configuration_in_segments: Option<::Value<bool>> = None;
                    let mut period_triggers: Option<::ValueList<String>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut segment_template_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DashManifests" => {
                                dash_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeEncoderConfigurationInSegments" => {
                                include_encoder_configuration_in_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeriodTriggers" => {
                                period_triggers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentTemplateFormat" => {
                                segment_template_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashPackage {
                        dash_manifests: dash_manifests.ok_or(::serde::de::Error::missing_field("DashManifests"))?,
                        encryption: encryption,
                        include_encoder_configuration_in_segments: include_encoder_configuration_in_segments,
                        period_triggers: period_triggers,
                        segment_duration_seconds: segment_duration_seconds,
                        segment_template_format: segment_template_format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.HlsEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsEncryption {
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsencryption.html#cfn-mediapackage-packagingconfiguration-hlsencryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`EncryptionMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsencryption.html#cfn-mediapackage-packagingconfiguration-hlsencryption-encryptionmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_method: Option<::Value<String>>,
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsencryption.html#cfn-mediapackage-packagingconfiguration-hlsencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for HlsEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref encryption_method) = self.encryption_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMethod", encryption_method)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut encryption_method: Option<::Value<String>> = None;
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionMethod" => {
                                encryption_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsEncryption {
                        constant_initialization_vector: constant_initialization_vector,
                        encryption_method: encryption_method,
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.HlsManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsManifest {
        /// Property [`AdMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-admarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markers: Option<::Value<String>>,
        /// Property [`IncludeIframeOnlyStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-includeiframeonlystream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_iframe_only_stream: Option<::Value<bool>>,
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: Option<::Value<String>>,
        /// Property [`ProgramDateTimeIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-programdatetimeintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_interval_seconds: Option<::Value<u32>>,
        /// Property [`RepeatExtXKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-repeatextxkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub repeat_ext_x_key: Option<::Value<bool>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlsmanifest.html#cfn-mediapackage-packagingconfiguration-hlsmanifest-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
    }

    impl ::codec::SerializeValue for HlsManifest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markers) = self.ad_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkers", ad_markers)?;
            }
            if let Some(ref include_iframe_only_stream) = self.include_iframe_only_stream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeIframeOnlyStream", include_iframe_only_stream)?;
            }
            if let Some(ref manifest_name) = self.manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", manifest_name)?;
            }
            if let Some(ref program_date_time_interval_seconds) = self.program_date_time_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimeIntervalSeconds", program_date_time_interval_seconds)?;
            }
            if let Some(ref repeat_ext_x_key) = self.repeat_ext_x_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepeatExtXKey", repeat_ext_x_key)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsManifest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsManifest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsManifest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsManifest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markers: Option<::Value<String>> = None;
                    let mut include_iframe_only_stream: Option<::Value<bool>> = None;
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut program_date_time_interval_seconds: Option<::Value<u32>> = None;
                    let mut repeat_ext_x_key: Option<::Value<bool>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkers" => {
                                ad_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeIframeOnlyStream" => {
                                include_iframe_only_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimeIntervalSeconds" => {
                                program_date_time_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepeatExtXKey" => {
                                repeat_ext_x_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsManifest {
                        ad_markers: ad_markers,
                        include_iframe_only_stream: include_iframe_only_stream,
                        manifest_name: manifest_name,
                        program_date_time_interval_seconds: program_date_time_interval_seconds,
                        repeat_ext_x_key: repeat_ext_x_key,
                        stream_selection: stream_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.HlsPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsPackage {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html#cfn-mediapackage-packagingconfiguration-hlspackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<HlsEncryption>>,
        /// Property [`HlsManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html#cfn-mediapackage-packagingconfiguration-hlspackage-hlsmanifests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_manifests: ::ValueList<HlsManifest>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html#cfn-mediapackage-packagingconfiguration-hlspackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
        /// Property [`UseAudioRenditionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-hlspackage.html#cfn-mediapackage-packagingconfiguration-hlspackage-useaudiorenditiongroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_audio_rendition_group: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for HlsPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsManifests", &self.hls_manifests)?;
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            if let Some(ref use_audio_rendition_group) = self.use_audio_rendition_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseAudioRenditionGroup", use_audio_rendition_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<HlsEncryption>> = None;
                    let mut hls_manifests: Option<::ValueList<HlsManifest>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;
                    let mut use_audio_rendition_group: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsManifests" => {
                                hls_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseAudioRenditionGroup" => {
                                use_audio_rendition_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsPackage {
                        encryption: encryption,
                        hls_manifests: hls_manifests.ok_or(::serde::de::Error::missing_field("HlsManifests"))?,
                        segment_duration_seconds: segment_duration_seconds,
                        use_audio_rendition_group: use_audio_rendition_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.MssEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct MssEncryption {
        /// Property [`SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssencryption.html#cfn-mediapackage-packagingconfiguration-mssencryption-spekekeyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub speke_key_provider: ::Value<SpekeKeyProvider>,
    }

    impl ::codec::SerializeValue for MssEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpekeKeyProvider", &self.speke_key_provider)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MssEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MssEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MssEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MssEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut speke_key_provider: Option<::Value<SpekeKeyProvider>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpekeKeyProvider" => {
                                speke_key_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MssEncryption {
                        speke_key_provider: speke_key_provider.ok_or(::serde::de::Error::missing_field("SpekeKeyProvider"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.MssManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssmanifest.html) property type.
    #[derive(Debug, Default)]
    pub struct MssManifest {
        /// Property [`ManifestName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssmanifest.html#cfn-mediapackage-packagingconfiguration-mssmanifest-manifestname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_name: Option<::Value<String>>,
        /// Property [`StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-mssmanifest.html#cfn-mediapackage-packagingconfiguration-mssmanifest-streamselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_selection: Option<::Value<StreamSelection>>,
    }

    impl ::codec::SerializeValue for MssManifest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref manifest_name) = self.manifest_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestName", manifest_name)?;
            }
            if let Some(ref stream_selection) = self.stream_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSelection", stream_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MssManifest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MssManifest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MssManifest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MssManifest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_name: Option<::Value<String>> = None;
                    let mut stream_selection: Option<::Value<StreamSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestName" => {
                                manifest_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamSelection" => {
                                stream_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MssManifest {
                        manifest_name: manifest_name,
                        stream_selection: stream_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.MssPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-msspackage.html) property type.
    #[derive(Debug, Default)]
    pub struct MssPackage {
        /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-msspackage.html#cfn-mediapackage-packagingconfiguration-msspackage-encryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption: Option<::Value<MssEncryption>>,
        /// Property [`MssManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-msspackage.html#cfn-mediapackage-packagingconfiguration-msspackage-mssmanifests).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mss_manifests: ::ValueList<MssManifest>,
        /// Property [`SegmentDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-msspackage.html#cfn-mediapackage-packagingconfiguration-msspackage-segmentdurationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_duration_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for MssPackage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption) = self.encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MssManifests", &self.mss_manifests)?;
            if let Some(ref segment_duration_seconds) = self.segment_duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentDurationSeconds", segment_duration_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MssPackage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MssPackage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MssPackage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MssPackage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption: Option<::Value<MssEncryption>> = None;
                    let mut mss_manifests: Option<::ValueList<MssManifest>> = None;
                    let mut segment_duration_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encryption" => {
                                encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MssManifests" => {
                                mss_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentDurationSeconds" => {
                                segment_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MssPackage {
                        encryption: encryption,
                        mss_manifests: mss_manifests.ok_or(::serde::de::Error::missing_field("MssManifests"))?,
                        segment_duration_seconds: segment_duration_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.SpekeKeyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-spekekeyprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct SpekeKeyProvider {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-spekekeyprovider.html#cfn-mediapackage-packagingconfiguration-spekekeyprovider-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SystemIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-spekekeyprovider.html#cfn-mediapackage-packagingconfiguration-spekekeyprovider-systemids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub system_ids: ::ValueList<String>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-spekekeyprovider.html#cfn-mediapackage-packagingconfiguration-spekekeyprovider-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for SpekeKeyProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemIds", &self.system_ids)?;
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
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut system_ids: Option<::ValueList<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SystemIds" => {
                                system_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SpekeKeyProvider {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        system_ids: system_ids.ok_or(::serde::de::Error::missing_field("SystemIds"))?,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingConfiguration.StreamSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-streamselection.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamSelection {
        /// Property [`MaxVideoBitsPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-streamselection.html#cfn-mediapackage-packagingconfiguration-streamselection-maxvideobitspersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_video_bits_per_second: Option<::Value<u32>>,
        /// Property [`MinVideoBitsPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-streamselection.html#cfn-mediapackage-packagingconfiguration-streamselection-minvideobitspersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_video_bits_per_second: Option<::Value<u32>>,
        /// Property [`StreamOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packagingconfiguration-streamselection.html#cfn-mediapackage-packagingconfiguration-streamselection-streamorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_order: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StreamSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_video_bits_per_second) = self.max_video_bits_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxVideoBitsPerSecond", max_video_bits_per_second)?;
            }
            if let Some(ref min_video_bits_per_second) = self.min_video_bits_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinVideoBitsPerSecond", min_video_bits_per_second)?;
            }
            if let Some(ref stream_order) = self.stream_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamOrder", stream_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_video_bits_per_second: Option<::Value<u32>> = None;
                    let mut min_video_bits_per_second: Option<::Value<u32>> = None;
                    let mut stream_order: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxVideoBitsPerSecond" => {
                                max_video_bits_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinVideoBitsPerSecond" => {
                                min_video_bits_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamOrder" => {
                                stream_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamSelection {
                        max_video_bits_per_second: max_video_bits_per_second,
                        min_video_bits_per_second: min_video_bits_per_second,
                        stream_order: stream_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod packaging_group {
    //! Property types for the `PackagingGroup` resource.

    /// The [`AWS::MediaPackage::PackagingGroup.Authorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-authorization.html) property type.
    #[derive(Debug, Default)]
    pub struct Authorization {
        /// Property [`CdnIdentifierSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-authorization.html#cfn-mediapackage-packaginggroup-authorization-cdnidentifiersecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdn_identifier_secret: ::Value<String>,
        /// Property [`SecretsRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-authorization.html#cfn-mediapackage-packaginggroup-authorization-secretsrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Authorization {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdnIdentifierSecret", &self.cdn_identifier_secret)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsRoleArn", &self.secrets_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Authorization {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Authorization, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Authorization;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Authorization")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cdn_identifier_secret: Option<::Value<String>> = None;
                    let mut secrets_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CdnIdentifierSecret" => {
                                cdn_identifier_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsRoleArn" => {
                                secrets_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Authorization {
                        cdn_identifier_secret: cdn_identifier_secret.ok_or(::serde::de::Error::missing_field("CdnIdentifierSecret"))?,
                        secrets_role_arn: secrets_role_arn.ok_or(::serde::de::Error::missing_field("SecretsRoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaPackage::PackagingGroup.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-logconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfiguration {
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediapackage-packaginggroup-logconfiguration.html#cfn-mediapackage-packaginggroup-logconfiguration-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_group_name: log_group_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
