//! Types for the `RolesAnywhere` service.

/// The [`AWS::RolesAnywhere::CRL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html) resource type.
#[derive(Debug, Default)]
pub struct CRL {
    properties: CRLProperties
}

/// Properties for the `CRL` resource.
#[derive(Debug, Default)]
pub struct CRLProperties {
    /// Property [`CrlData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html#cfn-rolesanywhere-crl-crldata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub crl_data: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html#cfn-rolesanywhere-crl-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html#cfn-rolesanywhere-crl-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html#cfn-rolesanywhere-crl-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrustAnchorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html#cfn-rolesanywhere-crl-trustanchorarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub trust_anchor_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for CRLProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrlData", &self.crl_data)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref trust_anchor_arn) = self.trust_anchor_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustAnchorArn", trust_anchor_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CRLProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CRLProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CRLProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CRLProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut crl_data: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut trust_anchor_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CrlData" => {
                            crl_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrustAnchorArn" => {
                            trust_anchor_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CRLProperties {
                    crl_data: crl_data.ok_or(::serde::de::Error::missing_field("CrlData"))?,
                    enabled: enabled,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    trust_anchor_arn: trust_anchor_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CRL {
    type Properties = CRLProperties;
    const TYPE: &'static str = "AWS::RolesAnywhere::CRL";
    fn properties(&self) -> &CRLProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CRLProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CRL {}

impl From<CRLProperties> for CRL {
    fn from(properties: CRLProperties) -> CRL {
        CRL { properties }
    }
}

/// The [`AWS::RolesAnywhere::Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html) resource type.
#[derive(Debug, Default)]
pub struct Profile {
    properties: ProfileProperties
}

/// Properties for the `Profile` resource.
#[derive(Debug, Default)]
pub struct ProfileProperties {
    /// Property [`DurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-durationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub duration_seconds: Option<::Value<f64>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`ManagedPolicyArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-managedpolicyarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RequireInstanceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-requireinstanceproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub require_instance_properties: Option<::Value<bool>>,
    /// Property [`RoleArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-rolearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arns: ::ValueList<String>,
    /// Property [`SessionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-sessionpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_policy: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html#cfn-rolesanywhere-profile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref duration_seconds) = self.duration_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationSeconds", duration_seconds)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref managed_policy_arns) = self.managed_policy_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", managed_policy_arns)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref require_instance_properties) = self.require_instance_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireInstanceProperties", require_instance_properties)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArns", &self.role_arns)?;
        if let Some(ref session_policy) = self.session_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionPolicy", session_policy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut duration_seconds: Option<::Value<f64>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut managed_policy_arns: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut require_instance_properties: Option<::Value<bool>> = None;
                let mut role_arns: Option<::ValueList<String>> = None;
                let mut session_policy: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DurationSeconds" => {
                            duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequireInstanceProperties" => {
                            require_instance_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArns" => {
                            role_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SessionPolicy" => {
                            session_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProfileProperties {
                    duration_seconds: duration_seconds,
                    enabled: enabled,
                    managed_policy_arns: managed_policy_arns,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    require_instance_properties: require_instance_properties,
                    role_arns: role_arns.ok_or(::serde::de::Error::missing_field("RoleArns"))?,
                    session_policy: session_policy,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Profile {
    type Properties = ProfileProperties;
    const TYPE: &'static str = "AWS::RolesAnywhere::Profile";
    fn properties(&self) -> &ProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Profile {}

impl From<ProfileProperties> for Profile {
    fn from(properties: ProfileProperties) -> Profile {
        Profile { properties }
    }
}

/// The [`AWS::RolesAnywhere::TrustAnchor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html) resource type.
#[derive(Debug, Default)]
pub struct TrustAnchor {
    properties: TrustAnchorProperties
}

/// Properties for the `TrustAnchor` resource.
#[derive(Debug, Default)]
pub struct TrustAnchorProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html#cfn-rolesanywhere-trustanchor-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html#cfn-rolesanywhere-trustanchor-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NotificationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html#cfn-rolesanywhere-trustanchor-notificationsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_settings: Option<::ValueList<self::trust_anchor::NotificationSetting>>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html#cfn-rolesanywhere-trustanchor-source).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source: ::Value<self::trust_anchor::Source>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html#cfn-rolesanywhere-trustanchor-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TrustAnchorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref notification_settings) = self.notification_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationSettings", notification_settings)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrustAnchorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrustAnchorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrustAnchorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrustAnchorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut notification_settings: Option<::ValueList<self::trust_anchor::NotificationSetting>> = None;
                let mut source: Option<::Value<self::trust_anchor::Source>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationSettings" => {
                            notification_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrustAnchorProperties {
                    enabled: enabled,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    notification_settings: notification_settings,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TrustAnchor {
    type Properties = TrustAnchorProperties;
    const TYPE: &'static str = "AWS::RolesAnywhere::TrustAnchor";
    fn properties(&self) -> &TrustAnchorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrustAnchorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrustAnchor {}

impl From<TrustAnchorProperties> for TrustAnchor {
    fn from(properties: TrustAnchorProperties) -> TrustAnchor {
        TrustAnchor { properties }
    }
}

pub mod trust_anchor {
    //! Property types for the `TrustAnchor` resource.

    /// The [`AWS::RolesAnywhere::TrustAnchor.NotificationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationSetting {
        /// Property [`Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html#cfn-rolesanywhere-trustanchor-notificationsetting-channel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html#cfn-rolesanywhere-trustanchor-notificationsetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Event`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html#cfn-rolesanywhere-trustanchor-notificationsetting-event).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event: ::Value<String>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html#cfn-rolesanywhere-trustanchor-notificationsetting-threshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for NotificationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref channel) = self.channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Channel", channel)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Event", &self.event)?;
            if let Some(ref threshold) = self.threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut event: Option<::Value<String>> = None;
                    let mut threshold: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Channel" => {
                                channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Event" => {
                                event = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationSetting {
                        channel: channel,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        event: event.ok_or(::serde::de::Error::missing_field("Event"))?,
                        threshold: threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RolesAnywhere::TrustAnchor.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-source.html) property type.
    #[derive(Debug, Default)]
    pub struct Source {
        /// Property [`SourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-source.html#cfn-rolesanywhere-trustanchor-source-sourcedata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_data: Option<::Value<SourceData>>,
        /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-source.html#cfn-rolesanywhere-trustanchor-source-sourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_data) = self.source_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceData", source_data)?;
            }
            if let Some(ref source_type) = self.source_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
            }
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
                    let mut source_data: Option<::Value<SourceData>> = None;
                    let mut source_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceData" => {
                                source_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceType" => {
                                source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        source_data: source_data,
                        source_type: source_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RolesAnywhere::TrustAnchor.SourceData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-sourcedata.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceData {
        /// Property [`AcmPcaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-sourcedata.html#cfn-rolesanywhere-trustanchor-sourcedata-acmpcaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acm_pca_arn: Option<::Value<String>>,
        /// Property [`X509CertificateData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-sourcedata.html#cfn-rolesanywhere-trustanchor-sourcedata-x509certificatedata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub x509_certificate_data: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SourceData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm_pca_arn) = self.acm_pca_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcmPcaArn", acm_pca_arn)?;
            }
            if let Some(ref x509_certificate_data) = self.x509_certificate_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "X509CertificateData", x509_certificate_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm_pca_arn: Option<::Value<String>> = None;
                    let mut x509_certificate_data: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcmPcaArn" => {
                                acm_pca_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "X509CertificateData" => {
                                x509_certificate_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceData {
                        acm_pca_arn: acm_pca_arn,
                        x509_certificate_data: x509_certificate_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
