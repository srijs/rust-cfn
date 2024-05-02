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
    pub plan: Option<::ValueList<self::contact::Stage>>,
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
        if let Some(ref plan) = self.plan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Plan", plan)?;
        }
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
                    plan: plan,
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

/// The [`AWS::SSMContacts::Plan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-plan.html) resource type.
#[derive(Debug, Default)]
pub struct Plan {
    properties: PlanProperties
}

/// Properties for the `Plan` resource.
#[derive(Debug, Default)]
pub struct PlanProperties {
    /// Property [`ContactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-plan.html#cfn-ssmcontacts-plan-contactid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub contact_id: ::Value<String>,
    /// Property [`RotationIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-plan.html#cfn-ssmcontacts-plan-rotationids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rotation_ids: Option<::ValueList<String>>,
    /// Property [`Stages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-plan.html#cfn-ssmcontacts-plan-stages).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stages: Option<::ValueList<self::plan::Stage>>,
}

impl ::serde::Serialize for PlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactId", &self.contact_id)?;
        if let Some(ref rotation_ids) = self.rotation_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationIds", rotation_ids)?;
        }
        if let Some(ref stages) = self.stages {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stages", stages)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut contact_id: Option<::Value<String>> = None;
                let mut rotation_ids: Option<::ValueList<String>> = None;
                let mut stages: Option<::ValueList<self::plan::Stage>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContactId" => {
                            contact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RotationIds" => {
                            rotation_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Stages" => {
                            stages = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PlanProperties {
                    contact_id: contact_id.ok_or(::serde::de::Error::missing_field("ContactId"))?,
                    rotation_ids: rotation_ids,
                    stages: stages,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Plan {
    type Properties = PlanProperties;
    const TYPE: &'static str = "AWS::SSMContacts::Plan";
    fn properties(&self) -> &PlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Plan {}

impl From<PlanProperties> for Plan {
    fn from(properties: PlanProperties) -> Plan {
        Plan { properties }
    }
}

/// The [`AWS::SSMContacts::Rotation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html) resource type.
#[derive(Debug, Default)]
pub struct Rotation {
    properties: RotationProperties
}

/// Properties for the `Rotation` resource.
#[derive(Debug, Default)]
pub struct RotationProperties {
    /// Property [`ContactIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-contactids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_ids: ::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Recurrence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-recurrence).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recurrence: ::Value<self::rotation::RecurrenceSettings>,
    /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-starttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_time: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssmcontacts-rotation.html#cfn-ssmcontacts-rotation-timezoneid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_zone_id: ::Value<String>,
}

impl ::serde::Serialize for RotationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactIds", &self.contact_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recurrence", &self.recurrence)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZoneId", &self.time_zone_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RotationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RotationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RotationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RotationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut contact_ids: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut recurrence: Option<::Value<self::rotation::RecurrenceSettings>> = None;
                let mut start_time: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_zone_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContactIds" => {
                            contact_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Recurrence" => {
                            recurrence = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartTime" => {
                            start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeZoneId" => {
                            time_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RotationProperties {
                    contact_ids: contact_ids.ok_or(::serde::de::Error::missing_field("ContactIds"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    recurrence: recurrence.ok_or(::serde::de::Error::missing_field("Recurrence"))?,
                    start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    tags: tags,
                    time_zone_id: time_zone_id.ok_or(::serde::de::Error::missing_field("TimeZoneId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Rotation {
    type Properties = RotationProperties;
    const TYPE: &'static str = "AWS::SSMContacts::Rotation";
    fn properties(&self) -> &RotationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RotationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Rotation {}

impl From<RotationProperties> for Rotation {
    fn from(properties: RotationProperties) -> Rotation {
        Rotation { properties }
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
        pub duration_in_minutes: Option<::Value<u32>>,
        /// Property [`RotationIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html#cfn-ssmcontacts-contact-stage-rotationids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rotation_ids: Option<::ValueList<String>>,
        /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-contact-stage.html#cfn-ssmcontacts-contact-stage-targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub targets: Option<::ValueList<Targets>>,
    }

    impl ::codec::SerializeValue for Stage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_minutes) = self.duration_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInMinutes", duration_in_minutes)?;
            }
            if let Some(ref rotation_ids) = self.rotation_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationIds", rotation_ids)?;
            }
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
                    let mut rotation_ids: Option<::ValueList<String>> = None;
                    let mut targets: Option<::ValueList<Targets>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInMinutes" => {
                                duration_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RotationIds" => {
                                rotation_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Targets" => {
                                targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Stage {
                        duration_in_minutes: duration_in_minutes,
                        rotation_ids: rotation_ids,
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

pub mod plan {
    //! Property types for the `Plan` resource.

    /// The [`AWS::SSMContacts::Plan.ChannelTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-channeltargetinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ChannelTargetInfo {
        /// Property [`ChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-channeltargetinfo.html#cfn-ssmcontacts-plan-channeltargetinfo-channelid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_id: ::Value<String>,
        /// Property [`RetryIntervalInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-channeltargetinfo.html#cfn-ssmcontacts-plan-channeltargetinfo-retryintervalinminutes).
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

    /// The [`AWS::SSMContacts::Plan.ContactTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-contacttargetinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ContactTargetInfo {
        /// Property [`ContactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-contacttargetinfo.html#cfn-ssmcontacts-plan-contacttargetinfo-contactid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_id: ::Value<String>,
        /// Property [`IsEssential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-contacttargetinfo.html#cfn-ssmcontacts-plan-contacttargetinfo-isessential).
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

    /// The [`AWS::SSMContacts::Plan.Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-stage.html) property type.
    #[derive(Debug, Default)]
    pub struct Stage {
        /// Property [`DurationInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-stage.html#cfn-ssmcontacts-plan-stage-durationinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_in_minutes: ::Value<u32>,
        /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-stage.html#cfn-ssmcontacts-plan-stage-targets).
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

    /// The [`AWS::SSMContacts::Plan.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-targets.html) property type.
    #[derive(Debug, Default)]
    pub struct Targets {
        /// Property [`ChannelTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-targets.html#cfn-ssmcontacts-plan-targets-channeltargetinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_target_info: Option<::Value<ChannelTargetInfo>>,
        /// Property [`ContactTargetInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-plan-targets.html#cfn-ssmcontacts-plan-targets-contacttargetinfo).
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

pub mod rotation {
    //! Property types for the `Rotation` resource.

    /// The [`AWS::SSMContacts::Rotation.CoverageTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-coveragetime.html) property type.
    #[derive(Debug, Default)]
    pub struct CoverageTime {
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-coveragetime.html#cfn-ssmcontacts-rotation-coveragetime-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: ::Value<String>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-coveragetime.html#cfn-ssmcontacts-rotation-coveragetime-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for CoverageTime {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CoverageTime {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CoverageTime, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CoverageTime;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CoverageTime")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time: Option<::Value<String>> = None;
                    let mut start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CoverageTime {
                        end_time: end_time.ok_or(::serde::de::Error::missing_field("EndTime"))?,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Rotation.MonthlySetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-monthlysetting.html) property type.
    #[derive(Debug, Default)]
    pub struct MonthlySetting {
        /// Property [`DayOfMonth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-monthlysetting.html#cfn-ssmcontacts-rotation-monthlysetting-dayofmonth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day_of_month: ::Value<u32>,
        /// Property [`HandOffTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-monthlysetting.html#cfn-ssmcontacts-rotation-monthlysetting-handofftime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hand_off_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for MonthlySetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfMonth", &self.day_of_month)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HandOffTime", &self.hand_off_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonthlySetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonthlySetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonthlySetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonthlySetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day_of_month: Option<::Value<u32>> = None;
                    let mut hand_off_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DayOfMonth" => {
                                day_of_month = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HandOffTime" => {
                                hand_off_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonthlySetting {
                        day_of_month: day_of_month.ok_or(::serde::de::Error::missing_field("DayOfMonth"))?,
                        hand_off_time: hand_off_time.ok_or(::serde::de::Error::missing_field("HandOffTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Rotation.RecurrenceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RecurrenceSettings {
        /// Property [`DailySettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-dailysettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily_settings: Option<::ValueList<String>>,
        /// Property [`MonthlySettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-monthlysettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monthly_settings: Option<::ValueList<MonthlySetting>>,
        /// Property [`NumberOfOnCalls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-numberofoncalls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_on_calls: ::Value<u32>,
        /// Property [`RecurrenceMultiplier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-recurrencemultiplier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recurrence_multiplier: ::Value<u32>,
        /// Property [`ShiftCoverages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-shiftcoverages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shift_coverages: Option<::ValueList<ShiftCoverage>>,
        /// Property [`WeeklySettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-recurrencesettings.html#cfn-ssmcontacts-rotation-recurrencesettings-weeklysettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weekly_settings: Option<::ValueList<WeeklySetting>>,
    }

    impl ::codec::SerializeValue for RecurrenceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref daily_settings) = self.daily_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailySettings", daily_settings)?;
            }
            if let Some(ref monthly_settings) = self.monthly_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonthlySettings", monthly_settings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfOnCalls", &self.number_of_on_calls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecurrenceMultiplier", &self.recurrence_multiplier)?;
            if let Some(ref shift_coverages) = self.shift_coverages {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShiftCoverages", shift_coverages)?;
            }
            if let Some(ref weekly_settings) = self.weekly_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklySettings", weekly_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecurrenceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecurrenceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecurrenceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecurrenceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut daily_settings: Option<::ValueList<String>> = None;
                    let mut monthly_settings: Option<::ValueList<MonthlySetting>> = None;
                    let mut number_of_on_calls: Option<::Value<u32>> = None;
                    let mut recurrence_multiplier: Option<::Value<u32>> = None;
                    let mut shift_coverages: Option<::ValueList<ShiftCoverage>> = None;
                    let mut weekly_settings: Option<::ValueList<WeeklySetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DailySettings" => {
                                daily_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonthlySettings" => {
                                monthly_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfOnCalls" => {
                                number_of_on_calls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecurrenceMultiplier" => {
                                recurrence_multiplier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShiftCoverages" => {
                                shift_coverages = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeeklySettings" => {
                                weekly_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecurrenceSettings {
                        daily_settings: daily_settings,
                        monthly_settings: monthly_settings,
                        number_of_on_calls: number_of_on_calls.ok_or(::serde::de::Error::missing_field("NumberOfOnCalls"))?,
                        recurrence_multiplier: recurrence_multiplier.ok_or(::serde::de::Error::missing_field("RecurrenceMultiplier"))?,
                        shift_coverages: shift_coverages,
                        weekly_settings: weekly_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Rotation.ShiftCoverage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-shiftcoverage.html) property type.
    #[derive(Debug, Default)]
    pub struct ShiftCoverage {
        /// Property [`CoverageTimes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-shiftcoverage.html#cfn-ssmcontacts-rotation-shiftcoverage-coveragetimes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coverage_times: ::ValueList<CoverageTime>,
        /// Property [`DayOfWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-shiftcoverage.html#cfn-ssmcontacts-rotation-shiftcoverage-dayofweek).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day_of_week: ::Value<String>,
    }

    impl ::codec::SerializeValue for ShiftCoverage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoverageTimes", &self.coverage_times)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfWeek", &self.day_of_week)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ShiftCoverage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ShiftCoverage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ShiftCoverage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ShiftCoverage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut coverage_times: Option<::ValueList<CoverageTime>> = None;
                    let mut day_of_week: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CoverageTimes" => {
                                coverage_times = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DayOfWeek" => {
                                day_of_week = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ShiftCoverage {
                        coverage_times: coverage_times.ok_or(::serde::de::Error::missing_field("CoverageTimes"))?,
                        day_of_week: day_of_week.ok_or(::serde::de::Error::missing_field("DayOfWeek"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSMContacts::Rotation.WeeklySetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-weeklysetting.html) property type.
    #[derive(Debug, Default)]
    pub struct WeeklySetting {
        /// Property [`DayOfWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-weeklysetting.html#cfn-ssmcontacts-rotation-weeklysetting-dayofweek).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day_of_week: ::Value<String>,
        /// Property [`HandOffTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssmcontacts-rotation-weeklysetting.html#cfn-ssmcontacts-rotation-weeklysetting-handofftime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hand_off_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for WeeklySetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfWeek", &self.day_of_week)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HandOffTime", &self.hand_off_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WeeklySetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WeeklySetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WeeklySetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WeeklySetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day_of_week: Option<::Value<String>> = None;
                    let mut hand_off_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DayOfWeek" => {
                                day_of_week = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HandOffTime" => {
                                hand_off_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WeeklySetting {
                        day_of_week: day_of_week.ok_or(::serde::de::Error::missing_field("DayOfWeek"))?,
                        hand_off_time: hand_off_time.ok_or(::serde::de::Error::missing_field("HandOffTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
