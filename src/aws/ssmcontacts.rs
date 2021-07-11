//! Types for the `SSMContacts` service.

/// The [`AWS::SSMContacts::Contact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html) resource type.
#[derive(Debug, Default)]
pub struct Contact {
    properties: ContactProperties
}

/// Properties for the `Contact` resource.
#[derive(Debug, Default)]
pub struct ContactProperties {
    /// Property [`Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html#cfn-ssmcontacts-contact-alias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub alias: ::Value<String>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html#cfn-ssmcontacts-contact-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: ::Value<String>,
    /// Property [`Plan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html#cfn-ssmcontacts-contact-plan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub plan: ::ValueList<self::contact::Stage>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contact.html#cfn-ssmcontacts-contact-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ContactProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alias", &self.alias)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Plan", &self.plan)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alias: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut plan: Option<::ValueList<self::contact::Stage>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Alias" => {
                            alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Plan" => {
                            plan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactProperties {
                    alias: alias.ok_or(::serde::de::Error::missing_field("Alias"))?,
                    display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                    plan: plan.ok_or(::serde::de::Error::missing_field("Plan"))?,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Contact {
    type Properties = ContactProperties;
    const TYPE: &'static str = "AWS::SSMContacts::Contact";
    fn properties(&self) -> &ContactProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Contact {}

impl From<ContactProperties> for Contact {
    fn from(properties: ContactProperties) -> Contact {
        Contact { properties }
    }
}

/// The [`AWS::SSMContacts::ContactChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html) resource type.
#[derive(Debug, Default)]
pub struct ContactChannel {
    properties: ContactChannelProperties
}

/// Properties for the `ContactChannel` resource.
#[derive(Debug, Default)]
pub struct ContactChannelProperties {
    /// Property [`ChannelAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html#cfn-ssmcontacts-contactchannel-channeladdress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_address: ::Value<String>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html#cfn-ssmcontacts-contactchannel-channelname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_name: ::Value<String>,
    /// Property [`ChannelType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html#cfn-ssmcontacts-contactchannel-channeltype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_type: ::Value<String>,
    /// Property [`ContactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html#cfn-ssmcontacts-contactchannel-contactid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub contact_id: ::Value<String>,
    /// Property [`DeferActivation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-contactchannel.html#cfn-ssmcontacts-contactchannel-deferactivation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defer_activation: Option<::Value<bool>>,
}

impl ::serde::Serialize for ContactChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelAddress", &self.channel_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", &self.channel_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelType", &self.channel_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactId", &self.contact_id)?;
        if let Some(ref defer_activation) = self.defer_activation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeferActivation", defer_activation)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut channel_address: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut channel_type: Option<::Value<String>> = None;
                let mut contact_id: Option<::Value<String>> = None;
                let mut defer_activation: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelAddress" => {
                            channel_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelType" => {
                            channel_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContactId" => {
                            contact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeferActivation" => {
                            defer_activation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactChannelProperties {
                    channel_address: channel_address.ok_or(::serde::de::Error::missing_field("ChannelAddress"))?,
                    channel_name: channel_name.ok_or(::serde::de::Error::missing_field("ChannelName"))?,
                    channel_type: channel_type.ok_or(::serde::de::Error::missing_field("ChannelType"))?,
                    contact_id: contact_id.ok_or(::serde::de::Error::missing_field("ContactId"))?,
                    defer_activation: defer_activation,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactChannel {
    type Properties = ContactChannelProperties;
    const TYPE: &'static str = "AWS::SSMContacts::ContactChannel";
    fn properties(&self) -> &ContactChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactChannel {}

impl From<ContactChannelProperties> for ContactChannel {
    fn from(properties: ContactChannelProperties) -> ContactChannel {
        ContactChannel { properties }
    }
}

pub mod contact {
    //! Property types for the `Contact` resource.

    /// The [`AWS::SSMContacts::Contact.ChannelTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-channeltargetinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ChannelTargetInfo {
        /// Property [`ChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-channeltargetinfo.html#cfn-ssmcontacts-contact-channeltargetinfo-channelid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_id: ::Value<String>,
        /// Property [`RetryIntervalInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-channeltargetinfo.html#cfn-ssmcontacts-contact-channeltargetinfo-retryintervalinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_interval_in_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ChannelTargetInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelId", &self.channel_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryIntervalInMinutes", &self.retry_interval_in_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChannelTargetInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChannelTargetInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChannelTargetInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChannelTargetInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_id: Option<::Value<String>> = None;
                    let mut retry_interval_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChannelId" => {
                                channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryIntervalInMinutes" => {
                                retry_interval_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChannelTargetInfo {
                        channel_id: channel_id.ok_or(::serde::de::Error::missing_field("ChannelId"))?,
                        retry_interval_in_minutes: retry_interval_in_minutes.ok_or(::serde::de::Error::missing_field("RetryIntervalInMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Contact.ContactTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-contacttargetinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ContactTargetInfo {
        /// Property [`ContactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-contacttargetinfo.html#cfn-ssmcontacts-contact-contacttargetinfo-contactid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_id: ::Value<String>,
        /// Property [`IsEssential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-contacttargetinfo.html#cfn-ssmcontacts-contact-contacttargetinfo-isessential).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_essential: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ContactTargetInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactId", &self.contact_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsEssential", &self.is_essential)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContactTargetInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactTargetInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContactTargetInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContactTargetInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_id: Option<::Value<String>> = None;
                    let mut is_essential: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactId" => {
                                contact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsEssential" => {
                                is_essential = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContactTargetInfo {
                        contact_id: contact_id.ok_or(::serde::de::Error::missing_field("ContactId"))?,
                        is_essential: is_essential.ok_or(::serde::de::Error::missing_field("IsEssential"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Contact.Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html) property type.
    #[derive(Debug, Default)]
    pub struct Stage {
        /// Property [`DurationInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html#cfn-ssmcontacts-contact-stage-durationinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_minutes: ::Value<u32>,
        /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html#cfn-ssmcontacts-contact-stage-targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub targets: Option<::ValueList<Targets>>,
    }

    impl ::codec::SerializeValue for Stage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInMinutes", &self.duration_in_minutes)?;
            if let Some(ref targets) = self.targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Stage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Stage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Stage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Stage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_in_minutes: Option<::Value<u32>> = None;
                    let mut targets: Option<::ValueList<Targets>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInMinutes" => {
                                duration_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Targets" => {
                                targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Stage {
                        duration_in_minutes: duration_in_minutes.ok_or(::serde::de::Error::missing_field("DurationInMinutes"))?,
                        targets: targets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Contact.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-targets.html) property type.
    #[derive(Debug, Default)]
    pub struct Targets {
        /// Property [`ChannelTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-targets.html#cfn-ssmcontacts-contact-targets-channeltargetinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_target_info: Option<::Value<ChannelTargetInfo>>,
        /// Property [`ContactTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-targets.html#cfn-ssmcontacts-contact-targets-contacttargetinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_target_info: Option<::Value<ContactTargetInfo>>,
    }

    impl ::codec::SerializeValue for Targets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel_target_info) = self.channel_target_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelTargetInfo", channel_target_info)?;
            }
            if let Some(ref contact_target_info) = self.contact_target_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactTargetInfo", contact_target_info)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Targets {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Targets, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Targets;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Targets")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel_target_info: Option<::Value<ChannelTargetInfo>> = None;
                    let mut contact_target_info: Option<::Value<ContactTargetInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChannelTargetInfo" => {
                                channel_target_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContactTargetInfo" => {
                                contact_target_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Targets {
                        channel_target_info: channel_target_info,
                        contact_target_info: contact_target_info,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
