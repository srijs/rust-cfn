//! Types for the `MediaLive` service.

/// The [`AWS::MediaLive::Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html) resource type.
#[derive(Debug, Default)]
pub struct Channel {
    properties: ChannelProperties
}

/// Properties for the `Channel` resource.
#[derive(Debug, Default)]
pub struct ChannelProperties {
    /// Property [`CdiInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-cdiinputspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cdi_input_specification: Option<::Value<self::channel::CdiInputSpecification>>,
    /// Property [`ChannelClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-channelclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_class: Option<::Value<String>>,
    /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-destinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destinations: Option<::ValueList<self::channel::OutputDestination>>,
    /// Property [`EncoderSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-encodersettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encoder_settings: Option<::Value<self::channel::EncoderSettings>>,
    /// Property [`InputAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-inputattachments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_attachments: Option<::ValueList<self::channel::InputAttachment>>,
    /// Property [`InputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-inputspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_specification: Option<::Value<self::channel::InputSpecification>>,
    /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-loglevel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_level: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Vpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-channel.html#cfn-medialive-channel-vpc).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc: Option<::Value<self::channel::VpcOutputSettings>>,
}

impl ::serde::Serialize for ChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cdi_input_specification) = self.cdi_input_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdiInputSpecification", cdi_input_specification)?;
        }
        if let Some(ref channel_class) = self.channel_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelClass", channel_class)?;
        }
        if let Some(ref destinations) = self.destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
        }
        if let Some(ref encoder_settings) = self.encoder_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncoderSettings", encoder_settings)?;
        }
        if let Some(ref input_attachments) = self.input_attachments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputAttachments", input_attachments)?;
        }
        if let Some(ref input_specification) = self.input_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSpecification", input_specification)?;
        }
        if let Some(ref log_level) = self.log_level {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", log_level)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc) = self.vpc {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vpc", vpc)?;
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
                let mut cdi_input_specification: Option<::Value<self::channel::CdiInputSpecification>> = None;
                let mut channel_class: Option<::Value<String>> = None;
                let mut destinations: Option<::ValueList<self::channel::OutputDestination>> = None;
                let mut encoder_settings: Option<::Value<self::channel::EncoderSettings>> = None;
                let mut input_attachments: Option<::ValueList<self::channel::InputAttachment>> = None;
                let mut input_specification: Option<::Value<self::channel::InputSpecification>> = None;
                let mut log_level: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut vpc: Option<::Value<self::channel::VpcOutputSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CdiInputSpecification" => {
                            cdi_input_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelClass" => {
                            channel_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Destinations" => {
                            destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncoderSettings" => {
                            encoder_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputAttachments" => {
                            input_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputSpecification" => {
                            input_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogLevel" => {
                            log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Vpc" => {
                            vpc = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ChannelProperties {
                    cdi_input_specification: cdi_input_specification,
                    channel_class: channel_class,
                    destinations: destinations,
                    encoder_settings: encoder_settings,
                    input_attachments: input_attachments,
                    input_specification: input_specification,
                    log_level: log_level,
                    name: name,
                    role_arn: role_arn,
                    tags: tags,
                    vpc: vpc,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Channel {
    type Properties = ChannelProperties;
    const TYPE: &'static str = "AWS::MediaLive::Channel";
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

/// The [`AWS::MediaLive::Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html) resource type.
#[derive(Debug, Default)]
pub struct Input {
    properties: InputProperties
}

/// Properties for the `Input` resource.
#[derive(Debug, Default)]
pub struct InputProperties {
    /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-destinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destinations: Option<::ValueList<self::input::InputDestinationRequest>>,
    /// Property [`InputDevices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-inputdevices).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_devices: Option<::ValueList<self::input::InputDeviceSettings>>,
    /// Property [`InputSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-inputsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_security_groups: Option<::ValueList<String>>,
    /// Property [`MediaConnectFlows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-mediaconnectflows).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub media_connect_flows: Option<::ValueList<self::input::MediaConnectFlowRequest>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-sources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sources: Option<::ValueList<self::input::InputSourceRequest>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
    /// Property [`Vpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-input.html#cfn-medialive-input-vpc).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc: Option<::Value<self::input::InputVpcRequest>>,
}

impl ::serde::Serialize for InputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref destinations) = self.destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
        }
        if let Some(ref input_devices) = self.input_devices {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputDevices", input_devices)?;
        }
        if let Some(ref input_security_groups) = self.input_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSecurityGroups", input_security_groups)?;
        }
        if let Some(ref media_connect_flows) = self.media_connect_flows {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaConnectFlows", media_connect_flows)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref sources) = self.sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        if let Some(ref vpc) = self.vpc {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vpc", vpc)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InputProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InputProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InputProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InputProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destinations: Option<::ValueList<self::input::InputDestinationRequest>> = None;
                let mut input_devices: Option<::ValueList<self::input::InputDeviceSettings>> = None;
                let mut input_security_groups: Option<::ValueList<String>> = None;
                let mut media_connect_flows: Option<::ValueList<self::input::MediaConnectFlowRequest>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut sources: Option<::ValueList<self::input::InputSourceRequest>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut vpc: Option<::Value<self::input::InputVpcRequest>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Destinations" => {
                            destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputDevices" => {
                            input_devices = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputSecurityGroups" => {
                            input_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MediaConnectFlows" => {
                            media_connect_flows = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sources" => {
                            sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Vpc" => {
                            vpc = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InputProperties {
                    destinations: destinations,
                    input_devices: input_devices,
                    input_security_groups: input_security_groups,
                    media_connect_flows: media_connect_flows,
                    name: name,
                    role_arn: role_arn,
                    sources: sources,
                    tags: tags,
                    r#type: r#type,
                    vpc: vpc,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Input {
    type Properties = InputProperties;
    const TYPE: &'static str = "AWS::MediaLive::Input";
    fn properties(&self) -> &InputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InputProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Input {}

impl From<InputProperties> for Input {
    fn from(properties: InputProperties) -> Input {
        Input { properties }
    }
}

/// The [`AWS::MediaLive::InputSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-inputsecuritygroup.html) resource type.
#[derive(Debug, Default)]
pub struct InputSecurityGroup {
    properties: InputSecurityGroupProperties
}

/// Properties for the `InputSecurityGroup` resource.
#[derive(Debug, Default)]
pub struct InputSecurityGroupProperties {
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-inputsecuritygroup.html#cfn-medialive-inputsecuritygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`WhitelistRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-medialive-inputsecuritygroup.html#cfn-medialive-inputsecuritygroup-whitelistrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub whitelist_rules: Option<::ValueList<self::input_security_group::InputWhitelistRuleCidr>>,
}

impl ::serde::Serialize for InputSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref whitelist_rules) = self.whitelist_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WhitelistRules", whitelist_rules)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InputSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InputSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InputSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut whitelist_rules: Option<::ValueList<self::input_security_group::InputWhitelistRuleCidr>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WhitelistRules" => {
                            whitelist_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InputSecurityGroupProperties {
                    tags: tags,
                    whitelist_rules: whitelist_rules,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InputSecurityGroup {
    type Properties = InputSecurityGroupProperties;
    const TYPE: &'static str = "AWS::MediaLive::InputSecurityGroup";
    fn properties(&self) -> &InputSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InputSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InputSecurityGroup {}

impl From<InputSecurityGroupProperties> for InputSecurityGroup {
    fn from(properties: InputSecurityGroupProperties) -> InputSecurityGroup {
        InputSecurityGroup { properties }
    }
}

pub mod channel {
    //! Property types for the `Channel` resource.

    /// The [`AWS::MediaLive::Channel.AacSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AacSettings {
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<f64>>,
        /// Property [`CodingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-codingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coding_mode: Option<::Value<String>>,
        /// Property [`InputType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-inputtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_type: Option<::Value<String>>,
        /// Property [`Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-profile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile: Option<::Value<String>>,
        /// Property [`RateControlMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-ratecontrolmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_control_mode: Option<::Value<String>>,
        /// Property [`RawFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-rawformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub raw_format: Option<::Value<String>>,
        /// Property [`SampleRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-samplerate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_rate: Option<::Value<f64>>,
        /// Property [`Spec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-spec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spec: Option<::Value<String>>,
        /// Property [`VbrQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aacsettings.html#cfn-medialive-channel-aacsettings-vbrquality).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vbr_quality: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AacSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref coding_mode) = self.coding_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodingMode", coding_mode)?;
            }
            if let Some(ref input_type) = self.input_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputType", input_type)?;
            }
            if let Some(ref profile) = self.profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Profile", profile)?;
            }
            if let Some(ref rate_control_mode) = self.rate_control_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateControlMode", rate_control_mode)?;
            }
            if let Some(ref raw_format) = self.raw_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RawFormat", raw_format)?;
            }
            if let Some(ref sample_rate) = self.sample_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleRate", sample_rate)?;
            }
            if let Some(ref spec) = self.spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Spec", spec)?;
            }
            if let Some(ref vbr_quality) = self.vbr_quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VbrQuality", vbr_quality)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AacSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AacSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AacSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AacSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bitrate: Option<::Value<f64>> = None;
                    let mut coding_mode: Option<::Value<String>> = None;
                    let mut input_type: Option<::Value<String>> = None;
                    let mut profile: Option<::Value<String>> = None;
                    let mut rate_control_mode: Option<::Value<String>> = None;
                    let mut raw_format: Option<::Value<String>> = None;
                    let mut sample_rate: Option<::Value<f64>> = None;
                    let mut spec: Option<::Value<String>> = None;
                    let mut vbr_quality: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodingMode" => {
                                coding_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputType" => {
                                input_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Profile" => {
                                profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateControlMode" => {
                                rate_control_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RawFormat" => {
                                raw_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleRate" => {
                                sample_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Spec" => {
                                spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VbrQuality" => {
                                vbr_quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AacSettings {
                        bitrate: bitrate,
                        coding_mode: coding_mode,
                        input_type: input_type,
                        profile: profile,
                        rate_control_mode: rate_control_mode,
                        raw_format: raw_format,
                        sample_rate: sample_rate,
                        spec: spec,
                        vbr_quality: vbr_quality,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Ac3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Ac3Settings {
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<f64>>,
        /// Property [`BitstreamMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-bitstreammode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitstream_mode: Option<::Value<String>>,
        /// Property [`CodingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-codingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coding_mode: Option<::Value<String>>,
        /// Property [`Dialnorm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-dialnorm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dialnorm: Option<::Value<u32>>,
        /// Property [`DrcProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-drcprofile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub drc_profile: Option<::Value<String>>,
        /// Property [`LfeFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-lfefilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lfe_filter: Option<::Value<String>>,
        /// Property [`MetadataControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ac3settings.html#cfn-medialive-channel-ac3settings-metadatacontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata_control: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Ac3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref bitstream_mode) = self.bitstream_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BitstreamMode", bitstream_mode)?;
            }
            if let Some(ref coding_mode) = self.coding_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodingMode", coding_mode)?;
            }
            if let Some(ref dialnorm) = self.dialnorm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dialnorm", dialnorm)?;
            }
            if let Some(ref drc_profile) = self.drc_profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrcProfile", drc_profile)?;
            }
            if let Some(ref lfe_filter) = self.lfe_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LfeFilter", lfe_filter)?;
            }
            if let Some(ref metadata_control) = self.metadata_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataControl", metadata_control)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ac3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ac3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ac3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ac3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bitrate: Option<::Value<f64>> = None;
                    let mut bitstream_mode: Option<::Value<String>> = None;
                    let mut coding_mode: Option<::Value<String>> = None;
                    let mut dialnorm: Option<::Value<u32>> = None;
                    let mut drc_profile: Option<::Value<String>> = None;
                    let mut lfe_filter: Option<::Value<String>> = None;
                    let mut metadata_control: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BitstreamMode" => {
                                bitstream_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodingMode" => {
                                coding_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dialnorm" => {
                                dialnorm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DrcProfile" => {
                                drc_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LfeFilter" => {
                                lfe_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetadataControl" => {
                                metadata_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ac3Settings {
                        bitrate: bitrate,
                        bitstream_mode: bitstream_mode,
                        coding_mode: coding_mode,
                        dialnorm: dialnorm,
                        drc_profile: drc_profile,
                        lfe_filter: lfe_filter,
                        metadata_control: metadata_control,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AncillarySourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ancillarysourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AncillarySourceSettings {
        /// Property [`SourceAncillaryChannelNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ancillarysourcesettings.html#cfn-medialive-channel-ancillarysourcesettings-sourceancillarychannelnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_ancillary_channel_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AncillarySourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_ancillary_channel_number) = self.source_ancillary_channel_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAncillaryChannelNumber", source_ancillary_channel_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AncillarySourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AncillarySourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AncillarySourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AncillarySourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_ancillary_channel_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceAncillaryChannelNumber" => {
                                source_ancillary_channel_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AncillarySourceSettings {
                        source_ancillary_channel_number: source_ancillary_channel_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ArchiveCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecdnsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveCdnSettings {
        /// Property [`ArchiveS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecdnsettings.html#cfn-medialive-channel-archivecdnsettings-archives3settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archive_s3_settings: Option<::Value<ArchiveS3Settings>>,
    }

    impl ::codec::SerializeValue for ArchiveCdnSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref archive_s3_settings) = self.archive_s3_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveS3Settings", archive_s3_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArchiveCdnSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveCdnSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveCdnSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveCdnSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut archive_s3_settings: Option<::Value<ArchiveS3Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArchiveS3Settings" => {
                                archive_s3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveCdnSettings {
                        archive_s3_settings: archive_s3_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ArchiveContainerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecontainersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveContainerSettings {
        /// Property [`M2tsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecontainersettings.html#cfn-medialive-channel-archivecontainersettings-m2tssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub m2ts_settings: Option<::Value<M2tsSettings>>,
        /// Property [`RawSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivecontainersettings.html#cfn-medialive-channel-archivecontainersettings-rawsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub raw_settings: Option<::Value<RawSettings>>,
    }

    impl ::codec::SerializeValue for ArchiveContainerSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref m2ts_settings) = self.m2ts_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "M2tsSettings", m2ts_settings)?;
            }
            if let Some(ref raw_settings) = self.raw_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RawSettings", raw_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArchiveContainerSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveContainerSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveContainerSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveContainerSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut m2ts_settings: Option<::Value<M2tsSettings>> = None;
                    let mut raw_settings: Option<::Value<RawSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "M2tsSettings" => {
                                m2ts_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RawSettings" => {
                                raw_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveContainerSettings {
                        m2ts_settings: m2ts_settings,
                        raw_settings: raw_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ArchiveGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivegroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveGroupSettings {
        /// Property [`ArchiveCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivegroupsettings.html#cfn-medialive-channel-archivegroupsettings-archivecdnsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archive_cdn_settings: Option<::Value<ArchiveCdnSettings>>,
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivegroupsettings.html#cfn-medialive-channel-archivegroupsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`RolloverInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archivegroupsettings.html#cfn-medialive-channel-archivegroupsettings-rolloverinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rollover_interval: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ArchiveGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref archive_cdn_settings) = self.archive_cdn_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveCdnSettings", archive_cdn_settings)?;
            }
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref rollover_interval) = self.rollover_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RolloverInterval", rollover_interval)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArchiveGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut archive_cdn_settings: Option<::Value<ArchiveCdnSettings>> = None;
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut rollover_interval: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArchiveCdnSettings" => {
                                archive_cdn_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RolloverInterval" => {
                                rollover_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveGroupSettings {
                        archive_cdn_settings: archive_cdn_settings,
                        destination: destination,
                        rollover_interval: rollover_interval,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ArchiveOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archiveoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveOutputSettings {
        /// Property [`ContainerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archiveoutputsettings.html#cfn-medialive-channel-archiveoutputsettings-containersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_settings: Option<::Value<ArchiveContainerSettings>>,
        /// Property [`Extension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archiveoutputsettings.html#cfn-medialive-channel-archiveoutputsettings-extension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extension: Option<::Value<String>>,
        /// Property [`NameModifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archiveoutputsettings.html#cfn-medialive-channel-archiveoutputsettings-namemodifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name_modifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ArchiveOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_settings) = self.container_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerSettings", container_settings)?;
            }
            if let Some(ref extension) = self.extension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Extension", extension)?;
            }
            if let Some(ref name_modifier) = self.name_modifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameModifier", name_modifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArchiveOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_settings: Option<::Value<ArchiveContainerSettings>> = None;
                    let mut extension: Option<::Value<String>> = None;
                    let mut name_modifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerSettings" => {
                                container_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Extension" => {
                                extension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NameModifier" => {
                                name_modifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveOutputSettings {
                        container_settings: container_settings,
                        extension: extension,
                        name_modifier: name_modifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ArchiveS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archives3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct ArchiveS3Settings {
        /// Property [`CannedAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-archives3settings.html#cfn-medialive-channel-archives3settings-cannedacl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canned_acl: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ArchiveS3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref canned_acl) = self.canned_acl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CannedAcl", canned_acl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ArchiveS3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ArchiveS3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ArchiveS3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ArchiveS3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut canned_acl: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CannedAcl" => {
                                canned_acl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ArchiveS3Settings {
                        canned_acl: canned_acl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AribDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aribdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AribDestinationSettings {
    }

    impl ::codec::SerializeValue for AribDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AribDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AribDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AribDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AribDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(AribDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AribSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-aribsourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AribSourceSettings {
    }

    impl ::codec::SerializeValue for AribSourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AribSourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AribSourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AribSourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AribSourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(AribSourceSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioChannelMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiochannelmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioChannelMapping {
        /// Property [`InputChannelLevels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiochannelmapping.html#cfn-medialive-channel-audiochannelmapping-inputchannellevels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_channel_levels: Option<::ValueList<InputChannelLevel>>,
        /// Property [`OutputChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiochannelmapping.html#cfn-medialive-channel-audiochannelmapping-outputchannel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_channel: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AudioChannelMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_channel_levels) = self.input_channel_levels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputChannelLevels", input_channel_levels)?;
            }
            if let Some(ref output_channel) = self.output_channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputChannel", output_channel)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioChannelMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioChannelMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioChannelMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioChannelMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_channel_levels: Option<::ValueList<InputChannelLevel>> = None;
                    let mut output_channel: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputChannelLevels" => {
                                input_channel_levels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputChannel" => {
                                output_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioChannelMapping {
                        input_channel_levels: input_channel_levels,
                        output_channel: output_channel,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioCodecSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioCodecSettings {
        /// Property [`AacSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-aacsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aac_settings: Option<::Value<AacSettings>>,
        /// Property [`Ac3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-ac3settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ac3_settings: Option<::Value<Ac3Settings>>,
        /// Property [`Eac3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-eac3settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub eac3_settings: Option<::Value<Eac3Settings>>,
        /// Property [`Mp2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-mp2settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mp2_settings: Option<::Value<Mp2Settings>>,
        /// Property [`PassThroughSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-passthroughsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pass_through_settings: Option<::Value<PassThroughSettings>>,
        /// Property [`WavSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiocodecsettings.html#cfn-medialive-channel-audiocodecsettings-wavsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wav_settings: Option<::Value<WavSettings>>,
    }

    impl ::codec::SerializeValue for AudioCodecSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aac_settings) = self.aac_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AacSettings", aac_settings)?;
            }
            if let Some(ref ac3_settings) = self.ac3_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ac3Settings", ac3_settings)?;
            }
            if let Some(ref eac3_settings) = self.eac3_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Eac3Settings", eac3_settings)?;
            }
            if let Some(ref mp2_settings) = self.mp2_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mp2Settings", mp2_settings)?;
            }
            if let Some(ref pass_through_settings) = self.pass_through_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassThroughSettings", pass_through_settings)?;
            }
            if let Some(ref wav_settings) = self.wav_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WavSettings", wav_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioCodecSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioCodecSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioCodecSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioCodecSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aac_settings: Option<::Value<AacSettings>> = None;
                    let mut ac3_settings: Option<::Value<Ac3Settings>> = None;
                    let mut eac3_settings: Option<::Value<Eac3Settings>> = None;
                    let mut mp2_settings: Option<::Value<Mp2Settings>> = None;
                    let mut pass_through_settings: Option<::Value<PassThroughSettings>> = None;
                    let mut wav_settings: Option<::Value<WavSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AacSettings" => {
                                aac_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ac3Settings" => {
                                ac3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Eac3Settings" => {
                                eac3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mp2Settings" => {
                                mp2_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PassThroughSettings" => {
                                pass_through_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WavSettings" => {
                                wav_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioCodecSettings {
                        aac_settings: aac_settings,
                        ac3_settings: ac3_settings,
                        eac3_settings: eac3_settings,
                        mp2_settings: mp2_settings,
                        pass_through_settings: pass_through_settings,
                        wav_settings: wav_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioDescription {
        /// Property [`AudioNormalizationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-audionormalizationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_normalization_settings: Option<::Value<AudioNormalizationSettings>>,
        /// Property [`AudioSelectorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-audioselectorname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_selector_name: Option<::Value<String>>,
        /// Property [`AudioType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-audiotype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_type: Option<::Value<String>>,
        /// Property [`AudioTypeControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-audiotypecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_type_control: Option<::Value<String>>,
        /// Property [`CodecSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-codecsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub codec_settings: Option<::Value<AudioCodecSettings>>,
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-languagecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code: Option<::Value<String>>,
        /// Property [`LanguageCodeControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-languagecodecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code_control: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`RemixSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-remixsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remix_settings: Option<::Value<RemixSettings>>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiodescription.html#cfn-medialive-channel-audiodescription-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AudioDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_normalization_settings) = self.audio_normalization_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioNormalizationSettings", audio_normalization_settings)?;
            }
            if let Some(ref audio_selector_name) = self.audio_selector_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSelectorName", audio_selector_name)?;
            }
            if let Some(ref audio_type) = self.audio_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioType", audio_type)?;
            }
            if let Some(ref audio_type_control) = self.audio_type_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioTypeControl", audio_type_control)?;
            }
            if let Some(ref codec_settings) = self.codec_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodecSettings", codec_settings)?;
            }
            if let Some(ref language_code) = self.language_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", language_code)?;
            }
            if let Some(ref language_code_control) = self.language_code_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCodeControl", language_code_control)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref remix_settings) = self.remix_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemixSettings", remix_settings)?;
            }
            if let Some(ref stream_name) = self.stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", stream_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_normalization_settings: Option<::Value<AudioNormalizationSettings>> = None;
                    let mut audio_selector_name: Option<::Value<String>> = None;
                    let mut audio_type: Option<::Value<String>> = None;
                    let mut audio_type_control: Option<::Value<String>> = None;
                    let mut codec_settings: Option<::Value<AudioCodecSettings>> = None;
                    let mut language_code: Option<::Value<String>> = None;
                    let mut language_code_control: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut remix_settings: Option<::Value<RemixSettings>> = None;
                    let mut stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioNormalizationSettings" => {
                                audio_normalization_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioSelectorName" => {
                                audio_selector_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioType" => {
                                audio_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioTypeControl" => {
                                audio_type_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodecSettings" => {
                                codec_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageCodeControl" => {
                                language_code_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemixSettings" => {
                                remix_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioDescription {
                        audio_normalization_settings: audio_normalization_settings,
                        audio_selector_name: audio_selector_name,
                        audio_type: audio_type,
                        audio_type_control: audio_type_control,
                        codec_settings: codec_settings,
                        language_code: language_code,
                        language_code_control: language_code_control,
                        name: name,
                        remix_settings: remix_settings,
                        stream_name: stream_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioLanguageSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiolanguageselection.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioLanguageSelection {
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiolanguageselection.html#cfn-medialive-channel-audiolanguageselection-languagecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code: Option<::Value<String>>,
        /// Property [`LanguageSelectionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiolanguageselection.html#cfn-medialive-channel-audiolanguageselection-languageselectionpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_selection_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AudioLanguageSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref language_code) = self.language_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", language_code)?;
            }
            if let Some(ref language_selection_policy) = self.language_selection_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageSelectionPolicy", language_selection_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioLanguageSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioLanguageSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioLanguageSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioLanguageSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut language_code: Option<::Value<String>> = None;
                    let mut language_selection_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageSelectionPolicy" => {
                                language_selection_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioLanguageSelection {
                        language_code: language_code,
                        language_selection_policy: language_selection_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioNormalizationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audionormalizationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioNormalizationSettings {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audionormalizationsettings.html#cfn-medialive-channel-audionormalizationsettings-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: Option<::Value<String>>,
        /// Property [`AlgorithmControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audionormalizationsettings.html#cfn-medialive-channel-audionormalizationsettings-algorithmcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm_control: Option<::Value<String>>,
        /// Property [`TargetLkfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audionormalizationsettings.html#cfn-medialive-channel-audionormalizationsettings-targetlkfs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_lkfs: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for AudioNormalizationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref algorithm) = self.algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", algorithm)?;
            }
            if let Some(ref algorithm_control) = self.algorithm_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlgorithmControl", algorithm_control)?;
            }
            if let Some(ref target_lkfs) = self.target_lkfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetLkfs", target_lkfs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioNormalizationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioNormalizationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioNormalizationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioNormalizationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut algorithm_control: Option<::Value<String>> = None;
                    let mut target_lkfs: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AlgorithmControl" => {
                                algorithm_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetLkfs" => {
                                target_lkfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioNormalizationSettings {
                        algorithm: algorithm,
                        algorithm_control: algorithm_control,
                        target_lkfs: target_lkfs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioOnlyHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioOnlyHlsSettings {
        /// Property [`AudioGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html#cfn-medialive-channel-audioonlyhlssettings-audiogroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_group_id: Option<::Value<String>>,
        /// Property [`AudioOnlyImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html#cfn-medialive-channel-audioonlyhlssettings-audioonlyimage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_only_image: Option<::Value<InputLocation>>,
        /// Property [`AudioTrackType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html#cfn-medialive-channel-audioonlyhlssettings-audiotracktype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_track_type: Option<::Value<String>>,
        /// Property [`SegmentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioonlyhlssettings.html#cfn-medialive-channel-audioonlyhlssettings-segmenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AudioOnlyHlsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_group_id) = self.audio_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioGroupId", audio_group_id)?;
            }
            if let Some(ref audio_only_image) = self.audio_only_image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioOnlyImage", audio_only_image)?;
            }
            if let Some(ref audio_track_type) = self.audio_track_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioTrackType", audio_track_type)?;
            }
            if let Some(ref segment_type) = self.segment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentType", segment_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioOnlyHlsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioOnlyHlsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioOnlyHlsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioOnlyHlsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_group_id: Option<::Value<String>> = None;
                    let mut audio_only_image: Option<::Value<InputLocation>> = None;
                    let mut audio_track_type: Option<::Value<String>> = None;
                    let mut segment_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioGroupId" => {
                                audio_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioOnlyImage" => {
                                audio_only_image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioTrackType" => {
                                audio_track_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentType" => {
                                segment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioOnlyHlsSettings {
                        audio_group_id: audio_group_id,
                        audio_only_image: audio_only_image,
                        audio_track_type: audio_track_type,
                        segment_type: segment_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioPidSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiopidselection.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioPidSelection {
        /// Property [`Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiopidselection.html#cfn-medialive-channel-audiopidselection-pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AudioPidSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pid) = self.pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pid", pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioPidSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioPidSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioPidSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioPidSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pid" => {
                                pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioPidSelection {
                        pid: pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioSelector {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselector.html#cfn-medialive-channel-audioselector-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselector.html#cfn-medialive-channel-audioselector-selectorsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selector_settings: Option<::Value<AudioSelectorSettings>>,
    }

    impl ::codec::SerializeValue for AudioSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref selector_settings) = self.selector_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectorSettings", selector_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut selector_settings: Option<::Value<AudioSelectorSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectorSettings" => {
                                selector_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioSelector {
                        name: name,
                        selector_settings: selector_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioSelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselectorsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioSelectorSettings {
        /// Property [`AudioLanguageSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselectorsettings.html#cfn-medialive-channel-audioselectorsettings-audiolanguageselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_language_selection: Option<::Value<AudioLanguageSelection>>,
        /// Property [`AudioPidSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselectorsettings.html#cfn-medialive-channel-audioselectorsettings-audiopidselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_pid_selection: Option<::Value<AudioPidSelection>>,
        /// Property [`AudioTrackSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audioselectorsettings.html#cfn-medialive-channel-audioselectorsettings-audiotrackselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_track_selection: Option<::Value<AudioTrackSelection>>,
    }

    impl ::codec::SerializeValue for AudioSelectorSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_language_selection) = self.audio_language_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioLanguageSelection", audio_language_selection)?;
            }
            if let Some(ref audio_pid_selection) = self.audio_pid_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioPidSelection", audio_pid_selection)?;
            }
            if let Some(ref audio_track_selection) = self.audio_track_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioTrackSelection", audio_track_selection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioSelectorSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioSelectorSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioSelectorSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioSelectorSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_language_selection: Option<::Value<AudioLanguageSelection>> = None;
                    let mut audio_pid_selection: Option<::Value<AudioPidSelection>> = None;
                    let mut audio_track_selection: Option<::Value<AudioTrackSelection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioLanguageSelection" => {
                                audio_language_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioPidSelection" => {
                                audio_pid_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioTrackSelection" => {
                                audio_track_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioSelectorSettings {
                        audio_language_selection: audio_language_selection,
                        audio_pid_selection: audio_pid_selection,
                        audio_track_selection: audio_track_selection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioSilenceFailoverSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiosilencefailoversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioSilenceFailoverSettings {
        /// Property [`AudioSelectorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiosilencefailoversettings.html#cfn-medialive-channel-audiosilencefailoversettings-audioselectorname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_selector_name: Option<::Value<String>>,
        /// Property [`AudioSilenceThresholdMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiosilencefailoversettings.html#cfn-medialive-channel-audiosilencefailoversettings-audiosilencethresholdmsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_silence_threshold_msec: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AudioSilenceFailoverSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_selector_name) = self.audio_selector_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSelectorName", audio_selector_name)?;
            }
            if let Some(ref audio_silence_threshold_msec) = self.audio_silence_threshold_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSilenceThresholdMsec", audio_silence_threshold_msec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioSilenceFailoverSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioSilenceFailoverSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioSilenceFailoverSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioSilenceFailoverSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_selector_name: Option<::Value<String>> = None;
                    let mut audio_silence_threshold_msec: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioSelectorName" => {
                                audio_selector_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioSilenceThresholdMsec" => {
                                audio_silence_threshold_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioSilenceFailoverSettings {
                        audio_selector_name: audio_selector_name,
                        audio_silence_threshold_msec: audio_silence_threshold_msec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioTrack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrack.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioTrack {
        /// Property [`Track`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrack.html#cfn-medialive-channel-audiotrack-track).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub track: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AudioTrack {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref track) = self.track {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Track", track)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioTrack {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioTrack, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioTrack;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioTrack")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut track: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Track" => {
                                track = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioTrack {
                        track: track,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AudioTrackSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrackselection.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioTrackSelection {
        /// Property [`Tracks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-audiotrackselection.html#cfn-medialive-channel-audiotrackselection-tracks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tracks: Option<::ValueList<AudioTrack>>,
    }

    impl ::codec::SerializeValue for AudioTrackSelection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tracks) = self.tracks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tracks", tracks)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioTrackSelection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioTrackSelection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioTrackSelection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioTrackSelection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tracks: Option<::ValueList<AudioTrack>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Tracks" => {
                                tracks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioTrackSelection {
                        tracks: tracks,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AutomaticInputFailoverSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AutomaticInputFailoverSettings {
        /// Property [`ErrorClearTimeMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html#cfn-medialive-channel-automaticinputfailoversettings-errorcleartimemsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_clear_time_msec: Option<::Value<u32>>,
        /// Property [`FailoverConditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html#cfn-medialive-channel-automaticinputfailoversettings-failoverconditions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_conditions: Option<::ValueList<FailoverCondition>>,
        /// Property [`InputPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html#cfn-medialive-channel-automaticinputfailoversettings-inputpreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_preference: Option<::Value<String>>,
        /// Property [`SecondaryInputId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-automaticinputfailoversettings.html#cfn-medialive-channel-automaticinputfailoversettings-secondaryinputid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_input_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AutomaticInputFailoverSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_clear_time_msec) = self.error_clear_time_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorClearTimeMsec", error_clear_time_msec)?;
            }
            if let Some(ref failover_conditions) = self.failover_conditions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverConditions", failover_conditions)?;
            }
            if let Some(ref input_preference) = self.input_preference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPreference", input_preference)?;
            }
            if let Some(ref secondary_input_id) = self.secondary_input_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryInputId", secondary_input_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutomaticInputFailoverSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutomaticInputFailoverSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutomaticInputFailoverSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutomaticInputFailoverSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_clear_time_msec: Option<::Value<u32>> = None;
                    let mut failover_conditions: Option<::ValueList<FailoverCondition>> = None;
                    let mut input_preference: Option<::Value<String>> = None;
                    let mut secondary_input_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorClearTimeMsec" => {
                                error_clear_time_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailoverConditions" => {
                                failover_conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputPreference" => {
                                input_preference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryInputId" => {
                                secondary_input_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutomaticInputFailoverSettings {
                        error_clear_time_msec: error_clear_time_msec,
                        failover_conditions: failover_conditions,
                        input_preference: input_preference,
                        secondary_input_id: secondary_input_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AvailBlanking`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availblanking.html) property type.
    #[derive(Debug, Default)]
    pub struct AvailBlanking {
        /// Property [`AvailBlankingImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availblanking.html#cfn-medialive-channel-availblanking-availblankingimage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub avail_blanking_image: Option<::Value<InputLocation>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availblanking.html#cfn-medialive-channel-availblanking-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AvailBlanking {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref avail_blanking_image) = self.avail_blanking_image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailBlankingImage", avail_blanking_image)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AvailBlanking {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AvailBlanking, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AvailBlanking;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AvailBlanking")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut avail_blanking_image: Option<::Value<InputLocation>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailBlankingImage" => {
                                avail_blanking_image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AvailBlanking {
                        avail_blanking_image: avail_blanking_image,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AvailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AvailConfiguration {
        /// Property [`AvailSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availconfiguration.html#cfn-medialive-channel-availconfiguration-availsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub avail_settings: Option<::Value<AvailSettings>>,
    }

    impl ::codec::SerializeValue for AvailConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref avail_settings) = self.avail_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailSettings", avail_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AvailConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AvailConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AvailConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AvailConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut avail_settings: Option<::Value<AvailSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailSettings" => {
                                avail_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AvailConfiguration {
                        avail_settings: avail_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.AvailSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct AvailSettings {
        /// Property [`Scte35SpliceInsert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availsettings.html#cfn-medialive-channel-availsettings-scte35spliceinsert).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_splice_insert: Option<::Value<Scte35SpliceInsert>>,
        /// Property [`Scte35TimeSignalApos`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-availsettings.html#cfn-medialive-channel-availsettings-scte35timesignalapos).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_time_signal_apos: Option<::Value<Scte35TimeSignalApos>>,
    }

    impl ::codec::SerializeValue for AvailSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref scte35_splice_insert) = self.scte35_splice_insert {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35SpliceInsert", scte35_splice_insert)?;
            }
            if let Some(ref scte35_time_signal_apos) = self.scte35_time_signal_apos {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35TimeSignalApos", scte35_time_signal_apos)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AvailSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AvailSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AvailSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AvailSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scte35_splice_insert: Option<::Value<Scte35SpliceInsert>> = None;
                    let mut scte35_time_signal_apos: Option<::Value<Scte35TimeSignalApos>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Scte35SpliceInsert" => {
                                scte35_splice_insert = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte35TimeSignalApos" => {
                                scte35_time_signal_apos = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AvailSettings {
                        scte35_splice_insert: scte35_splice_insert,
                        scte35_time_signal_apos: scte35_time_signal_apos,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.BlackoutSlate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html) property type.
    #[derive(Debug, Default)]
    pub struct BlackoutSlate {
        /// Property [`BlackoutSlateImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html#cfn-medialive-channel-blackoutslate-blackoutslateimage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blackout_slate_image: Option<::Value<InputLocation>>,
        /// Property [`NetworkEndBlackout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html#cfn-medialive-channel-blackoutslate-networkendblackout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_end_blackout: Option<::Value<String>>,
        /// Property [`NetworkEndBlackoutImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html#cfn-medialive-channel-blackoutslate-networkendblackoutimage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_end_blackout_image: Option<::Value<InputLocation>>,
        /// Property [`NetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html#cfn-medialive-channel-blackoutslate-networkid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_id: Option<::Value<String>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-blackoutslate.html#cfn-medialive-channel-blackoutslate-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlackoutSlate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref blackout_slate_image) = self.blackout_slate_image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlackoutSlateImage", blackout_slate_image)?;
            }
            if let Some(ref network_end_blackout) = self.network_end_blackout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkEndBlackout", network_end_blackout)?;
            }
            if let Some(ref network_end_blackout_image) = self.network_end_blackout_image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkEndBlackoutImage", network_end_blackout_image)?;
            }
            if let Some(ref network_id) = self.network_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkId", network_id)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlackoutSlate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlackoutSlate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlackoutSlate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlackoutSlate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut blackout_slate_image: Option<::Value<InputLocation>> = None;
                    let mut network_end_blackout: Option<::Value<String>> = None;
                    let mut network_end_blackout_image: Option<::Value<InputLocation>> = None;
                    let mut network_id: Option<::Value<String>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlackoutSlateImage" => {
                                blackout_slate_image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkEndBlackout" => {
                                network_end_blackout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkEndBlackoutImage" => {
                                network_end_blackout_image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkId" => {
                                network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlackoutSlate {
                        blackout_slate_image: blackout_slate_image,
                        network_end_blackout: network_end_blackout,
                        network_end_blackout_image: network_end_blackout_image,
                        network_id: network_id,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.BurnInDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct BurnInDestinationSettings {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BackgroundOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-backgroundopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_opacity: Option<::Value<u32>>,
        /// Property [`Font`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-font).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font: Option<::Value<InputLocation>>,
        /// Property [`FontColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-fontcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_color: Option<::Value<String>>,
        /// Property [`FontOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-fontopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_opacity: Option<::Value<u32>>,
        /// Property [`FontResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-fontresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_resolution: Option<::Value<u32>>,
        /// Property [`FontSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-fontsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_size: Option<::Value<String>>,
        /// Property [`OutlineColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-outlinecolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outline_color: Option<::Value<String>>,
        /// Property [`OutlineSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-outlinesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outline_size: Option<::Value<u32>>,
        /// Property [`ShadowColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-shadowcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_color: Option<::Value<String>>,
        /// Property [`ShadowOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-shadowopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_opacity: Option<::Value<u32>>,
        /// Property [`ShadowXOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-shadowxoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_x_offset: Option<::Value<u32>>,
        /// Property [`ShadowYOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-shadowyoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_y_offset: Option<::Value<u32>>,
        /// Property [`TeletextGridControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-teletextgridcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub teletext_grid_control: Option<::Value<String>>,
        /// Property [`XPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-xposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub x_position: Option<::Value<u32>>,
        /// Property [`YPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-burnindestinationsettings.html#cfn-medialive-channel-burnindestinationsettings-yposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub y_position: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for BurnInDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref background_opacity) = self.background_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundOpacity", background_opacity)?;
            }
            if let Some(ref font) = self.font {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Font", font)?;
            }
            if let Some(ref font_color) = self.font_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontColor", font_color)?;
            }
            if let Some(ref font_opacity) = self.font_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontOpacity", font_opacity)?;
            }
            if let Some(ref font_resolution) = self.font_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontResolution", font_resolution)?;
            }
            if let Some(ref font_size) = self.font_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontSize", font_size)?;
            }
            if let Some(ref outline_color) = self.outline_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutlineColor", outline_color)?;
            }
            if let Some(ref outline_size) = self.outline_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutlineSize", outline_size)?;
            }
            if let Some(ref shadow_color) = self.shadow_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowColor", shadow_color)?;
            }
            if let Some(ref shadow_opacity) = self.shadow_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowOpacity", shadow_opacity)?;
            }
            if let Some(ref shadow_x_offset) = self.shadow_x_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowXOffset", shadow_x_offset)?;
            }
            if let Some(ref shadow_y_offset) = self.shadow_y_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowYOffset", shadow_y_offset)?;
            }
            if let Some(ref teletext_grid_control) = self.teletext_grid_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeletextGridControl", teletext_grid_control)?;
            }
            if let Some(ref x_position) = self.x_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "XPosition", x_position)?;
            }
            if let Some(ref y_position) = self.y_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "YPosition", y_position)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BurnInDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BurnInDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BurnInDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BurnInDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut background_color: Option<::Value<String>> = None;
                    let mut background_opacity: Option<::Value<u32>> = None;
                    let mut font: Option<::Value<InputLocation>> = None;
                    let mut font_color: Option<::Value<String>> = None;
                    let mut font_opacity: Option<::Value<u32>> = None;
                    let mut font_resolution: Option<::Value<u32>> = None;
                    let mut font_size: Option<::Value<String>> = None;
                    let mut outline_color: Option<::Value<String>> = None;
                    let mut outline_size: Option<::Value<u32>> = None;
                    let mut shadow_color: Option<::Value<String>> = None;
                    let mut shadow_opacity: Option<::Value<u32>> = None;
                    let mut shadow_x_offset: Option<::Value<u32>> = None;
                    let mut shadow_y_offset: Option<::Value<u32>> = None;
                    let mut teletext_grid_control: Option<::Value<String>> = None;
                    let mut x_position: Option<::Value<u32>> = None;
                    let mut y_position: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackgroundOpacity" => {
                                background_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Font" => {
                                font = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontColor" => {
                                font_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontOpacity" => {
                                font_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontResolution" => {
                                font_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontSize" => {
                                font_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutlineColor" => {
                                outline_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutlineSize" => {
                                outline_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowColor" => {
                                shadow_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowOpacity" => {
                                shadow_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowXOffset" => {
                                shadow_x_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowYOffset" => {
                                shadow_y_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TeletextGridControl" => {
                                teletext_grid_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "XPosition" => {
                                x_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "YPosition" => {
                                y_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BurnInDestinationSettings {
                        alignment: alignment,
                        background_color: background_color,
                        background_opacity: background_opacity,
                        font: font,
                        font_color: font_color,
                        font_opacity: font_opacity,
                        font_resolution: font_resolution,
                        font_size: font_size,
                        outline_color: outline_color,
                        outline_size: outline_size,
                        shadow_color: shadow_color,
                        shadow_opacity: shadow_opacity,
                        shadow_x_offset: shadow_x_offset,
                        shadow_y_offset: shadow_y_offset,
                        teletext_grid_control: teletext_grid_control,
                        x_position: x_position,
                        y_position: y_position,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionDescription {
        /// Property [`CaptionSelectorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html#cfn-medialive-channel-captiondescription-captionselectorname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_selector_name: Option<::Value<String>>,
        /// Property [`DestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html#cfn-medialive-channel-captiondescription-destinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_settings: Option<::Value<CaptionDestinationSettings>>,
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html#cfn-medialive-channel-captiondescription-languagecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code: Option<::Value<String>>,
        /// Property [`LanguageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html#cfn-medialive-channel-captiondescription-languagedescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondescription.html#cfn-medialive-channel-captiondescription-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CaptionDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref caption_selector_name) = self.caption_selector_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionSelectorName", caption_selector_name)?;
            }
            if let Some(ref destination_settings) = self.destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationSettings", destination_settings)?;
            }
            if let Some(ref language_code) = self.language_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", language_code)?;
            }
            if let Some(ref language_description) = self.language_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageDescription", language_description)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut caption_selector_name: Option<::Value<String>> = None;
                    let mut destination_settings: Option<::Value<CaptionDestinationSettings>> = None;
                    let mut language_code: Option<::Value<String>> = None;
                    let mut language_description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptionSelectorName" => {
                                caption_selector_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationSettings" => {
                                destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageDescription" => {
                                language_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionDescription {
                        caption_selector_name: caption_selector_name,
                        destination_settings: destination_settings,
                        language_code: language_code,
                        language_description: language_description,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionDestinationSettings {
        /// Property [`AribDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-aribdestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arib_destination_settings: Option<::Value<AribDestinationSettings>>,
        /// Property [`BurnInDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-burnindestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub burn_in_destination_settings: Option<::Value<BurnInDestinationSettings>>,
        /// Property [`DvbSubDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-dvbsubdestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_sub_destination_settings: Option<::Value<DvbSubDestinationSettings>>,
        /// Property [`EbuTtDDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-ebuttddestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebu_tt_d_destination_settings: Option<::Value<EbuTtDDestinationSettings>>,
        /// Property [`EmbeddedDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-embeddeddestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub embedded_destination_settings: Option<::Value<EmbeddedDestinationSettings>>,
        /// Property [`EmbeddedPlusScte20DestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-embeddedplusscte20destinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub embedded_plus_scte20_destination_settings: Option<::Value<EmbeddedPlusScte20DestinationSettings>>,
        /// Property [`RtmpCaptionInfoDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-rtmpcaptioninfodestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rtmp_caption_info_destination_settings: Option<::Value<RtmpCaptionInfoDestinationSettings>>,
        /// Property [`Scte20PlusEmbeddedDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-scte20plusembeddeddestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte20_plus_embedded_destination_settings: Option<::Value<Scte20PlusEmbeddedDestinationSettings>>,
        /// Property [`Scte27DestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-scte27destinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte27_destination_settings: Option<::Value<Scte27DestinationSettings>>,
        /// Property [`SmpteTtDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-smptettdestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smpte_tt_destination_settings: Option<::Value<SmpteTtDestinationSettings>>,
        /// Property [`TeletextDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-teletextdestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub teletext_destination_settings: Option<::Value<TeletextDestinationSettings>>,
        /// Property [`TtmlDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-ttmldestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ttml_destination_settings: Option<::Value<TtmlDestinationSettings>>,
        /// Property [`WebvttDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captiondestinationsettings.html#cfn-medialive-channel-captiondestinationsettings-webvttdestinationsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub webvtt_destination_settings: Option<::Value<WebvttDestinationSettings>>,
    }

    impl ::codec::SerializeValue for CaptionDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arib_destination_settings) = self.arib_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AribDestinationSettings", arib_destination_settings)?;
            }
            if let Some(ref burn_in_destination_settings) = self.burn_in_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BurnInDestinationSettings", burn_in_destination_settings)?;
            }
            if let Some(ref dvb_sub_destination_settings) = self.dvb_sub_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbSubDestinationSettings", dvb_sub_destination_settings)?;
            }
            if let Some(ref ebu_tt_d_destination_settings) = self.ebu_tt_d_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbuTtDDestinationSettings", ebu_tt_d_destination_settings)?;
            }
            if let Some(ref embedded_destination_settings) = self.embedded_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmbeddedDestinationSettings", embedded_destination_settings)?;
            }
            if let Some(ref embedded_plus_scte20_destination_settings) = self.embedded_plus_scte20_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmbeddedPlusScte20DestinationSettings", embedded_plus_scte20_destination_settings)?;
            }
            if let Some(ref rtmp_caption_info_destination_settings) = self.rtmp_caption_info_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RtmpCaptionInfoDestinationSettings", rtmp_caption_info_destination_settings)?;
            }
            if let Some(ref scte20_plus_embedded_destination_settings) = self.scte20_plus_embedded_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte20PlusEmbeddedDestinationSettings", scte20_plus_embedded_destination_settings)?;
            }
            if let Some(ref scte27_destination_settings) = self.scte27_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte27DestinationSettings", scte27_destination_settings)?;
            }
            if let Some(ref smpte_tt_destination_settings) = self.smpte_tt_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmpteTtDestinationSettings", smpte_tt_destination_settings)?;
            }
            if let Some(ref teletext_destination_settings) = self.teletext_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeletextDestinationSettings", teletext_destination_settings)?;
            }
            if let Some(ref ttml_destination_settings) = self.ttml_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TtmlDestinationSettings", ttml_destination_settings)?;
            }
            if let Some(ref webvtt_destination_settings) = self.webvtt_destination_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebvttDestinationSettings", webvtt_destination_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arib_destination_settings: Option<::Value<AribDestinationSettings>> = None;
                    let mut burn_in_destination_settings: Option<::Value<BurnInDestinationSettings>> = None;
                    let mut dvb_sub_destination_settings: Option<::Value<DvbSubDestinationSettings>> = None;
                    let mut ebu_tt_d_destination_settings: Option<::Value<EbuTtDDestinationSettings>> = None;
                    let mut embedded_destination_settings: Option<::Value<EmbeddedDestinationSettings>> = None;
                    let mut embedded_plus_scte20_destination_settings: Option<::Value<EmbeddedPlusScte20DestinationSettings>> = None;
                    let mut rtmp_caption_info_destination_settings: Option<::Value<RtmpCaptionInfoDestinationSettings>> = None;
                    let mut scte20_plus_embedded_destination_settings: Option<::Value<Scte20PlusEmbeddedDestinationSettings>> = None;
                    let mut scte27_destination_settings: Option<::Value<Scte27DestinationSettings>> = None;
                    let mut smpte_tt_destination_settings: Option<::Value<SmpteTtDestinationSettings>> = None;
                    let mut teletext_destination_settings: Option<::Value<TeletextDestinationSettings>> = None;
                    let mut ttml_destination_settings: Option<::Value<TtmlDestinationSettings>> = None;
                    let mut webvtt_destination_settings: Option<::Value<WebvttDestinationSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AribDestinationSettings" => {
                                arib_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BurnInDestinationSettings" => {
                                burn_in_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbSubDestinationSettings" => {
                                dvb_sub_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbuTtDDestinationSettings" => {
                                ebu_tt_d_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmbeddedDestinationSettings" => {
                                embedded_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmbeddedPlusScte20DestinationSettings" => {
                                embedded_plus_scte20_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RtmpCaptionInfoDestinationSettings" => {
                                rtmp_caption_info_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte20PlusEmbeddedDestinationSettings" => {
                                scte20_plus_embedded_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte27DestinationSettings" => {
                                scte27_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmpteTtDestinationSettings" => {
                                smpte_tt_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TeletextDestinationSettings" => {
                                teletext_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TtmlDestinationSettings" => {
                                ttml_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebvttDestinationSettings" => {
                                webvtt_destination_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionDestinationSettings {
                        arib_destination_settings: arib_destination_settings,
                        burn_in_destination_settings: burn_in_destination_settings,
                        dvb_sub_destination_settings: dvb_sub_destination_settings,
                        ebu_tt_d_destination_settings: ebu_tt_d_destination_settings,
                        embedded_destination_settings: embedded_destination_settings,
                        embedded_plus_scte20_destination_settings: embedded_plus_scte20_destination_settings,
                        rtmp_caption_info_destination_settings: rtmp_caption_info_destination_settings,
                        scte20_plus_embedded_destination_settings: scte20_plus_embedded_destination_settings,
                        scte27_destination_settings: scte27_destination_settings,
                        smpte_tt_destination_settings: smpte_tt_destination_settings,
                        teletext_destination_settings: teletext_destination_settings,
                        ttml_destination_settings: ttml_destination_settings,
                        webvtt_destination_settings: webvtt_destination_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionLanguageMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionlanguagemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionLanguageMapping {
        /// Property [`CaptionChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionlanguagemapping.html#cfn-medialive-channel-captionlanguagemapping-captionchannel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_channel: Option<::Value<u32>>,
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionlanguagemapping.html#cfn-medialive-channel-captionlanguagemapping-languagecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code: Option<::Value<String>>,
        /// Property [`LanguageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionlanguagemapping.html#cfn-medialive-channel-captionlanguagemapping-languagedescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_description: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CaptionLanguageMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref caption_channel) = self.caption_channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionChannel", caption_channel)?;
            }
            if let Some(ref language_code) = self.language_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", language_code)?;
            }
            if let Some(ref language_description) = self.language_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageDescription", language_description)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionLanguageMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionLanguageMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionLanguageMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionLanguageMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut caption_channel: Option<::Value<u32>> = None;
                    let mut language_code: Option<::Value<String>> = None;
                    let mut language_description: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptionChannel" => {
                                caption_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageDescription" => {
                                language_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionLanguageMapping {
                        caption_channel: caption_channel,
                        language_code: language_code,
                        language_description: language_description,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionRectangle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionRectangle {
        /// Property [`Height`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html#cfn-medialive-channel-captionrectangle-height).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub height: Option<::Value<f64>>,
        /// Property [`LeftOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html#cfn-medialive-channel-captionrectangle-leftoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub left_offset: Option<::Value<f64>>,
        /// Property [`TopOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html#cfn-medialive-channel-captionrectangle-topoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub top_offset: Option<::Value<f64>>,
        /// Property [`Width`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionrectangle.html#cfn-medialive-channel-captionrectangle-width).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub width: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for CaptionRectangle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref height) = self.height {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Height", height)?;
            }
            if let Some(ref left_offset) = self.left_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LeftOffset", left_offset)?;
            }
            if let Some(ref top_offset) = self.top_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopOffset", top_offset)?;
            }
            if let Some(ref width) = self.width {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Width", width)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionRectangle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionRectangle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionRectangle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionRectangle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut height: Option<::Value<f64>> = None;
                    let mut left_offset: Option<::Value<f64>> = None;
                    let mut top_offset: Option<::Value<f64>> = None;
                    let mut width: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Height" => {
                                height = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LeftOffset" => {
                                left_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopOffset" => {
                                top_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Width" => {
                                width = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionRectangle {
                        height: height,
                        left_offset: left_offset,
                        top_offset: top_offset,
                        width: width,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselector.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionSelector {
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselector.html#cfn-medialive-channel-captionselector-languagecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language_code: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselector.html#cfn-medialive-channel-captionselector-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselector.html#cfn-medialive-channel-captionselector-selectorsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selector_settings: Option<::Value<CaptionSelectorSettings>>,
    }

    impl ::codec::SerializeValue for CaptionSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref language_code) = self.language_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", language_code)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref selector_settings) = self.selector_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectorSettings", selector_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut language_code: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut selector_settings: Option<::Value<CaptionSelectorSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectorSettings" => {
                                selector_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionSelector {
                        language_code: language_code,
                        name: name,
                        selector_settings: selector_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CaptionSelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptionSelectorSettings {
        /// Property [`AncillarySourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-ancillarysourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ancillary_source_settings: Option<::Value<AncillarySourceSettings>>,
        /// Property [`AribSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-aribsourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arib_source_settings: Option<::Value<AribSourceSettings>>,
        /// Property [`DvbSubSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-dvbsubsourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_sub_source_settings: Option<::Value<DvbSubSourceSettings>>,
        /// Property [`EmbeddedSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-embeddedsourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub embedded_source_settings: Option<::Value<EmbeddedSourceSettings>>,
        /// Property [`Scte20SourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-scte20sourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte20_source_settings: Option<::Value<Scte20SourceSettings>>,
        /// Property [`Scte27SourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-scte27sourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte27_source_settings: Option<::Value<Scte27SourceSettings>>,
        /// Property [`TeletextSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-captionselectorsettings.html#cfn-medialive-channel-captionselectorsettings-teletextsourcesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub teletext_source_settings: Option<::Value<TeletextSourceSettings>>,
    }

    impl ::codec::SerializeValue for CaptionSelectorSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ancillary_source_settings) = self.ancillary_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AncillarySourceSettings", ancillary_source_settings)?;
            }
            if let Some(ref arib_source_settings) = self.arib_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AribSourceSettings", arib_source_settings)?;
            }
            if let Some(ref dvb_sub_source_settings) = self.dvb_sub_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbSubSourceSettings", dvb_sub_source_settings)?;
            }
            if let Some(ref embedded_source_settings) = self.embedded_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmbeddedSourceSettings", embedded_source_settings)?;
            }
            if let Some(ref scte20_source_settings) = self.scte20_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte20SourceSettings", scte20_source_settings)?;
            }
            if let Some(ref scte27_source_settings) = self.scte27_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte27SourceSettings", scte27_source_settings)?;
            }
            if let Some(ref teletext_source_settings) = self.teletext_source_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeletextSourceSettings", teletext_source_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptionSelectorSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptionSelectorSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptionSelectorSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptionSelectorSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ancillary_source_settings: Option<::Value<AncillarySourceSettings>> = None;
                    let mut arib_source_settings: Option<::Value<AribSourceSettings>> = None;
                    let mut dvb_sub_source_settings: Option<::Value<DvbSubSourceSettings>> = None;
                    let mut embedded_source_settings: Option<::Value<EmbeddedSourceSettings>> = None;
                    let mut scte20_source_settings: Option<::Value<Scte20SourceSettings>> = None;
                    let mut scte27_source_settings: Option<::Value<Scte27SourceSettings>> = None;
                    let mut teletext_source_settings: Option<::Value<TeletextSourceSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AncillarySourceSettings" => {
                                ancillary_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AribSourceSettings" => {
                                arib_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbSubSourceSettings" => {
                                dvb_sub_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmbeddedSourceSettings" => {
                                embedded_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte20SourceSettings" => {
                                scte20_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte27SourceSettings" => {
                                scte27_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TeletextSourceSettings" => {
                                teletext_source_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptionSelectorSettings {
                        ancillary_source_settings: ancillary_source_settings,
                        arib_source_settings: arib_source_settings,
                        dvb_sub_source_settings: dvb_sub_source_settings,
                        embedded_source_settings: embedded_source_settings,
                        scte20_source_settings: scte20_source_settings,
                        scte27_source_settings: scte27_source_settings,
                        teletext_source_settings: teletext_source_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.CdiInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cdiinputspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CdiInputSpecification {
        /// Property [`Resolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-cdiinputspecification.html#cfn-medialive-channel-cdiinputspecification-resolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resolution: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CdiInputSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref resolution) = self.resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resolution", resolution)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CdiInputSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CdiInputSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CdiInputSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CdiInputSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resolution: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Resolution" => {
                                resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CdiInputSpecification {
                        resolution: resolution,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.ColorSpacePassthroughSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-colorspacepassthroughsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ColorSpacePassthroughSettings {
    }

    impl ::codec::SerializeValue for ColorSpacePassthroughSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColorSpacePassthroughSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColorSpacePassthroughSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColorSpacePassthroughSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColorSpacePassthroughSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ColorSpacePassthroughSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.DvbNitSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbnitsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DvbNitSettings {
        /// Property [`NetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbnitsettings.html#cfn-medialive-channel-dvbnitsettings-networkid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_id: Option<::Value<u32>>,
        /// Property [`NetworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbnitsettings.html#cfn-medialive-channel-dvbnitsettings-networkname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_name: Option<::Value<String>>,
        /// Property [`RepInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbnitsettings.html#cfn-medialive-channel-dvbnitsettings-repinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rep_interval: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DvbNitSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref network_id) = self.network_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkId", network_id)?;
            }
            if let Some(ref network_name) = self.network_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkName", network_name)?;
            }
            if let Some(ref rep_interval) = self.rep_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepInterval", rep_interval)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DvbNitSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DvbNitSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DvbNitSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DvbNitSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut network_id: Option<::Value<u32>> = None;
                    let mut network_name: Option<::Value<String>> = None;
                    let mut rep_interval: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetworkId" => {
                                network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkName" => {
                                network_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepInterval" => {
                                rep_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DvbNitSettings {
                        network_id: network_id,
                        network_name: network_name,
                        rep_interval: rep_interval,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.DvbSdtSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DvbSdtSettings {
        /// Property [`OutputSdt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html#cfn-medialive-channel-dvbsdtsettings-outputsdt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_sdt: Option<::Value<String>>,
        /// Property [`RepInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html#cfn-medialive-channel-dvbsdtsettings-repinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rep_interval: Option<::Value<u32>>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html#cfn-medialive-channel-dvbsdtsettings-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
        /// Property [`ServiceProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsdtsettings.html#cfn-medialive-channel-dvbsdtsettings-serviceprovidername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_provider_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DvbSdtSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref output_sdt) = self.output_sdt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSdt", output_sdt)?;
            }
            if let Some(ref rep_interval) = self.rep_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepInterval", rep_interval)?;
            }
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            if let Some(ref service_provider_name) = self.service_provider_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceProviderName", service_provider_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DvbSdtSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DvbSdtSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DvbSdtSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DvbSdtSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut output_sdt: Option<::Value<String>> = None;
                    let mut rep_interval: Option<::Value<u32>> = None;
                    let mut service_name: Option<::Value<String>> = None;
                    let mut service_provider_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OutputSdt" => {
                                output_sdt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepInterval" => {
                                rep_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceProviderName" => {
                                service_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DvbSdtSettings {
                        output_sdt: output_sdt,
                        rep_interval: rep_interval,
                        service_name: service_name,
                        service_provider_name: service_provider_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.DvbSubDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DvbSubDestinationSettings {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BackgroundOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-backgroundopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_opacity: Option<::Value<u32>>,
        /// Property [`Font`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-font).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font: Option<::Value<InputLocation>>,
        /// Property [`FontColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-fontcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_color: Option<::Value<String>>,
        /// Property [`FontOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-fontopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_opacity: Option<::Value<u32>>,
        /// Property [`FontResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-fontresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_resolution: Option<::Value<u32>>,
        /// Property [`FontSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-fontsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_size: Option<::Value<String>>,
        /// Property [`OutlineColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-outlinecolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outline_color: Option<::Value<String>>,
        /// Property [`OutlineSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-outlinesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outline_size: Option<::Value<u32>>,
        /// Property [`ShadowColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-shadowcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_color: Option<::Value<String>>,
        /// Property [`ShadowOpacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-shadowopacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_opacity: Option<::Value<u32>>,
        /// Property [`ShadowXOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-shadowxoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_x_offset: Option<::Value<u32>>,
        /// Property [`ShadowYOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-shadowyoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shadow_y_offset: Option<::Value<u32>>,
        /// Property [`TeletextGridControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-teletextgridcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub teletext_grid_control: Option<::Value<String>>,
        /// Property [`XPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-xposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub x_position: Option<::Value<u32>>,
        /// Property [`YPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubdestinationsettings.html#cfn-medialive-channel-dvbsubdestinationsettings-yposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub y_position: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DvbSubDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref background_opacity) = self.background_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundOpacity", background_opacity)?;
            }
            if let Some(ref font) = self.font {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Font", font)?;
            }
            if let Some(ref font_color) = self.font_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontColor", font_color)?;
            }
            if let Some(ref font_opacity) = self.font_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontOpacity", font_opacity)?;
            }
            if let Some(ref font_resolution) = self.font_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontResolution", font_resolution)?;
            }
            if let Some(ref font_size) = self.font_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontSize", font_size)?;
            }
            if let Some(ref outline_color) = self.outline_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutlineColor", outline_color)?;
            }
            if let Some(ref outline_size) = self.outline_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutlineSize", outline_size)?;
            }
            if let Some(ref shadow_color) = self.shadow_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowColor", shadow_color)?;
            }
            if let Some(ref shadow_opacity) = self.shadow_opacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowOpacity", shadow_opacity)?;
            }
            if let Some(ref shadow_x_offset) = self.shadow_x_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowXOffset", shadow_x_offset)?;
            }
            if let Some(ref shadow_y_offset) = self.shadow_y_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShadowYOffset", shadow_y_offset)?;
            }
            if let Some(ref teletext_grid_control) = self.teletext_grid_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeletextGridControl", teletext_grid_control)?;
            }
            if let Some(ref x_position) = self.x_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "XPosition", x_position)?;
            }
            if let Some(ref y_position) = self.y_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "YPosition", y_position)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DvbSubDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DvbSubDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DvbSubDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DvbSubDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut background_color: Option<::Value<String>> = None;
                    let mut background_opacity: Option<::Value<u32>> = None;
                    let mut font: Option<::Value<InputLocation>> = None;
                    let mut font_color: Option<::Value<String>> = None;
                    let mut font_opacity: Option<::Value<u32>> = None;
                    let mut font_resolution: Option<::Value<u32>> = None;
                    let mut font_size: Option<::Value<String>> = None;
                    let mut outline_color: Option<::Value<String>> = None;
                    let mut outline_size: Option<::Value<u32>> = None;
                    let mut shadow_color: Option<::Value<String>> = None;
                    let mut shadow_opacity: Option<::Value<u32>> = None;
                    let mut shadow_x_offset: Option<::Value<u32>> = None;
                    let mut shadow_y_offset: Option<::Value<u32>> = None;
                    let mut teletext_grid_control: Option<::Value<String>> = None;
                    let mut x_position: Option<::Value<u32>> = None;
                    let mut y_position: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BackgroundOpacity" => {
                                background_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Font" => {
                                font = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontColor" => {
                                font_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontOpacity" => {
                                font_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontResolution" => {
                                font_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontSize" => {
                                font_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutlineColor" => {
                                outline_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutlineSize" => {
                                outline_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowColor" => {
                                shadow_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowOpacity" => {
                                shadow_opacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowXOffset" => {
                                shadow_x_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShadowYOffset" => {
                                shadow_y_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TeletextGridControl" => {
                                teletext_grid_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "XPosition" => {
                                x_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "YPosition" => {
                                y_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DvbSubDestinationSettings {
                        alignment: alignment,
                        background_color: background_color,
                        background_opacity: background_opacity,
                        font: font,
                        font_color: font_color,
                        font_opacity: font_opacity,
                        font_resolution: font_resolution,
                        font_size: font_size,
                        outline_color: outline_color,
                        outline_size: outline_size,
                        shadow_color: shadow_color,
                        shadow_opacity: shadow_opacity,
                        shadow_x_offset: shadow_x_offset,
                        shadow_y_offset: shadow_y_offset,
                        teletext_grid_control: teletext_grid_control,
                        x_position: x_position,
                        y_position: y_position,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.DvbSubSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubsourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DvbSubSourceSettings {
        /// Property [`Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbsubsourcesettings.html#cfn-medialive-channel-dvbsubsourcesettings-pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DvbSubSourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pid) = self.pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pid", pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DvbSubSourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DvbSubSourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DvbSubSourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DvbSubSourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pid" => {
                                pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DvbSubSourceSettings {
                        pid: pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.DvbTdtSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbtdtsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DvbTdtSettings {
        /// Property [`RepInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-dvbtdtsettings.html#cfn-medialive-channel-dvbtdtsettings-repinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rep_interval: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DvbTdtSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rep_interval) = self.rep_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepInterval", rep_interval)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DvbTdtSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DvbTdtSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DvbTdtSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DvbTdtSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rep_interval: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RepInterval" => {
                                rep_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DvbTdtSettings {
                        rep_interval: rep_interval,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Eac3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Eac3Settings {
        /// Property [`AttenuationControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-attenuationcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attenuation_control: Option<::Value<String>>,
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<f64>>,
        /// Property [`BitstreamMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-bitstreammode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitstream_mode: Option<::Value<String>>,
        /// Property [`CodingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-codingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coding_mode: Option<::Value<String>>,
        /// Property [`DcFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-dcfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dc_filter: Option<::Value<String>>,
        /// Property [`Dialnorm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-dialnorm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dialnorm: Option<::Value<u32>>,
        /// Property [`DrcLine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-drcline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub drc_line: Option<::Value<String>>,
        /// Property [`DrcRf`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-drcrf).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub drc_rf: Option<::Value<String>>,
        /// Property [`LfeControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-lfecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lfe_control: Option<::Value<String>>,
        /// Property [`LfeFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-lfefilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lfe_filter: Option<::Value<String>>,
        /// Property [`LoRoCenterMixLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-lorocentermixlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lo_ro_center_mix_level: Option<::Value<f64>>,
        /// Property [`LoRoSurroundMixLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-lorosurroundmixlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lo_ro_surround_mix_level: Option<::Value<f64>>,
        /// Property [`LtRtCenterMixLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-ltrtcentermixlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lt_rt_center_mix_level: Option<::Value<f64>>,
        /// Property [`LtRtSurroundMixLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-ltrtsurroundmixlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lt_rt_surround_mix_level: Option<::Value<f64>>,
        /// Property [`MetadataControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-metadatacontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata_control: Option<::Value<String>>,
        /// Property [`PassthroughControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-passthroughcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub passthrough_control: Option<::Value<String>>,
        /// Property [`PhaseControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-phasecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phase_control: Option<::Value<String>>,
        /// Property [`StereoDownmix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-stereodownmix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stereo_downmix: Option<::Value<String>>,
        /// Property [`SurroundExMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-surroundexmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub surround_ex_mode: Option<::Value<String>>,
        /// Property [`SurroundMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-eac3settings.html#cfn-medialive-channel-eac3settings-surroundmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub surround_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Eac3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attenuation_control) = self.attenuation_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttenuationControl", attenuation_control)?;
            }
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref bitstream_mode) = self.bitstream_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BitstreamMode", bitstream_mode)?;
            }
            if let Some(ref coding_mode) = self.coding_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodingMode", coding_mode)?;
            }
            if let Some(ref dc_filter) = self.dc_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DcFilter", dc_filter)?;
            }
            if let Some(ref dialnorm) = self.dialnorm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dialnorm", dialnorm)?;
            }
            if let Some(ref drc_line) = self.drc_line {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrcLine", drc_line)?;
            }
            if let Some(ref drc_rf) = self.drc_rf {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrcRf", drc_rf)?;
            }
            if let Some(ref lfe_control) = self.lfe_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LfeControl", lfe_control)?;
            }
            if let Some(ref lfe_filter) = self.lfe_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LfeFilter", lfe_filter)?;
            }
            if let Some(ref lo_ro_center_mix_level) = self.lo_ro_center_mix_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRoCenterMixLevel", lo_ro_center_mix_level)?;
            }
            if let Some(ref lo_ro_surround_mix_level) = self.lo_ro_surround_mix_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRoSurroundMixLevel", lo_ro_surround_mix_level)?;
            }
            if let Some(ref lt_rt_center_mix_level) = self.lt_rt_center_mix_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LtRtCenterMixLevel", lt_rt_center_mix_level)?;
            }
            if let Some(ref lt_rt_surround_mix_level) = self.lt_rt_surround_mix_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LtRtSurroundMixLevel", lt_rt_surround_mix_level)?;
            }
            if let Some(ref metadata_control) = self.metadata_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataControl", metadata_control)?;
            }
            if let Some(ref passthrough_control) = self.passthrough_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassthroughControl", passthrough_control)?;
            }
            if let Some(ref phase_control) = self.phase_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhaseControl", phase_control)?;
            }
            if let Some(ref stereo_downmix) = self.stereo_downmix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StereoDownmix", stereo_downmix)?;
            }
            if let Some(ref surround_ex_mode) = self.surround_ex_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SurroundExMode", surround_ex_mode)?;
            }
            if let Some(ref surround_mode) = self.surround_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SurroundMode", surround_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Eac3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Eac3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Eac3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Eac3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attenuation_control: Option<::Value<String>> = None;
                    let mut bitrate: Option<::Value<f64>> = None;
                    let mut bitstream_mode: Option<::Value<String>> = None;
                    let mut coding_mode: Option<::Value<String>> = None;
                    let mut dc_filter: Option<::Value<String>> = None;
                    let mut dialnorm: Option<::Value<u32>> = None;
                    let mut drc_line: Option<::Value<String>> = None;
                    let mut drc_rf: Option<::Value<String>> = None;
                    let mut lfe_control: Option<::Value<String>> = None;
                    let mut lfe_filter: Option<::Value<String>> = None;
                    let mut lo_ro_center_mix_level: Option<::Value<f64>> = None;
                    let mut lo_ro_surround_mix_level: Option<::Value<f64>> = None;
                    let mut lt_rt_center_mix_level: Option<::Value<f64>> = None;
                    let mut lt_rt_surround_mix_level: Option<::Value<f64>> = None;
                    let mut metadata_control: Option<::Value<String>> = None;
                    let mut passthrough_control: Option<::Value<String>> = None;
                    let mut phase_control: Option<::Value<String>> = None;
                    let mut stereo_downmix: Option<::Value<String>> = None;
                    let mut surround_ex_mode: Option<::Value<String>> = None;
                    let mut surround_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttenuationControl" => {
                                attenuation_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BitstreamMode" => {
                                bitstream_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodingMode" => {
                                coding_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DcFilter" => {
                                dc_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dialnorm" => {
                                dialnorm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DrcLine" => {
                                drc_line = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DrcRf" => {
                                drc_rf = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LfeControl" => {
                                lfe_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LfeFilter" => {
                                lfe_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoRoCenterMixLevel" => {
                                lo_ro_center_mix_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoRoSurroundMixLevel" => {
                                lo_ro_surround_mix_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LtRtCenterMixLevel" => {
                                lt_rt_center_mix_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LtRtSurroundMixLevel" => {
                                lt_rt_surround_mix_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetadataControl" => {
                                metadata_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PassthroughControl" => {
                                passthrough_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhaseControl" => {
                                phase_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StereoDownmix" => {
                                stereo_downmix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SurroundExMode" => {
                                surround_ex_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SurroundMode" => {
                                surround_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Eac3Settings {
                        attenuation_control: attenuation_control,
                        bitrate: bitrate,
                        bitstream_mode: bitstream_mode,
                        coding_mode: coding_mode,
                        dc_filter: dc_filter,
                        dialnorm: dialnorm,
                        drc_line: drc_line,
                        drc_rf: drc_rf,
                        lfe_control: lfe_control,
                        lfe_filter: lfe_filter,
                        lo_ro_center_mix_level: lo_ro_center_mix_level,
                        lo_ro_surround_mix_level: lo_ro_surround_mix_level,
                        lt_rt_center_mix_level: lt_rt_center_mix_level,
                        lt_rt_surround_mix_level: lt_rt_surround_mix_level,
                        metadata_control: metadata_control,
                        passthrough_control: passthrough_control,
                        phase_control: phase_control,
                        stereo_downmix: stereo_downmix,
                        surround_ex_mode: surround_ex_mode,
                        surround_mode: surround_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.EbuTtDDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct EbuTtDDestinationSettings {
        /// Property [`CopyrightHolder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html#cfn-medialive-channel-ebuttddestinationsettings-copyrightholder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copyright_holder: Option<::Value<String>>,
        /// Property [`FillLineGap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html#cfn-medialive-channel-ebuttddestinationsettings-filllinegap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fill_line_gap: Option<::Value<String>>,
        /// Property [`FontFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html#cfn-medialive-channel-ebuttddestinationsettings-fontfamily).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_family: Option<::Value<String>>,
        /// Property [`StyleControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ebuttddestinationsettings.html#cfn-medialive-channel-ebuttddestinationsettings-stylecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub style_control: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbuTtDDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copyright_holder) = self.copyright_holder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyrightHolder", copyright_holder)?;
            }
            if let Some(ref fill_line_gap) = self.fill_line_gap {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FillLineGap", fill_line_gap)?;
            }
            if let Some(ref font_family) = self.font_family {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontFamily", font_family)?;
            }
            if let Some(ref style_control) = self.style_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StyleControl", style_control)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbuTtDDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbuTtDDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbuTtDDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbuTtDDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copyright_holder: Option<::Value<String>> = None;
                    let mut fill_line_gap: Option<::Value<String>> = None;
                    let mut font_family: Option<::Value<String>> = None;
                    let mut style_control: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyrightHolder" => {
                                copyright_holder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FillLineGap" => {
                                fill_line_gap = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FontFamily" => {
                                font_family = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StyleControl" => {
                                style_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EbuTtDDestinationSettings {
                        copyright_holder: copyright_holder,
                        fill_line_gap: fill_line_gap,
                        font_family: font_family,
                        style_control: style_control,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.EmbeddedDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddeddestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct EmbeddedDestinationSettings {
    }

    impl ::codec::SerializeValue for EmbeddedDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EmbeddedDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EmbeddedDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EmbeddedDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EmbeddedDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(EmbeddedDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.EmbeddedPlusScte20DestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedplusscte20destinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct EmbeddedPlusScte20DestinationSettings {
    }

    impl ::codec::SerializeValue for EmbeddedPlusScte20DestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EmbeddedPlusScte20DestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EmbeddedPlusScte20DestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EmbeddedPlusScte20DestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EmbeddedPlusScte20DestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(EmbeddedPlusScte20DestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.EmbeddedSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct EmbeddedSourceSettings {
        /// Property [`Convert608To708`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html#cfn-medialive-channel-embeddedsourcesettings-convert608to708).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub convert608_to708: Option<::Value<String>>,
        /// Property [`Scte20Detection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html#cfn-medialive-channel-embeddedsourcesettings-scte20detection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte20_detection: Option<::Value<String>>,
        /// Property [`Source608ChannelNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html#cfn-medialive-channel-embeddedsourcesettings-source608channelnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source608_channel_number: Option<::Value<u32>>,
        /// Property [`Source608TrackNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-embeddedsourcesettings.html#cfn-medialive-channel-embeddedsourcesettings-source608tracknumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source608_track_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EmbeddedSourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref convert608_to708) = self.convert608_to708 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Convert608To708", convert608_to708)?;
            }
            if let Some(ref scte20_detection) = self.scte20_detection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte20Detection", scte20_detection)?;
            }
            if let Some(ref source608_channel_number) = self.source608_channel_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source608ChannelNumber", source608_channel_number)?;
            }
            if let Some(ref source608_track_number) = self.source608_track_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source608TrackNumber", source608_track_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EmbeddedSourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EmbeddedSourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EmbeddedSourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EmbeddedSourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut convert608_to708: Option<::Value<String>> = None;
                    let mut scte20_detection: Option<::Value<String>> = None;
                    let mut source608_channel_number: Option<::Value<u32>> = None;
                    let mut source608_track_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Convert608To708" => {
                                convert608_to708 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte20Detection" => {
                                scte20_detection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source608ChannelNumber" => {
                                source608_channel_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source608TrackNumber" => {
                                source608_track_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EmbeddedSourceSettings {
                        convert608_to708: convert608_to708,
                        scte20_detection: scte20_detection,
                        source608_channel_number: source608_channel_number,
                        source608_track_number: source608_track_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.EncoderSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct EncoderSettings {
        /// Property [`AudioDescriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-audiodescriptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_descriptions: Option<::ValueList<AudioDescription>>,
        /// Property [`AvailBlanking`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-availblanking).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub avail_blanking: Option<::Value<AvailBlanking>>,
        /// Property [`AvailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-availconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub avail_configuration: Option<::Value<AvailConfiguration>>,
        /// Property [`BlackoutSlate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-blackoutslate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blackout_slate: Option<::Value<BlackoutSlate>>,
        /// Property [`CaptionDescriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-captiondescriptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_descriptions: Option<::ValueList<CaptionDescription>>,
        /// Property [`FeatureActivations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-featureactivations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature_activations: Option<::Value<FeatureActivations>>,
        /// Property [`GlobalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-globalconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub global_configuration: Option<::Value<GlobalConfiguration>>,
        /// Property [`MotionGraphicsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-motiongraphicsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub motion_graphics_configuration: Option<::Value<MotionGraphicsConfiguration>>,
        /// Property [`NielsenConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-nielsenconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nielsen_configuration: Option<::Value<NielsenConfiguration>>,
        /// Property [`OutputGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-outputgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_groups: Option<::ValueList<OutputGroup>>,
        /// Property [`TimecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-timecodeconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timecode_config: Option<::Value<TimecodeConfig>>,
        /// Property [`VideoDescriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-encodersettings.html#cfn-medialive-channel-encodersettings-videodescriptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_descriptions: Option<::ValueList<VideoDescription>>,
    }

    impl ::codec::SerializeValue for EncoderSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_descriptions) = self.audio_descriptions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioDescriptions", audio_descriptions)?;
            }
            if let Some(ref avail_blanking) = self.avail_blanking {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailBlanking", avail_blanking)?;
            }
            if let Some(ref avail_configuration) = self.avail_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailConfiguration", avail_configuration)?;
            }
            if let Some(ref blackout_slate) = self.blackout_slate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlackoutSlate", blackout_slate)?;
            }
            if let Some(ref caption_descriptions) = self.caption_descriptions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionDescriptions", caption_descriptions)?;
            }
            if let Some(ref feature_activations) = self.feature_activations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureActivations", feature_activations)?;
            }
            if let Some(ref global_configuration) = self.global_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalConfiguration", global_configuration)?;
            }
            if let Some(ref motion_graphics_configuration) = self.motion_graphics_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MotionGraphicsConfiguration", motion_graphics_configuration)?;
            }
            if let Some(ref nielsen_configuration) = self.nielsen_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NielsenConfiguration", nielsen_configuration)?;
            }
            if let Some(ref output_groups) = self.output_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputGroups", output_groups)?;
            }
            if let Some(ref timecode_config) = self.timecode_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimecodeConfig", timecode_config)?;
            }
            if let Some(ref video_descriptions) = self.video_descriptions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoDescriptions", video_descriptions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncoderSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncoderSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncoderSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncoderSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_descriptions: Option<::ValueList<AudioDescription>> = None;
                    let mut avail_blanking: Option<::Value<AvailBlanking>> = None;
                    let mut avail_configuration: Option<::Value<AvailConfiguration>> = None;
                    let mut blackout_slate: Option<::Value<BlackoutSlate>> = None;
                    let mut caption_descriptions: Option<::ValueList<CaptionDescription>> = None;
                    let mut feature_activations: Option<::Value<FeatureActivations>> = None;
                    let mut global_configuration: Option<::Value<GlobalConfiguration>> = None;
                    let mut motion_graphics_configuration: Option<::Value<MotionGraphicsConfiguration>> = None;
                    let mut nielsen_configuration: Option<::Value<NielsenConfiguration>> = None;
                    let mut output_groups: Option<::ValueList<OutputGroup>> = None;
                    let mut timecode_config: Option<::Value<TimecodeConfig>> = None;
                    let mut video_descriptions: Option<::ValueList<VideoDescription>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioDescriptions" => {
                                audio_descriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AvailBlanking" => {
                                avail_blanking = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AvailConfiguration" => {
                                avail_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlackoutSlate" => {
                                blackout_slate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionDescriptions" => {
                                caption_descriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureActivations" => {
                                feature_activations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GlobalConfiguration" => {
                                global_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MotionGraphicsConfiguration" => {
                                motion_graphics_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NielsenConfiguration" => {
                                nielsen_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputGroups" => {
                                output_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimecodeConfig" => {
                                timecode_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoDescriptions" => {
                                video_descriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncoderSettings {
                        audio_descriptions: audio_descriptions,
                        avail_blanking: avail_blanking,
                        avail_configuration: avail_configuration,
                        blackout_slate: blackout_slate,
                        caption_descriptions: caption_descriptions,
                        feature_activations: feature_activations,
                        global_configuration: global_configuration,
                        motion_graphics_configuration: motion_graphics_configuration,
                        nielsen_configuration: nielsen_configuration,
                        output_groups: output_groups,
                        timecode_config: timecode_config,
                        video_descriptions: video_descriptions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FailoverCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failovercondition.html) property type.
    #[derive(Debug, Default)]
    pub struct FailoverCondition {
        /// Property [`FailoverConditionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failovercondition.html#cfn-medialive-channel-failovercondition-failoverconditionsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_condition_settings: Option<::Value<FailoverConditionSettings>>,
    }

    impl ::codec::SerializeValue for FailoverCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failover_condition_settings) = self.failover_condition_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverConditionSettings", failover_condition_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FailoverCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FailoverCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FailoverCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FailoverCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failover_condition_settings: Option<::Value<FailoverConditionSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailoverConditionSettings" => {
                                failover_condition_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailoverCondition {
                        failover_condition_settings: failover_condition_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FailoverConditionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failoverconditionsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FailoverConditionSettings {
        /// Property [`AudioSilenceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failoverconditionsettings.html#cfn-medialive-channel-failoverconditionsettings-audiosilencesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_silence_settings: Option<::Value<AudioSilenceFailoverSettings>>,
        /// Property [`InputLossSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failoverconditionsettings.html#cfn-medialive-channel-failoverconditionsettings-inputlosssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_settings: Option<::Value<InputLossFailoverSettings>>,
        /// Property [`VideoBlackSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-failoverconditionsettings.html#cfn-medialive-channel-failoverconditionsettings-videoblacksettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_black_settings: Option<::Value<VideoBlackFailoverSettings>>,
    }

    impl ::codec::SerializeValue for FailoverConditionSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_silence_settings) = self.audio_silence_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSilenceSettings", audio_silence_settings)?;
            }
            if let Some(ref input_loss_settings) = self.input_loss_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossSettings", input_loss_settings)?;
            }
            if let Some(ref video_black_settings) = self.video_black_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoBlackSettings", video_black_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FailoverConditionSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FailoverConditionSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FailoverConditionSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FailoverConditionSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_silence_settings: Option<::Value<AudioSilenceFailoverSettings>> = None;
                    let mut input_loss_settings: Option<::Value<InputLossFailoverSettings>> = None;
                    let mut video_black_settings: Option<::Value<VideoBlackFailoverSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioSilenceSettings" => {
                                audio_silence_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossSettings" => {
                                input_loss_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoBlackSettings" => {
                                video_black_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailoverConditionSettings {
                        audio_silence_settings: audio_silence_settings,
                        input_loss_settings: input_loss_settings,
                        video_black_settings: video_black_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FeatureActivations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-featureactivations.html) property type.
    #[derive(Debug, Default)]
    pub struct FeatureActivations {
        /// Property [`InputPrepareScheduleActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-featureactivations.html#cfn-medialive-channel-featureactivations-inputpreparescheduleactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_prepare_schedule_actions: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FeatureActivations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_prepare_schedule_actions) = self.input_prepare_schedule_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputPrepareScheduleActions", input_prepare_schedule_actions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FeatureActivations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FeatureActivations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FeatureActivations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FeatureActivations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_prepare_schedule_actions: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputPrepareScheduleActions" => {
                                input_prepare_schedule_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FeatureActivations {
                        input_prepare_schedule_actions: input_prepare_schedule_actions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FecOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fecoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FecOutputSettings {
        /// Property [`ColumnDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fecoutputsettings.html#cfn-medialive-channel-fecoutputsettings-columndepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_depth: Option<::Value<u32>>,
        /// Property [`IncludeFec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fecoutputsettings.html#cfn-medialive-channel-fecoutputsettings-includefec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_fec: Option<::Value<String>>,
        /// Property [`RowLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fecoutputsettings.html#cfn-medialive-channel-fecoutputsettings-rowlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub row_length: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for FecOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref column_depth) = self.column_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnDepth", column_depth)?;
            }
            if let Some(ref include_fec) = self.include_fec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeFec", include_fec)?;
            }
            if let Some(ref row_length) = self.row_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowLength", row_length)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FecOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FecOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FecOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FecOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_depth: Option<::Value<u32>> = None;
                    let mut include_fec: Option<::Value<String>> = None;
                    let mut row_length: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnDepth" => {
                                column_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeFec" => {
                                include_fec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RowLength" => {
                                row_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FecOutputSettings {
                        column_depth: column_depth,
                        include_fec: include_fec,
                        row_length: row_length,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Fmp4HlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fmp4hlssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Fmp4HlsSettings {
        /// Property [`AudioRenditionSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fmp4hlssettings.html#cfn-medialive-channel-fmp4hlssettings-audiorenditionsets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_rendition_sets: Option<::Value<String>>,
        /// Property [`NielsenId3Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fmp4hlssettings.html#cfn-medialive-channel-fmp4hlssettings-nielsenid3behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nielsen_id3_behavior: Option<::Value<String>>,
        /// Property [`TimedMetadataBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-fmp4hlssettings.html#cfn-medialive-channel-fmp4hlssettings-timedmetadatabehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_behavior: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Fmp4HlsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_rendition_sets) = self.audio_rendition_sets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioRenditionSets", audio_rendition_sets)?;
            }
            if let Some(ref nielsen_id3_behavior) = self.nielsen_id3_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NielsenId3Behavior", nielsen_id3_behavior)?;
            }
            if let Some(ref timed_metadata_behavior) = self.timed_metadata_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataBehavior", timed_metadata_behavior)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Fmp4HlsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Fmp4HlsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Fmp4HlsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Fmp4HlsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_rendition_sets: Option<::Value<String>> = None;
                    let mut nielsen_id3_behavior: Option<::Value<String>> = None;
                    let mut timed_metadata_behavior: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioRenditionSets" => {
                                audio_rendition_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NielsenId3Behavior" => {
                                nielsen_id3_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataBehavior" => {
                                timed_metadata_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Fmp4HlsSettings {
                        audio_rendition_sets: audio_rendition_sets,
                        nielsen_id3_behavior: nielsen_id3_behavior,
                        timed_metadata_behavior: timed_metadata_behavior,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturecdnsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureCdnSettings {
        /// Property [`FrameCaptureS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturecdnsettings.html#cfn-medialive-channel-framecapturecdnsettings-framecaptures3settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_s3_settings: Option<::Value<FrameCaptureS3Settings>>,
    }

    impl ::codec::SerializeValue for FrameCaptureCdnSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref frame_capture_s3_settings) = self.frame_capture_s3_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureS3Settings", frame_capture_s3_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureCdnSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureCdnSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureCdnSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureCdnSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut frame_capture_s3_settings: Option<::Value<FrameCaptureS3Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FrameCaptureS3Settings" => {
                                frame_capture_s3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameCaptureCdnSettings {
                        frame_capture_s3_settings: frame_capture_s3_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturegroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureGroupSettings {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturegroupsettings.html#cfn-medialive-channel-framecapturegroupsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`FrameCaptureCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturegroupsettings.html#cfn-medialive-channel-framecapturegroupsettings-framecapturecdnsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_cdn_settings: Option<::Value<FrameCaptureCdnSettings>>,
    }

    impl ::codec::SerializeValue for FrameCaptureGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref frame_capture_cdn_settings) = self.frame_capture_cdn_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureCdnSettings", frame_capture_cdn_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut frame_capture_cdn_settings: Option<::Value<FrameCaptureCdnSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameCaptureCdnSettings" => {
                                frame_capture_cdn_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameCaptureGroupSettings {
                        destination: destination,
                        frame_capture_cdn_settings: frame_capture_cdn_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturehlssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureHlsSettings {
    }

    impl ::codec::SerializeValue for FrameCaptureHlsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureHlsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureHlsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureHlsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureHlsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(FrameCaptureHlsSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptureoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureOutputSettings {
        /// Property [`NameModifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptureoutputsettings.html#cfn-medialive-channel-framecaptureoutputsettings-namemodifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name_modifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FrameCaptureOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name_modifier) = self.name_modifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameModifier", name_modifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name_modifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NameModifier" => {
                                name_modifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameCaptureOutputSettings {
                        name_modifier: name_modifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptures3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureS3Settings {
        /// Property [`CannedAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecaptures3settings.html#cfn-medialive-channel-framecaptures3settings-cannedacl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canned_acl: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FrameCaptureS3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref canned_acl) = self.canned_acl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CannedAcl", canned_acl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureS3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureS3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureS3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureS3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut canned_acl: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CannedAcl" => {
                                canned_acl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameCaptureS3Settings {
                        canned_acl: canned_acl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.FrameCaptureSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct FrameCaptureSettings {
        /// Property [`CaptureInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturesettings.html#cfn-medialive-channel-framecapturesettings-captureinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_interval: Option<::Value<u32>>,
        /// Property [`CaptureIntervalUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-framecapturesettings.html#cfn-medialive-channel-framecapturesettings-captureintervalunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_interval_units: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FrameCaptureSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capture_interval) = self.capture_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureInterval", capture_interval)?;
            }
            if let Some(ref capture_interval_units) = self.capture_interval_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureIntervalUnits", capture_interval_units)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrameCaptureSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrameCaptureSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrameCaptureSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrameCaptureSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capture_interval: Option<::Value<u32>> = None;
                    let mut capture_interval_units: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptureInterval" => {
                                capture_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptureIntervalUnits" => {
                                capture_interval_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrameCaptureSettings {
                        capture_interval: capture_interval,
                        capture_interval_units: capture_interval_units,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.GlobalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct GlobalConfiguration {
        /// Property [`InitialAudioGain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-initialaudiogain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub initial_audio_gain: Option<::Value<u32>>,
        /// Property [`InputEndAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-inputendaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_end_action: Option<::Value<String>>,
        /// Property [`InputLossBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-inputlossbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_behavior: Option<::Value<InputLossBehavior>>,
        /// Property [`OutputLockingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-outputlockingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_locking_mode: Option<::Value<String>>,
        /// Property [`OutputTimingSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-outputtimingsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_timing_source: Option<::Value<String>>,
        /// Property [`SupportLowFramerateInputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-globalconfiguration.html#cfn-medialive-channel-globalconfiguration-supportlowframerateinputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub support_low_framerate_inputs: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GlobalConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref initial_audio_gain) = self.initial_audio_gain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialAudioGain", initial_audio_gain)?;
            }
            if let Some(ref input_end_action) = self.input_end_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputEndAction", input_end_action)?;
            }
            if let Some(ref input_loss_behavior) = self.input_loss_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossBehavior", input_loss_behavior)?;
            }
            if let Some(ref output_locking_mode) = self.output_locking_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputLockingMode", output_locking_mode)?;
            }
            if let Some(ref output_timing_source) = self.output_timing_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputTimingSource", output_timing_source)?;
            }
            if let Some(ref support_low_framerate_inputs) = self.support_low_framerate_inputs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportLowFramerateInputs", support_low_framerate_inputs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlobalConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlobalConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlobalConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut initial_audio_gain: Option<::Value<u32>> = None;
                    let mut input_end_action: Option<::Value<String>> = None;
                    let mut input_loss_behavior: Option<::Value<InputLossBehavior>> = None;
                    let mut output_locking_mode: Option<::Value<String>> = None;
                    let mut output_timing_source: Option<::Value<String>> = None;
                    let mut support_low_framerate_inputs: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InitialAudioGain" => {
                                initial_audio_gain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputEndAction" => {
                                input_end_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossBehavior" => {
                                input_loss_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputLockingMode" => {
                                output_locking_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputTimingSource" => {
                                output_timing_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportLowFramerateInputs" => {
                                support_low_framerate_inputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlobalConfiguration {
                        initial_audio_gain: initial_audio_gain,
                        input_end_action: input_end_action,
                        input_loss_behavior: input_loss_behavior,
                        output_locking_mode: output_locking_mode,
                        output_timing_source: output_timing_source,
                        support_low_framerate_inputs: support_low_framerate_inputs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H264ColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264colorspacesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct H264ColorSpaceSettings {
        /// Property [`ColorSpacePassthroughSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264colorspacesettings.html#cfn-medialive-channel-h264colorspacesettings-colorspacepassthroughsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_passthrough_settings: Option<::Value<ColorSpacePassthroughSettings>>,
        /// Property [`Rec601Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264colorspacesettings.html#cfn-medialive-channel-h264colorspacesettings-rec601settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rec601_settings: Option<::Value<Rec601Settings>>,
        /// Property [`Rec709Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264colorspacesettings.html#cfn-medialive-channel-h264colorspacesettings-rec709settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rec709_settings: Option<::Value<Rec709Settings>>,
    }

    impl ::codec::SerializeValue for H264ColorSpaceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref color_space_passthrough_settings) = self.color_space_passthrough_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpacePassthroughSettings", color_space_passthrough_settings)?;
            }
            if let Some(ref rec601_settings) = self.rec601_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rec601Settings", rec601_settings)?;
            }
            if let Some(ref rec709_settings) = self.rec709_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rec709Settings", rec709_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H264ColorSpaceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H264ColorSpaceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H264ColorSpaceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H264ColorSpaceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut color_space_passthrough_settings: Option<::Value<ColorSpacePassthroughSettings>> = None;
                    let mut rec601_settings: Option<::Value<Rec601Settings>> = None;
                    let mut rec709_settings: Option<::Value<Rec709Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColorSpacePassthroughSettings" => {
                                color_space_passthrough_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rec601Settings" => {
                                rec601_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rec709Settings" => {
                                rec709_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H264ColorSpaceSettings {
                        color_space_passthrough_settings: color_space_passthrough_settings,
                        rec601_settings: rec601_settings,
                        rec709_settings: rec709_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H264FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264filtersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct H264FilterSettings {
        /// Property [`TemporalFilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264filtersettings.html#cfn-medialive-channel-h264filtersettings-temporalfiltersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temporal_filter_settings: Option<::Value<TemporalFilterSettings>>,
    }

    impl ::codec::SerializeValue for H264FilterSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref temporal_filter_settings) = self.temporal_filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemporalFilterSettings", temporal_filter_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H264FilterSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H264FilterSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H264FilterSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H264FilterSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut temporal_filter_settings: Option<::Value<TemporalFilterSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemporalFilterSettings" => {
                                temporal_filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H264FilterSettings {
                        temporal_filter_settings: temporal_filter_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H264Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html) property type.
    #[derive(Debug, Default)]
    pub struct H264Settings {
        /// Property [`AdaptiveQuantization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-adaptivequantization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adaptive_quantization: Option<::Value<String>>,
        /// Property [`AfdSignaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-afdsignaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub afd_signaling: Option<::Value<String>>,
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<u32>>,
        /// Property [`BufFillPct`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-buffillpct).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buf_fill_pct: Option<::Value<u32>>,
        /// Property [`BufSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-bufsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buf_size: Option<::Value<u32>>,
        /// Property [`ColorMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-colormetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_metadata: Option<::Value<String>>,
        /// Property [`ColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-colorspacesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_settings: Option<::Value<H264ColorSpaceSettings>>,
        /// Property [`EntropyEncoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-entropyencoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entropy_encoding: Option<::Value<String>>,
        /// Property [`FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-filtersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_settings: Option<::Value<H264FilterSettings>>,
        /// Property [`FixedAfd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-fixedafd).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_afd: Option<::Value<String>>,
        /// Property [`FlickerAq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-flickeraq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flicker_aq: Option<::Value<String>>,
        /// Property [`ForceFieldPictures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-forcefieldpictures).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub force_field_pictures: Option<::Value<String>>,
        /// Property [`FramerateControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-frameratecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_control: Option<::Value<String>>,
        /// Property [`FramerateDenominator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-frameratedenominator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_denominator: Option<::Value<u32>>,
        /// Property [`FramerateNumerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-frameratenumerator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_numerator: Option<::Value<u32>>,
        /// Property [`GopBReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-gopbreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_b_reference: Option<::Value<String>>,
        /// Property [`GopClosedCadence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-gopclosedcadence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_closed_cadence: Option<::Value<u32>>,
        /// Property [`GopNumBFrames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-gopnumbframes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_num_b_frames: Option<::Value<u32>>,
        /// Property [`GopSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-gopsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size: Option<::Value<f64>>,
        /// Property [`GopSizeUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-gopsizeunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size_units: Option<::Value<String>>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-level).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub level: Option<::Value<String>>,
        /// Property [`LookAheadRateControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-lookaheadratecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub look_ahead_rate_control: Option<::Value<String>>,
        /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-maxbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_bitrate: Option<::Value<u32>>,
        /// Property [`MinIInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-miniinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_i_interval: Option<::Value<u32>>,
        /// Property [`NumRefFrames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-numrefframes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_ref_frames: Option<::Value<u32>>,
        /// Property [`ParControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-parcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub par_control: Option<::Value<String>>,
        /// Property [`ParDenominator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-pardenominator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub par_denominator: Option<::Value<u32>>,
        /// Property [`ParNumerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-parnumerator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub par_numerator: Option<::Value<u32>>,
        /// Property [`Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-profile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile: Option<::Value<String>>,
        /// Property [`QualityLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-qualitylevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quality_level: Option<::Value<String>>,
        /// Property [`QvbrQualityLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-qvbrqualitylevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub qvbr_quality_level: Option<::Value<u32>>,
        /// Property [`RateControlMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-ratecontrolmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_control_mode: Option<::Value<String>>,
        /// Property [`ScanType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-scantype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scan_type: Option<::Value<String>>,
        /// Property [`SceneChangeDetect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-scenechangedetect).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scene_change_detect: Option<::Value<String>>,
        /// Property [`Slices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-slices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slices: Option<::Value<u32>>,
        /// Property [`Softness`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-softness).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub softness: Option<::Value<u32>>,
        /// Property [`SpatialAq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-spatialaq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spatial_aq: Option<::Value<String>>,
        /// Property [`SubgopLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-subgoplength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subgop_length: Option<::Value<String>>,
        /// Property [`Syntax`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-syntax).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub syntax: Option<::Value<String>>,
        /// Property [`TemporalAq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-temporalaq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temporal_aq: Option<::Value<String>>,
        /// Property [`TimecodeInsertion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h264settings.html#cfn-medialive-channel-h264settings-timecodeinsertion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timecode_insertion: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for H264Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adaptive_quantization) = self.adaptive_quantization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdaptiveQuantization", adaptive_quantization)?;
            }
            if let Some(ref afd_signaling) = self.afd_signaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfdSignaling", afd_signaling)?;
            }
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref buf_fill_pct) = self.buf_fill_pct {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufFillPct", buf_fill_pct)?;
            }
            if let Some(ref buf_size) = self.buf_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufSize", buf_size)?;
            }
            if let Some(ref color_metadata) = self.color_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorMetadata", color_metadata)?;
            }
            if let Some(ref color_space_settings) = self.color_space_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpaceSettings", color_space_settings)?;
            }
            if let Some(ref entropy_encoding) = self.entropy_encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntropyEncoding", entropy_encoding)?;
            }
            if let Some(ref filter_settings) = self.filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterSettings", filter_settings)?;
            }
            if let Some(ref fixed_afd) = self.fixed_afd {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedAfd", fixed_afd)?;
            }
            if let Some(ref flicker_aq) = self.flicker_aq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlickerAq", flicker_aq)?;
            }
            if let Some(ref force_field_pictures) = self.force_field_pictures {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceFieldPictures", force_field_pictures)?;
            }
            if let Some(ref framerate_control) = self.framerate_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateControl", framerate_control)?;
            }
            if let Some(ref framerate_denominator) = self.framerate_denominator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateDenominator", framerate_denominator)?;
            }
            if let Some(ref framerate_numerator) = self.framerate_numerator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateNumerator", framerate_numerator)?;
            }
            if let Some(ref gop_b_reference) = self.gop_b_reference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopBReference", gop_b_reference)?;
            }
            if let Some(ref gop_closed_cadence) = self.gop_closed_cadence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopClosedCadence", gop_closed_cadence)?;
            }
            if let Some(ref gop_num_b_frames) = self.gop_num_b_frames {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopNumBFrames", gop_num_b_frames)?;
            }
            if let Some(ref gop_size) = self.gop_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSize", gop_size)?;
            }
            if let Some(ref gop_size_units) = self.gop_size_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSizeUnits", gop_size_units)?;
            }
            if let Some(ref level) = self.level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", level)?;
            }
            if let Some(ref look_ahead_rate_control) = self.look_ahead_rate_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LookAheadRateControl", look_ahead_rate_control)?;
            }
            if let Some(ref max_bitrate) = self.max_bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", max_bitrate)?;
            }
            if let Some(ref min_i_interval) = self.min_i_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinIInterval", min_i_interval)?;
            }
            if let Some(ref num_ref_frames) = self.num_ref_frames {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRefFrames", num_ref_frames)?;
            }
            if let Some(ref par_control) = self.par_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParControl", par_control)?;
            }
            if let Some(ref par_denominator) = self.par_denominator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParDenominator", par_denominator)?;
            }
            if let Some(ref par_numerator) = self.par_numerator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParNumerator", par_numerator)?;
            }
            if let Some(ref profile) = self.profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Profile", profile)?;
            }
            if let Some(ref quality_level) = self.quality_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QualityLevel", quality_level)?;
            }
            if let Some(ref qvbr_quality_level) = self.qvbr_quality_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QvbrQualityLevel", qvbr_quality_level)?;
            }
            if let Some(ref rate_control_mode) = self.rate_control_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateControlMode", rate_control_mode)?;
            }
            if let Some(ref scan_type) = self.scan_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanType", scan_type)?;
            }
            if let Some(ref scene_change_detect) = self.scene_change_detect {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SceneChangeDetect", scene_change_detect)?;
            }
            if let Some(ref slices) = self.slices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slices", slices)?;
            }
            if let Some(ref softness) = self.softness {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Softness", softness)?;
            }
            if let Some(ref spatial_aq) = self.spatial_aq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpatialAq", spatial_aq)?;
            }
            if let Some(ref subgop_length) = self.subgop_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubgopLength", subgop_length)?;
            }
            if let Some(ref syntax) = self.syntax {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Syntax", syntax)?;
            }
            if let Some(ref temporal_aq) = self.temporal_aq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemporalAq", temporal_aq)?;
            }
            if let Some(ref timecode_insertion) = self.timecode_insertion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimecodeInsertion", timecode_insertion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H264Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H264Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H264Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H264Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adaptive_quantization: Option<::Value<String>> = None;
                    let mut afd_signaling: Option<::Value<String>> = None;
                    let mut bitrate: Option<::Value<u32>> = None;
                    let mut buf_fill_pct: Option<::Value<u32>> = None;
                    let mut buf_size: Option<::Value<u32>> = None;
                    let mut color_metadata: Option<::Value<String>> = None;
                    let mut color_space_settings: Option<::Value<H264ColorSpaceSettings>> = None;
                    let mut entropy_encoding: Option<::Value<String>> = None;
                    let mut filter_settings: Option<::Value<H264FilterSettings>> = None;
                    let mut fixed_afd: Option<::Value<String>> = None;
                    let mut flicker_aq: Option<::Value<String>> = None;
                    let mut force_field_pictures: Option<::Value<String>> = None;
                    let mut framerate_control: Option<::Value<String>> = None;
                    let mut framerate_denominator: Option<::Value<u32>> = None;
                    let mut framerate_numerator: Option<::Value<u32>> = None;
                    let mut gop_b_reference: Option<::Value<String>> = None;
                    let mut gop_closed_cadence: Option<::Value<u32>> = None;
                    let mut gop_num_b_frames: Option<::Value<u32>> = None;
                    let mut gop_size: Option<::Value<f64>> = None;
                    let mut gop_size_units: Option<::Value<String>> = None;
                    let mut level: Option<::Value<String>> = None;
                    let mut look_ahead_rate_control: Option<::Value<String>> = None;
                    let mut max_bitrate: Option<::Value<u32>> = None;
                    let mut min_i_interval: Option<::Value<u32>> = None;
                    let mut num_ref_frames: Option<::Value<u32>> = None;
                    let mut par_control: Option<::Value<String>> = None;
                    let mut par_denominator: Option<::Value<u32>> = None;
                    let mut par_numerator: Option<::Value<u32>> = None;
                    let mut profile: Option<::Value<String>> = None;
                    let mut quality_level: Option<::Value<String>> = None;
                    let mut qvbr_quality_level: Option<::Value<u32>> = None;
                    let mut rate_control_mode: Option<::Value<String>> = None;
                    let mut scan_type: Option<::Value<String>> = None;
                    let mut scene_change_detect: Option<::Value<String>> = None;
                    let mut slices: Option<::Value<u32>> = None;
                    let mut softness: Option<::Value<u32>> = None;
                    let mut spatial_aq: Option<::Value<String>> = None;
                    let mut subgop_length: Option<::Value<String>> = None;
                    let mut syntax: Option<::Value<String>> = None;
                    let mut temporal_aq: Option<::Value<String>> = None;
                    let mut timecode_insertion: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdaptiveQuantization" => {
                                adaptive_quantization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AfdSignaling" => {
                                afd_signaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufFillPct" => {
                                buf_fill_pct = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufSize" => {
                                buf_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorMetadata" => {
                                color_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorSpaceSettings" => {
                                color_space_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntropyEncoding" => {
                                entropy_encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterSettings" => {
                                filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedAfd" => {
                                fixed_afd = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlickerAq" => {
                                flicker_aq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForceFieldPictures" => {
                                force_field_pictures = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateControl" => {
                                framerate_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateDenominator" => {
                                framerate_denominator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateNumerator" => {
                                framerate_numerator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopBReference" => {
                                gop_b_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopClosedCadence" => {
                                gop_closed_cadence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopNumBFrames" => {
                                gop_num_b_frames = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSize" => {
                                gop_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSizeUnits" => {
                                gop_size_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LookAheadRateControl" => {
                                look_ahead_rate_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxBitrate" => {
                                max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinIInterval" => {
                                min_i_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRefFrames" => {
                                num_ref_frames = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParControl" => {
                                par_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParDenominator" => {
                                par_denominator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParNumerator" => {
                                par_numerator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Profile" => {
                                profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QualityLevel" => {
                                quality_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QvbrQualityLevel" => {
                                qvbr_quality_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateControlMode" => {
                                rate_control_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScanType" => {
                                scan_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SceneChangeDetect" => {
                                scene_change_detect = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slices" => {
                                slices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Softness" => {
                                softness = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpatialAq" => {
                                spatial_aq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubgopLength" => {
                                subgop_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Syntax" => {
                                syntax = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemporalAq" => {
                                temporal_aq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimecodeInsertion" => {
                                timecode_insertion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H264Settings {
                        adaptive_quantization: adaptive_quantization,
                        afd_signaling: afd_signaling,
                        bitrate: bitrate,
                        buf_fill_pct: buf_fill_pct,
                        buf_size: buf_size,
                        color_metadata: color_metadata,
                        color_space_settings: color_space_settings,
                        entropy_encoding: entropy_encoding,
                        filter_settings: filter_settings,
                        fixed_afd: fixed_afd,
                        flicker_aq: flicker_aq,
                        force_field_pictures: force_field_pictures,
                        framerate_control: framerate_control,
                        framerate_denominator: framerate_denominator,
                        framerate_numerator: framerate_numerator,
                        gop_b_reference: gop_b_reference,
                        gop_closed_cadence: gop_closed_cadence,
                        gop_num_b_frames: gop_num_b_frames,
                        gop_size: gop_size,
                        gop_size_units: gop_size_units,
                        level: level,
                        look_ahead_rate_control: look_ahead_rate_control,
                        max_bitrate: max_bitrate,
                        min_i_interval: min_i_interval,
                        num_ref_frames: num_ref_frames,
                        par_control: par_control,
                        par_denominator: par_denominator,
                        par_numerator: par_numerator,
                        profile: profile,
                        quality_level: quality_level,
                        qvbr_quality_level: qvbr_quality_level,
                        rate_control_mode: rate_control_mode,
                        scan_type: scan_type,
                        scene_change_detect: scene_change_detect,
                        slices: slices,
                        softness: softness,
                        spatial_aq: spatial_aq,
                        subgop_length: subgop_length,
                        syntax: syntax,
                        temporal_aq: temporal_aq,
                        timecode_insertion: timecode_insertion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H265ColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct H265ColorSpaceSettings {
        /// Property [`ColorSpacePassthroughSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html#cfn-medialive-channel-h265colorspacesettings-colorspacepassthroughsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_passthrough_settings: Option<::Value<ColorSpacePassthroughSettings>>,
        /// Property [`Hdr10Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html#cfn-medialive-channel-h265colorspacesettings-hdr10settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hdr10_settings: Option<::Value<Hdr10Settings>>,
        /// Property [`Rec601Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html#cfn-medialive-channel-h265colorspacesettings-rec601settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rec601_settings: Option<::Value<Rec601Settings>>,
        /// Property [`Rec709Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265colorspacesettings.html#cfn-medialive-channel-h265colorspacesettings-rec709settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rec709_settings: Option<::Value<Rec709Settings>>,
    }

    impl ::codec::SerializeValue for H265ColorSpaceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref color_space_passthrough_settings) = self.color_space_passthrough_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpacePassthroughSettings", color_space_passthrough_settings)?;
            }
            if let Some(ref hdr10_settings) = self.hdr10_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hdr10Settings", hdr10_settings)?;
            }
            if let Some(ref rec601_settings) = self.rec601_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rec601Settings", rec601_settings)?;
            }
            if let Some(ref rec709_settings) = self.rec709_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rec709Settings", rec709_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H265ColorSpaceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H265ColorSpaceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H265ColorSpaceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H265ColorSpaceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut color_space_passthrough_settings: Option<::Value<ColorSpacePassthroughSettings>> = None;
                    let mut hdr10_settings: Option<::Value<Hdr10Settings>> = None;
                    let mut rec601_settings: Option<::Value<Rec601Settings>> = None;
                    let mut rec709_settings: Option<::Value<Rec709Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColorSpacePassthroughSettings" => {
                                color_space_passthrough_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Hdr10Settings" => {
                                hdr10_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rec601Settings" => {
                                rec601_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rec709Settings" => {
                                rec709_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H265ColorSpaceSettings {
                        color_space_passthrough_settings: color_space_passthrough_settings,
                        hdr10_settings: hdr10_settings,
                        rec601_settings: rec601_settings,
                        rec709_settings: rec709_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H265FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265filtersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct H265FilterSettings {
        /// Property [`TemporalFilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265filtersettings.html#cfn-medialive-channel-h265filtersettings-temporalfiltersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temporal_filter_settings: Option<::Value<TemporalFilterSettings>>,
    }

    impl ::codec::SerializeValue for H265FilterSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref temporal_filter_settings) = self.temporal_filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemporalFilterSettings", temporal_filter_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H265FilterSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H265FilterSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H265FilterSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H265FilterSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut temporal_filter_settings: Option<::Value<TemporalFilterSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemporalFilterSettings" => {
                                temporal_filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H265FilterSettings {
                        temporal_filter_settings: temporal_filter_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.H265Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html) property type.
    #[derive(Debug, Default)]
    pub struct H265Settings {
        /// Property [`AdaptiveQuantization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-adaptivequantization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adaptive_quantization: Option<::Value<String>>,
        /// Property [`AfdSignaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-afdsignaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub afd_signaling: Option<::Value<String>>,
        /// Property [`AlternativeTransferFunction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-alternativetransferfunction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alternative_transfer_function: Option<::Value<String>>,
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<u32>>,
        /// Property [`BufSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-bufsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buf_size: Option<::Value<u32>>,
        /// Property [`ColorMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-colormetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_metadata: Option<::Value<String>>,
        /// Property [`ColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-colorspacesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_settings: Option<::Value<H265ColorSpaceSettings>>,
        /// Property [`FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-filtersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_settings: Option<::Value<H265FilterSettings>>,
        /// Property [`FixedAfd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-fixedafd).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_afd: Option<::Value<String>>,
        /// Property [`FlickerAq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-flickeraq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flicker_aq: Option<::Value<String>>,
        /// Property [`FramerateDenominator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-frameratedenominator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_denominator: Option<::Value<u32>>,
        /// Property [`FramerateNumerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-frameratenumerator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_numerator: Option<::Value<u32>>,
        /// Property [`GopClosedCadence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-gopclosedcadence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_closed_cadence: Option<::Value<u32>>,
        /// Property [`GopSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-gopsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size: Option<::Value<f64>>,
        /// Property [`GopSizeUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-gopsizeunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size_units: Option<::Value<String>>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-level).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub level: Option<::Value<String>>,
        /// Property [`LookAheadRateControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-lookaheadratecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub look_ahead_rate_control: Option<::Value<String>>,
        /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-maxbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_bitrate: Option<::Value<u32>>,
        /// Property [`MinIInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-miniinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_i_interval: Option<::Value<u32>>,
        /// Property [`ParDenominator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-pardenominator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub par_denominator: Option<::Value<u32>>,
        /// Property [`ParNumerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-parnumerator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub par_numerator: Option<::Value<u32>>,
        /// Property [`Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-profile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile: Option<::Value<String>>,
        /// Property [`QvbrQualityLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-qvbrqualitylevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub qvbr_quality_level: Option<::Value<u32>>,
        /// Property [`RateControlMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-ratecontrolmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_control_mode: Option<::Value<String>>,
        /// Property [`ScanType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-scantype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scan_type: Option<::Value<String>>,
        /// Property [`SceneChangeDetect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-scenechangedetect).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scene_change_detect: Option<::Value<String>>,
        /// Property [`Slices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-slices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slices: Option<::Value<u32>>,
        /// Property [`Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-tier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tier: Option<::Value<String>>,
        /// Property [`TimecodeInsertion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-h265settings.html#cfn-medialive-channel-h265settings-timecodeinsertion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timecode_insertion: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for H265Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adaptive_quantization) = self.adaptive_quantization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdaptiveQuantization", adaptive_quantization)?;
            }
            if let Some(ref afd_signaling) = self.afd_signaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfdSignaling", afd_signaling)?;
            }
            if let Some(ref alternative_transfer_function) = self.alternative_transfer_function {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlternativeTransferFunction", alternative_transfer_function)?;
            }
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref buf_size) = self.buf_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufSize", buf_size)?;
            }
            if let Some(ref color_metadata) = self.color_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorMetadata", color_metadata)?;
            }
            if let Some(ref color_space_settings) = self.color_space_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpaceSettings", color_space_settings)?;
            }
            if let Some(ref filter_settings) = self.filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterSettings", filter_settings)?;
            }
            if let Some(ref fixed_afd) = self.fixed_afd {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedAfd", fixed_afd)?;
            }
            if let Some(ref flicker_aq) = self.flicker_aq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlickerAq", flicker_aq)?;
            }
            if let Some(ref framerate_denominator) = self.framerate_denominator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateDenominator", framerate_denominator)?;
            }
            if let Some(ref framerate_numerator) = self.framerate_numerator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateNumerator", framerate_numerator)?;
            }
            if let Some(ref gop_closed_cadence) = self.gop_closed_cadence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopClosedCadence", gop_closed_cadence)?;
            }
            if let Some(ref gop_size) = self.gop_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSize", gop_size)?;
            }
            if let Some(ref gop_size_units) = self.gop_size_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSizeUnits", gop_size_units)?;
            }
            if let Some(ref level) = self.level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", level)?;
            }
            if let Some(ref look_ahead_rate_control) = self.look_ahead_rate_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LookAheadRateControl", look_ahead_rate_control)?;
            }
            if let Some(ref max_bitrate) = self.max_bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", max_bitrate)?;
            }
            if let Some(ref min_i_interval) = self.min_i_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinIInterval", min_i_interval)?;
            }
            if let Some(ref par_denominator) = self.par_denominator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParDenominator", par_denominator)?;
            }
            if let Some(ref par_numerator) = self.par_numerator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParNumerator", par_numerator)?;
            }
            if let Some(ref profile) = self.profile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Profile", profile)?;
            }
            if let Some(ref qvbr_quality_level) = self.qvbr_quality_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QvbrQualityLevel", qvbr_quality_level)?;
            }
            if let Some(ref rate_control_mode) = self.rate_control_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateControlMode", rate_control_mode)?;
            }
            if let Some(ref scan_type) = self.scan_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanType", scan_type)?;
            }
            if let Some(ref scene_change_detect) = self.scene_change_detect {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SceneChangeDetect", scene_change_detect)?;
            }
            if let Some(ref slices) = self.slices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slices", slices)?;
            }
            if let Some(ref tier) = self.tier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", tier)?;
            }
            if let Some(ref timecode_insertion) = self.timecode_insertion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimecodeInsertion", timecode_insertion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for H265Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<H265Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = H265Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type H265Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adaptive_quantization: Option<::Value<String>> = None;
                    let mut afd_signaling: Option<::Value<String>> = None;
                    let mut alternative_transfer_function: Option<::Value<String>> = None;
                    let mut bitrate: Option<::Value<u32>> = None;
                    let mut buf_size: Option<::Value<u32>> = None;
                    let mut color_metadata: Option<::Value<String>> = None;
                    let mut color_space_settings: Option<::Value<H265ColorSpaceSettings>> = None;
                    let mut filter_settings: Option<::Value<H265FilterSettings>> = None;
                    let mut fixed_afd: Option<::Value<String>> = None;
                    let mut flicker_aq: Option<::Value<String>> = None;
                    let mut framerate_denominator: Option<::Value<u32>> = None;
                    let mut framerate_numerator: Option<::Value<u32>> = None;
                    let mut gop_closed_cadence: Option<::Value<u32>> = None;
                    let mut gop_size: Option<::Value<f64>> = None;
                    let mut gop_size_units: Option<::Value<String>> = None;
                    let mut level: Option<::Value<String>> = None;
                    let mut look_ahead_rate_control: Option<::Value<String>> = None;
                    let mut max_bitrate: Option<::Value<u32>> = None;
                    let mut min_i_interval: Option<::Value<u32>> = None;
                    let mut par_denominator: Option<::Value<u32>> = None;
                    let mut par_numerator: Option<::Value<u32>> = None;
                    let mut profile: Option<::Value<String>> = None;
                    let mut qvbr_quality_level: Option<::Value<u32>> = None;
                    let mut rate_control_mode: Option<::Value<String>> = None;
                    let mut scan_type: Option<::Value<String>> = None;
                    let mut scene_change_detect: Option<::Value<String>> = None;
                    let mut slices: Option<::Value<u32>> = None;
                    let mut tier: Option<::Value<String>> = None;
                    let mut timecode_insertion: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdaptiveQuantization" => {
                                adaptive_quantization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AfdSignaling" => {
                                afd_signaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AlternativeTransferFunction" => {
                                alternative_transfer_function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufSize" => {
                                buf_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorMetadata" => {
                                color_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorSpaceSettings" => {
                                color_space_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterSettings" => {
                                filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedAfd" => {
                                fixed_afd = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlickerAq" => {
                                flicker_aq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateDenominator" => {
                                framerate_denominator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateNumerator" => {
                                framerate_numerator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopClosedCadence" => {
                                gop_closed_cadence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSize" => {
                                gop_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSizeUnits" => {
                                gop_size_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LookAheadRateControl" => {
                                look_ahead_rate_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxBitrate" => {
                                max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinIInterval" => {
                                min_i_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParDenominator" => {
                                par_denominator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParNumerator" => {
                                par_numerator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Profile" => {
                                profile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QvbrQualityLevel" => {
                                qvbr_quality_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateControlMode" => {
                                rate_control_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScanType" => {
                                scan_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SceneChangeDetect" => {
                                scene_change_detect = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slices" => {
                                slices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tier" => {
                                tier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimecodeInsertion" => {
                                timecode_insertion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(H265Settings {
                        adaptive_quantization: adaptive_quantization,
                        afd_signaling: afd_signaling,
                        alternative_transfer_function: alternative_transfer_function,
                        bitrate: bitrate,
                        buf_size: buf_size,
                        color_metadata: color_metadata,
                        color_space_settings: color_space_settings,
                        filter_settings: filter_settings,
                        fixed_afd: fixed_afd,
                        flicker_aq: flicker_aq,
                        framerate_denominator: framerate_denominator,
                        framerate_numerator: framerate_numerator,
                        gop_closed_cadence: gop_closed_cadence,
                        gop_size: gop_size,
                        gop_size_units: gop_size_units,
                        level: level,
                        look_ahead_rate_control: look_ahead_rate_control,
                        max_bitrate: max_bitrate,
                        min_i_interval: min_i_interval,
                        par_denominator: par_denominator,
                        par_numerator: par_numerator,
                        profile: profile,
                        qvbr_quality_level: qvbr_quality_level,
                        rate_control_mode: rate_control_mode,
                        scan_type: scan_type,
                        scene_change_detect: scene_change_detect,
                        slices: slices,
                        tier: tier,
                        timecode_insertion: timecode_insertion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Hdr10Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hdr10settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Hdr10Settings {
        /// Property [`MaxCll`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hdr10settings.html#cfn-medialive-channel-hdr10settings-maxcll).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_cll: Option<::Value<u32>>,
        /// Property [`MaxFall`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hdr10settings.html#cfn-medialive-channel-hdr10settings-maxfall).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_fall: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Hdr10Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_cll) = self.max_cll {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCll", max_cll)?;
            }
            if let Some(ref max_fall) = self.max_fall {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFall", max_fall)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Hdr10Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Hdr10Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Hdr10Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Hdr10Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_cll: Option<::Value<u32>> = None;
                    let mut max_fall: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCll" => {
                                max_cll = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFall" => {
                                max_fall = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Hdr10Settings {
                        max_cll: max_cll,
                        max_fall: max_fall,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsAkamaiSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsAkamaiSettings {
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`FilecacheDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-filecacheduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filecache_duration: Option<::Value<u32>>,
        /// Property [`HttpTransferMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-httptransfermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_transfer_mode: Option<::Value<String>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
        /// Property [`Salt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-salt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salt: Option<::Value<String>>,
        /// Property [`Token`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsakamaisettings.html#cfn-medialive-channel-hlsakamaisettings-token).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsAkamaiSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref filecache_duration) = self.filecache_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilecacheDuration", filecache_duration)?;
            }
            if let Some(ref http_transfer_mode) = self.http_transfer_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpTransferMode", http_transfer_mode)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            if let Some(ref salt) = self.salt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salt", salt)?;
            }
            if let Some(ref token) = self.token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Token", token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsAkamaiSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsAkamaiSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsAkamaiSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsAkamaiSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut filecache_duration: Option<::Value<u32>> = None;
                    let mut http_transfer_mode: Option<::Value<String>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;
                    let mut salt: Option<::Value<String>> = None;
                    let mut token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilecacheDuration" => {
                                filecache_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpTransferMode" => {
                                http_transfer_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salt" => {
                                salt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Token" => {
                                token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsAkamaiSettings {
                        connection_retry_interval: connection_retry_interval,
                        filecache_duration: filecache_duration,
                        http_transfer_mode: http_transfer_mode,
                        num_retries: num_retries,
                        restart_delay: restart_delay,
                        salt: salt,
                        token: token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsBasicPutSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsBasicPutSettings {
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html#cfn-medialive-channel-hlsbasicputsettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`FilecacheDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html#cfn-medialive-channel-hlsbasicputsettings-filecacheduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filecache_duration: Option<::Value<u32>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html#cfn-medialive-channel-hlsbasicputsettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsbasicputsettings.html#cfn-medialive-channel-hlsbasicputsettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HlsBasicPutSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref filecache_duration) = self.filecache_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilecacheDuration", filecache_duration)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsBasicPutSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsBasicPutSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsBasicPutSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsBasicPutSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut filecache_duration: Option<::Value<u32>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilecacheDuration" => {
                                filecache_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsBasicPutSettings {
                        connection_retry_interval: connection_retry_interval,
                        filecache_duration: filecache_duration,
                        num_retries: num_retries,
                        restart_delay: restart_delay,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsCdnSettings {
        /// Property [`HlsAkamaiSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html#cfn-medialive-channel-hlscdnsettings-hlsakamaisettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_akamai_settings: Option<::Value<HlsAkamaiSettings>>,
        /// Property [`HlsBasicPutSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html#cfn-medialive-channel-hlscdnsettings-hlsbasicputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_basic_put_settings: Option<::Value<HlsBasicPutSettings>>,
        /// Property [`HlsMediaStoreSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html#cfn-medialive-channel-hlscdnsettings-hlsmediastoresettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_media_store_settings: Option<::Value<HlsMediaStoreSettings>>,
        /// Property [`HlsS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html#cfn-medialive-channel-hlscdnsettings-hlss3settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_s3_settings: Option<::Value<HlsS3Settings>>,
        /// Property [`HlsWebdavSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlscdnsettings.html#cfn-medialive-channel-hlscdnsettings-hlswebdavsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_webdav_settings: Option<::Value<HlsWebdavSettings>>,
    }

    impl ::codec::SerializeValue for HlsCdnSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hls_akamai_settings) = self.hls_akamai_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsAkamaiSettings", hls_akamai_settings)?;
            }
            if let Some(ref hls_basic_put_settings) = self.hls_basic_put_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsBasicPutSettings", hls_basic_put_settings)?;
            }
            if let Some(ref hls_media_store_settings) = self.hls_media_store_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsMediaStoreSettings", hls_media_store_settings)?;
            }
            if let Some(ref hls_s3_settings) = self.hls_s3_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsS3Settings", hls_s3_settings)?;
            }
            if let Some(ref hls_webdav_settings) = self.hls_webdav_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsWebdavSettings", hls_webdav_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsCdnSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsCdnSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsCdnSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsCdnSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hls_akamai_settings: Option<::Value<HlsAkamaiSettings>> = None;
                    let mut hls_basic_put_settings: Option<::Value<HlsBasicPutSettings>> = None;
                    let mut hls_media_store_settings: Option<::Value<HlsMediaStoreSettings>> = None;
                    let mut hls_s3_settings: Option<::Value<HlsS3Settings>> = None;
                    let mut hls_webdav_settings: Option<::Value<HlsWebdavSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HlsAkamaiSettings" => {
                                hls_akamai_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsBasicPutSettings" => {
                                hls_basic_put_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsMediaStoreSettings" => {
                                hls_media_store_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsS3Settings" => {
                                hls_s3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsWebdavSettings" => {
                                hls_webdav_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsCdnSettings {
                        hls_akamai_settings: hls_akamai_settings,
                        hls_basic_put_settings: hls_basic_put_settings,
                        hls_media_store_settings: hls_media_store_settings,
                        hls_s3_settings: hls_s3_settings,
                        hls_webdav_settings: hls_webdav_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsGroupSettings {
        /// Property [`AdMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-admarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markers: Option<::ValueList<String>>,
        /// Property [`BaseUrlContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-baseurlcontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url_content: Option<::Value<String>>,
        /// Property [`BaseUrlContent1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-baseurlcontent1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url_content1: Option<::Value<String>>,
        /// Property [`BaseUrlManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-baseurlmanifest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url_manifest: Option<::Value<String>>,
        /// Property [`BaseUrlManifest1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-baseurlmanifest1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_url_manifest1: Option<::Value<String>>,
        /// Property [`CaptionLanguageMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-captionlanguagemappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_language_mappings: Option<::ValueList<CaptionLanguageMapping>>,
        /// Property [`CaptionLanguageSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-captionlanguagesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_language_setting: Option<::Value<String>>,
        /// Property [`ClientCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-clientcache).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_cache: Option<::Value<String>>,
        /// Property [`CodecSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-codecspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub codec_specification: Option<::Value<String>>,
        /// Property [`ConstantIv`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-constantiv).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_iv: Option<::Value<String>>,
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`DirectoryStructure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-directorystructure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_structure: Option<::Value<String>>,
        /// Property [`DiscontinuityTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-discontinuitytags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub discontinuity_tags: Option<::Value<String>>,
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-encryptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_type: Option<::Value<String>>,
        /// Property [`HlsCdnSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-hlscdnsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_cdn_settings: Option<::Value<HlsCdnSettings>>,
        /// Property [`HlsId3SegmentTagging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-hlsid3segmenttagging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_id3_segment_tagging: Option<::Value<String>>,
        /// Property [`IFrameOnlyPlaylists`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-iframeonlyplaylists).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub i_frame_only_playlists: Option<::Value<String>>,
        /// Property [`IncompleteSegmentBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-incompletesegmentbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub incomplete_segment_behavior: Option<::Value<String>>,
        /// Property [`IndexNSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-indexnsegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_n_segments: Option<::Value<u32>>,
        /// Property [`InputLossAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-inputlossaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_action: Option<::Value<String>>,
        /// Property [`IvInManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-ivinmanifest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iv_in_manifest: Option<::Value<String>>,
        /// Property [`IvSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-ivsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iv_source: Option<::Value<String>>,
        /// Property [`KeepSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-keepsegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub keep_segments: Option<::Value<u32>>,
        /// Property [`KeyFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-keyformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_format: Option<::Value<String>>,
        /// Property [`KeyFormatVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-keyformatversions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_format_versions: Option<::Value<String>>,
        /// Property [`KeyProviderSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-keyprovidersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_provider_settings: Option<::Value<KeyProviderSettings>>,
        /// Property [`ManifestCompression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-manifestcompression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_compression: Option<::Value<String>>,
        /// Property [`ManifestDurationFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-manifestdurationformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_duration_format: Option<::Value<String>>,
        /// Property [`MinSegmentLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-minsegmentlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_segment_length: Option<::Value<u32>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`OutputSelection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-outputselection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_selection: Option<::Value<String>>,
        /// Property [`ProgramDateTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-programdatetime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time: Option<::Value<String>>,
        /// Property [`ProgramDateTimePeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-programdatetimeperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_date_time_period: Option<::Value<u32>>,
        /// Property [`RedundantManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-redundantmanifest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redundant_manifest: Option<::Value<String>>,
        /// Property [`SegmentLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-segmentlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_length: Option<::Value<u32>>,
        /// Property [`SegmentationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-segmentationmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segmentation_mode: Option<::Value<String>>,
        /// Property [`SegmentsPerSubdirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-segmentspersubdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segments_per_subdirectory: Option<::Value<u32>>,
        /// Property [`StreamInfResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-streaminfresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_inf_resolution: Option<::Value<String>>,
        /// Property [`TimedMetadataId3Frame`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-timedmetadataid3frame).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_id3_frame: Option<::Value<String>>,
        /// Property [`TimedMetadataId3Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-timedmetadataid3period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_id3_period: Option<::Value<u32>>,
        /// Property [`TimestampDeltaMilliseconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-timestampdeltamilliseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_delta_milliseconds: Option<::Value<u32>>,
        /// Property [`TsFileMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsgroupsettings.html#cfn-medialive-channel-hlsgroupsettings-tsfilemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ts_file_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markers) = self.ad_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkers", ad_markers)?;
            }
            if let Some(ref base_url_content) = self.base_url_content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrlContent", base_url_content)?;
            }
            if let Some(ref base_url_content1) = self.base_url_content1 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrlContent1", base_url_content1)?;
            }
            if let Some(ref base_url_manifest) = self.base_url_manifest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrlManifest", base_url_manifest)?;
            }
            if let Some(ref base_url_manifest1) = self.base_url_manifest1 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseUrlManifest1", base_url_manifest1)?;
            }
            if let Some(ref caption_language_mappings) = self.caption_language_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionLanguageMappings", caption_language_mappings)?;
            }
            if let Some(ref caption_language_setting) = self.caption_language_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionLanguageSetting", caption_language_setting)?;
            }
            if let Some(ref client_cache) = self.client_cache {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCache", client_cache)?;
            }
            if let Some(ref codec_specification) = self.codec_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodecSpecification", codec_specification)?;
            }
            if let Some(ref constant_iv) = self.constant_iv {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantIv", constant_iv)?;
            }
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref directory_structure) = self.directory_structure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryStructure", directory_structure)?;
            }
            if let Some(ref discontinuity_tags) = self.discontinuity_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiscontinuityTags", discontinuity_tags)?;
            }
            if let Some(ref encryption_type) = self.encryption_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", encryption_type)?;
            }
            if let Some(ref hls_cdn_settings) = self.hls_cdn_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsCdnSettings", hls_cdn_settings)?;
            }
            if let Some(ref hls_id3_segment_tagging) = self.hls_id3_segment_tagging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsId3SegmentTagging", hls_id3_segment_tagging)?;
            }
            if let Some(ref i_frame_only_playlists) = self.i_frame_only_playlists {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IFrameOnlyPlaylists", i_frame_only_playlists)?;
            }
            if let Some(ref incomplete_segment_behavior) = self.incomplete_segment_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncompleteSegmentBehavior", incomplete_segment_behavior)?;
            }
            if let Some(ref index_n_segments) = self.index_n_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexNSegments", index_n_segments)?;
            }
            if let Some(ref input_loss_action) = self.input_loss_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossAction", input_loss_action)?;
            }
            if let Some(ref iv_in_manifest) = self.iv_in_manifest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IvInManifest", iv_in_manifest)?;
            }
            if let Some(ref iv_source) = self.iv_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IvSource", iv_source)?;
            }
            if let Some(ref keep_segments) = self.keep_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeepSegments", keep_segments)?;
            }
            if let Some(ref key_format) = self.key_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyFormat", key_format)?;
            }
            if let Some(ref key_format_versions) = self.key_format_versions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyFormatVersions", key_format_versions)?;
            }
            if let Some(ref key_provider_settings) = self.key_provider_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyProviderSettings", key_provider_settings)?;
            }
            if let Some(ref manifest_compression) = self.manifest_compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestCompression", manifest_compression)?;
            }
            if let Some(ref manifest_duration_format) = self.manifest_duration_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestDurationFormat", manifest_duration_format)?;
            }
            if let Some(ref min_segment_length) = self.min_segment_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinSegmentLength", min_segment_length)?;
            }
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref output_selection) = self.output_selection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSelection", output_selection)?;
            }
            if let Some(ref program_date_time) = self.program_date_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTime", program_date_time)?;
            }
            if let Some(ref program_date_time_period) = self.program_date_time_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramDateTimePeriod", program_date_time_period)?;
            }
            if let Some(ref redundant_manifest) = self.redundant_manifest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedundantManifest", redundant_manifest)?;
            }
            if let Some(ref segment_length) = self.segment_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentLength", segment_length)?;
            }
            if let Some(ref segmentation_mode) = self.segmentation_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentationMode", segmentation_mode)?;
            }
            if let Some(ref segments_per_subdirectory) = self.segments_per_subdirectory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentsPerSubdirectory", segments_per_subdirectory)?;
            }
            if let Some(ref stream_inf_resolution) = self.stream_inf_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamInfResolution", stream_inf_resolution)?;
            }
            if let Some(ref timed_metadata_id3_frame) = self.timed_metadata_id3_frame {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataId3Frame", timed_metadata_id3_frame)?;
            }
            if let Some(ref timed_metadata_id3_period) = self.timed_metadata_id3_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataId3Period", timed_metadata_id3_period)?;
            }
            if let Some(ref timestamp_delta_milliseconds) = self.timestamp_delta_milliseconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampDeltaMilliseconds", timestamp_delta_milliseconds)?;
            }
            if let Some(ref ts_file_mode) = self.ts_file_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TsFileMode", ts_file_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markers: Option<::ValueList<String>> = None;
                    let mut base_url_content: Option<::Value<String>> = None;
                    let mut base_url_content1: Option<::Value<String>> = None;
                    let mut base_url_manifest: Option<::Value<String>> = None;
                    let mut base_url_manifest1: Option<::Value<String>> = None;
                    let mut caption_language_mappings: Option<::ValueList<CaptionLanguageMapping>> = None;
                    let mut caption_language_setting: Option<::Value<String>> = None;
                    let mut client_cache: Option<::Value<String>> = None;
                    let mut codec_specification: Option<::Value<String>> = None;
                    let mut constant_iv: Option<::Value<String>> = None;
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut directory_structure: Option<::Value<String>> = None;
                    let mut discontinuity_tags: Option<::Value<String>> = None;
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut hls_cdn_settings: Option<::Value<HlsCdnSettings>> = None;
                    let mut hls_id3_segment_tagging: Option<::Value<String>> = None;
                    let mut i_frame_only_playlists: Option<::Value<String>> = None;
                    let mut incomplete_segment_behavior: Option<::Value<String>> = None;
                    let mut index_n_segments: Option<::Value<u32>> = None;
                    let mut input_loss_action: Option<::Value<String>> = None;
                    let mut iv_in_manifest: Option<::Value<String>> = None;
                    let mut iv_source: Option<::Value<String>> = None;
                    let mut keep_segments: Option<::Value<u32>> = None;
                    let mut key_format: Option<::Value<String>> = None;
                    let mut key_format_versions: Option<::Value<String>> = None;
                    let mut key_provider_settings: Option<::Value<KeyProviderSettings>> = None;
                    let mut manifest_compression: Option<::Value<String>> = None;
                    let mut manifest_duration_format: Option<::Value<String>> = None;
                    let mut min_segment_length: Option<::Value<u32>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut output_selection: Option<::Value<String>> = None;
                    let mut program_date_time: Option<::Value<String>> = None;
                    let mut program_date_time_period: Option<::Value<u32>> = None;
                    let mut redundant_manifest: Option<::Value<String>> = None;
                    let mut segment_length: Option<::Value<u32>> = None;
                    let mut segmentation_mode: Option<::Value<String>> = None;
                    let mut segments_per_subdirectory: Option<::Value<u32>> = None;
                    let mut stream_inf_resolution: Option<::Value<String>> = None;
                    let mut timed_metadata_id3_frame: Option<::Value<String>> = None;
                    let mut timed_metadata_id3_period: Option<::Value<u32>> = None;
                    let mut timestamp_delta_milliseconds: Option<::Value<u32>> = None;
                    let mut ts_file_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkers" => {
                                ad_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BaseUrlContent" => {
                                base_url_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BaseUrlContent1" => {
                                base_url_content1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BaseUrlManifest" => {
                                base_url_manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BaseUrlManifest1" => {
                                base_url_manifest1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionLanguageMappings" => {
                                caption_language_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionLanguageSetting" => {
                                caption_language_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientCache" => {
                                client_cache = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodecSpecification" => {
                                codec_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstantIv" => {
                                constant_iv = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DirectoryStructure" => {
                                directory_structure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiscontinuityTags" => {
                                discontinuity_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsCdnSettings" => {
                                hls_cdn_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsId3SegmentTagging" => {
                                hls_id3_segment_tagging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IFrameOnlyPlaylists" => {
                                i_frame_only_playlists = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncompleteSegmentBehavior" => {
                                incomplete_segment_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexNSegments" => {
                                index_n_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossAction" => {
                                input_loss_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IvInManifest" => {
                                iv_in_manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IvSource" => {
                                iv_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeepSegments" => {
                                keep_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyFormat" => {
                                key_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyFormatVersions" => {
                                key_format_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyProviderSettings" => {
                                key_provider_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestCompression" => {
                                manifest_compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManifestDurationFormat" => {
                                manifest_duration_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinSegmentLength" => {
                                min_segment_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputSelection" => {
                                output_selection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTime" => {
                                program_date_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramDateTimePeriod" => {
                                program_date_time_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedundantManifest" => {
                                redundant_manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentLength" => {
                                segment_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentationMode" => {
                                segmentation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentsPerSubdirectory" => {
                                segments_per_subdirectory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamInfResolution" => {
                                stream_inf_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataId3Frame" => {
                                timed_metadata_id3_frame = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataId3Period" => {
                                timed_metadata_id3_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampDeltaMilliseconds" => {
                                timestamp_delta_milliseconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TsFileMode" => {
                                ts_file_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsGroupSettings {
                        ad_markers: ad_markers,
                        base_url_content: base_url_content,
                        base_url_content1: base_url_content1,
                        base_url_manifest: base_url_manifest,
                        base_url_manifest1: base_url_manifest1,
                        caption_language_mappings: caption_language_mappings,
                        caption_language_setting: caption_language_setting,
                        client_cache: client_cache,
                        codec_specification: codec_specification,
                        constant_iv: constant_iv,
                        destination: destination,
                        directory_structure: directory_structure,
                        discontinuity_tags: discontinuity_tags,
                        encryption_type: encryption_type,
                        hls_cdn_settings: hls_cdn_settings,
                        hls_id3_segment_tagging: hls_id3_segment_tagging,
                        i_frame_only_playlists: i_frame_only_playlists,
                        incomplete_segment_behavior: incomplete_segment_behavior,
                        index_n_segments: index_n_segments,
                        input_loss_action: input_loss_action,
                        iv_in_manifest: iv_in_manifest,
                        iv_source: iv_source,
                        keep_segments: keep_segments,
                        key_format: key_format,
                        key_format_versions: key_format_versions,
                        key_provider_settings: key_provider_settings,
                        manifest_compression: manifest_compression,
                        manifest_duration_format: manifest_duration_format,
                        min_segment_length: min_segment_length,
                        mode: mode,
                        output_selection: output_selection,
                        program_date_time: program_date_time,
                        program_date_time_period: program_date_time_period,
                        redundant_manifest: redundant_manifest,
                        segment_length: segment_length,
                        segmentation_mode: segmentation_mode,
                        segments_per_subdirectory: segments_per_subdirectory,
                        stream_inf_resolution: stream_inf_resolution,
                        timed_metadata_id3_frame: timed_metadata_id3_frame,
                        timed_metadata_id3_period: timed_metadata_id3_period,
                        timestamp_delta_milliseconds: timestamp_delta_milliseconds,
                        ts_file_mode: ts_file_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsInputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsInputSettings {
        /// Property [`Bandwidth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html#cfn-medialive-channel-hlsinputsettings-bandwidth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bandwidth: Option<::Value<u32>>,
        /// Property [`BufferSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html#cfn-medialive-channel-hlsinputsettings-buffersegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffer_segments: Option<::Value<u32>>,
        /// Property [`Retries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html#cfn-medialive-channel-hlsinputsettings-retries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retries: Option<::Value<u32>>,
        /// Property [`RetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsinputsettings.html#cfn-medialive-channel-hlsinputsettings-retryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_interval: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HlsInputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bandwidth) = self.bandwidth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bandwidth", bandwidth)?;
            }
            if let Some(ref buffer_segments) = self.buffer_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferSegments", buffer_segments)?;
            }
            if let Some(ref retries) = self.retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Retries", retries)?;
            }
            if let Some(ref retry_interval) = self.retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryInterval", retry_interval)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsInputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsInputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsInputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsInputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bandwidth: Option<::Value<u32>> = None;
                    let mut buffer_segments: Option<::Value<u32>> = None;
                    let mut retries: Option<::Value<u32>> = None;
                    let mut retry_interval: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bandwidth" => {
                                bandwidth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufferSegments" => {
                                buffer_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Retries" => {
                                retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryInterval" => {
                                retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsInputSettings {
                        bandwidth: bandwidth,
                        buffer_segments: buffer_segments,
                        retries: retries,
                        retry_interval: retry_interval,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsMediaStoreSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsMediaStoreSettings {
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html#cfn-medialive-channel-hlsmediastoresettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`FilecacheDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html#cfn-medialive-channel-hlsmediastoresettings-filecacheduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filecache_duration: Option<::Value<u32>>,
        /// Property [`MediaStoreStorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html#cfn-medialive-channel-hlsmediastoresettings-mediastorestorageclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_store_storage_class: Option<::Value<String>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html#cfn-medialive-channel-hlsmediastoresettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsmediastoresettings.html#cfn-medialive-channel-hlsmediastoresettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HlsMediaStoreSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref filecache_duration) = self.filecache_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilecacheDuration", filecache_duration)?;
            }
            if let Some(ref media_store_storage_class) = self.media_store_storage_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaStoreStorageClass", media_store_storage_class)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsMediaStoreSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsMediaStoreSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsMediaStoreSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsMediaStoreSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut filecache_duration: Option<::Value<u32>> = None;
                    let mut media_store_storage_class: Option<::Value<String>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilecacheDuration" => {
                                filecache_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaStoreStorageClass" => {
                                media_store_storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsMediaStoreSettings {
                        connection_retry_interval: connection_retry_interval,
                        filecache_duration: filecache_duration,
                        media_store_storage_class: media_store_storage_class,
                        num_retries: num_retries,
                        restart_delay: restart_delay,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsOutputSettings {
        /// Property [`H265PackagingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html#cfn-medialive-channel-hlsoutputsettings-h265packagingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub h265_packaging_type: Option<::Value<String>>,
        /// Property [`HlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html#cfn-medialive-channel-hlsoutputsettings-hlssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_settings: Option<::Value<HlsSettings>>,
        /// Property [`NameModifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html#cfn-medialive-channel-hlsoutputsettings-namemodifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name_modifier: Option<::Value<String>>,
        /// Property [`SegmentModifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlsoutputsettings.html#cfn-medialive-channel-hlsoutputsettings-segmentmodifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_modifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref h265_packaging_type) = self.h265_packaging_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "H265PackagingType", h265_packaging_type)?;
            }
            if let Some(ref hls_settings) = self.hls_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsSettings", hls_settings)?;
            }
            if let Some(ref name_modifier) = self.name_modifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameModifier", name_modifier)?;
            }
            if let Some(ref segment_modifier) = self.segment_modifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentModifier", segment_modifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut h265_packaging_type: Option<::Value<String>> = None;
                    let mut hls_settings: Option<::Value<HlsSettings>> = None;
                    let mut name_modifier: Option<::Value<String>> = None;
                    let mut segment_modifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "H265PackagingType" => {
                                h265_packaging_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsSettings" => {
                                hls_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NameModifier" => {
                                name_modifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentModifier" => {
                                segment_modifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsOutputSettings {
                        h265_packaging_type: h265_packaging_type,
                        hls_settings: hls_settings,
                        name_modifier: name_modifier,
                        segment_modifier: segment_modifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsS3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlss3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsS3Settings {
        /// Property [`CannedAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlss3settings.html#cfn-medialive-channel-hlss3settings-cannedacl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canned_acl: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HlsS3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref canned_acl) = self.canned_acl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CannedAcl", canned_acl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsS3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsS3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsS3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsS3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut canned_acl: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CannedAcl" => {
                                canned_acl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsS3Settings {
                        canned_acl: canned_acl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsSettings {
        /// Property [`AudioOnlyHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html#cfn-medialive-channel-hlssettings-audioonlyhlssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_only_hls_settings: Option<::Value<AudioOnlyHlsSettings>>,
        /// Property [`Fmp4HlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html#cfn-medialive-channel-hlssettings-fmp4hlssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fmp4_hls_settings: Option<::Value<Fmp4HlsSettings>>,
        /// Property [`FrameCaptureHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html#cfn-medialive-channel-hlssettings-framecapturehlssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_hls_settings: Option<::Value<FrameCaptureHlsSettings>>,
        /// Property [`StandardHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlssettings.html#cfn-medialive-channel-hlssettings-standardhlssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_hls_settings: Option<::Value<StandardHlsSettings>>,
    }

    impl ::codec::SerializeValue for HlsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_only_hls_settings) = self.audio_only_hls_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioOnlyHlsSettings", audio_only_hls_settings)?;
            }
            if let Some(ref fmp4_hls_settings) = self.fmp4_hls_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fmp4HlsSettings", fmp4_hls_settings)?;
            }
            if let Some(ref frame_capture_hls_settings) = self.frame_capture_hls_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureHlsSettings", frame_capture_hls_settings)?;
            }
            if let Some(ref standard_hls_settings) = self.standard_hls_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardHlsSettings", standard_hls_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_only_hls_settings: Option<::Value<AudioOnlyHlsSettings>> = None;
                    let mut fmp4_hls_settings: Option<::Value<Fmp4HlsSettings>> = None;
                    let mut frame_capture_hls_settings: Option<::Value<FrameCaptureHlsSettings>> = None;
                    let mut standard_hls_settings: Option<::Value<StandardHlsSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioOnlyHlsSettings" => {
                                audio_only_hls_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Fmp4HlsSettings" => {
                                fmp4_hls_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameCaptureHlsSettings" => {
                                frame_capture_hls_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardHlsSettings" => {
                                standard_hls_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsSettings {
                        audio_only_hls_settings: audio_only_hls_settings,
                        fmp4_hls_settings: fmp4_hls_settings,
                        frame_capture_hls_settings: frame_capture_hls_settings,
                        standard_hls_settings: standard_hls_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HlsWebdavSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HlsWebdavSettings {
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html#cfn-medialive-channel-hlswebdavsettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`FilecacheDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html#cfn-medialive-channel-hlswebdavsettings-filecacheduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filecache_duration: Option<::Value<u32>>,
        /// Property [`HttpTransferMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html#cfn-medialive-channel-hlswebdavsettings-httptransfermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_transfer_mode: Option<::Value<String>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html#cfn-medialive-channel-hlswebdavsettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-hlswebdavsettings.html#cfn-medialive-channel-hlswebdavsettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HlsWebdavSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref filecache_duration) = self.filecache_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilecacheDuration", filecache_duration)?;
            }
            if let Some(ref http_transfer_mode) = self.http_transfer_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpTransferMode", http_transfer_mode)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HlsWebdavSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HlsWebdavSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HlsWebdavSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HlsWebdavSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut filecache_duration: Option<::Value<u32>> = None;
                    let mut http_transfer_mode: Option<::Value<String>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilecacheDuration" => {
                                filecache_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpTransferMode" => {
                                http_transfer_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HlsWebdavSettings {
                        connection_retry_interval: connection_retry_interval,
                        filecache_duration: filecache_duration,
                        http_transfer_mode: http_transfer_mode,
                        num_retries: num_retries,
                        restart_delay: restart_delay,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.HtmlMotionGraphicsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-htmlmotiongraphicssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct HtmlMotionGraphicsSettings {
    }

    impl ::codec::SerializeValue for HtmlMotionGraphicsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HtmlMotionGraphicsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HtmlMotionGraphicsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HtmlMotionGraphicsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HtmlMotionGraphicsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(HtmlMotionGraphicsSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct InputAttachment {
        /// Property [`AutomaticInputFailoverSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html#cfn-medialive-channel-inputattachment-automaticinputfailoversettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_input_failover_settings: Option<::Value<AutomaticInputFailoverSettings>>,
        /// Property [`InputAttachmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html#cfn-medialive-channel-inputattachment-inputattachmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_attachment_name: Option<::Value<String>>,
        /// Property [`InputId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html#cfn-medialive-channel-inputattachment-inputid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub input_id: Option<::Value<String>>,
        /// Property [`InputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputattachment.html#cfn-medialive-channel-inputattachment-inputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_settings: Option<::Value<InputSettings>>,
    }

    impl ::codec::SerializeValue for InputAttachment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatic_input_failover_settings) = self.automatic_input_failover_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticInputFailoverSettings", automatic_input_failover_settings)?;
            }
            if let Some(ref input_attachment_name) = self.input_attachment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputAttachmentName", input_attachment_name)?;
            }
            if let Some(ref input_id) = self.input_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputId", input_id)?;
            }
            if let Some(ref input_settings) = self.input_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSettings", input_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputAttachment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputAttachment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputAttachment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputAttachment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatic_input_failover_settings: Option<::Value<AutomaticInputFailoverSettings>> = None;
                    let mut input_attachment_name: Option<::Value<String>> = None;
                    let mut input_id: Option<::Value<String>> = None;
                    let mut input_settings: Option<::Value<InputSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticInputFailoverSettings" => {
                                automatic_input_failover_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputAttachmentName" => {
                                input_attachment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputId" => {
                                input_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputSettings" => {
                                input_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputAttachment {
                        automatic_input_failover_settings: automatic_input_failover_settings,
                        input_attachment_name: input_attachment_name,
                        input_id: input_id,
                        input_settings: input_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputChannelLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputchannellevel.html) property type.
    #[derive(Debug, Default)]
    pub struct InputChannelLevel {
        /// Property [`Gain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputchannellevel.html#cfn-medialive-channel-inputchannellevel-gain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gain: Option<::Value<u32>>,
        /// Property [`InputChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputchannellevel.html#cfn-medialive-channel-inputchannellevel-inputchannel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_channel: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InputChannelLevel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref gain) = self.gain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gain", gain)?;
            }
            if let Some(ref input_channel) = self.input_channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputChannel", input_channel)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputChannelLevel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputChannelLevel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputChannelLevel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputChannelLevel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gain: Option<::Value<u32>> = None;
                    let mut input_channel: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gain" => {
                                gain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputChannel" => {
                                input_channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputChannelLevel {
                        gain: gain,
                        input_channel: input_channel,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct InputLocation {
        /// Property [`PasswordParam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlocation.html#cfn-medialive-channel-inputlocation-passwordparam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_param: Option<::Value<String>>,
        /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlocation.html#cfn-medialive-channel-inputlocation-uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlocation.html#cfn-medialive-channel-inputlocation-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password_param) = self.password_param {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordParam", password_param)?;
            }
            if let Some(ref uri) = self.uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", uri)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password_param: Option<::Value<String>> = None;
                    let mut uri: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordParam" => {
                                password_param = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uri" => {
                                uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputLocation {
                        password_param: password_param,
                        uri: uri,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputLossBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html) property type.
    #[derive(Debug, Default)]
    pub struct InputLossBehavior {
        /// Property [`BlackFrameMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html#cfn-medialive-channel-inputlossbehavior-blackframemsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub black_frame_msec: Option<::Value<u32>>,
        /// Property [`InputLossImageColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html#cfn-medialive-channel-inputlossbehavior-inputlossimagecolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_image_color: Option<::Value<String>>,
        /// Property [`InputLossImageSlate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html#cfn-medialive-channel-inputlossbehavior-inputlossimageslate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_image_slate: Option<::Value<InputLocation>>,
        /// Property [`InputLossImageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html#cfn-medialive-channel-inputlossbehavior-inputlossimagetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_image_type: Option<::Value<String>>,
        /// Property [`RepeatFrameMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossbehavior.html#cfn-medialive-channel-inputlossbehavior-repeatframemsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub repeat_frame_msec: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InputLossBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref black_frame_msec) = self.black_frame_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlackFrameMsec", black_frame_msec)?;
            }
            if let Some(ref input_loss_image_color) = self.input_loss_image_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossImageColor", input_loss_image_color)?;
            }
            if let Some(ref input_loss_image_slate) = self.input_loss_image_slate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossImageSlate", input_loss_image_slate)?;
            }
            if let Some(ref input_loss_image_type) = self.input_loss_image_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossImageType", input_loss_image_type)?;
            }
            if let Some(ref repeat_frame_msec) = self.repeat_frame_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepeatFrameMsec", repeat_frame_msec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputLossBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputLossBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputLossBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputLossBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut black_frame_msec: Option<::Value<u32>> = None;
                    let mut input_loss_image_color: Option<::Value<String>> = None;
                    let mut input_loss_image_slate: Option<::Value<InputLocation>> = None;
                    let mut input_loss_image_type: Option<::Value<String>> = None;
                    let mut repeat_frame_msec: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlackFrameMsec" => {
                                black_frame_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossImageColor" => {
                                input_loss_image_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossImageSlate" => {
                                input_loss_image_slate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossImageType" => {
                                input_loss_image_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepeatFrameMsec" => {
                                repeat_frame_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputLossBehavior {
                        black_frame_msec: black_frame_msec,
                        input_loss_image_color: input_loss_image_color,
                        input_loss_image_slate: input_loss_image_slate,
                        input_loss_image_type: input_loss_image_type,
                        repeat_frame_msec: repeat_frame_msec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputLossFailoverSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossfailoversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct InputLossFailoverSettings {
        /// Property [`InputLossThresholdMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputlossfailoversettings.html#cfn-medialive-channel-inputlossfailoversettings-inputlossthresholdmsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_threshold_msec: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InputLossFailoverSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_loss_threshold_msec) = self.input_loss_threshold_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossThresholdMsec", input_loss_threshold_msec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputLossFailoverSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputLossFailoverSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputLossFailoverSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputLossFailoverSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_loss_threshold_msec: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputLossThresholdMsec" => {
                                input_loss_threshold_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputLossFailoverSettings {
                        input_loss_threshold_msec: input_loss_threshold_msec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct InputSettings {
        /// Property [`AudioSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-audioselectors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_selectors: Option<::ValueList<AudioSelector>>,
        /// Property [`CaptionSelectors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-captionselectors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_selectors: Option<::ValueList<CaptionSelector>>,
        /// Property [`DeblockFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-deblockfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deblock_filter: Option<::Value<String>>,
        /// Property [`DenoiseFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-denoisefilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub denoise_filter: Option<::Value<String>>,
        /// Property [`FilterStrength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-filterstrength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_strength: Option<::Value<u32>>,
        /// Property [`InputFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-inputfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_filter: Option<::Value<String>>,
        /// Property [`NetworkInputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-networkinputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_input_settings: Option<::Value<NetworkInputSettings>>,
        /// Property [`Smpte2038DataPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-smpte2038datapreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smpte2038_data_preference: Option<::Value<String>>,
        /// Property [`SourceEndBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-sourceendbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_end_behavior: Option<::Value<String>>,
        /// Property [`VideoSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputsettings.html#cfn-medialive-channel-inputsettings-videoselector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_selector: Option<::Value<VideoSelector>>,
    }

    impl ::codec::SerializeValue for InputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_selectors) = self.audio_selectors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSelectors", audio_selectors)?;
            }
            if let Some(ref caption_selectors) = self.caption_selectors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionSelectors", caption_selectors)?;
            }
            if let Some(ref deblock_filter) = self.deblock_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeblockFilter", deblock_filter)?;
            }
            if let Some(ref denoise_filter) = self.denoise_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DenoiseFilter", denoise_filter)?;
            }
            if let Some(ref filter_strength) = self.filter_strength {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterStrength", filter_strength)?;
            }
            if let Some(ref input_filter) = self.input_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputFilter", input_filter)?;
            }
            if let Some(ref network_input_settings) = self.network_input_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInputSettings", network_input_settings)?;
            }
            if let Some(ref smpte2038_data_preference) = self.smpte2038_data_preference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Smpte2038DataPreference", smpte2038_data_preference)?;
            }
            if let Some(ref source_end_behavior) = self.source_end_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEndBehavior", source_end_behavior)?;
            }
            if let Some(ref video_selector) = self.video_selector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoSelector", video_selector)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_selectors: Option<::ValueList<AudioSelector>> = None;
                    let mut caption_selectors: Option<::ValueList<CaptionSelector>> = None;
                    let mut deblock_filter: Option<::Value<String>> = None;
                    let mut denoise_filter: Option<::Value<String>> = None;
                    let mut filter_strength: Option<::Value<u32>> = None;
                    let mut input_filter: Option<::Value<String>> = None;
                    let mut network_input_settings: Option<::Value<NetworkInputSettings>> = None;
                    let mut smpte2038_data_preference: Option<::Value<String>> = None;
                    let mut source_end_behavior: Option<::Value<String>> = None;
                    let mut video_selector: Option<::Value<VideoSelector>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioSelectors" => {
                                audio_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionSelectors" => {
                                caption_selectors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeblockFilter" => {
                                deblock_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DenoiseFilter" => {
                                denoise_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterStrength" => {
                                filter_strength = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputFilter" => {
                                input_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkInputSettings" => {
                                network_input_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Smpte2038DataPreference" => {
                                smpte2038_data_preference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceEndBehavior" => {
                                source_end_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoSelector" => {
                                video_selector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputSettings {
                        audio_selectors: audio_selectors,
                        caption_selectors: caption_selectors,
                        deblock_filter: deblock_filter,
                        denoise_filter: denoise_filter,
                        filter_strength: filter_strength,
                        input_filter: input_filter,
                        network_input_settings: network_input_settings,
                        smpte2038_data_preference: smpte2038_data_preference,
                        source_end_behavior: source_end_behavior,
                        video_selector: video_selector,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.InputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct InputSpecification {
        /// Property [`Codec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputspecification.html#cfn-medialive-channel-inputspecification-codec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub codec: Option<::Value<String>>,
        /// Property [`MaximumBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputspecification.html#cfn-medialive-channel-inputspecification-maximumbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_bitrate: Option<::Value<String>>,
        /// Property [`Resolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-inputspecification.html#cfn-medialive-channel-inputspecification-resolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resolution: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref codec) = self.codec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Codec", codec)?;
            }
            if let Some(ref maximum_bitrate) = self.maximum_bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumBitrate", maximum_bitrate)?;
            }
            if let Some(ref resolution) = self.resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resolution", resolution)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut codec: Option<::Value<String>> = None;
                    let mut maximum_bitrate: Option<::Value<String>> = None;
                    let mut resolution: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Codec" => {
                                codec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumBitrate" => {
                                maximum_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Resolution" => {
                                resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputSpecification {
                        codec: codec,
                        maximum_bitrate: maximum_bitrate,
                        resolution: resolution,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.KeyProviderSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-keyprovidersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyProviderSettings {
        /// Property [`StaticKeySettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-keyprovidersettings.html#cfn-medialive-channel-keyprovidersettings-statickeysettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub static_key_settings: Option<::Value<StaticKeySettings>>,
    }

    impl ::codec::SerializeValue for KeyProviderSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref static_key_settings) = self.static_key_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StaticKeySettings", static_key_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyProviderSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyProviderSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyProviderSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyProviderSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut static_key_settings: Option<::Value<StaticKeySettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StaticKeySettings" => {
                                static_key_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyProviderSettings {
                        static_key_settings: static_key_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.M2tsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct M2tsSettings {
        /// Property [`AbsentInputAudioBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-absentinputaudiobehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub absent_input_audio_behavior: Option<::Value<String>>,
        /// Property [`Arib`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-arib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arib: Option<::Value<String>>,
        /// Property [`AribCaptionsPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-aribcaptionspid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arib_captions_pid: Option<::Value<String>>,
        /// Property [`AribCaptionsPidControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-aribcaptionspidcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arib_captions_pid_control: Option<::Value<String>>,
        /// Property [`AudioBufferModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-audiobuffermodel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_buffer_model: Option<::Value<String>>,
        /// Property [`AudioFramesPerPes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-audioframesperpes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_frames_per_pes: Option<::Value<u32>>,
        /// Property [`AudioPids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-audiopids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_pids: Option<::Value<String>>,
        /// Property [`AudioStreamType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-audiostreamtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_stream_type: Option<::Value<String>>,
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<u32>>,
        /// Property [`BufferModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-buffermodel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffer_model: Option<::Value<String>>,
        /// Property [`CcDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ccdescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cc_descriptor: Option<::Value<String>>,
        /// Property [`DvbNitSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-dvbnitsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_nit_settings: Option<::Value<DvbNitSettings>>,
        /// Property [`DvbSdtSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-dvbsdtsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_sdt_settings: Option<::Value<DvbSdtSettings>>,
        /// Property [`DvbSubPids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-dvbsubpids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_sub_pids: Option<::Value<String>>,
        /// Property [`DvbTdtSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-dvbtdtsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_tdt_settings: Option<::Value<DvbTdtSettings>>,
        /// Property [`DvbTeletextPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-dvbteletextpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dvb_teletext_pid: Option<::Value<String>>,
        /// Property [`Ebif`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ebif).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebif: Option<::Value<String>>,
        /// Property [`EbpAudioInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ebpaudiointerval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebp_audio_interval: Option<::Value<String>>,
        /// Property [`EbpLookaheadMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ebplookaheadms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebp_lookahead_ms: Option<::Value<u32>>,
        /// Property [`EbpPlacement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ebpplacement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebp_placement: Option<::Value<String>>,
        /// Property [`EcmPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ecmpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecm_pid: Option<::Value<String>>,
        /// Property [`EsRateInPes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-esrateinpes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub es_rate_in_pes: Option<::Value<String>>,
        /// Property [`EtvPlatformPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-etvplatformpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub etv_platform_pid: Option<::Value<String>>,
        /// Property [`EtvSignalPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-etvsignalpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub etv_signal_pid: Option<::Value<String>>,
        /// Property [`FragmentTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-fragmenttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fragment_time: Option<::Value<f64>>,
        /// Property [`Klv`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-klv).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub klv: Option<::Value<String>>,
        /// Property [`KlvDataPids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-klvdatapids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub klv_data_pids: Option<::Value<String>>,
        /// Property [`NielsenId3Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-nielsenid3behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nielsen_id3_behavior: Option<::Value<String>>,
        /// Property [`NullPacketBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-nullpacketbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub null_packet_bitrate: Option<::Value<f64>>,
        /// Property [`PatInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-patinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pat_interval: Option<::Value<u32>>,
        /// Property [`PcrControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-pcrcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_control: Option<::Value<String>>,
        /// Property [`PcrPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-pcrperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_period: Option<::Value<u32>>,
        /// Property [`PcrPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-pcrpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_pid: Option<::Value<String>>,
        /// Property [`PmtInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-pmtinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pmt_interval: Option<::Value<u32>>,
        /// Property [`PmtPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-pmtpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pmt_pid: Option<::Value<String>>,
        /// Property [`ProgramNum`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-programnum).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_num: Option<::Value<u32>>,
        /// Property [`RateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-ratemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_mode: Option<::Value<String>>,
        /// Property [`Scte27Pids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-scte27pids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte27_pids: Option<::Value<String>>,
        /// Property [`Scte35Control`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-scte35control).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_control: Option<::Value<String>>,
        /// Property [`Scte35Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-scte35pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_pid: Option<::Value<String>>,
        /// Property [`SegmentationMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-segmentationmarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segmentation_markers: Option<::Value<String>>,
        /// Property [`SegmentationStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-segmentationstyle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segmentation_style: Option<::Value<String>>,
        /// Property [`SegmentationTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-segmentationtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segmentation_time: Option<::Value<f64>>,
        /// Property [`TimedMetadataBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-timedmetadatabehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_behavior: Option<::Value<String>>,
        /// Property [`TimedMetadataPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-timedmetadatapid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_pid: Option<::Value<String>>,
        /// Property [`TransportStreamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-transportstreamid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transport_stream_id: Option<::Value<u32>>,
        /// Property [`VideoPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m2tssettings.html#cfn-medialive-channel-m2tssettings-videopid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_pid: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for M2tsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref absent_input_audio_behavior) = self.absent_input_audio_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AbsentInputAudioBehavior", absent_input_audio_behavior)?;
            }
            if let Some(ref arib) = self.arib {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arib", arib)?;
            }
            if let Some(ref arib_captions_pid) = self.arib_captions_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AribCaptionsPid", arib_captions_pid)?;
            }
            if let Some(ref arib_captions_pid_control) = self.arib_captions_pid_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AribCaptionsPidControl", arib_captions_pid_control)?;
            }
            if let Some(ref audio_buffer_model) = self.audio_buffer_model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioBufferModel", audio_buffer_model)?;
            }
            if let Some(ref audio_frames_per_pes) = self.audio_frames_per_pes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioFramesPerPes", audio_frames_per_pes)?;
            }
            if let Some(ref audio_pids) = self.audio_pids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioPids", audio_pids)?;
            }
            if let Some(ref audio_stream_type) = self.audio_stream_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioStreamType", audio_stream_type)?;
            }
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref buffer_model) = self.buffer_model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferModel", buffer_model)?;
            }
            if let Some(ref cc_descriptor) = self.cc_descriptor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CcDescriptor", cc_descriptor)?;
            }
            if let Some(ref dvb_nit_settings) = self.dvb_nit_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbNitSettings", dvb_nit_settings)?;
            }
            if let Some(ref dvb_sdt_settings) = self.dvb_sdt_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbSdtSettings", dvb_sdt_settings)?;
            }
            if let Some(ref dvb_sub_pids) = self.dvb_sub_pids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbSubPids", dvb_sub_pids)?;
            }
            if let Some(ref dvb_tdt_settings) = self.dvb_tdt_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbTdtSettings", dvb_tdt_settings)?;
            }
            if let Some(ref dvb_teletext_pid) = self.dvb_teletext_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DvbTeletextPid", dvb_teletext_pid)?;
            }
            if let Some(ref ebif) = self.ebif {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebif", ebif)?;
            }
            if let Some(ref ebp_audio_interval) = self.ebp_audio_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbpAudioInterval", ebp_audio_interval)?;
            }
            if let Some(ref ebp_lookahead_ms) = self.ebp_lookahead_ms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbpLookaheadMs", ebp_lookahead_ms)?;
            }
            if let Some(ref ebp_placement) = self.ebp_placement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbpPlacement", ebp_placement)?;
            }
            if let Some(ref ecm_pid) = self.ecm_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcmPid", ecm_pid)?;
            }
            if let Some(ref es_rate_in_pes) = self.es_rate_in_pes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EsRateInPes", es_rate_in_pes)?;
            }
            if let Some(ref etv_platform_pid) = self.etv_platform_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EtvPlatformPid", etv_platform_pid)?;
            }
            if let Some(ref etv_signal_pid) = self.etv_signal_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EtvSignalPid", etv_signal_pid)?;
            }
            if let Some(ref fragment_time) = self.fragment_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FragmentTime", fragment_time)?;
            }
            if let Some(ref klv) = self.klv {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Klv", klv)?;
            }
            if let Some(ref klv_data_pids) = self.klv_data_pids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KlvDataPids", klv_data_pids)?;
            }
            if let Some(ref nielsen_id3_behavior) = self.nielsen_id3_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NielsenId3Behavior", nielsen_id3_behavior)?;
            }
            if let Some(ref null_packet_bitrate) = self.null_packet_bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NullPacketBitrate", null_packet_bitrate)?;
            }
            if let Some(ref pat_interval) = self.pat_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatInterval", pat_interval)?;
            }
            if let Some(ref pcr_control) = self.pcr_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrControl", pcr_control)?;
            }
            if let Some(ref pcr_period) = self.pcr_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrPeriod", pcr_period)?;
            }
            if let Some(ref pcr_pid) = self.pcr_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrPid", pcr_pid)?;
            }
            if let Some(ref pmt_interval) = self.pmt_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PmtInterval", pmt_interval)?;
            }
            if let Some(ref pmt_pid) = self.pmt_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PmtPid", pmt_pid)?;
            }
            if let Some(ref program_num) = self.program_num {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramNum", program_num)?;
            }
            if let Some(ref rate_mode) = self.rate_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateMode", rate_mode)?;
            }
            if let Some(ref scte27_pids) = self.scte27_pids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte27Pids", scte27_pids)?;
            }
            if let Some(ref scte35_control) = self.scte35_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35Control", scte35_control)?;
            }
            if let Some(ref scte35_pid) = self.scte35_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35Pid", scte35_pid)?;
            }
            if let Some(ref segmentation_markers) = self.segmentation_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentationMarkers", segmentation_markers)?;
            }
            if let Some(ref segmentation_style) = self.segmentation_style {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentationStyle", segmentation_style)?;
            }
            if let Some(ref segmentation_time) = self.segmentation_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentationTime", segmentation_time)?;
            }
            if let Some(ref timed_metadata_behavior) = self.timed_metadata_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataBehavior", timed_metadata_behavior)?;
            }
            if let Some(ref timed_metadata_pid) = self.timed_metadata_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataPid", timed_metadata_pid)?;
            }
            if let Some(ref transport_stream_id) = self.transport_stream_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransportStreamId", transport_stream_id)?;
            }
            if let Some(ref video_pid) = self.video_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoPid", video_pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for M2tsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<M2tsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = M2tsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type M2tsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut absent_input_audio_behavior: Option<::Value<String>> = None;
                    let mut arib: Option<::Value<String>> = None;
                    let mut arib_captions_pid: Option<::Value<String>> = None;
                    let mut arib_captions_pid_control: Option<::Value<String>> = None;
                    let mut audio_buffer_model: Option<::Value<String>> = None;
                    let mut audio_frames_per_pes: Option<::Value<u32>> = None;
                    let mut audio_pids: Option<::Value<String>> = None;
                    let mut audio_stream_type: Option<::Value<String>> = None;
                    let mut bitrate: Option<::Value<u32>> = None;
                    let mut buffer_model: Option<::Value<String>> = None;
                    let mut cc_descriptor: Option<::Value<String>> = None;
                    let mut dvb_nit_settings: Option<::Value<DvbNitSettings>> = None;
                    let mut dvb_sdt_settings: Option<::Value<DvbSdtSettings>> = None;
                    let mut dvb_sub_pids: Option<::Value<String>> = None;
                    let mut dvb_tdt_settings: Option<::Value<DvbTdtSettings>> = None;
                    let mut dvb_teletext_pid: Option<::Value<String>> = None;
                    let mut ebif: Option<::Value<String>> = None;
                    let mut ebp_audio_interval: Option<::Value<String>> = None;
                    let mut ebp_lookahead_ms: Option<::Value<u32>> = None;
                    let mut ebp_placement: Option<::Value<String>> = None;
                    let mut ecm_pid: Option<::Value<String>> = None;
                    let mut es_rate_in_pes: Option<::Value<String>> = None;
                    let mut etv_platform_pid: Option<::Value<String>> = None;
                    let mut etv_signal_pid: Option<::Value<String>> = None;
                    let mut fragment_time: Option<::Value<f64>> = None;
                    let mut klv: Option<::Value<String>> = None;
                    let mut klv_data_pids: Option<::Value<String>> = None;
                    let mut nielsen_id3_behavior: Option<::Value<String>> = None;
                    let mut null_packet_bitrate: Option<::Value<f64>> = None;
                    let mut pat_interval: Option<::Value<u32>> = None;
                    let mut pcr_control: Option<::Value<String>> = None;
                    let mut pcr_period: Option<::Value<u32>> = None;
                    let mut pcr_pid: Option<::Value<String>> = None;
                    let mut pmt_interval: Option<::Value<u32>> = None;
                    let mut pmt_pid: Option<::Value<String>> = None;
                    let mut program_num: Option<::Value<u32>> = None;
                    let mut rate_mode: Option<::Value<String>> = None;
                    let mut scte27_pids: Option<::Value<String>> = None;
                    let mut scte35_control: Option<::Value<String>> = None;
                    let mut scte35_pid: Option<::Value<String>> = None;
                    let mut segmentation_markers: Option<::Value<String>> = None;
                    let mut segmentation_style: Option<::Value<String>> = None;
                    let mut segmentation_time: Option<::Value<f64>> = None;
                    let mut timed_metadata_behavior: Option<::Value<String>> = None;
                    let mut timed_metadata_pid: Option<::Value<String>> = None;
                    let mut transport_stream_id: Option<::Value<u32>> = None;
                    let mut video_pid: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AbsentInputAudioBehavior" => {
                                absent_input_audio_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Arib" => {
                                arib = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AribCaptionsPid" => {
                                arib_captions_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AribCaptionsPidControl" => {
                                arib_captions_pid_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioBufferModel" => {
                                audio_buffer_model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioFramesPerPes" => {
                                audio_frames_per_pes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioPids" => {
                                audio_pids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioStreamType" => {
                                audio_stream_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BufferModel" => {
                                buffer_model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CcDescriptor" => {
                                cc_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbNitSettings" => {
                                dvb_nit_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbSdtSettings" => {
                                dvb_sdt_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbSubPids" => {
                                dvb_sub_pids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbTdtSettings" => {
                                dvb_tdt_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DvbTeletextPid" => {
                                dvb_teletext_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ebif" => {
                                ebif = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbpAudioInterval" => {
                                ebp_audio_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbpLookaheadMs" => {
                                ebp_lookahead_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EbpPlacement" => {
                                ebp_placement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcmPid" => {
                                ecm_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EsRateInPes" => {
                                es_rate_in_pes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EtvPlatformPid" => {
                                etv_platform_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EtvSignalPid" => {
                                etv_signal_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FragmentTime" => {
                                fragment_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Klv" => {
                                klv = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KlvDataPids" => {
                                klv_data_pids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NielsenId3Behavior" => {
                                nielsen_id3_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NullPacketBitrate" => {
                                null_packet_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatInterval" => {
                                pat_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrControl" => {
                                pcr_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrPeriod" => {
                                pcr_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrPid" => {
                                pcr_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PmtInterval" => {
                                pmt_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PmtPid" => {
                                pmt_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramNum" => {
                                program_num = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateMode" => {
                                rate_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte27Pids" => {
                                scte27_pids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte35Control" => {
                                scte35_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte35Pid" => {
                                scte35_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentationMarkers" => {
                                segmentation_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentationStyle" => {
                                segmentation_style = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentationTime" => {
                                segmentation_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataBehavior" => {
                                timed_metadata_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataPid" => {
                                timed_metadata_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransportStreamId" => {
                                transport_stream_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoPid" => {
                                video_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(M2tsSettings {
                        absent_input_audio_behavior: absent_input_audio_behavior,
                        arib: arib,
                        arib_captions_pid: arib_captions_pid,
                        arib_captions_pid_control: arib_captions_pid_control,
                        audio_buffer_model: audio_buffer_model,
                        audio_frames_per_pes: audio_frames_per_pes,
                        audio_pids: audio_pids,
                        audio_stream_type: audio_stream_type,
                        bitrate: bitrate,
                        buffer_model: buffer_model,
                        cc_descriptor: cc_descriptor,
                        dvb_nit_settings: dvb_nit_settings,
                        dvb_sdt_settings: dvb_sdt_settings,
                        dvb_sub_pids: dvb_sub_pids,
                        dvb_tdt_settings: dvb_tdt_settings,
                        dvb_teletext_pid: dvb_teletext_pid,
                        ebif: ebif,
                        ebp_audio_interval: ebp_audio_interval,
                        ebp_lookahead_ms: ebp_lookahead_ms,
                        ebp_placement: ebp_placement,
                        ecm_pid: ecm_pid,
                        es_rate_in_pes: es_rate_in_pes,
                        etv_platform_pid: etv_platform_pid,
                        etv_signal_pid: etv_signal_pid,
                        fragment_time: fragment_time,
                        klv: klv,
                        klv_data_pids: klv_data_pids,
                        nielsen_id3_behavior: nielsen_id3_behavior,
                        null_packet_bitrate: null_packet_bitrate,
                        pat_interval: pat_interval,
                        pcr_control: pcr_control,
                        pcr_period: pcr_period,
                        pcr_pid: pcr_pid,
                        pmt_interval: pmt_interval,
                        pmt_pid: pmt_pid,
                        program_num: program_num,
                        rate_mode: rate_mode,
                        scte27_pids: scte27_pids,
                        scte35_control: scte35_control,
                        scte35_pid: scte35_pid,
                        segmentation_markers: segmentation_markers,
                        segmentation_style: segmentation_style,
                        segmentation_time: segmentation_time,
                        timed_metadata_behavior: timed_metadata_behavior,
                        timed_metadata_pid: timed_metadata_pid,
                        transport_stream_id: transport_stream_id,
                        video_pid: video_pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.M3u8Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html) property type.
    #[derive(Debug, Default)]
    pub struct M3u8Settings {
        /// Property [`AudioFramesPerPes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-audioframesperpes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_frames_per_pes: Option<::Value<u32>>,
        /// Property [`AudioPids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-audiopids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_pids: Option<::Value<String>>,
        /// Property [`EcmPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-ecmpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecm_pid: Option<::Value<String>>,
        /// Property [`NielsenId3Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-nielsenid3behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nielsen_id3_behavior: Option<::Value<String>>,
        /// Property [`PatInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-patinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pat_interval: Option<::Value<u32>>,
        /// Property [`PcrControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-pcrcontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_control: Option<::Value<String>>,
        /// Property [`PcrPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-pcrperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_period: Option<::Value<u32>>,
        /// Property [`PcrPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-pcrpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pcr_pid: Option<::Value<String>>,
        /// Property [`PmtInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-pmtinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pmt_interval: Option<::Value<u32>>,
        /// Property [`PmtPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-pmtpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pmt_pid: Option<::Value<String>>,
        /// Property [`ProgramNum`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-programnum).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_num: Option<::Value<u32>>,
        /// Property [`Scte35Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-scte35behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_behavior: Option<::Value<String>>,
        /// Property [`Scte35Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-scte35pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scte35_pid: Option<::Value<String>>,
        /// Property [`TimedMetadataBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-timedmetadatabehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_behavior: Option<::Value<String>>,
        /// Property [`TimedMetadataPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-timedmetadatapid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_pid: Option<::Value<String>>,
        /// Property [`TransportStreamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-transportstreamid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transport_stream_id: Option<::Value<u32>>,
        /// Property [`VideoPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-m3u8settings.html#cfn-medialive-channel-m3u8settings-videopid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_pid: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for M3u8Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_frames_per_pes) = self.audio_frames_per_pes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioFramesPerPes", audio_frames_per_pes)?;
            }
            if let Some(ref audio_pids) = self.audio_pids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioPids", audio_pids)?;
            }
            if let Some(ref ecm_pid) = self.ecm_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcmPid", ecm_pid)?;
            }
            if let Some(ref nielsen_id3_behavior) = self.nielsen_id3_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NielsenId3Behavior", nielsen_id3_behavior)?;
            }
            if let Some(ref pat_interval) = self.pat_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatInterval", pat_interval)?;
            }
            if let Some(ref pcr_control) = self.pcr_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrControl", pcr_control)?;
            }
            if let Some(ref pcr_period) = self.pcr_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrPeriod", pcr_period)?;
            }
            if let Some(ref pcr_pid) = self.pcr_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PcrPid", pcr_pid)?;
            }
            if let Some(ref pmt_interval) = self.pmt_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PmtInterval", pmt_interval)?;
            }
            if let Some(ref pmt_pid) = self.pmt_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PmtPid", pmt_pid)?;
            }
            if let Some(ref program_num) = self.program_num {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramNum", program_num)?;
            }
            if let Some(ref scte35_behavior) = self.scte35_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35Behavior", scte35_behavior)?;
            }
            if let Some(ref scte35_pid) = self.scte35_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scte35Pid", scte35_pid)?;
            }
            if let Some(ref timed_metadata_behavior) = self.timed_metadata_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataBehavior", timed_metadata_behavior)?;
            }
            if let Some(ref timed_metadata_pid) = self.timed_metadata_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataPid", timed_metadata_pid)?;
            }
            if let Some(ref transport_stream_id) = self.transport_stream_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransportStreamId", transport_stream_id)?;
            }
            if let Some(ref video_pid) = self.video_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoPid", video_pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for M3u8Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<M3u8Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = M3u8Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type M3u8Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_frames_per_pes: Option<::Value<u32>> = None;
                    let mut audio_pids: Option<::Value<String>> = None;
                    let mut ecm_pid: Option<::Value<String>> = None;
                    let mut nielsen_id3_behavior: Option<::Value<String>> = None;
                    let mut pat_interval: Option<::Value<u32>> = None;
                    let mut pcr_control: Option<::Value<String>> = None;
                    let mut pcr_period: Option<::Value<u32>> = None;
                    let mut pcr_pid: Option<::Value<String>> = None;
                    let mut pmt_interval: Option<::Value<u32>> = None;
                    let mut pmt_pid: Option<::Value<String>> = None;
                    let mut program_num: Option<::Value<u32>> = None;
                    let mut scte35_behavior: Option<::Value<String>> = None;
                    let mut scte35_pid: Option<::Value<String>> = None;
                    let mut timed_metadata_behavior: Option<::Value<String>> = None;
                    let mut timed_metadata_pid: Option<::Value<String>> = None;
                    let mut transport_stream_id: Option<::Value<u32>> = None;
                    let mut video_pid: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioFramesPerPes" => {
                                audio_frames_per_pes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioPids" => {
                                audio_pids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcmPid" => {
                                ecm_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NielsenId3Behavior" => {
                                nielsen_id3_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatInterval" => {
                                pat_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrControl" => {
                                pcr_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrPeriod" => {
                                pcr_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PcrPid" => {
                                pcr_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PmtInterval" => {
                                pmt_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PmtPid" => {
                                pmt_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramNum" => {
                                program_num = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte35Behavior" => {
                                scte35_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scte35Pid" => {
                                scte35_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataBehavior" => {
                                timed_metadata_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataPid" => {
                                timed_metadata_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransportStreamId" => {
                                transport_stream_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoPid" => {
                                video_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(M3u8Settings {
                        audio_frames_per_pes: audio_frames_per_pes,
                        audio_pids: audio_pids,
                        ecm_pid: ecm_pid,
                        nielsen_id3_behavior: nielsen_id3_behavior,
                        pat_interval: pat_interval,
                        pcr_control: pcr_control,
                        pcr_period: pcr_period,
                        pcr_pid: pcr_pid,
                        pmt_interval: pmt_interval,
                        pmt_pid: pmt_pid,
                        program_num: program_num,
                        scte35_behavior: scte35_behavior,
                        scte35_pid: scte35_pid,
                        timed_metadata_behavior: timed_metadata_behavior,
                        timed_metadata_pid: timed_metadata_pid,
                        transport_stream_id: transport_stream_id,
                        video_pid: video_pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MediaPackageGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackagegroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MediaPackageGroupSettings {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackagegroupsettings.html#cfn-medialive-channel-mediapackagegroupsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
    }

    impl ::codec::SerializeValue for MediaPackageGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MediaPackageGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MediaPackageGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MediaPackageGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MediaPackageGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<OutputLocationRef>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MediaPackageGroupSettings {
                        destination: destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MediaPackageOutputDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackageoutputdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MediaPackageOutputDestinationSettings {
        /// Property [`ChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackageoutputdestinationsettings.html#cfn-medialive-channel-mediapackageoutputdestinationsettings-channelid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MediaPackageOutputDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel_id) = self.channel_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelId", channel_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MediaPackageOutputDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MediaPackageOutputDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MediaPackageOutputDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MediaPackageOutputDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChannelId" => {
                                channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MediaPackageOutputDestinationSettings {
                        channel_id: channel_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MediaPackageOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mediapackageoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MediaPackageOutputSettings {
    }

    impl ::codec::SerializeValue for MediaPackageOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MediaPackageOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MediaPackageOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MediaPackageOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MediaPackageOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(MediaPackageOutputSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MotionGraphicsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MotionGraphicsConfiguration {
        /// Property [`MotionGraphicsInsertion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicsconfiguration.html#cfn-medialive-channel-motiongraphicsconfiguration-motiongraphicsinsertion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub motion_graphics_insertion: Option<::Value<String>>,
        /// Property [`MotionGraphicsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicsconfiguration.html#cfn-medialive-channel-motiongraphicsconfiguration-motiongraphicssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub motion_graphics_settings: Option<::Value<MotionGraphicsSettings>>,
    }

    impl ::codec::SerializeValue for MotionGraphicsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref motion_graphics_insertion) = self.motion_graphics_insertion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MotionGraphicsInsertion", motion_graphics_insertion)?;
            }
            if let Some(ref motion_graphics_settings) = self.motion_graphics_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MotionGraphicsSettings", motion_graphics_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MotionGraphicsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MotionGraphicsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MotionGraphicsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MotionGraphicsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut motion_graphics_insertion: Option<::Value<String>> = None;
                    let mut motion_graphics_settings: Option<::Value<MotionGraphicsSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MotionGraphicsInsertion" => {
                                motion_graphics_insertion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MotionGraphicsSettings" => {
                                motion_graphics_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MotionGraphicsConfiguration {
                        motion_graphics_insertion: motion_graphics_insertion,
                        motion_graphics_settings: motion_graphics_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MotionGraphicsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MotionGraphicsSettings {
        /// Property [`HtmlMotionGraphicsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-motiongraphicssettings.html#cfn-medialive-channel-motiongraphicssettings-htmlmotiongraphicssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub html_motion_graphics_settings: Option<::Value<HtmlMotionGraphicsSettings>>,
    }

    impl ::codec::SerializeValue for MotionGraphicsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref html_motion_graphics_settings) = self.html_motion_graphics_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlMotionGraphicsSettings", html_motion_graphics_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MotionGraphicsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MotionGraphicsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MotionGraphicsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MotionGraphicsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut html_motion_graphics_settings: Option<::Value<HtmlMotionGraphicsSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HtmlMotionGraphicsSettings" => {
                                html_motion_graphics_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MotionGraphicsSettings {
                        html_motion_graphics_settings: html_motion_graphics_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Mp2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mp2settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Mp2Settings {
        /// Property [`Bitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mp2settings.html#cfn-medialive-channel-mp2settings-bitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bitrate: Option<::Value<f64>>,
        /// Property [`CodingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mp2settings.html#cfn-medialive-channel-mp2settings-codingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coding_mode: Option<::Value<String>>,
        /// Property [`SampleRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mp2settings.html#cfn-medialive-channel-mp2settings-samplerate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_rate: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for Mp2Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bitrate) = self.bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bitrate", bitrate)?;
            }
            if let Some(ref coding_mode) = self.coding_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodingMode", coding_mode)?;
            }
            if let Some(ref sample_rate) = self.sample_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleRate", sample_rate)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Mp2Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Mp2Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Mp2Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Mp2Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bitrate: Option<::Value<f64>> = None;
                    let mut coding_mode: Option<::Value<String>> = None;
                    let mut sample_rate: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bitrate" => {
                                bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodingMode" => {
                                coding_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleRate" => {
                                sample_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Mp2Settings {
                        bitrate: bitrate,
                        coding_mode: coding_mode,
                        sample_rate: sample_rate,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Mpeg2FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2filtersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Mpeg2FilterSettings {
        /// Property [`TemporalFilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2filtersettings.html#cfn-medialive-channel-mpeg2filtersettings-temporalfiltersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temporal_filter_settings: Option<::Value<TemporalFilterSettings>>,
    }

    impl ::codec::SerializeValue for Mpeg2FilterSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref temporal_filter_settings) = self.temporal_filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemporalFilterSettings", temporal_filter_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Mpeg2FilterSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Mpeg2FilterSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Mpeg2FilterSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Mpeg2FilterSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut temporal_filter_settings: Option<::Value<TemporalFilterSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemporalFilterSettings" => {
                                temporal_filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Mpeg2FilterSettings {
                        temporal_filter_settings: temporal_filter_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Mpeg2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Mpeg2Settings {
        /// Property [`AdaptiveQuantization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-adaptivequantization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adaptive_quantization: Option<::Value<String>>,
        /// Property [`AfdSignaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-afdsignaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub afd_signaling: Option<::Value<String>>,
        /// Property [`ColorMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-colormetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_metadata: Option<::Value<String>>,
        /// Property [`ColorSpace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-colorspace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space: Option<::Value<String>>,
        /// Property [`DisplayAspectRatio`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-displayaspectratio).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_aspect_ratio: Option<::Value<String>>,
        /// Property [`FilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-filtersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_settings: Option<::Value<Mpeg2FilterSettings>>,
        /// Property [`FixedAfd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-fixedafd).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_afd: Option<::Value<String>>,
        /// Property [`FramerateDenominator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-frameratedenominator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_denominator: Option<::Value<u32>>,
        /// Property [`FramerateNumerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-frameratenumerator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framerate_numerator: Option<::Value<u32>>,
        /// Property [`GopClosedCadence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-gopclosedcadence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_closed_cadence: Option<::Value<u32>>,
        /// Property [`GopNumBFrames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-gopnumbframes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_num_b_frames: Option<::Value<u32>>,
        /// Property [`GopSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-gopsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size: Option<::Value<f64>>,
        /// Property [`GopSizeUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-gopsizeunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gop_size_units: Option<::Value<String>>,
        /// Property [`ScanType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-scantype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scan_type: Option<::Value<String>>,
        /// Property [`SubgopLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-subgoplength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subgop_length: Option<::Value<String>>,
        /// Property [`TimecodeInsertion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mpeg2settings.html#cfn-medialive-channel-mpeg2settings-timecodeinsertion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timecode_insertion: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Mpeg2Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adaptive_quantization) = self.adaptive_quantization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdaptiveQuantization", adaptive_quantization)?;
            }
            if let Some(ref afd_signaling) = self.afd_signaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfdSignaling", afd_signaling)?;
            }
            if let Some(ref color_metadata) = self.color_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorMetadata", color_metadata)?;
            }
            if let Some(ref color_space) = self.color_space {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpace", color_space)?;
            }
            if let Some(ref display_aspect_ratio) = self.display_aspect_ratio {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayAspectRatio", display_aspect_ratio)?;
            }
            if let Some(ref filter_settings) = self.filter_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterSettings", filter_settings)?;
            }
            if let Some(ref fixed_afd) = self.fixed_afd {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedAfd", fixed_afd)?;
            }
            if let Some(ref framerate_denominator) = self.framerate_denominator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateDenominator", framerate_denominator)?;
            }
            if let Some(ref framerate_numerator) = self.framerate_numerator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FramerateNumerator", framerate_numerator)?;
            }
            if let Some(ref gop_closed_cadence) = self.gop_closed_cadence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopClosedCadence", gop_closed_cadence)?;
            }
            if let Some(ref gop_num_b_frames) = self.gop_num_b_frames {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopNumBFrames", gop_num_b_frames)?;
            }
            if let Some(ref gop_size) = self.gop_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSize", gop_size)?;
            }
            if let Some(ref gop_size_units) = self.gop_size_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GopSizeUnits", gop_size_units)?;
            }
            if let Some(ref scan_type) = self.scan_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanType", scan_type)?;
            }
            if let Some(ref subgop_length) = self.subgop_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubgopLength", subgop_length)?;
            }
            if let Some(ref timecode_insertion) = self.timecode_insertion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimecodeInsertion", timecode_insertion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Mpeg2Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Mpeg2Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Mpeg2Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Mpeg2Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adaptive_quantization: Option<::Value<String>> = None;
                    let mut afd_signaling: Option<::Value<String>> = None;
                    let mut color_metadata: Option<::Value<String>> = None;
                    let mut color_space: Option<::Value<String>> = None;
                    let mut display_aspect_ratio: Option<::Value<String>> = None;
                    let mut filter_settings: Option<::Value<Mpeg2FilterSettings>> = None;
                    let mut fixed_afd: Option<::Value<String>> = None;
                    let mut framerate_denominator: Option<::Value<u32>> = None;
                    let mut framerate_numerator: Option<::Value<u32>> = None;
                    let mut gop_closed_cadence: Option<::Value<u32>> = None;
                    let mut gop_num_b_frames: Option<::Value<u32>> = None;
                    let mut gop_size: Option<::Value<f64>> = None;
                    let mut gop_size_units: Option<::Value<String>> = None;
                    let mut scan_type: Option<::Value<String>> = None;
                    let mut subgop_length: Option<::Value<String>> = None;
                    let mut timecode_insertion: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdaptiveQuantization" => {
                                adaptive_quantization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AfdSignaling" => {
                                afd_signaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorMetadata" => {
                                color_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorSpace" => {
                                color_space = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisplayAspectRatio" => {
                                display_aspect_ratio = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterSettings" => {
                                filter_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedAfd" => {
                                fixed_afd = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateDenominator" => {
                                framerate_denominator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FramerateNumerator" => {
                                framerate_numerator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopClosedCadence" => {
                                gop_closed_cadence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopNumBFrames" => {
                                gop_num_b_frames = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSize" => {
                                gop_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GopSizeUnits" => {
                                gop_size_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScanType" => {
                                scan_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubgopLength" => {
                                subgop_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimecodeInsertion" => {
                                timecode_insertion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Mpeg2Settings {
                        adaptive_quantization: adaptive_quantization,
                        afd_signaling: afd_signaling,
                        color_metadata: color_metadata,
                        color_space: color_space,
                        display_aspect_ratio: display_aspect_ratio,
                        filter_settings: filter_settings,
                        fixed_afd: fixed_afd,
                        framerate_denominator: framerate_denominator,
                        framerate_numerator: framerate_numerator,
                        gop_closed_cadence: gop_closed_cadence,
                        gop_num_b_frames: gop_num_b_frames,
                        gop_size: gop_size,
                        gop_size_units: gop_size_units,
                        scan_type: scan_type,
                        subgop_length: subgop_length,
                        timecode_insertion: timecode_insertion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MsSmoothGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MsSmoothGroupSettings {
        /// Property [`AcquisitionPointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-acquisitionpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acquisition_point_id: Option<::Value<String>>,
        /// Property [`AudioOnlyTimecodeControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-audioonlytimecodecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_only_timecode_control: Option<::Value<String>>,
        /// Property [`CertificateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-certificatemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_mode: Option<::Value<String>>,
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`EventId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-eventid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_id: Option<::Value<String>>,
        /// Property [`EventIdMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-eventidmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_id_mode: Option<::Value<String>>,
        /// Property [`EventStopBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-eventstopbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_stop_behavior: Option<::Value<String>>,
        /// Property [`FilecacheDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-filecacheduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filecache_duration: Option<::Value<u32>>,
        /// Property [`FragmentLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-fragmentlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fragment_length: Option<::Value<u32>>,
        /// Property [`InputLossAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-inputlossaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_action: Option<::Value<String>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
        /// Property [`SegmentationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-segmentationmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segmentation_mode: Option<::Value<String>>,
        /// Property [`SendDelayMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-senddelayms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub send_delay_ms: Option<::Value<u32>>,
        /// Property [`SparseTrackType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-sparsetracktype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sparse_track_type: Option<::Value<String>>,
        /// Property [`StreamManifestBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-streammanifestbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_manifest_behavior: Option<::Value<String>>,
        /// Property [`TimestampOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-timestampoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_offset: Option<::Value<String>>,
        /// Property [`TimestampOffsetMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothgroupsettings.html#cfn-medialive-channel-mssmoothgroupsettings-timestampoffsetmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_offset_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MsSmoothGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acquisition_point_id) = self.acquisition_point_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcquisitionPointId", acquisition_point_id)?;
            }
            if let Some(ref audio_only_timecode_control) = self.audio_only_timecode_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioOnlyTimecodeControl", audio_only_timecode_control)?;
            }
            if let Some(ref certificate_mode) = self.certificate_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateMode", certificate_mode)?;
            }
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref event_id) = self.event_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventId", event_id)?;
            }
            if let Some(ref event_id_mode) = self.event_id_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventIdMode", event_id_mode)?;
            }
            if let Some(ref event_stop_behavior) = self.event_stop_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventStopBehavior", event_stop_behavior)?;
            }
            if let Some(ref filecache_duration) = self.filecache_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilecacheDuration", filecache_duration)?;
            }
            if let Some(ref fragment_length) = self.fragment_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FragmentLength", fragment_length)?;
            }
            if let Some(ref input_loss_action) = self.input_loss_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossAction", input_loss_action)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            if let Some(ref segmentation_mode) = self.segmentation_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentationMode", segmentation_mode)?;
            }
            if let Some(ref send_delay_ms) = self.send_delay_ms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendDelayMs", send_delay_ms)?;
            }
            if let Some(ref sparse_track_type) = self.sparse_track_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SparseTrackType", sparse_track_type)?;
            }
            if let Some(ref stream_manifest_behavior) = self.stream_manifest_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamManifestBehavior", stream_manifest_behavior)?;
            }
            if let Some(ref timestamp_offset) = self.timestamp_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampOffset", timestamp_offset)?;
            }
            if let Some(ref timestamp_offset_mode) = self.timestamp_offset_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampOffsetMode", timestamp_offset_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MsSmoothGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MsSmoothGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MsSmoothGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MsSmoothGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acquisition_point_id: Option<::Value<String>> = None;
                    let mut audio_only_timecode_control: Option<::Value<String>> = None;
                    let mut certificate_mode: Option<::Value<String>> = None;
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut event_id: Option<::Value<String>> = None;
                    let mut event_id_mode: Option<::Value<String>> = None;
                    let mut event_stop_behavior: Option<::Value<String>> = None;
                    let mut filecache_duration: Option<::Value<u32>> = None;
                    let mut fragment_length: Option<::Value<u32>> = None;
                    let mut input_loss_action: Option<::Value<String>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;
                    let mut segmentation_mode: Option<::Value<String>> = None;
                    let mut send_delay_ms: Option<::Value<u32>> = None;
                    let mut sparse_track_type: Option<::Value<String>> = None;
                    let mut stream_manifest_behavior: Option<::Value<String>> = None;
                    let mut timestamp_offset: Option<::Value<String>> = None;
                    let mut timestamp_offset_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcquisitionPointId" => {
                                acquisition_point_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioOnlyTimecodeControl" => {
                                audio_only_timecode_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CertificateMode" => {
                                certificate_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventId" => {
                                event_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventIdMode" => {
                                event_id_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventStopBehavior" => {
                                event_stop_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilecacheDuration" => {
                                filecache_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FragmentLength" => {
                                fragment_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossAction" => {
                                input_loss_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentationMode" => {
                                segmentation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SendDelayMs" => {
                                send_delay_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SparseTrackType" => {
                                sparse_track_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamManifestBehavior" => {
                                stream_manifest_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampOffset" => {
                                timestamp_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampOffsetMode" => {
                                timestamp_offset_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MsSmoothGroupSettings {
                        acquisition_point_id: acquisition_point_id,
                        audio_only_timecode_control: audio_only_timecode_control,
                        certificate_mode: certificate_mode,
                        connection_retry_interval: connection_retry_interval,
                        destination: destination,
                        event_id: event_id,
                        event_id_mode: event_id_mode,
                        event_stop_behavior: event_stop_behavior,
                        filecache_duration: filecache_duration,
                        fragment_length: fragment_length,
                        input_loss_action: input_loss_action,
                        num_retries: num_retries,
                        restart_delay: restart_delay,
                        segmentation_mode: segmentation_mode,
                        send_delay_ms: send_delay_ms,
                        sparse_track_type: sparse_track_type,
                        stream_manifest_behavior: stream_manifest_behavior,
                        timestamp_offset: timestamp_offset,
                        timestamp_offset_mode: timestamp_offset_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MsSmoothOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MsSmoothOutputSettings {
        /// Property [`H265PackagingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothoutputsettings.html#cfn-medialive-channel-mssmoothoutputsettings-h265packagingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub h265_packaging_type: Option<::Value<String>>,
        /// Property [`NameModifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-mssmoothoutputsettings.html#cfn-medialive-channel-mssmoothoutputsettings-namemodifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name_modifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MsSmoothOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref h265_packaging_type) = self.h265_packaging_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "H265PackagingType", h265_packaging_type)?;
            }
            if let Some(ref name_modifier) = self.name_modifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameModifier", name_modifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MsSmoothOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MsSmoothOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MsSmoothOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MsSmoothOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut h265_packaging_type: Option<::Value<String>> = None;
                    let mut name_modifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "H265PackagingType" => {
                                h265_packaging_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NameModifier" => {
                                name_modifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MsSmoothOutputSettings {
                        h265_packaging_type: h265_packaging_type,
                        name_modifier: name_modifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MultiplexGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MultiplexGroupSettings {
    }

    impl ::codec::SerializeValue for MultiplexGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MultiplexGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MultiplexGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MultiplexGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MultiplexGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(MultiplexGroupSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MultiplexOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MultiplexOutputSettings {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexoutputsettings.html#cfn-medialive-channel-multiplexoutputsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
    }

    impl ::codec::SerializeValue for MultiplexOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MultiplexOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MultiplexOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MultiplexOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MultiplexOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<OutputLocationRef>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MultiplexOutputSettings {
                        destination: destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.MultiplexProgramChannelDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexprogramchanneldestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MultiplexProgramChannelDestinationSettings {
        /// Property [`MultiplexId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexprogramchanneldestinationsettings.html#cfn-medialive-channel-multiplexprogramchanneldestinationsettings-multiplexid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multiplex_id: Option<::Value<String>>,
        /// Property [`ProgramName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-multiplexprogramchanneldestinationsettings.html#cfn-medialive-channel-multiplexprogramchanneldestinationsettings-programname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MultiplexProgramChannelDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref multiplex_id) = self.multiplex_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiplexId", multiplex_id)?;
            }
            if let Some(ref program_name) = self.program_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramName", program_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MultiplexProgramChannelDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MultiplexProgramChannelDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MultiplexProgramChannelDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MultiplexProgramChannelDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut multiplex_id: Option<::Value<String>> = None;
                    let mut program_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MultiplexId" => {
                                multiplex_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgramName" => {
                                program_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MultiplexProgramChannelDestinationSettings {
                        multiplex_id: multiplex_id,
                        program_name: program_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.NetworkInputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-networkinputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkInputSettings {
        /// Property [`HlsInputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-networkinputsettings.html#cfn-medialive-channel-networkinputsettings-hlsinputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_input_settings: Option<::Value<HlsInputSettings>>,
        /// Property [`ServerValidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-networkinputsettings.html#cfn-medialive-channel-networkinputsettings-servervalidation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_validation: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NetworkInputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hls_input_settings) = self.hls_input_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsInputSettings", hls_input_settings)?;
            }
            if let Some(ref server_validation) = self.server_validation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerValidation", server_validation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkInputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkInputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkInputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hls_input_settings: Option<::Value<HlsInputSettings>> = None;
                    let mut server_validation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HlsInputSettings" => {
                                hls_input_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerValidation" => {
                                server_validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkInputSettings {
                        hls_input_settings: hls_input_settings,
                        server_validation: server_validation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.NielsenConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsenconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NielsenConfiguration {
        /// Property [`DistributorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsenconfiguration.html#cfn-medialive-channel-nielsenconfiguration-distributorid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub distributor_id: Option<::Value<String>>,
        /// Property [`NielsenPcmToId3Tagging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-nielsenconfiguration.html#cfn-medialive-channel-nielsenconfiguration-nielsenpcmtoid3tagging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nielsen_pcm_to_id3_tagging: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NielsenConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref distributor_id) = self.distributor_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributorId", distributor_id)?;
            }
            if let Some(ref nielsen_pcm_to_id3_tagging) = self.nielsen_pcm_to_id3_tagging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NielsenPcmToId3Tagging", nielsen_pcm_to_id3_tagging)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NielsenConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NielsenConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NielsenConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NielsenConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut distributor_id: Option<::Value<String>> = None;
                    let mut nielsen_pcm_to_id3_tagging: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DistributorId" => {
                                distributor_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NielsenPcmToId3Tagging" => {
                                nielsen_pcm_to_id3_tagging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NielsenConfiguration {
                        distributor_id: distributor_id,
                        nielsen_pcm_to_id3_tagging: nielsen_pcm_to_id3_tagging,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html) property type.
    #[derive(Debug, Default)]
    pub struct Output {
        /// Property [`AudioDescriptionNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html#cfn-medialive-channel-output-audiodescriptionnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_description_names: Option<::ValueList<String>>,
        /// Property [`CaptionDescriptionNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html#cfn-medialive-channel-output-captiondescriptionnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_description_names: Option<::ValueList<String>>,
        /// Property [`OutputName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html#cfn-medialive-channel-output-outputname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_name: Option<::Value<String>>,
        /// Property [`OutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html#cfn-medialive-channel-output-outputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_settings: Option<::Value<OutputSettings>>,
        /// Property [`VideoDescriptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-output.html#cfn-medialive-channel-output-videodescriptionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_description_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_description_names) = self.audio_description_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioDescriptionNames", audio_description_names)?;
            }
            if let Some(ref caption_description_names) = self.caption_description_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionDescriptionNames", caption_description_names)?;
            }
            if let Some(ref output_name) = self.output_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputName", output_name)?;
            }
            if let Some(ref output_settings) = self.output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSettings", output_settings)?;
            }
            if let Some(ref video_description_name) = self.video_description_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoDescriptionName", video_description_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_description_names: Option<::ValueList<String>> = None;
                    let mut caption_description_names: Option<::ValueList<String>> = None;
                    let mut output_name: Option<::Value<String>> = None;
                    let mut output_settings: Option<::Value<OutputSettings>> = None;
                    let mut video_description_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioDescriptionNames" => {
                                audio_description_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionDescriptionNames" => {
                                caption_description_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputName" => {
                                output_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputSettings" => {
                                output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoDescriptionName" => {
                                video_description_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Output {
                        audio_description_names: audio_description_names,
                        caption_description_names: caption_description_names,
                        output_name: output_name,
                        output_settings: output_settings,
                        video_description_name: video_description_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputDestination {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html#cfn-medialive-channel-outputdestination-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`MediaPackageSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html#cfn-medialive-channel-outputdestination-mediapackagesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_package_settings: Option<::ValueList<MediaPackageOutputDestinationSettings>>,
        /// Property [`MultiplexSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html#cfn-medialive-channel-outputdestination-multiplexsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multiplex_settings: Option<::Value<MultiplexProgramChannelDestinationSettings>>,
        /// Property [`Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestination.html#cfn-medialive-channel-outputdestination-settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub settings: Option<::ValueList<OutputDestinationSettings>>,
    }

    impl ::codec::SerializeValue for OutputDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref media_package_settings) = self.media_package_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaPackageSettings", media_package_settings)?;
            }
            if let Some(ref multiplex_settings) = self.multiplex_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiplexSettings", multiplex_settings)?;
            }
            if let Some(ref settings) = self.settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Settings", settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut media_package_settings: Option<::ValueList<MediaPackageOutputDestinationSettings>> = None;
                    let mut multiplex_settings: Option<::Value<MultiplexProgramChannelDestinationSettings>> = None;
                    let mut settings: Option<::ValueList<OutputDestinationSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaPackageSettings" => {
                                media_package_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiplexSettings" => {
                                multiplex_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Settings" => {
                                settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputDestination {
                        id: id,
                        media_package_settings: media_package_settings,
                        multiplex_settings: multiplex_settings,
                        settings: settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputDestinationSettings {
        /// Property [`PasswordParam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html#cfn-medialive-channel-outputdestinationsettings-passwordparam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_param: Option<::Value<String>>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html#cfn-medialive-channel-outputdestinationsettings-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html#cfn-medialive-channel-outputdestinationsettings-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputdestinationsettings.html#cfn-medialive-channel-outputdestinationsettings-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutputDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password_param) = self.password_param {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordParam", password_param)?;
            }
            if let Some(ref stream_name) = self.stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", stream_name)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password_param: Option<::Value<String>> = None;
                    let mut stream_name: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordParam" => {
                                password_param = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputDestinationSettings {
                        password_param: password_param,
                        stream_name: stream_name,
                        url: url,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroup.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputGroup {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroup.html#cfn-medialive-channel-outputgroup-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`OutputGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroup.html#cfn-medialive-channel-outputgroup-outputgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_group_settings: Option<::Value<OutputGroupSettings>>,
        /// Property [`Outputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroup.html#cfn-medialive-channel-outputgroup-outputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outputs: Option<::ValueList<Output>>,
    }

    impl ::codec::SerializeValue for OutputGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref output_group_settings) = self.output_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputGroupSettings", output_group_settings)?;
            }
            if let Some(ref outputs) = self.outputs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Outputs", outputs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut output_group_settings: Option<::Value<OutputGroupSettings>> = None;
                    let mut outputs: Option<::ValueList<Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputGroupSettings" => {
                                output_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Outputs" => {
                                outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputGroup {
                        name: name,
                        output_group_settings: output_group_settings,
                        outputs: outputs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputGroupSettings {
        /// Property [`ArchiveGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-archivegroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archive_group_settings: Option<::Value<ArchiveGroupSettings>>,
        /// Property [`FrameCaptureGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-framecapturegroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_group_settings: Option<::Value<FrameCaptureGroupSettings>>,
        /// Property [`HlsGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-hlsgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_group_settings: Option<::Value<HlsGroupSettings>>,
        /// Property [`MediaPackageGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-mediapackagegroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_package_group_settings: Option<::Value<MediaPackageGroupSettings>>,
        /// Property [`MsSmoothGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-mssmoothgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ms_smooth_group_settings: Option<::Value<MsSmoothGroupSettings>>,
        /// Property [`MultiplexGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-multiplexgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multiplex_group_settings: Option<::Value<MultiplexGroupSettings>>,
        /// Property [`RtmpGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-rtmpgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rtmp_group_settings: Option<::Value<RtmpGroupSettings>>,
        /// Property [`UdpGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputgroupsettings.html#cfn-medialive-channel-outputgroupsettings-udpgroupsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub udp_group_settings: Option<::Value<UdpGroupSettings>>,
    }

    impl ::codec::SerializeValue for OutputGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref archive_group_settings) = self.archive_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveGroupSettings", archive_group_settings)?;
            }
            if let Some(ref frame_capture_group_settings) = self.frame_capture_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureGroupSettings", frame_capture_group_settings)?;
            }
            if let Some(ref hls_group_settings) = self.hls_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsGroupSettings", hls_group_settings)?;
            }
            if let Some(ref media_package_group_settings) = self.media_package_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaPackageGroupSettings", media_package_group_settings)?;
            }
            if let Some(ref ms_smooth_group_settings) = self.ms_smooth_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MsSmoothGroupSettings", ms_smooth_group_settings)?;
            }
            if let Some(ref multiplex_group_settings) = self.multiplex_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiplexGroupSettings", multiplex_group_settings)?;
            }
            if let Some(ref rtmp_group_settings) = self.rtmp_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RtmpGroupSettings", rtmp_group_settings)?;
            }
            if let Some(ref udp_group_settings) = self.udp_group_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UdpGroupSettings", udp_group_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut archive_group_settings: Option<::Value<ArchiveGroupSettings>> = None;
                    let mut frame_capture_group_settings: Option<::Value<FrameCaptureGroupSettings>> = None;
                    let mut hls_group_settings: Option<::Value<HlsGroupSettings>> = None;
                    let mut media_package_group_settings: Option<::Value<MediaPackageGroupSettings>> = None;
                    let mut ms_smooth_group_settings: Option<::Value<MsSmoothGroupSettings>> = None;
                    let mut multiplex_group_settings: Option<::Value<MultiplexGroupSettings>> = None;
                    let mut rtmp_group_settings: Option<::Value<RtmpGroupSettings>> = None;
                    let mut udp_group_settings: Option<::Value<UdpGroupSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArchiveGroupSettings" => {
                                archive_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameCaptureGroupSettings" => {
                                frame_capture_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsGroupSettings" => {
                                hls_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaPackageGroupSettings" => {
                                media_package_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MsSmoothGroupSettings" => {
                                ms_smooth_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiplexGroupSettings" => {
                                multiplex_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RtmpGroupSettings" => {
                                rtmp_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UdpGroupSettings" => {
                                udp_group_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputGroupSettings {
                        archive_group_settings: archive_group_settings,
                        frame_capture_group_settings: frame_capture_group_settings,
                        hls_group_settings: hls_group_settings,
                        media_package_group_settings: media_package_group_settings,
                        ms_smooth_group_settings: ms_smooth_group_settings,
                        multiplex_group_settings: multiplex_group_settings,
                        rtmp_group_settings: rtmp_group_settings,
                        udp_group_settings: udp_group_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputLocationRef`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputlocationref.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputLocationRef {
        /// Property [`DestinationRefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputlocationref.html#cfn-medialive-channel-outputlocationref-destinationrefid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_ref_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutputLocationRef {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_ref_id) = self.destination_ref_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationRefId", destination_ref_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputLocationRef {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputLocationRef, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputLocationRef;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputLocationRef")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_ref_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationRefId" => {
                                destination_ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputLocationRef {
                        destination_ref_id: destination_ref_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.OutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputSettings {
        /// Property [`ArchiveOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-archiveoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archive_output_settings: Option<::Value<ArchiveOutputSettings>>,
        /// Property [`FrameCaptureOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-framecaptureoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_output_settings: Option<::Value<FrameCaptureOutputSettings>>,
        /// Property [`HlsOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-hlsoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hls_output_settings: Option<::Value<HlsOutputSettings>>,
        /// Property [`MediaPackageOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-mediapackageoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_package_output_settings: Option<::Value<MediaPackageOutputSettings>>,
        /// Property [`MsSmoothOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-mssmoothoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ms_smooth_output_settings: Option<::Value<MsSmoothOutputSettings>>,
        /// Property [`MultiplexOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-multiplexoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multiplex_output_settings: Option<::Value<MultiplexOutputSettings>>,
        /// Property [`RtmpOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-rtmpoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rtmp_output_settings: Option<::Value<RtmpOutputSettings>>,
        /// Property [`UdpOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-outputsettings.html#cfn-medialive-channel-outputsettings-udpoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub udp_output_settings: Option<::Value<UdpOutputSettings>>,
    }

    impl ::codec::SerializeValue for OutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref archive_output_settings) = self.archive_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchiveOutputSettings", archive_output_settings)?;
            }
            if let Some(ref frame_capture_output_settings) = self.frame_capture_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureOutputSettings", frame_capture_output_settings)?;
            }
            if let Some(ref hls_output_settings) = self.hls_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HlsOutputSettings", hls_output_settings)?;
            }
            if let Some(ref media_package_output_settings) = self.media_package_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaPackageOutputSettings", media_package_output_settings)?;
            }
            if let Some(ref ms_smooth_output_settings) = self.ms_smooth_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MsSmoothOutputSettings", ms_smooth_output_settings)?;
            }
            if let Some(ref multiplex_output_settings) = self.multiplex_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiplexOutputSettings", multiplex_output_settings)?;
            }
            if let Some(ref rtmp_output_settings) = self.rtmp_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RtmpOutputSettings", rtmp_output_settings)?;
            }
            if let Some(ref udp_output_settings) = self.udp_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UdpOutputSettings", udp_output_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut archive_output_settings: Option<::Value<ArchiveOutputSettings>> = None;
                    let mut frame_capture_output_settings: Option<::Value<FrameCaptureOutputSettings>> = None;
                    let mut hls_output_settings: Option<::Value<HlsOutputSettings>> = None;
                    let mut media_package_output_settings: Option<::Value<MediaPackageOutputSettings>> = None;
                    let mut ms_smooth_output_settings: Option<::Value<MsSmoothOutputSettings>> = None;
                    let mut multiplex_output_settings: Option<::Value<MultiplexOutputSettings>> = None;
                    let mut rtmp_output_settings: Option<::Value<RtmpOutputSettings>> = None;
                    let mut udp_output_settings: Option<::Value<UdpOutputSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArchiveOutputSettings" => {
                                archive_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameCaptureOutputSettings" => {
                                frame_capture_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HlsOutputSettings" => {
                                hls_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaPackageOutputSettings" => {
                                media_package_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MsSmoothOutputSettings" => {
                                ms_smooth_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiplexOutputSettings" => {
                                multiplex_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RtmpOutputSettings" => {
                                rtmp_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UdpOutputSettings" => {
                                udp_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputSettings {
                        archive_output_settings: archive_output_settings,
                        frame_capture_output_settings: frame_capture_output_settings,
                        hls_output_settings: hls_output_settings,
                        media_package_output_settings: media_package_output_settings,
                        ms_smooth_output_settings: ms_smooth_output_settings,
                        multiplex_output_settings: multiplex_output_settings,
                        rtmp_output_settings: rtmp_output_settings,
                        udp_output_settings: udp_output_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.PassThroughSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-passthroughsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct PassThroughSettings {
    }

    impl ::codec::SerializeValue for PassThroughSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PassThroughSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PassThroughSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PassThroughSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PassThroughSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(PassThroughSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.RawSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rawsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RawSettings {
    }

    impl ::codec::SerializeValue for RawSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RawSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RawSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RawSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RawSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(RawSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Rec601Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rec601settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Rec601Settings {
    }

    impl ::codec::SerializeValue for Rec601Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rec601Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rec601Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rec601Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rec601Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Rec601Settings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Rec709Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rec709settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Rec709Settings {
    }

    impl ::codec::SerializeValue for Rec709Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rec709Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rec709Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rec709Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rec709Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Rec709Settings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.RemixSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-remixsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RemixSettings {
        /// Property [`ChannelMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-remixsettings.html#cfn-medialive-channel-remixsettings-channelmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_mappings: Option<::ValueList<AudioChannelMapping>>,
        /// Property [`ChannelsIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-remixsettings.html#cfn-medialive-channel-remixsettings-channelsin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channels_in: Option<::Value<u32>>,
        /// Property [`ChannelsOut`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-remixsettings.html#cfn-medialive-channel-remixsettings-channelsout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channels_out: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RemixSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel_mappings) = self.channel_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelMappings", channel_mappings)?;
            }
            if let Some(ref channels_in) = self.channels_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelsIn", channels_in)?;
            }
            if let Some(ref channels_out) = self.channels_out {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelsOut", channels_out)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RemixSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RemixSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RemixSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RemixSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_mappings: Option<::ValueList<AudioChannelMapping>> = None;
                    let mut channels_in: Option<::Value<u32>> = None;
                    let mut channels_out: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChannelMappings" => {
                                channel_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChannelsIn" => {
                                channels_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChannelsOut" => {
                                channels_out = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RemixSettings {
                        channel_mappings: channel_mappings,
                        channels_in: channels_in,
                        channels_out: channels_out,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.RtmpCaptionInfoDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpcaptioninfodestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RtmpCaptionInfoDestinationSettings {
    }

    impl ::codec::SerializeValue for RtmpCaptionInfoDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RtmpCaptionInfoDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RtmpCaptionInfoDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RtmpCaptionInfoDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RtmpCaptionInfoDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(RtmpCaptionInfoDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.RtmpGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RtmpGroupSettings {
        /// Property [`AdMarkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-admarkers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_markers: Option<::ValueList<String>>,
        /// Property [`AuthenticationScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-authenticationscheme).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_scheme: Option<::Value<String>>,
        /// Property [`CacheFullBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-cachefullbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_full_behavior: Option<::Value<String>>,
        /// Property [`CacheLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-cachelength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cache_length: Option<::Value<u32>>,
        /// Property [`CaptionData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-captiondata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caption_data: Option<::Value<String>>,
        /// Property [`InputLossAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-inputlossaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_action: Option<::Value<String>>,
        /// Property [`RestartDelay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpgroupsettings.html#cfn-medialive-channel-rtmpgroupsettings-restartdelay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub restart_delay: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RtmpGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_markers) = self.ad_markers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdMarkers", ad_markers)?;
            }
            if let Some(ref authentication_scheme) = self.authentication_scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationScheme", authentication_scheme)?;
            }
            if let Some(ref cache_full_behavior) = self.cache_full_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheFullBehavior", cache_full_behavior)?;
            }
            if let Some(ref cache_length) = self.cache_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheLength", cache_length)?;
            }
            if let Some(ref caption_data) = self.caption_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptionData", caption_data)?;
            }
            if let Some(ref input_loss_action) = self.input_loss_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossAction", input_loss_action)?;
            }
            if let Some(ref restart_delay) = self.restart_delay {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestartDelay", restart_delay)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RtmpGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RtmpGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RtmpGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RtmpGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_markers: Option<::ValueList<String>> = None;
                    let mut authentication_scheme: Option<::Value<String>> = None;
                    let mut cache_full_behavior: Option<::Value<String>> = None;
                    let mut cache_length: Option<::Value<u32>> = None;
                    let mut caption_data: Option<::Value<String>> = None;
                    let mut input_loss_action: Option<::Value<String>> = None;
                    let mut restart_delay: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdMarkers" => {
                                ad_markers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthenticationScheme" => {
                                authentication_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheFullBehavior" => {
                                cache_full_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheLength" => {
                                cache_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptionData" => {
                                caption_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputLossAction" => {
                                input_loss_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestartDelay" => {
                                restart_delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RtmpGroupSettings {
                        ad_markers: ad_markers,
                        authentication_scheme: authentication_scheme,
                        cache_full_behavior: cache_full_behavior,
                        cache_length: cache_length,
                        caption_data: caption_data,
                        input_loss_action: input_loss_action,
                        restart_delay: restart_delay,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.RtmpOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RtmpOutputSettings {
        /// Property [`CertificateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html#cfn-medialive-channel-rtmpoutputsettings-certificatemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_mode: Option<::Value<String>>,
        /// Property [`ConnectionRetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html#cfn-medialive-channel-rtmpoutputsettings-connectionretryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_retry_interval: Option<::Value<u32>>,
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html#cfn-medialive-channel-rtmpoutputsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`NumRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-rtmpoutputsettings.html#cfn-medialive-channel-rtmpoutputsettings-numretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_retries: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RtmpOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_mode) = self.certificate_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateMode", certificate_mode)?;
            }
            if let Some(ref connection_retry_interval) = self.connection_retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionRetryInterval", connection_retry_interval)?;
            }
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref num_retries) = self.num_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRetries", num_retries)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RtmpOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RtmpOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RtmpOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RtmpOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_mode: Option<::Value<String>> = None;
                    let mut connection_retry_interval: Option<::Value<u32>> = None;
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut num_retries: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateMode" => {
                                certificate_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionRetryInterval" => {
                                connection_retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRetries" => {
                                num_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RtmpOutputSettings {
                        certificate_mode: certificate_mode,
                        connection_retry_interval: connection_retry_interval,
                        destination: destination,
                        num_retries: num_retries,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte20PlusEmbeddedDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20plusembeddeddestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte20PlusEmbeddedDestinationSettings {
    }

    impl ::codec::SerializeValue for Scte20PlusEmbeddedDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte20PlusEmbeddedDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte20PlusEmbeddedDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte20PlusEmbeddedDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte20PlusEmbeddedDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Scte20PlusEmbeddedDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte20SourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20sourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte20SourceSettings {
        /// Property [`Convert608To708`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20sourcesettings.html#cfn-medialive-channel-scte20sourcesettings-convert608to708).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub convert608_to708: Option<::Value<String>>,
        /// Property [`Source608ChannelNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte20sourcesettings.html#cfn-medialive-channel-scte20sourcesettings-source608channelnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source608_channel_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Scte20SourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref convert608_to708) = self.convert608_to708 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Convert608To708", convert608_to708)?;
            }
            if let Some(ref source608_channel_number) = self.source608_channel_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source608ChannelNumber", source608_channel_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte20SourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte20SourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte20SourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte20SourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut convert608_to708: Option<::Value<String>> = None;
                    let mut source608_channel_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Convert608To708" => {
                                convert608_to708 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source608ChannelNumber" => {
                                source608_channel_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scte20SourceSettings {
                        convert608_to708: convert608_to708,
                        source608_channel_number: source608_channel_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte27DestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte27destinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte27DestinationSettings {
    }

    impl ::codec::SerializeValue for Scte27DestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte27DestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte27DestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte27DestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte27DestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Scte27DestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte27SourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte27sourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte27SourceSettings {
        /// Property [`Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte27sourcesettings.html#cfn-medialive-channel-scte27sourcesettings-pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Scte27SourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pid) = self.pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pid", pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte27SourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte27SourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte27SourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte27SourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pid" => {
                                pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scte27SourceSettings {
                        pid: pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte35SpliceInsert`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35spliceinsert.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte35SpliceInsert {
        /// Property [`AdAvailOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35spliceinsert.html#cfn-medialive-channel-scte35spliceinsert-adavailoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_avail_offset: Option<::Value<u32>>,
        /// Property [`NoRegionalBlackoutFlag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35spliceinsert.html#cfn-medialive-channel-scte35spliceinsert-noregionalblackoutflag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_regional_blackout_flag: Option<::Value<String>>,
        /// Property [`WebDeliveryAllowedFlag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35spliceinsert.html#cfn-medialive-channel-scte35spliceinsert-webdeliveryallowedflag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_delivery_allowed_flag: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Scte35SpliceInsert {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_avail_offset) = self.ad_avail_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdAvailOffset", ad_avail_offset)?;
            }
            if let Some(ref no_regional_blackout_flag) = self.no_regional_blackout_flag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoRegionalBlackoutFlag", no_regional_blackout_flag)?;
            }
            if let Some(ref web_delivery_allowed_flag) = self.web_delivery_allowed_flag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebDeliveryAllowedFlag", web_delivery_allowed_flag)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte35SpliceInsert {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte35SpliceInsert, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte35SpliceInsert;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte35SpliceInsert")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_avail_offset: Option<::Value<u32>> = None;
                    let mut no_regional_blackout_flag: Option<::Value<String>> = None;
                    let mut web_delivery_allowed_flag: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdAvailOffset" => {
                                ad_avail_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoRegionalBlackoutFlag" => {
                                no_regional_blackout_flag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebDeliveryAllowedFlag" => {
                                web_delivery_allowed_flag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scte35SpliceInsert {
                        ad_avail_offset: ad_avail_offset,
                        no_regional_blackout_flag: no_regional_blackout_flag,
                        web_delivery_allowed_flag: web_delivery_allowed_flag,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.Scte35TimeSignalApos`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35timesignalapos.html) property type.
    #[derive(Debug, Default)]
    pub struct Scte35TimeSignalApos {
        /// Property [`AdAvailOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35timesignalapos.html#cfn-medialive-channel-scte35timesignalapos-adavailoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_avail_offset: Option<::Value<u32>>,
        /// Property [`NoRegionalBlackoutFlag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35timesignalapos.html#cfn-medialive-channel-scte35timesignalapos-noregionalblackoutflag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_regional_blackout_flag: Option<::Value<String>>,
        /// Property [`WebDeliveryAllowedFlag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-scte35timesignalapos.html#cfn-medialive-channel-scte35timesignalapos-webdeliveryallowedflag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_delivery_allowed_flag: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Scte35TimeSignalApos {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_avail_offset) = self.ad_avail_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdAvailOffset", ad_avail_offset)?;
            }
            if let Some(ref no_regional_blackout_flag) = self.no_regional_blackout_flag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoRegionalBlackoutFlag", no_regional_blackout_flag)?;
            }
            if let Some(ref web_delivery_allowed_flag) = self.web_delivery_allowed_flag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebDeliveryAllowedFlag", web_delivery_allowed_flag)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scte35TimeSignalApos {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scte35TimeSignalApos, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scte35TimeSignalApos;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scte35TimeSignalApos")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_avail_offset: Option<::Value<u32>> = None;
                    let mut no_regional_blackout_flag: Option<::Value<String>> = None;
                    let mut web_delivery_allowed_flag: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdAvailOffset" => {
                                ad_avail_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoRegionalBlackoutFlag" => {
                                no_regional_blackout_flag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebDeliveryAllowedFlag" => {
                                web_delivery_allowed_flag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scte35TimeSignalApos {
                        ad_avail_offset: ad_avail_offset,
                        no_regional_blackout_flag: no_regional_blackout_flag,
                        web_delivery_allowed_flag: web_delivery_allowed_flag,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.SmpteTtDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-smptettdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SmpteTtDestinationSettings {
    }

    impl ::codec::SerializeValue for SmpteTtDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SmpteTtDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SmpteTtDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SmpteTtDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SmpteTtDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(SmpteTtDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.StandardHlsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-standardhlssettings.html) property type.
    #[derive(Debug, Default)]
    pub struct StandardHlsSettings {
        /// Property [`AudioRenditionSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-standardhlssettings.html#cfn-medialive-channel-standardhlssettings-audiorenditionsets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_rendition_sets: Option<::Value<String>>,
        /// Property [`M3u8Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-standardhlssettings.html#cfn-medialive-channel-standardhlssettings-m3u8settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub m3u8_settings: Option<::Value<M3u8Settings>>,
    }

    impl ::codec::SerializeValue for StandardHlsSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_rendition_sets) = self.audio_rendition_sets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioRenditionSets", audio_rendition_sets)?;
            }
            if let Some(ref m3u8_settings) = self.m3u8_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "M3u8Settings", m3u8_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StandardHlsSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StandardHlsSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StandardHlsSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StandardHlsSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_rendition_sets: Option<::Value<String>> = None;
                    let mut m3u8_settings: Option<::Value<M3u8Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioRenditionSets" => {
                                audio_rendition_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "M3u8Settings" => {
                                m3u8_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StandardHlsSettings {
                        audio_rendition_sets: audio_rendition_sets,
                        m3u8_settings: m3u8_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.StaticKeySettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-statickeysettings.html) property type.
    #[derive(Debug, Default)]
    pub struct StaticKeySettings {
        /// Property [`KeyProviderServer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-statickeysettings.html#cfn-medialive-channel-statickeysettings-keyproviderserver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_provider_server: Option<::Value<InputLocation>>,
        /// Property [`StaticKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-statickeysettings.html#cfn-medialive-channel-statickeysettings-statickeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub static_key_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StaticKeySettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_provider_server) = self.key_provider_server {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyProviderServer", key_provider_server)?;
            }
            if let Some(ref static_key_value) = self.static_key_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StaticKeyValue", static_key_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StaticKeySettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StaticKeySettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StaticKeySettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StaticKeySettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_provider_server: Option<::Value<InputLocation>> = None;
                    let mut static_key_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyProviderServer" => {
                                key_provider_server = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StaticKeyValue" => {
                                static_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StaticKeySettings {
                        key_provider_server: key_provider_server,
                        static_key_value: static_key_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.TeletextDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct TeletextDestinationSettings {
    }

    impl ::codec::SerializeValue for TeletextDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TeletextDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TeletextDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TeletextDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TeletextDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(TeletextDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.TeletextSourceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextsourcesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct TeletextSourceSettings {
        /// Property [`OutputRectangle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextsourcesettings.html#cfn-medialive-channel-teletextsourcesettings-outputrectangle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_rectangle: Option<::Value<CaptionRectangle>>,
        /// Property [`PageNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-teletextsourcesettings.html#cfn-medialive-channel-teletextsourcesettings-pagenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub page_number: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TeletextSourceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref output_rectangle) = self.output_rectangle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputRectangle", output_rectangle)?;
            }
            if let Some(ref page_number) = self.page_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PageNumber", page_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TeletextSourceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TeletextSourceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TeletextSourceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TeletextSourceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut output_rectangle: Option<::Value<CaptionRectangle>> = None;
                    let mut page_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OutputRectangle" => {
                                output_rectangle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PageNumber" => {
                                page_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TeletextSourceSettings {
                        output_rectangle: output_rectangle,
                        page_number: page_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.TemporalFilterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-temporalfiltersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct TemporalFilterSettings {
        /// Property [`PostFilterSharpening`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-temporalfiltersettings.html#cfn-medialive-channel-temporalfiltersettings-postfiltersharpening).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_filter_sharpening: Option<::Value<String>>,
        /// Property [`Strength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-temporalfiltersettings.html#cfn-medialive-channel-temporalfiltersettings-strength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strength: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TemporalFilterSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref post_filter_sharpening) = self.post_filter_sharpening {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostFilterSharpening", post_filter_sharpening)?;
            }
            if let Some(ref strength) = self.strength {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Strength", strength)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemporalFilterSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemporalFilterSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemporalFilterSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemporalFilterSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut post_filter_sharpening: Option<::Value<String>> = None;
                    let mut strength: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PostFilterSharpening" => {
                                post_filter_sharpening = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Strength" => {
                                strength = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemporalFilterSettings {
                        post_filter_sharpening: post_filter_sharpening,
                        strength: strength,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.TimecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-timecodeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TimecodeConfig {
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-timecodeconfig.html#cfn-medialive-channel-timecodeconfig-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<String>>,
        /// Property [`SyncThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-timecodeconfig.html#cfn-medialive-channel-timecodeconfig-syncthreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sync_threshold: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TimecodeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            if let Some(ref sync_threshold) = self.sync_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncThreshold", sync_threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimecodeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimecodeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimecodeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimecodeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source: Option<::Value<String>> = None;
                    let mut sync_threshold: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SyncThreshold" => {
                                sync_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimecodeConfig {
                        source: source,
                        sync_threshold: sync_threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.TtmlDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ttmldestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct TtmlDestinationSettings {
        /// Property [`StyleControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-ttmldestinationsettings.html#cfn-medialive-channel-ttmldestinationsettings-stylecontrol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub style_control: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TtmlDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref style_control) = self.style_control {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StyleControl", style_control)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TtmlDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TtmlDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TtmlDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TtmlDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut style_control: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StyleControl" => {
                                style_control = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TtmlDestinationSettings {
                        style_control: style_control,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.UdpContainerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpcontainersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UdpContainerSettings {
        /// Property [`M2tsSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpcontainersettings.html#cfn-medialive-channel-udpcontainersettings-m2tssettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub m2ts_settings: Option<::Value<M2tsSettings>>,
    }

    impl ::codec::SerializeValue for UdpContainerSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref m2ts_settings) = self.m2ts_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "M2tsSettings", m2ts_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UdpContainerSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UdpContainerSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UdpContainerSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UdpContainerSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut m2ts_settings: Option<::Value<M2tsSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "M2tsSettings" => {
                                m2ts_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UdpContainerSettings {
                        m2ts_settings: m2ts_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.UdpGroupSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpgroupsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UdpGroupSettings {
        /// Property [`InputLossAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpgroupsettings.html#cfn-medialive-channel-udpgroupsettings-inputlossaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_loss_action: Option<::Value<String>>,
        /// Property [`TimedMetadataId3Frame`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpgroupsettings.html#cfn-medialive-channel-udpgroupsettings-timedmetadataid3frame).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_id3_frame: Option<::Value<String>>,
        /// Property [`TimedMetadataId3Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpgroupsettings.html#cfn-medialive-channel-udpgroupsettings-timedmetadataid3period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timed_metadata_id3_period: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for UdpGroupSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_loss_action) = self.input_loss_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLossAction", input_loss_action)?;
            }
            if let Some(ref timed_metadata_id3_frame) = self.timed_metadata_id3_frame {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataId3Frame", timed_metadata_id3_frame)?;
            }
            if let Some(ref timed_metadata_id3_period) = self.timed_metadata_id3_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimedMetadataId3Period", timed_metadata_id3_period)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UdpGroupSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UdpGroupSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UdpGroupSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UdpGroupSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_loss_action: Option<::Value<String>> = None;
                    let mut timed_metadata_id3_frame: Option<::Value<String>> = None;
                    let mut timed_metadata_id3_period: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputLossAction" => {
                                input_loss_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataId3Frame" => {
                                timed_metadata_id3_frame = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimedMetadataId3Period" => {
                                timed_metadata_id3_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UdpGroupSettings {
                        input_loss_action: input_loss_action,
                        timed_metadata_id3_frame: timed_metadata_id3_frame,
                        timed_metadata_id3_period: timed_metadata_id3_period,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.UdpOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UdpOutputSettings {
        /// Property [`BufferMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html#cfn-medialive-channel-udpoutputsettings-buffermsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buffer_msec: Option<::Value<u32>>,
        /// Property [`ContainerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html#cfn-medialive-channel-udpoutputsettings-containersettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_settings: Option<::Value<UdpContainerSettings>>,
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html#cfn-medialive-channel-udpoutputsettings-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<OutputLocationRef>>,
        /// Property [`FecOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-udpoutputsettings.html#cfn-medialive-channel-udpoutputsettings-fecoutputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fec_output_settings: Option<::Value<FecOutputSettings>>,
    }

    impl ::codec::SerializeValue for UdpOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buffer_msec) = self.buffer_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BufferMsec", buffer_msec)?;
            }
            if let Some(ref container_settings) = self.container_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerSettings", container_settings)?;
            }
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref fec_output_settings) = self.fec_output_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FecOutputSettings", fec_output_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UdpOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UdpOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UdpOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UdpOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buffer_msec: Option<::Value<u32>> = None;
                    let mut container_settings: Option<::Value<UdpContainerSettings>> = None;
                    let mut destination: Option<::Value<OutputLocationRef>> = None;
                    let mut fec_output_settings: Option<::Value<FecOutputSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BufferMsec" => {
                                buffer_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerSettings" => {
                                container_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FecOutputSettings" => {
                                fec_output_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UdpOutputSettings {
                        buffer_msec: buffer_msec,
                        container_settings: container_settings,
                        destination: destination,
                        fec_output_settings: fec_output_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoBlackFailoverSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoblackfailoversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoBlackFailoverSettings {
        /// Property [`BlackDetectThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoblackfailoversettings.html#cfn-medialive-channel-videoblackfailoversettings-blackdetectthreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub black_detect_threshold: Option<::Value<f64>>,
        /// Property [`VideoBlackThresholdMsec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoblackfailoversettings.html#cfn-medialive-channel-videoblackfailoversettings-videoblackthresholdmsec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_black_threshold_msec: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VideoBlackFailoverSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref black_detect_threshold) = self.black_detect_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlackDetectThreshold", black_detect_threshold)?;
            }
            if let Some(ref video_black_threshold_msec) = self.video_black_threshold_msec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoBlackThresholdMsec", video_black_threshold_msec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoBlackFailoverSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoBlackFailoverSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoBlackFailoverSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoBlackFailoverSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut black_detect_threshold: Option<::Value<f64>> = None;
                    let mut video_black_threshold_msec: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlackDetectThreshold" => {
                                black_detect_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoBlackThresholdMsec" => {
                                video_black_threshold_msec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoBlackFailoverSettings {
                        black_detect_threshold: black_detect_threshold,
                        video_black_threshold_msec: video_black_threshold_msec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoCodecSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoCodecSettings {
        /// Property [`FrameCaptureSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html#cfn-medialive-channel-videocodecsettings-framecapturesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frame_capture_settings: Option<::Value<FrameCaptureSettings>>,
        /// Property [`H264Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html#cfn-medialive-channel-videocodecsettings-h264settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub h264_settings: Option<::Value<H264Settings>>,
        /// Property [`H265Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html#cfn-medialive-channel-videocodecsettings-h265settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub h265_settings: Option<::Value<H265Settings>>,
        /// Property [`Mpeg2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videocodecsettings.html#cfn-medialive-channel-videocodecsettings-mpeg2settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mpeg2_settings: Option<::Value<Mpeg2Settings>>,
    }

    impl ::codec::SerializeValue for VideoCodecSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref frame_capture_settings) = self.frame_capture_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameCaptureSettings", frame_capture_settings)?;
            }
            if let Some(ref h264_settings) = self.h264_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "H264Settings", h264_settings)?;
            }
            if let Some(ref h265_settings) = self.h265_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "H265Settings", h265_settings)?;
            }
            if let Some(ref mpeg2_settings) = self.mpeg2_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mpeg2Settings", mpeg2_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoCodecSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoCodecSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoCodecSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoCodecSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut frame_capture_settings: Option<::Value<FrameCaptureSettings>> = None;
                    let mut h264_settings: Option<::Value<H264Settings>> = None;
                    let mut h265_settings: Option<::Value<H265Settings>> = None;
                    let mut mpeg2_settings: Option<::Value<Mpeg2Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FrameCaptureSettings" => {
                                frame_capture_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "H264Settings" => {
                                h264_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "H265Settings" => {
                                h265_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mpeg2Settings" => {
                                mpeg2_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoCodecSettings {
                        frame_capture_settings: frame_capture_settings,
                        h264_settings: h264_settings,
                        h265_settings: h265_settings,
                        mpeg2_settings: mpeg2_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoDescription {
        /// Property [`CodecSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-codecsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub codec_settings: Option<::Value<VideoCodecSettings>>,
        /// Property [`Height`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-height).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub height: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`RespondToAfd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-respondtoafd).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub respond_to_afd: Option<::Value<String>>,
        /// Property [`ScalingBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-scalingbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling_behavior: Option<::Value<String>>,
        /// Property [`Sharpness`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-sharpness).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sharpness: Option<::Value<u32>>,
        /// Property [`Width`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videodescription.html#cfn-medialive-channel-videodescription-width).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub width: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VideoDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref codec_settings) = self.codec_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodecSettings", codec_settings)?;
            }
            if let Some(ref height) = self.height {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Height", height)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref respond_to_afd) = self.respond_to_afd {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RespondToAfd", respond_to_afd)?;
            }
            if let Some(ref scaling_behavior) = self.scaling_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingBehavior", scaling_behavior)?;
            }
            if let Some(ref sharpness) = self.sharpness {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sharpness", sharpness)?;
            }
            if let Some(ref width) = self.width {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Width", width)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut codec_settings: Option<::Value<VideoCodecSettings>> = None;
                    let mut height: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut respond_to_afd: Option<::Value<String>> = None;
                    let mut scaling_behavior: Option<::Value<String>> = None;
                    let mut sharpness: Option<::Value<u32>> = None;
                    let mut width: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodecSettings" => {
                                codec_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Height" => {
                                height = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RespondToAfd" => {
                                respond_to_afd = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingBehavior" => {
                                scaling_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sharpness" => {
                                sharpness = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Width" => {
                                width = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoDescription {
                        codec_settings: codec_settings,
                        height: height,
                        name: name,
                        respond_to_afd: respond_to_afd,
                        scaling_behavior: scaling_behavior,
                        sharpness: sharpness,
                        width: width,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoSelector {
        /// Property [`ColorSpace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html#cfn-medialive-channel-videoselector-colorspace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space: Option<::Value<String>>,
        /// Property [`ColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html#cfn-medialive-channel-videoselector-colorspacesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_settings: Option<::Value<VideoSelectorColorSpaceSettings>>,
        /// Property [`ColorSpaceUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html#cfn-medialive-channel-videoselector-colorspaceusage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub color_space_usage: Option<::Value<String>>,
        /// Property [`SelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselector.html#cfn-medialive-channel-videoselector-selectorsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selector_settings: Option<::Value<VideoSelectorSettings>>,
    }

    impl ::codec::SerializeValue for VideoSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref color_space) = self.color_space {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpace", color_space)?;
            }
            if let Some(ref color_space_settings) = self.color_space_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpaceSettings", color_space_settings)?;
            }
            if let Some(ref color_space_usage) = self.color_space_usage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColorSpaceUsage", color_space_usage)?;
            }
            if let Some(ref selector_settings) = self.selector_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectorSettings", selector_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut color_space: Option<::Value<String>> = None;
                    let mut color_space_settings: Option<::Value<VideoSelectorColorSpaceSettings>> = None;
                    let mut color_space_usage: Option<::Value<String>> = None;
                    let mut selector_settings: Option<::Value<VideoSelectorSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColorSpace" => {
                                color_space = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorSpaceSettings" => {
                                color_space_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColorSpaceUsage" => {
                                color_space_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectorSettings" => {
                                selector_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoSelector {
                        color_space: color_space,
                        color_space_settings: color_space_settings,
                        color_space_usage: color_space_usage,
                        selector_settings: selector_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoSelectorColorSpaceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorcolorspacesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoSelectorColorSpaceSettings {
        /// Property [`Hdr10Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorcolorspacesettings.html#cfn-medialive-channel-videoselectorcolorspacesettings-hdr10settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hdr10_settings: Option<::Value<Hdr10Settings>>,
    }

    impl ::codec::SerializeValue for VideoSelectorColorSpaceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hdr10_settings) = self.hdr10_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hdr10Settings", hdr10_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoSelectorColorSpaceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoSelectorColorSpaceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoSelectorColorSpaceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoSelectorColorSpaceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hdr10_settings: Option<::Value<Hdr10Settings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hdr10Settings" => {
                                hdr10_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoSelectorColorSpaceSettings {
                        hdr10_settings: hdr10_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoSelectorPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorpid.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoSelectorPid {
        /// Property [`Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorpid.html#cfn-medialive-channel-videoselectorpid-pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VideoSelectorPid {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pid) = self.pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pid", pid)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoSelectorPid {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoSelectorPid, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoSelectorPid;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoSelectorPid")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pid: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pid" => {
                                pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoSelectorPid {
                        pid: pid,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoSelectorProgramId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorprogramid.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoSelectorProgramId {
        /// Property [`ProgramId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorprogramid.html#cfn-medialive-channel-videoselectorprogramid-programid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub program_id: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for VideoSelectorProgramId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref program_id) = self.program_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgramId", program_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoSelectorProgramId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoSelectorProgramId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoSelectorProgramId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoSelectorProgramId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut program_id: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProgramId" => {
                                program_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoSelectorProgramId {
                        program_id: program_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VideoSelectorSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VideoSelectorSettings {
        /// Property [`VideoSelectorPid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorsettings.html#cfn-medialive-channel-videoselectorsettings-videoselectorpid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_selector_pid: Option<::Value<VideoSelectorPid>>,
        /// Property [`VideoSelectorProgramId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-videoselectorsettings.html#cfn-medialive-channel-videoselectorsettings-videoselectorprogramid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub video_selector_program_id: Option<::Value<VideoSelectorProgramId>>,
    }

    impl ::codec::SerializeValue for VideoSelectorSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref video_selector_pid) = self.video_selector_pid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoSelectorPid", video_selector_pid)?;
            }
            if let Some(ref video_selector_program_id) = self.video_selector_program_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VideoSelectorProgramId", video_selector_program_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VideoSelectorSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VideoSelectorSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VideoSelectorSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VideoSelectorSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut video_selector_pid: Option<::Value<VideoSelectorPid>> = None;
                    let mut video_selector_program_id: Option<::Value<VideoSelectorProgramId>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VideoSelectorPid" => {
                                video_selector_pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VideoSelectorProgramId" => {
                                video_selector_program_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VideoSelectorSettings {
                        video_selector_pid: video_selector_pid,
                        video_selector_program_id: video_selector_program_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.VpcOutputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-vpcoutputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcOutputSettings {
        /// Property [`PublicAddressAllocationIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-vpcoutputsettings.html#cfn-medialive-channel-vpcoutputsettings-publicaddressallocationids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub public_address_allocation_ids: Option<::ValueList<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-vpcoutputsettings.html#cfn-medialive-channel-vpcoutputsettings-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-vpcoutputsettings.html#cfn-medialive-channel-vpcoutputsettings-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for VpcOutputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref public_address_allocation_ids) = self.public_address_allocation_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicAddressAllocationIds", public_address_allocation_ids)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcOutputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcOutputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcOutputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcOutputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut public_address_allocation_ids: Option<::ValueList<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PublicAddressAllocationIds" => {
                                public_address_allocation_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcOutputSettings {
                        public_address_allocation_ids: public_address_allocation_ids,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.WavSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-wavsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct WavSettings {
        /// Property [`BitDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-wavsettings.html#cfn-medialive-channel-wavsettings-bitdepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bit_depth: Option<::Value<f64>>,
        /// Property [`CodingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-wavsettings.html#cfn-medialive-channel-wavsettings-codingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coding_mode: Option<::Value<String>>,
        /// Property [`SampleRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-wavsettings.html#cfn-medialive-channel-wavsettings-samplerate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_rate: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for WavSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bit_depth) = self.bit_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BitDepth", bit_depth)?;
            }
            if let Some(ref coding_mode) = self.coding_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodingMode", coding_mode)?;
            }
            if let Some(ref sample_rate) = self.sample_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleRate", sample_rate)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WavSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WavSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WavSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WavSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bit_depth: Option<::Value<f64>> = None;
                    let mut coding_mode: Option<::Value<String>> = None;
                    let mut sample_rate: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BitDepth" => {
                                bit_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodingMode" => {
                                coding_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleRate" => {
                                sample_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WavSettings {
                        bit_depth: bit_depth,
                        coding_mode: coding_mode,
                        sample_rate: sample_rate,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Channel.WebvttDestinationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-channel-webvttdestinationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct WebvttDestinationSettings {
    }

    impl ::codec::SerializeValue for WebvttDestinationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebvttDestinationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebvttDestinationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebvttDestinationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebvttDestinationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(WebvttDestinationSettings {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod input {
    //! Property types for the `Input` resource.

    /// The [`AWS::MediaLive::Input.InputDestinationRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdestinationrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct InputDestinationRequest {
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdestinationrequest.html#cfn-medialive-input-inputdestinationrequest-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputDestinationRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref stream_name) = self.stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", stream_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputDestinationRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputDestinationRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputDestinationRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputDestinationRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputDestinationRequest {
                        stream_name: stream_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Input.InputDeviceRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicerequest.html) property type.
    #[derive(Debug, Default)]
    pub struct InputDeviceRequest {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicerequest.html#cfn-medialive-input-inputdevicerequest-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputDeviceRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputDeviceRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputDeviceRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputDeviceRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputDeviceRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputDeviceRequest {
                        id: id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Input.InputDeviceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct InputDeviceSettings {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputdevicesettings.html#cfn-medialive-input-inputdevicesettings-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputDeviceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputDeviceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputDeviceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputDeviceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputDeviceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputDeviceSettings {
                        id: id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Input.InputSourceRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsourcerequest.html) property type.
    #[derive(Debug, Default)]
    pub struct InputSourceRequest {
        /// Property [`PasswordParam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsourcerequest.html#cfn-medialive-input-inputsourcerequest-passwordparam).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_param: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsourcerequest.html#cfn-medialive-input-inputsourcerequest-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputsourcerequest.html#cfn-medialive-input-inputsourcerequest-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputSourceRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password_param) = self.password_param {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordParam", password_param)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputSourceRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSourceRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputSourceRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputSourceRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password_param: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordParam" => {
                                password_param = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputSourceRequest {
                        password_param: password_param,
                        url: url,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Input.InputVpcRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputvpcrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct InputVpcRequest {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputvpcrequest.html#cfn-medialive-input-inputvpcrequest-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-inputvpcrequest.html#cfn-medialive-input-inputvpcrequest-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for InputVpcRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputVpcRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputVpcRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputVpcRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputVpcRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputVpcRequest {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaLive::Input.MediaConnectFlowRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-mediaconnectflowrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct MediaConnectFlowRequest {
        /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-input-mediaconnectflowrequest.html#cfn-medialive-input-mediaconnectflowrequest-flowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MediaConnectFlowRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref flow_arn) = self.flow_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", flow_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MediaConnectFlowRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MediaConnectFlowRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MediaConnectFlowRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MediaConnectFlowRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flow_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FlowArn" => {
                                flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MediaConnectFlowRequest {
                        flow_arn: flow_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod input_security_group {
    //! Property types for the `InputSecurityGroup` resource.

    /// The [`AWS::MediaLive::InputSecurityGroup.InputWhitelistRuleCidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-inputsecuritygroup-inputwhitelistrulecidr.html) property type.
    #[derive(Debug, Default)]
    pub struct InputWhitelistRuleCidr {
        /// Property [`Cidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-medialive-inputsecuritygroup-inputwhitelistrulecidr.html#cfn-medialive-inputsecuritygroup-inputwhitelistrulecidr-cidr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidr: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputWhitelistRuleCidr {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cidr) = self.cidr {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cidr", cidr)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputWhitelistRuleCidr {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputWhitelistRuleCidr, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputWhitelistRuleCidr;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputWhitelistRuleCidr")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cidr" => {
                                cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputWhitelistRuleCidr {
                        cidr: cidr,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
