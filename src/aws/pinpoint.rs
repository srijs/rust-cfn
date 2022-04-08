//! Types for the `Pinpoint` service.

/// The [`AWS::Pinpoint::ADMChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html) resource type.
#[derive(Debug, Default)]
pub struct ADMChannel {
    properties: ADMChannelProperties
}

/// Properties for the `ADMChannel` resource.
#[derive(Debug, Default)]
pub struct ADMChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html#cfn-pinpoint-admchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html#cfn-pinpoint-admchannel-clientid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_id: ::Value<String>,
    /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html#cfn-pinpoint-admchannel-clientsecret).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_secret: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html#cfn-pinpoint-admchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for ADMChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ADMChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ADMChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ADMChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ADMChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut client_id: Option<::Value<String>> = None;
                let mut client_secret: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientId" => {
                            client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientSecret" => {
                            client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ADMChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                    client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                    enabled: enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ADMChannel {
    type Properties = ADMChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::ADMChannel";
    fn properties(&self) -> &ADMChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ADMChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ADMChannel {}

impl From<ADMChannelProperties> for ADMChannel {
    fn from(properties: ADMChannelProperties) -> ADMChannel {
        ADMChannel { properties }
    }
}

/// The [`AWS::Pinpoint::APNSChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html) resource type.
#[derive(Debug, Default)]
pub struct APNSChannel {
    properties: APNSChannelProperties
}

/// Properties for the `APNSChannel` resource.
#[derive(Debug, Default)]
pub struct APNSChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: Option<::Value<String>>,
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`DefaultAuthenticationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-defaultauthenticationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_authentication_method: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-privatekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub private_key: Option<::Value<String>>,
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-teamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub team_id: Option<::Value<String>>,
    /// Property [`TokenKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-tokenkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key: Option<::Value<String>>,
    /// Property [`TokenKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html#cfn-pinpoint-apnschannel-tokenkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key_id: Option<::Value<String>>,
}

impl ::serde::Serialize for APNSChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref bundle_id) = self.bundle_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", bundle_id)?;
        }
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref default_authentication_method) = self.default_authentication_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAuthenticationMethod", default_authentication_method)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref private_key) = self.private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
        }
        if let Some(ref team_id) = self.team_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", team_id)?;
        }
        if let Some(ref token_key) = self.token_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKey", token_key)?;
        }
        if let Some(ref token_key_id) = self.token_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKeyId", token_key_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for APNSChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<APNSChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APNSChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type APNSChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut certificate: Option<::Value<String>> = None;
                let mut default_authentication_method: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut private_key: Option<::Value<String>> = None;
                let mut team_id: Option<::Value<String>> = None;
                let mut token_key: Option<::Value<String>> = None;
                let mut token_key_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAuthenticationMethod" => {
                            default_authentication_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateKey" => {
                            private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKey" => {
                            token_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKeyId" => {
                            token_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(APNSChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    bundle_id: bundle_id,
                    certificate: certificate,
                    default_authentication_method: default_authentication_method,
                    enabled: enabled,
                    private_key: private_key,
                    team_id: team_id,
                    token_key: token_key,
                    token_key_id: token_key_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for APNSChannel {
    type Properties = APNSChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::APNSChannel";
    fn properties(&self) -> &APNSChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut APNSChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for APNSChannel {}

impl From<APNSChannelProperties> for APNSChannel {
    fn from(properties: APNSChannelProperties) -> APNSChannel {
        APNSChannel { properties }
    }
}

/// The [`AWS::Pinpoint::APNSSandboxChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html) resource type.
#[derive(Debug, Default)]
pub struct APNSSandboxChannel {
    properties: APNSSandboxChannelProperties
}

/// Properties for the `APNSSandboxChannel` resource.
#[derive(Debug, Default)]
pub struct APNSSandboxChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: Option<::Value<String>>,
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`DefaultAuthenticationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-defaultauthenticationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_authentication_method: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-privatekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub private_key: Option<::Value<String>>,
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-teamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub team_id: Option<::Value<String>>,
    /// Property [`TokenKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-tokenkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key: Option<::Value<String>>,
    /// Property [`TokenKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html#cfn-pinpoint-apnssandboxchannel-tokenkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key_id: Option<::Value<String>>,
}

impl ::serde::Serialize for APNSSandboxChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref bundle_id) = self.bundle_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", bundle_id)?;
        }
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref default_authentication_method) = self.default_authentication_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAuthenticationMethod", default_authentication_method)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref private_key) = self.private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
        }
        if let Some(ref team_id) = self.team_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", team_id)?;
        }
        if let Some(ref token_key) = self.token_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKey", token_key)?;
        }
        if let Some(ref token_key_id) = self.token_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKeyId", token_key_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for APNSSandboxChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<APNSSandboxChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APNSSandboxChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type APNSSandboxChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut certificate: Option<::Value<String>> = None;
                let mut default_authentication_method: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut private_key: Option<::Value<String>> = None;
                let mut team_id: Option<::Value<String>> = None;
                let mut token_key: Option<::Value<String>> = None;
                let mut token_key_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAuthenticationMethod" => {
                            default_authentication_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateKey" => {
                            private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKey" => {
                            token_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKeyId" => {
                            token_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(APNSSandboxChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    bundle_id: bundle_id,
                    certificate: certificate,
                    default_authentication_method: default_authentication_method,
                    enabled: enabled,
                    private_key: private_key,
                    team_id: team_id,
                    token_key: token_key,
                    token_key_id: token_key_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for APNSSandboxChannel {
    type Properties = APNSSandboxChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::APNSSandboxChannel";
    fn properties(&self) -> &APNSSandboxChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut APNSSandboxChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for APNSSandboxChannel {}

impl From<APNSSandboxChannelProperties> for APNSSandboxChannel {
    fn from(properties: APNSSandboxChannelProperties) -> APNSSandboxChannel {
        APNSSandboxChannel { properties }
    }
}

/// The [`AWS::Pinpoint::APNSVoipChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html) resource type.
#[derive(Debug, Default)]
pub struct APNSVoipChannel {
    properties: APNSVoipChannelProperties
}

/// Properties for the `APNSVoipChannel` resource.
#[derive(Debug, Default)]
pub struct APNSVoipChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: Option<::Value<String>>,
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`DefaultAuthenticationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-defaultauthenticationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_authentication_method: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-privatekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub private_key: Option<::Value<String>>,
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-teamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub team_id: Option<::Value<String>>,
    /// Property [`TokenKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-tokenkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key: Option<::Value<String>>,
    /// Property [`TokenKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html#cfn-pinpoint-apnsvoipchannel-tokenkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key_id: Option<::Value<String>>,
}

impl ::serde::Serialize for APNSVoipChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref bundle_id) = self.bundle_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", bundle_id)?;
        }
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref default_authentication_method) = self.default_authentication_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAuthenticationMethod", default_authentication_method)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref private_key) = self.private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
        }
        if let Some(ref team_id) = self.team_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", team_id)?;
        }
        if let Some(ref token_key) = self.token_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKey", token_key)?;
        }
        if let Some(ref token_key_id) = self.token_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKeyId", token_key_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for APNSVoipChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<APNSVoipChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APNSVoipChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type APNSVoipChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut certificate: Option<::Value<String>> = None;
                let mut default_authentication_method: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut private_key: Option<::Value<String>> = None;
                let mut team_id: Option<::Value<String>> = None;
                let mut token_key: Option<::Value<String>> = None;
                let mut token_key_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAuthenticationMethod" => {
                            default_authentication_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateKey" => {
                            private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKey" => {
                            token_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKeyId" => {
                            token_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(APNSVoipChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    bundle_id: bundle_id,
                    certificate: certificate,
                    default_authentication_method: default_authentication_method,
                    enabled: enabled,
                    private_key: private_key,
                    team_id: team_id,
                    token_key: token_key,
                    token_key_id: token_key_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for APNSVoipChannel {
    type Properties = APNSVoipChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::APNSVoipChannel";
    fn properties(&self) -> &APNSVoipChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut APNSVoipChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for APNSVoipChannel {}

impl From<APNSVoipChannelProperties> for APNSVoipChannel {
    fn from(properties: APNSVoipChannelProperties) -> APNSVoipChannel {
        APNSVoipChannel { properties }
    }
}

/// The [`AWS::Pinpoint::APNSVoipSandboxChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html) resource type.
#[derive(Debug, Default)]
pub struct APNSVoipSandboxChannel {
    properties: APNSVoipSandboxChannelProperties
}

/// Properties for the `APNSVoipSandboxChannel` resource.
#[derive(Debug, Default)]
pub struct APNSVoipSandboxChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-bundleid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bundle_id: Option<::Value<String>>,
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`DefaultAuthenticationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-defaultauthenticationmethod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_authentication_method: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-privatekey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub private_key: Option<::Value<String>>,
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-teamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub team_id: Option<::Value<String>>,
    /// Property [`TokenKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-tokenkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key: Option<::Value<String>>,
    /// Property [`TokenKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html#cfn-pinpoint-apnsvoipsandboxchannel-tokenkeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key_id: Option<::Value<String>>,
}

impl ::serde::Serialize for APNSVoipSandboxChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref bundle_id) = self.bundle_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", bundle_id)?;
        }
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref default_authentication_method) = self.default_authentication_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAuthenticationMethod", default_authentication_method)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref private_key) = self.private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
        }
        if let Some(ref team_id) = self.team_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", team_id)?;
        }
        if let Some(ref token_key) = self.token_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKey", token_key)?;
        }
        if let Some(ref token_key_id) = self.token_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKeyId", token_key_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for APNSVoipSandboxChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<APNSVoipSandboxChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = APNSVoipSandboxChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type APNSVoipSandboxChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut bundle_id: Option<::Value<String>> = None;
                let mut certificate: Option<::Value<String>> = None;
                let mut default_authentication_method: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut private_key: Option<::Value<String>> = None;
                let mut team_id: Option<::Value<String>> = None;
                let mut token_key: Option<::Value<String>> = None;
                let mut token_key_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAuthenticationMethod" => {
                            default_authentication_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateKey" => {
                            private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKey" => {
                            token_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKeyId" => {
                            token_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(APNSVoipSandboxChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    bundle_id: bundle_id,
                    certificate: certificate,
                    default_authentication_method: default_authentication_method,
                    enabled: enabled,
                    private_key: private_key,
                    team_id: team_id,
                    token_key: token_key,
                    token_key_id: token_key_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for APNSVoipSandboxChannel {
    type Properties = APNSVoipSandboxChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::APNSVoipSandboxChannel";
    fn properties(&self) -> &APNSVoipSandboxChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut APNSVoipSandboxChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for APNSVoipSandboxChannel {}

impl From<APNSVoipSandboxChannelProperties> for APNSVoipSandboxChannel {
    fn from(properties: APNSVoipSandboxChannelProperties) -> APNSVoipSandboxChannel {
        APNSVoipSandboxChannel { properties }
    }
}

/// The [`AWS::Pinpoint::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-app.html) resource type.
#[derive(Debug, Default)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Default)]
pub struct AppProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-app.html#cfn-pinpoint-app-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-app.html#cfn-pinpoint-app-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::Pinpoint::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::Pinpoint::ApplicationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationSettings {
    properties: ApplicationSettingsProperties
}

/// Properties for the `ApplicationSettings` resource.
#[derive(Debug, Default)]
pub struct ApplicationSettingsProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html#cfn-pinpoint-applicationsettings-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`CampaignHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html#cfn-pinpoint-applicationsettings-campaignhook).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub campaign_hook: Option<::Value<self::application_settings::CampaignHook>>,
    /// Property [`CloudWatchMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html#cfn-pinpoint-applicationsettings-cloudwatchmetricsenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_metrics_enabled: Option<::Value<bool>>,
    /// Property [`Limits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html#cfn-pinpoint-applicationsettings-limits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub limits: Option<::Value<self::application_settings::Limits>>,
    /// Property [`QuietTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html#cfn-pinpoint-applicationsettings-quiettime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quiet_time: Option<::Value<self::application_settings::QuietTime>>,
}

impl ::serde::Serialize for ApplicationSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref campaign_hook) = self.campaign_hook {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CampaignHook", campaign_hook)?;
        }
        if let Some(ref cloud_watch_metrics_enabled) = self.cloud_watch_metrics_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchMetricsEnabled", cloud_watch_metrics_enabled)?;
        }
        if let Some(ref limits) = self.limits {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limits", limits)?;
        }
        if let Some(ref quiet_time) = self.quiet_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuietTime", quiet_time)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut campaign_hook: Option<::Value<self::application_settings::CampaignHook>> = None;
                let mut cloud_watch_metrics_enabled: Option<::Value<bool>> = None;
                let mut limits: Option<::Value<self::application_settings::Limits>> = None;
                let mut quiet_time: Option<::Value<self::application_settings::QuietTime>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CampaignHook" => {
                            campaign_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchMetricsEnabled" => {
                            cloud_watch_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Limits" => {
                            limits = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QuietTime" => {
                            quiet_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationSettingsProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    campaign_hook: campaign_hook,
                    cloud_watch_metrics_enabled: cloud_watch_metrics_enabled,
                    limits: limits,
                    quiet_time: quiet_time,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationSettings {
    type Properties = ApplicationSettingsProperties;
    const TYPE: &'static str = "AWS::Pinpoint::ApplicationSettings";
    fn properties(&self) -> &ApplicationSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationSettings {}

impl From<ApplicationSettingsProperties> for ApplicationSettings {
    fn from(properties: ApplicationSettingsProperties) -> ApplicationSettings {
        ApplicationSettings { properties }
    }
}

/// The [`AWS::Pinpoint::BaiduChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html) resource type.
#[derive(Debug, Default)]
pub struct BaiduChannel {
    properties: BaiduChannelProperties
}

/// Properties for the `BaiduChannel` resource.
#[derive(Debug, Default)]
pub struct BaiduChannelProperties {
    /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html#cfn-pinpoint-baiduchannel-apikey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key: ::Value<String>,
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html#cfn-pinpoint-baiduchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html#cfn-pinpoint-baiduchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`SecretKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html#cfn-pinpoint-baiduchannel-secretkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secret_key: ::Value<String>,
}

impl ::serde::Serialize for BaiduChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretKey", &self.secret_key)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BaiduChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BaiduChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BaiduChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BaiduChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_key: Option<::Value<String>> = None;
                let mut application_id: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut secret_key: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKey" => {
                            api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretKey" => {
                            secret_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BaiduChannelProperties {
                    api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    enabled: enabled,
                    secret_key: secret_key.ok_or(::serde::de::Error::missing_field("SecretKey"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BaiduChannel {
    type Properties = BaiduChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::BaiduChannel";
    fn properties(&self) -> &BaiduChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BaiduChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BaiduChannel {}

impl From<BaiduChannelProperties> for BaiduChannel {
    fn from(properties: BaiduChannelProperties) -> BaiduChannel {
        BaiduChannel { properties }
    }
}

/// The [`AWS::Pinpoint::Campaign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html) resource type.
#[derive(Debug, Default)]
pub struct Campaign {
    properties: CampaignProperties
}

/// Properties for the `Campaign` resource.
#[derive(Debug, Default)]
pub struct CampaignProperties {
    /// Property [`AdditionalTreatments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-additionaltreatments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_treatments: Option<::ValueList<self::campaign::WriteTreatmentResource>>,
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`CampaignHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-campaignhook).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub campaign_hook: Option<::Value<self::campaign::CampaignHook>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`HoldoutPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-holdoutpercent).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub holdout_percent: Option<::Value<u32>>,
    /// Property [`IsPaused`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-ispaused).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_paused: Option<::Value<bool>>,
    /// Property [`Limits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-limits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub limits: Option<::Value<self::campaign::Limits>>,
    /// Property [`MessageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-messageconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub message_configuration: ::Value<self::campaign::MessageConfiguration>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-priority).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority: Option<::Value<u32>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: ::Value<self::campaign::Schedule>,
    /// Property [`SegmentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-segmentid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment_id: ::Value<String>,
    /// Property [`SegmentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-segmentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment_version: Option<::Value<u32>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TreatmentDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-treatmentdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treatment_description: Option<::Value<String>>,
    /// Property [`TreatmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html#cfn-pinpoint-campaign-treatmentname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treatment_name: Option<::Value<String>>,
}

impl ::serde::Serialize for CampaignProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_treatments) = self.additional_treatments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalTreatments", additional_treatments)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref campaign_hook) = self.campaign_hook {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CampaignHook", campaign_hook)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref holdout_percent) = self.holdout_percent {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HoldoutPercent", holdout_percent)?;
        }
        if let Some(ref is_paused) = self.is_paused {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsPaused", is_paused)?;
        }
        if let Some(ref limits) = self.limits {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limits", limits)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageConfiguration", &self.message_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref priority) = self.priority {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", &self.schedule)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentId", &self.segment_id)?;
        if let Some(ref segment_version) = self.segment_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentVersion", segment_version)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref treatment_description) = self.treatment_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentDescription", treatment_description)?;
        }
        if let Some(ref treatment_name) = self.treatment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentName", treatment_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CampaignProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CampaignProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CampaignProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_treatments: Option<::ValueList<self::campaign::WriteTreatmentResource>> = None;
                let mut application_id: Option<::Value<String>> = None;
                let mut campaign_hook: Option<::Value<self::campaign::CampaignHook>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut holdout_percent: Option<::Value<u32>> = None;
                let mut is_paused: Option<::Value<bool>> = None;
                let mut limits: Option<::Value<self::campaign::Limits>> = None;
                let mut message_configuration: Option<::Value<self::campaign::MessageConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut priority: Option<::Value<u32>> = None;
                let mut schedule: Option<::Value<self::campaign::Schedule>> = None;
                let mut segment_id: Option<::Value<String>> = None;
                let mut segment_version: Option<::Value<u32>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut treatment_description: Option<::Value<String>> = None;
                let mut treatment_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalTreatments" => {
                            additional_treatments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CampaignHook" => {
                            campaign_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HoldoutPercent" => {
                            holdout_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsPaused" => {
                            is_paused = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Limits" => {
                            limits = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MessageConfiguration" => {
                            message_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SegmentId" => {
                            segment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SegmentVersion" => {
                            segment_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TreatmentDescription" => {
                            treatment_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TreatmentName" => {
                            treatment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CampaignProperties {
                    additional_treatments: additional_treatments,
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    campaign_hook: campaign_hook,
                    description: description,
                    holdout_percent: holdout_percent,
                    is_paused: is_paused,
                    limits: limits,
                    message_configuration: message_configuration.ok_or(::serde::de::Error::missing_field("MessageConfiguration"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    priority: priority,
                    schedule: schedule.ok_or(::serde::de::Error::missing_field("Schedule"))?,
                    segment_id: segment_id.ok_or(::serde::de::Error::missing_field("SegmentId"))?,
                    segment_version: segment_version,
                    tags: tags,
                    treatment_description: treatment_description,
                    treatment_name: treatment_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Campaign {
    type Properties = CampaignProperties;
    const TYPE: &'static str = "AWS::Pinpoint::Campaign";
    fn properties(&self) -> &CampaignProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CampaignProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Campaign {}

impl From<CampaignProperties> for Campaign {
    fn from(properties: CampaignProperties) -> Campaign {
        Campaign { properties }
    }
}

/// The [`AWS::Pinpoint::EmailChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html) resource type.
#[derive(Debug, Default)]
pub struct EmailChannel {
    properties: EmailChannelProperties
}

/// Properties for the `EmailChannel` resource.
#[derive(Debug, Default)]
pub struct EmailChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-configurationset).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration_set: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`FromAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-fromaddress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub from_address: ::Value<String>,
    /// Property [`Identity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-identity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html#cfn-pinpoint-emailchannel-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for EmailChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref configuration_set) = self.configuration_set {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSet", configuration_set)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromAddress", &self.from_address)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identity", &self.identity)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EmailChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EmailChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EmailChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EmailChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut configuration_set: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut from_address: Option<::Value<String>> = None;
                let mut identity: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationSet" => {
                            configuration_set = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FromAddress" => {
                            from_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Identity" => {
                            identity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EmailChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    configuration_set: configuration_set,
                    enabled: enabled,
                    from_address: from_address.ok_or(::serde::de::Error::missing_field("FromAddress"))?,
                    identity: identity.ok_or(::serde::de::Error::missing_field("Identity"))?,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EmailChannel {
    type Properties = EmailChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::EmailChannel";
    fn properties(&self) -> &EmailChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EmailChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EmailChannel {}

impl From<EmailChannelProperties> for EmailChannel {
    fn from(properties: EmailChannelProperties) -> EmailChannel {
        EmailChannel { properties }
    }
}

/// The [`AWS::Pinpoint::EmailTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html) resource type.
#[derive(Debug, Default)]
pub struct EmailTemplate {
    properties: EmailTemplateProperties
}

/// Properties for the `EmailTemplate` resource.
#[derive(Debug, Default)]
pub struct EmailTemplateProperties {
    /// Property [`DefaultSubstitutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-defaultsubstitutions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_substitutions: Option<::Value<String>>,
    /// Property [`HtmlPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-htmlpart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub html_part: Option<::Value<String>>,
    /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-subject).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subject: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TemplateDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-templatedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_description: Option<::Value<String>>,
    /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-templatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_name: ::Value<String>,
    /// Property [`TextPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html#cfn-pinpoint-emailtemplate-textpart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub text_part: Option<::Value<String>>,
}

impl ::serde::Serialize for EmailTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref default_substitutions) = self.default_substitutions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubstitutions", default_substitutions)?;
        }
        if let Some(ref html_part) = self.html_part {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlPart", html_part)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", &self.subject)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_description) = self.template_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateDescription", template_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
        if let Some(ref text_part) = self.text_part {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextPart", text_part)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EmailTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EmailTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EmailTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EmailTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut default_substitutions: Option<::Value<String>> = None;
                let mut html_part: Option<::Value<String>> = None;
                let mut subject: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut template_description: Option<::Value<String>> = None;
                let mut template_name: Option<::Value<String>> = None;
                let mut text_part: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultSubstitutions" => {
                            default_substitutions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HtmlPart" => {
                            html_part = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subject" => {
                            subject = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateDescription" => {
                            template_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateName" => {
                            template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TextPart" => {
                            text_part = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EmailTemplateProperties {
                    default_substitutions: default_substitutions,
                    html_part: html_part,
                    subject: subject.ok_or(::serde::de::Error::missing_field("Subject"))?,
                    tags: tags,
                    template_description: template_description,
                    template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                    text_part: text_part,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EmailTemplate {
    type Properties = EmailTemplateProperties;
    const TYPE: &'static str = "AWS::Pinpoint::EmailTemplate";
    fn properties(&self) -> &EmailTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EmailTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EmailTemplate {}

impl From<EmailTemplateProperties> for EmailTemplate {
    fn from(properties: EmailTemplateProperties) -> EmailTemplate {
        EmailTemplate { properties }
    }
}

/// The [`AWS::Pinpoint::EventStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-eventstream.html) resource type.
#[derive(Debug, Default)]
pub struct EventStream {
    properties: EventStreamProperties
}

/// Properties for the `EventStream` resource.
#[derive(Debug, Default)]
pub struct EventStreamProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-eventstream.html#cfn-pinpoint-eventstream-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`DestinationStreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-eventstream.html#cfn-pinpoint-eventstream-destinationstreamarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_stream_arn: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-eventstream.html#cfn-pinpoint-eventstream-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for EventStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationStreamArn", &self.destination_stream_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventStreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventStreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventStreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventStreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut destination_stream_arn: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationStreamArn" => {
                            destination_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventStreamProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    destination_stream_arn: destination_stream_arn.ok_or(::serde::de::Error::missing_field("DestinationStreamArn"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventStream {
    type Properties = EventStreamProperties;
    const TYPE: &'static str = "AWS::Pinpoint::EventStream";
    fn properties(&self) -> &EventStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventStreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventStream {}

impl From<EventStreamProperties> for EventStream {
    fn from(properties: EventStreamProperties) -> EventStream {
        EventStream { properties }
    }
}

/// The [`AWS::Pinpoint::GCMChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-gcmchannel.html) resource type.
#[derive(Debug, Default)]
pub struct GCMChannel {
    properties: GCMChannelProperties
}

/// Properties for the `GCMChannel` resource.
#[derive(Debug, Default)]
pub struct GCMChannelProperties {
    /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-gcmchannel.html#cfn-pinpoint-gcmchannel-apikey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key: ::Value<String>,
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-gcmchannel.html#cfn-pinpoint-gcmchannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-gcmchannel.html#cfn-pinpoint-gcmchannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for GCMChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GCMChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GCMChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GCMChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GCMChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_key: Option<::Value<String>> = None;
                let mut application_id: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiKey" => {
                            api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GCMChannelProperties {
                    api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    enabled: enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GCMChannel {
    type Properties = GCMChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::GCMChannel";
    fn properties(&self) -> &GCMChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GCMChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GCMChannel {}

impl From<GCMChannelProperties> for GCMChannel {
    fn from(properties: GCMChannelProperties) -> GCMChannel {
        GCMChannel { properties }
    }
}

/// The [`AWS::Pinpoint::InAppTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html) resource type.
#[derive(Debug, Default)]
pub struct InAppTemplate {
    properties: InAppTemplateProperties
}

/// Properties for the `InAppTemplate` resource.
#[derive(Debug, Default)]
pub struct InAppTemplateProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: Option<::ValueList<self::in_app_template::InAppMessageContent>>,
    /// Property [`CustomConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-customconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_config: Option<::Value<::json::Value>>,
    /// Property [`Layout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-layout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub layout: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TemplateDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-templatedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_description: Option<::Value<String>>,
    /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html#cfn-pinpoint-inapptemplate-templatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_name: ::Value<String>,
}

impl ::serde::Serialize for InAppTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref content) = self.content {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", content)?;
        }
        if let Some(ref custom_config) = self.custom_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConfig", custom_config)?;
        }
        if let Some(ref layout) = self.layout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Layout", layout)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_description) = self.template_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateDescription", template_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InAppTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InAppTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InAppTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::ValueList<self::in_app_template::InAppMessageContent>> = None;
                let mut custom_config: Option<::Value<::json::Value>> = None;
                let mut layout: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut template_description: Option<::Value<String>> = None;
                let mut template_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomConfig" => {
                            custom_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Layout" => {
                            layout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateDescription" => {
                            template_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateName" => {
                            template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InAppTemplateProperties {
                    content: content,
                    custom_config: custom_config,
                    layout: layout,
                    tags: tags,
                    template_description: template_description,
                    template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InAppTemplate {
    type Properties = InAppTemplateProperties;
    const TYPE: &'static str = "AWS::Pinpoint::InAppTemplate";
    fn properties(&self) -> &InAppTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InAppTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InAppTemplate {}

impl From<InAppTemplateProperties> for InAppTemplate {
    fn from(properties: InAppTemplateProperties) -> InAppTemplate {
        InAppTemplate { properties }
    }
}

/// The [`AWS::Pinpoint::PushTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html) resource type.
#[derive(Debug, Default)]
pub struct PushTemplate {
    properties: PushTemplateProperties
}

/// Properties for the `PushTemplate` resource.
#[derive(Debug, Default)]
pub struct PushTemplateProperties {
    /// Property [`ADM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-adm).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub adm: Option<::Value<self::push_template::AndroidPushNotificationTemplate>>,
    /// Property [`APNS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-apns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub apns: Option<::Value<self::push_template::APNSPushNotificationTemplate>>,
    /// Property [`Baidu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-baidu).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub baidu: Option<::Value<self::push_template::AndroidPushNotificationTemplate>>,
    /// Property [`Default`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-default).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default: Option<::Value<self::push_template::DefaultPushNotificationTemplate>>,
    /// Property [`DefaultSubstitutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-defaultsubstitutions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_substitutions: Option<::Value<String>>,
    /// Property [`GCM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-gcm).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gcm: Option<::Value<self::push_template::AndroidPushNotificationTemplate>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TemplateDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-templatedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_description: Option<::Value<String>>,
    /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html#cfn-pinpoint-pushtemplate-templatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_name: ::Value<String>,
}

impl ::serde::Serialize for PushTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref adm) = self.adm {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ADM", adm)?;
        }
        if let Some(ref apns) = self.apns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "APNS", apns)?;
        }
        if let Some(ref baidu) = self.baidu {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Baidu", baidu)?;
        }
        if let Some(ref default) = self.default {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Default", default)?;
        }
        if let Some(ref default_substitutions) = self.default_substitutions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubstitutions", default_substitutions)?;
        }
        if let Some(ref gcm) = self.gcm {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GCM", gcm)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_description) = self.template_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateDescription", template_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PushTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PushTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PushTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PushTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut adm: Option<::Value<self::push_template::AndroidPushNotificationTemplate>> = None;
                let mut apns: Option<::Value<self::push_template::APNSPushNotificationTemplate>> = None;
                let mut baidu: Option<::Value<self::push_template::AndroidPushNotificationTemplate>> = None;
                let mut default: Option<::Value<self::push_template::DefaultPushNotificationTemplate>> = None;
                let mut default_substitutions: Option<::Value<String>> = None;
                let mut gcm: Option<::Value<self::push_template::AndroidPushNotificationTemplate>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut template_description: Option<::Value<String>> = None;
                let mut template_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ADM" => {
                            adm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "APNS" => {
                            apns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Baidu" => {
                            baidu = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Default" => {
                            default = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultSubstitutions" => {
                            default_substitutions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GCM" => {
                            gcm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateDescription" => {
                            template_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateName" => {
                            template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PushTemplateProperties {
                    adm: adm,
                    apns: apns,
                    baidu: baidu,
                    default: default,
                    default_substitutions: default_substitutions,
                    gcm: gcm,
                    tags: tags,
                    template_description: template_description,
                    template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PushTemplate {
    type Properties = PushTemplateProperties;
    const TYPE: &'static str = "AWS::Pinpoint::PushTemplate";
    fn properties(&self) -> &PushTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PushTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PushTemplate {}

impl From<PushTemplateProperties> for PushTemplate {
    fn from(properties: PushTemplateProperties) -> PushTemplate {
        PushTemplate { properties }
    }
}

/// The [`AWS::Pinpoint::SMSChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html) resource type.
#[derive(Debug, Default)]
pub struct SMSChannel {
    properties: SMSChannelProperties
}

/// Properties for the `SMSChannel` resource.
#[derive(Debug, Default)]
pub struct SMSChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html#cfn-pinpoint-smschannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html#cfn-pinpoint-smschannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`SenderId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html#cfn-pinpoint-smschannel-senderid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sender_id: Option<::Value<String>>,
    /// Property [`ShortCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html#cfn-pinpoint-smschannel-shortcode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub short_code: Option<::Value<String>>,
}

impl ::serde::Serialize for SMSChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref sender_id) = self.sender_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderId", sender_id)?;
        }
        if let Some(ref short_code) = self.short_code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShortCode", short_code)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SMSChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SMSChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SMSChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SMSChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut sender_id: Option<::Value<String>> = None;
                let mut short_code: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SenderId" => {
                            sender_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ShortCode" => {
                            short_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SMSChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    enabled: enabled,
                    sender_id: sender_id,
                    short_code: short_code,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SMSChannel {
    type Properties = SMSChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::SMSChannel";
    fn properties(&self) -> &SMSChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SMSChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SMSChannel {}

impl From<SMSChannelProperties> for SMSChannel {
    fn from(properties: SMSChannelProperties) -> SMSChannel {
        SMSChannel { properties }
    }
}

/// The [`AWS::Pinpoint::Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html) resource type.
#[derive(Debug, Default)]
pub struct Segment {
    properties: SegmentProperties
}

/// Properties for the `Segment` resource.
#[derive(Debug, Default)]
pub struct SegmentProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html#cfn-pinpoint-segment-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html#cfn-pinpoint-segment-dimensions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dimensions: Option<::Value<self::segment::SegmentDimensions>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html#cfn-pinpoint-segment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SegmentGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html#cfn-pinpoint-segment-segmentgroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment_groups: Option<::Value<self::segment::SegmentGroups>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html#cfn-pinpoint-segment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for SegmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref dimensions) = self.dimensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref segment_groups) = self.segment_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentGroups", segment_groups)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SegmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SegmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SegmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut dimensions: Option<::Value<self::segment::SegmentDimensions>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut segment_groups: Option<::Value<self::segment::SegmentGroups>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Dimensions" => {
                            dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SegmentGroups" => {
                            segment_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SegmentProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    dimensions: dimensions,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    segment_groups: segment_groups,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Segment {
    type Properties = SegmentProperties;
    const TYPE: &'static str = "AWS::Pinpoint::Segment";
    fn properties(&self) -> &SegmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SegmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Segment {}

impl From<SegmentProperties> for Segment {
    fn from(properties: SegmentProperties) -> Segment {
        Segment { properties }
    }
}

/// The [`AWS::Pinpoint::SmsTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html) resource type.
#[derive(Debug, Default)]
pub struct SmsTemplate {
    properties: SmsTemplateProperties
}

/// Properties for the `SmsTemplate` resource.
#[derive(Debug, Default)]
pub struct SmsTemplateProperties {
    /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html#cfn-pinpoint-smstemplate-body).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub body: ::Value<String>,
    /// Property [`DefaultSubstitutions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html#cfn-pinpoint-smstemplate-defaultsubstitutions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_substitutions: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html#cfn-pinpoint-smstemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TemplateDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html#cfn-pinpoint-smstemplate-templatedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_description: Option<::Value<String>>,
    /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html#cfn-pinpoint-smstemplate-templatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_name: ::Value<String>,
}

impl ::serde::Serialize for SmsTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", &self.body)?;
        if let Some(ref default_substitutions) = self.default_substitutions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubstitutions", default_substitutions)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_description) = self.template_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateDescription", template_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SmsTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SmsTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SmsTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SmsTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut body: Option<::Value<String>> = None;
                let mut default_substitutions: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut template_description: Option<::Value<String>> = None;
                let mut template_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Body" => {
                            body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultSubstitutions" => {
                            default_substitutions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateDescription" => {
                            template_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateName" => {
                            template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SmsTemplateProperties {
                    body: body.ok_or(::serde::de::Error::missing_field("Body"))?,
                    default_substitutions: default_substitutions,
                    tags: tags,
                    template_description: template_description,
                    template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SmsTemplate {
    type Properties = SmsTemplateProperties;
    const TYPE: &'static str = "AWS::Pinpoint::SmsTemplate";
    fn properties(&self) -> &SmsTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SmsTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SmsTemplate {}

impl From<SmsTemplateProperties> for SmsTemplate {
    fn from(properties: SmsTemplateProperties) -> SmsTemplate {
        SmsTemplate { properties }
    }
}

/// The [`AWS::Pinpoint::VoiceChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-voicechannel.html) resource type.
#[derive(Debug, Default)]
pub struct VoiceChannel {
    properties: VoiceChannelProperties
}

/// Properties for the `VoiceChannel` resource.
#[derive(Debug, Default)]
pub struct VoiceChannelProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-voicechannel.html#cfn-pinpoint-voicechannel-applicationid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-voicechannel.html#cfn-pinpoint-voicechannel-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for VoiceChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VoiceChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VoiceChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VoiceChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VoiceChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VoiceChannelProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    enabled: enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VoiceChannel {
    type Properties = VoiceChannelProperties;
    const TYPE: &'static str = "AWS::Pinpoint::VoiceChannel";
    fn properties(&self) -> &VoiceChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VoiceChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VoiceChannel {}

impl From<VoiceChannelProperties> for VoiceChannel {
    fn from(properties: VoiceChannelProperties) -> VoiceChannel {
        VoiceChannel { properties }
    }
}

pub mod application_settings {
    //! Property types for the `ApplicationSettings` resource.

    /// The [`AWS::Pinpoint::ApplicationSettings.CampaignHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-campaignhook.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignHook {
        /// Property [`LambdaFunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-campaignhook.html#cfn-pinpoint-applicationsettings-campaignhook-lambdafunctionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_name: Option<::Value<String>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-campaignhook.html#cfn-pinpoint-applicationsettings-campaignhook-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`WebUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-campaignhook.html#cfn-pinpoint-applicationsettings-campaignhook-weburl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_function_name) = self.lambda_function_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionName", lambda_function_name)?;
            }
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref web_url) = self.web_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebUrl", web_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_function_name: Option<::Value<String>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut web_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaFunctionName" => {
                                lambda_function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebUrl" => {
                                web_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignHook {
                        lambda_function_name: lambda_function_name,
                        mode: mode,
                        web_url: web_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::ApplicationSettings.Limits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html) property type.
    #[derive(Debug, Default)]
    pub struct Limits {
        /// Property [`Daily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html#cfn-pinpoint-applicationsettings-limits-daily).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily: Option<::Value<u32>>,
        /// Property [`MaximumDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html#cfn-pinpoint-applicationsettings-limits-maximumduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_duration: Option<::Value<u32>>,
        /// Property [`MessagesPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html#cfn-pinpoint-applicationsettings-limits-messagespersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub messages_per_second: Option<::Value<u32>>,
        /// Property [`Total`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html#cfn-pinpoint-applicationsettings-limits-total).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Limits {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref daily) = self.daily {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Daily", daily)?;
            }
            if let Some(ref maximum_duration) = self.maximum_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumDuration", maximum_duration)?;
            }
            if let Some(ref messages_per_second) = self.messages_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessagesPerSecond", messages_per_second)?;
            }
            if let Some(ref total) = self.total {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Total", total)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Limits {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Limits, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Limits;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Limits")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut daily: Option<::Value<u32>> = None;
                    let mut maximum_duration: Option<::Value<u32>> = None;
                    let mut messages_per_second: Option<::Value<u32>> = None;
                    let mut total: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Daily" => {
                                daily = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumDuration" => {
                                maximum_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessagesPerSecond" => {
                                messages_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Total" => {
                                total = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Limits {
                        daily: daily,
                        maximum_duration: maximum_duration,
                        messages_per_second: messages_per_second,
                        total: total,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::ApplicationSettings.QuietTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-quiettime.html) property type.
    #[derive(Debug, Default)]
    pub struct QuietTime {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-quiettime.html#cfn-pinpoint-applicationsettings-quiettime-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: ::Value<String>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-quiettime.html#cfn-pinpoint-applicationsettings-quiettime-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: ::Value<String>,
    }

    impl ::codec::SerializeValue for QuietTime {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", &self.end)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", &self.start)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuietTime {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuietTime, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuietTime;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuietTime")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<String>> = None;
                    let mut start: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QuietTime {
                        end: end.ok_or(::serde::de::Error::missing_field("End"))?,
                        start: start.ok_or(::serde::de::Error::missing_field("Start"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod campaign {
    //! Property types for the `Campaign` resource.

    /// The [`AWS::Pinpoint::Campaign.AttributeDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-attributedimension.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeDimension {
        /// Property [`AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-attributedimension.html#cfn-pinpoint-campaign-attributedimension-attributetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_type: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-attributedimension.html#cfn-pinpoint-campaign-attributedimension-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AttributeDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_type) = self.attribute_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeType", attribute_type)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_type: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeType" => {
                                attribute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeDimension {
                        attribute_type: attribute_type,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.CampaignEmailMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignEmailMessage {
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html#cfn-pinpoint-campaign-campaignemailmessage-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`FromAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html#cfn-pinpoint-campaign-campaignemailmessage-fromaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_address: Option<::Value<String>>,
        /// Property [`HtmlBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html#cfn-pinpoint-campaign-campaignemailmessage-htmlbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub html_body: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html#cfn-pinpoint-campaign-campaignemailmessage-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignEmailMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref from_address) = self.from_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromAddress", from_address)?;
            }
            if let Some(ref html_body) = self.html_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlBody", html_body)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignEmailMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignEmailMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignEmailMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignEmailMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut body: Option<::Value<String>> = None;
                    let mut from_address: Option<::Value<String>> = None;
                    let mut html_body: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FromAddress" => {
                                from_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HtmlBody" => {
                                html_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignEmailMessage {
                        body: body,
                        from_address: from_address,
                        html_body: html_body,
                        title: title,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.CampaignEventFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigneventfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignEventFilter {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigneventfilter.html#cfn-pinpoint-campaign-campaigneventfilter-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::Value<EventDimensions>>,
        /// Property [`FilterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigneventfilter.html#cfn-pinpoint-campaign-campaigneventfilter-filtertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignEventFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            if let Some(ref filter_type) = self.filter_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterType", filter_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignEventFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignEventFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignEventFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignEventFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<::Value<EventDimensions>> = None;
                    let mut filter_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterType" => {
                                filter_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignEventFilter {
                        dimensions: dimensions,
                        filter_type: filter_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.CampaignHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignhook.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignHook {
        /// Property [`LambdaFunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignhook.html#cfn-pinpoint-campaign-campaignhook-lambdafunctionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_name: Option<::Value<String>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignhook.html#cfn-pinpoint-campaign-campaignhook-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`WebUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignhook.html#cfn-pinpoint-campaign-campaignhook-weburl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_function_name) = self.lambda_function_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionName", lambda_function_name)?;
            }
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref web_url) = self.web_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebUrl", web_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_function_name: Option<::Value<String>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut web_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaFunctionName" => {
                                lambda_function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebUrl" => {
                                web_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignHook {
                        lambda_function_name: lambda_function_name,
                        mode: mode,
                        web_url: web_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.CampaignInAppMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigninappmessage.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignInAppMessage {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigninappmessage.html#cfn-pinpoint-campaign-campaigninappmessage-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: Option<::ValueList<InAppMessageContent>>,
        /// Property [`CustomConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigninappmessage.html#cfn-pinpoint-campaign-campaigninappmessage-customconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_config: Option<::Value<::json::Value>>,
        /// Property [`Layout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigninappmessage.html#cfn-pinpoint-campaign-campaigninappmessage-layout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub layout: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignInAppMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content) = self.content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", content)?;
            }
            if let Some(ref custom_config) = self.custom_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConfig", custom_config)?;
            }
            if let Some(ref layout) = self.layout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Layout", layout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignInAppMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignInAppMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignInAppMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignInAppMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::ValueList<InAppMessageContent>> = None;
                    let mut custom_config: Option<::Value<::json::Value>> = None;
                    let mut layout: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomConfig" => {
                                custom_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Layout" => {
                                layout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignInAppMessage {
                        content: content,
                        custom_config: custom_config,
                        layout: layout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.CampaignSmsMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html) property type.
    #[derive(Debug, Default)]
    pub struct CampaignSmsMessage {
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`EntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-entityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id: Option<::Value<String>>,
        /// Property [`MessageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-messagetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_type: Option<::Value<String>>,
        /// Property [`OriginationNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-originationnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub origination_number: Option<::Value<String>>,
        /// Property [`SenderId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-senderid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sender_id: Option<::Value<String>>,
        /// Property [`TemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html#cfn-pinpoint-campaign-campaignsmsmessage-templateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CampaignSmsMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref entity_id) = self.entity_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityId", entity_id)?;
            }
            if let Some(ref message_type) = self.message_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageType", message_type)?;
            }
            if let Some(ref origination_number) = self.origination_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginationNumber", origination_number)?;
            }
            if let Some(ref sender_id) = self.sender_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderId", sender_id)?;
            }
            if let Some(ref template_id) = self.template_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateId", template_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CampaignSmsMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignSmsMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CampaignSmsMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CampaignSmsMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut body: Option<::Value<String>> = None;
                    let mut entity_id: Option<::Value<String>> = None;
                    let mut message_type: Option<::Value<String>> = None;
                    let mut origination_number: Option<::Value<String>> = None;
                    let mut sender_id: Option<::Value<String>> = None;
                    let mut template_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityId" => {
                                entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageType" => {
                                message_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginationNumber" => {
                                origination_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SenderId" => {
                                sender_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateId" => {
                                template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CampaignSmsMessage {
                        body: body,
                        entity_id: entity_id,
                        message_type: message_type,
                        origination_number: origination_number,
                        sender_id: sender_id,
                        template_id: template_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.DefaultButtonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultButtonConfiguration {
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BorderRadius`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-borderradius).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub border_radius: Option<::Value<u32>>,
        /// Property [`ButtonAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-buttonaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub button_action: Option<::Value<String>>,
        /// Property [`Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-link).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub link: Option<::Value<String>>,
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html#cfn-pinpoint-campaign-defaultbuttonconfiguration-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DefaultButtonConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref border_radius) = self.border_radius {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BorderRadius", border_radius)?;
            }
            if let Some(ref button_action) = self.button_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ButtonAction", button_action)?;
            }
            if let Some(ref link) = self.link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Link", link)?;
            }
            if let Some(ref text) = self.text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", text)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultButtonConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultButtonConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultButtonConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultButtonConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut background_color: Option<::Value<String>> = None;
                    let mut border_radius: Option<::Value<u32>> = None;
                    let mut button_action: Option<::Value<String>> = None;
                    let mut link: Option<::Value<String>> = None;
                    let mut text: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BorderRadius" => {
                                border_radius = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ButtonAction" => {
                                button_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Link" => {
                                link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultButtonConfiguration {
                        background_color: background_color,
                        border_radius: border_radius,
                        button_action: button_action,
                        link: link,
                        text: text,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.EventDimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-eventdimensions.html) property type.
    #[derive(Debug, Default)]
    pub struct EventDimensions {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-eventdimensions.html#cfn-pinpoint-campaign-eventdimensions-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::Value<::json::Value>>,
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-eventdimensions.html#cfn-pinpoint-campaign-eventdimensions-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: Option<::Value<SetDimension>>,
        /// Property [`Metrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-eventdimensions.html#cfn-pinpoint-campaign-eventdimensions-metrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for EventDimensions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref metrics) = self.metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metrics", metrics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventDimensions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventDimensions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventDimensions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventDimensions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::Value<::json::Value>> = None;
                    let mut event_type: Option<::Value<SetDimension>> = None;
                    let mut metrics: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metrics" => {
                                metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventDimensions {
                        attributes: attributes,
                        event_type: event_type,
                        metrics: metrics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.InAppMessageBodyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebodyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct InAppMessageBodyConfig {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebodyconfig.html#cfn-pinpoint-campaign-inappmessagebodyconfig-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebodyconfig.html#cfn-pinpoint-campaign-inappmessagebodyconfig-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebodyconfig.html#cfn-pinpoint-campaign-inappmessagebodyconfig-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InAppMessageBodyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InAppMessageBodyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppMessageBodyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InAppMessageBodyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InAppMessageBodyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InAppMessageBodyConfig {
                        alignment: alignment,
                        body: body,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.InAppMessageButton`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html) property type.
    #[derive(Debug, Default)]
    pub struct InAppMessageButton {
        /// Property [`Android`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html#cfn-pinpoint-campaign-inappmessagebutton-android).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub android: Option<::Value<OverrideButtonConfiguration>>,
        /// Property [`DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html#cfn-pinpoint-campaign-inappmessagebutton-defaultconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_config: Option<::Value<DefaultButtonConfiguration>>,
        /// Property [`IOS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html#cfn-pinpoint-campaign-inappmessagebutton-ios).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ios: Option<::Value<OverrideButtonConfiguration>>,
        /// Property [`Web`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html#cfn-pinpoint-campaign-inappmessagebutton-web).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web: Option<::Value<OverrideButtonConfiguration>>,
    }

    impl ::codec::SerializeValue for InAppMessageButton {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref android) = self.android {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Android", android)?;
            }
            if let Some(ref default_config) = self.default_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultConfig", default_config)?;
            }
            if let Some(ref ios) = self.ios {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IOS", ios)?;
            }
            if let Some(ref web) = self.web {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Web", web)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InAppMessageButton {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppMessageButton, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InAppMessageButton;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InAppMessageButton")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut android: Option<::Value<OverrideButtonConfiguration>> = None;
                    let mut default_config: Option<::Value<DefaultButtonConfiguration>> = None;
                    let mut ios: Option<::Value<OverrideButtonConfiguration>> = None;
                    let mut web: Option<::Value<OverrideButtonConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Android" => {
                                android = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultConfig" => {
                                default_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IOS" => {
                                ios = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Web" => {
                                web = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InAppMessageButton {
                        android: android,
                        default_config: default_config,
                        ios: ios,
                        web: web,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.InAppMessageContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html) property type.
    #[derive(Debug, Default)]
    pub struct InAppMessageContent {
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BodyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-bodyconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body_config: Option<::Value<InAppMessageBodyConfig>>,
        /// Property [`HeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-headerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_config: Option<::Value<InAppMessageHeaderConfig>>,
        /// Property [`ImageUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-imageurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_url: Option<::Value<String>>,
        /// Property [`PrimaryBtn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-primarybtn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_btn: Option<::Value<InAppMessageButton>>,
        /// Property [`SecondaryBtn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html#cfn-pinpoint-campaign-inappmessagecontent-secondarybtn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_btn: Option<::Value<InAppMessageButton>>,
    }

    impl ::codec::SerializeValue for InAppMessageContent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref body_config) = self.body_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyConfig", body_config)?;
            }
            if let Some(ref header_config) = self.header_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderConfig", header_config)?;
            }
            if let Some(ref image_url) = self.image_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUrl", image_url)?;
            }
            if let Some(ref primary_btn) = self.primary_btn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryBtn", primary_btn)?;
            }
            if let Some(ref secondary_btn) = self.secondary_btn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryBtn", secondary_btn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InAppMessageContent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppMessageContent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InAppMessageContent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InAppMessageContent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut background_color: Option<::Value<String>> = None;
                    let mut body_config: Option<::Value<InAppMessageBodyConfig>> = None;
                    let mut header_config: Option<::Value<InAppMessageHeaderConfig>> = None;
                    let mut image_url: Option<::Value<String>> = None;
                    let mut primary_btn: Option<::Value<InAppMessageButton>> = None;
                    let mut secondary_btn: Option<::Value<InAppMessageButton>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BodyConfig" => {
                                body_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderConfig" => {
                                header_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUrl" => {
                                image_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryBtn" => {
                                primary_btn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryBtn" => {
                                secondary_btn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InAppMessageContent {
                        background_color: background_color,
                        body_config: body_config,
                        header_config: header_config,
                        image_url: image_url,
                        primary_btn: primary_btn,
                        secondary_btn: secondary_btn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.InAppMessageHeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessageheaderconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct InAppMessageHeaderConfig {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessageheaderconfig.html#cfn-pinpoint-campaign-inappmessageheaderconfig-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessageheaderconfig.html#cfn-pinpoint-campaign-inappmessageheaderconfig-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessageheaderconfig.html#cfn-pinpoint-campaign-inappmessageheaderconfig-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InAppMessageHeaderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InAppMessageHeaderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppMessageHeaderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InAppMessageHeaderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InAppMessageHeaderConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut header: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InAppMessageHeaderConfig {
                        alignment: alignment,
                        header: header,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.Limits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html) property type.
    #[derive(Debug, Default)]
    pub struct Limits {
        /// Property [`Daily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html#cfn-pinpoint-campaign-limits-daily).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily: Option<::Value<u32>>,
        /// Property [`MaximumDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html#cfn-pinpoint-campaign-limits-maximumduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_duration: Option<::Value<u32>>,
        /// Property [`MessagesPerSecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html#cfn-pinpoint-campaign-limits-messagespersecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub messages_per_second: Option<::Value<u32>>,
        /// Property [`Session`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html#cfn-pinpoint-campaign-limits-session).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session: Option<::Value<u32>>,
        /// Property [`Total`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html#cfn-pinpoint-campaign-limits-total).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Limits {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref daily) = self.daily {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Daily", daily)?;
            }
            if let Some(ref maximum_duration) = self.maximum_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumDuration", maximum_duration)?;
            }
            if let Some(ref messages_per_second) = self.messages_per_second {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessagesPerSecond", messages_per_second)?;
            }
            if let Some(ref session) = self.session {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Session", session)?;
            }
            if let Some(ref total) = self.total {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Total", total)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Limits {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Limits, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Limits;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Limits")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut daily: Option<::Value<u32>> = None;
                    let mut maximum_duration: Option<::Value<u32>> = None;
                    let mut messages_per_second: Option<::Value<u32>> = None;
                    let mut session: Option<::Value<u32>> = None;
                    let mut total: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Daily" => {
                                daily = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumDuration" => {
                                maximum_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessagesPerSecond" => {
                                messages_per_second = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Session" => {
                                session = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Total" => {
                                total = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Limits {
                        daily: daily,
                        maximum_duration: maximum_duration,
                        messages_per_second: messages_per_second,
                        session: session,
                        total: total,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html) property type.
    #[derive(Debug, Default)]
    pub struct Message {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`ImageIconUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-imageiconurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_icon_url: Option<::Value<String>>,
        /// Property [`ImageSmallIconUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-imagesmalliconurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_small_icon_url: Option<::Value<String>>,
        /// Property [`ImageUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-imageurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_url: Option<::Value<String>>,
        /// Property [`JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-jsonbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_body: Option<::Value<String>>,
        /// Property [`MediaUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-mediaurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_url: Option<::Value<String>>,
        /// Property [`RawContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-rawcontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub raw_content: Option<::Value<String>>,
        /// Property [`SilentPush`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-silentpush).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub silent_push: Option<::Value<bool>>,
        /// Property [`TimeToLive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-timetolive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_to_live: Option<::Value<u32>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html#cfn-pinpoint-campaign-message-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Message {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref image_icon_url) = self.image_icon_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageIconUrl", image_icon_url)?;
            }
            if let Some(ref image_small_icon_url) = self.image_small_icon_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageSmallIconUrl", image_small_icon_url)?;
            }
            if let Some(ref image_url) = self.image_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUrl", image_url)?;
            }
            if let Some(ref json_body) = self.json_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonBody", json_body)?;
            }
            if let Some(ref media_url) = self.media_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaUrl", media_url)?;
            }
            if let Some(ref raw_content) = self.raw_content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RawContent", raw_content)?;
            }
            if let Some(ref silent_push) = self.silent_push {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SilentPush", silent_push)?;
            }
            if let Some(ref time_to_live) = self.time_to_live {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeToLive", time_to_live)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Message {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Message, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Message;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Message")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut image_icon_url: Option<::Value<String>> = None;
                    let mut image_small_icon_url: Option<::Value<String>> = None;
                    let mut image_url: Option<::Value<String>> = None;
                    let mut json_body: Option<::Value<String>> = None;
                    let mut media_url: Option<::Value<String>> = None;
                    let mut raw_content: Option<::Value<String>> = None;
                    let mut silent_push: Option<::Value<bool>> = None;
                    let mut time_to_live: Option<::Value<u32>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageIconUrl" => {
                                image_icon_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageSmallIconUrl" => {
                                image_small_icon_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUrl" => {
                                image_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JsonBody" => {
                                json_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaUrl" => {
                                media_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RawContent" => {
                                raw_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SilentPush" => {
                                silent_push = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeToLive" => {
                                time_to_live = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Message {
                        action: action,
                        body: body,
                        image_icon_url: image_icon_url,
                        image_small_icon_url: image_small_icon_url,
                        image_url: image_url,
                        json_body: json_body,
                        media_url: media_url,
                        raw_content: raw_content,
                        silent_push: silent_push,
                        time_to_live: time_to_live,
                        title: title,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.MessageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MessageConfiguration {
        /// Property [`ADMMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-admmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adm_message: Option<::Value<Message>>,
        /// Property [`APNSMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-apnsmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apns_message: Option<::Value<Message>>,
        /// Property [`BaiduMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-baidumessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub baidu_message: Option<::Value<Message>>,
        /// Property [`DefaultMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-defaultmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_message: Option<::Value<Message>>,
        /// Property [`EmailMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-emailmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_message: Option<::Value<CampaignEmailMessage>>,
        /// Property [`GCMMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-gcmmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gcm_message: Option<::Value<Message>>,
        /// Property [`InAppMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-inappmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub in_app_message: Option<::Value<CampaignInAppMessage>>,
        /// Property [`SMSMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html#cfn-pinpoint-campaign-messageconfiguration-smsmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sms_message: Option<::Value<CampaignSmsMessage>>,
    }

    impl ::codec::SerializeValue for MessageConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adm_message) = self.adm_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ADMMessage", adm_message)?;
            }
            if let Some(ref apns_message) = self.apns_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "APNSMessage", apns_message)?;
            }
            if let Some(ref baidu_message) = self.baidu_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaiduMessage", baidu_message)?;
            }
            if let Some(ref default_message) = self.default_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultMessage", default_message)?;
            }
            if let Some(ref email_message) = self.email_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailMessage", email_message)?;
            }
            if let Some(ref gcm_message) = self.gcm_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GCMMessage", gcm_message)?;
            }
            if let Some(ref in_app_message) = self.in_app_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InAppMessage", in_app_message)?;
            }
            if let Some(ref sms_message) = self.sms_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SMSMessage", sms_message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MessageConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MessageConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MessageConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MessageConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adm_message: Option<::Value<Message>> = None;
                    let mut apns_message: Option<::Value<Message>> = None;
                    let mut baidu_message: Option<::Value<Message>> = None;
                    let mut default_message: Option<::Value<Message>> = None;
                    let mut email_message: Option<::Value<CampaignEmailMessage>> = None;
                    let mut gcm_message: Option<::Value<Message>> = None;
                    let mut in_app_message: Option<::Value<CampaignInAppMessage>> = None;
                    let mut sms_message: Option<::Value<CampaignSmsMessage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ADMMessage" => {
                                adm_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "APNSMessage" => {
                                apns_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BaiduMessage" => {
                                baidu_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultMessage" => {
                                default_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailMessage" => {
                                email_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GCMMessage" => {
                                gcm_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InAppMessage" => {
                                in_app_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SMSMessage" => {
                                sms_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MessageConfiguration {
                        adm_message: adm_message,
                        apns_message: apns_message,
                        baidu_message: baidu_message,
                        default_message: default_message,
                        email_message: email_message,
                        gcm_message: gcm_message,
                        in_app_message: in_app_message,
                        sms_message: sms_message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-metricdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-metricdimension.html#cfn-pinpoint-campaign-metricdimension-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-metricdimension.html#cfn-pinpoint-campaign-metricdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comparison_operator) = self.comparison_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", comparison_operator)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDimension {
                        comparison_operator: comparison_operator,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.OverrideButtonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-overridebuttonconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OverrideButtonConfiguration {
        /// Property [`ButtonAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-overridebuttonconfiguration.html#cfn-pinpoint-campaign-overridebuttonconfiguration-buttonaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub button_action: Option<::Value<String>>,
        /// Property [`Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-overridebuttonconfiguration.html#cfn-pinpoint-campaign-overridebuttonconfiguration-link).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub link: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OverrideButtonConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref button_action) = self.button_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ButtonAction", button_action)?;
            }
            if let Some(ref link) = self.link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Link", link)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OverrideButtonConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OverrideButtonConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OverrideButtonConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OverrideButtonConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut button_action: Option<::Value<String>> = None;
                    let mut link: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ButtonAction" => {
                                button_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Link" => {
                                link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OverrideButtonConfiguration {
                        button_action: button_action,
                        link: link,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.QuietTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule-quiettime.html) property type.
    #[derive(Debug, Default)]
    pub struct QuietTime {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule-quiettime.html#cfn-pinpoint-campaign-schedule-quiettime-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: ::Value<String>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule-quiettime.html#cfn-pinpoint-campaign-schedule-quiettime-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: ::Value<String>,
    }

    impl ::codec::SerializeValue for QuietTime {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", &self.end)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", &self.start)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuietTime {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuietTime, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuietTime;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuietTime")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<String>> = None;
                    let mut start: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QuietTime {
                        end: end.ok_or(::serde::de::Error::missing_field("End"))?,
                        start: start.ok_or(::serde::de::Error::missing_field("Start"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: Option<::Value<String>>,
        /// Property [`EventFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-eventfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_filter: Option<::Value<CampaignEventFilter>>,
        /// Property [`Frequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-frequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frequency: Option<::Value<String>>,
        /// Property [`IsLocalTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-islocaltime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_local_time: Option<::Value<bool>>,
        /// Property [`QuietTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-quiettime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quiet_time: Option<::Value<QuietTime>>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: Option<::Value<String>>,
        /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html#cfn-pinpoint-campaign-schedule-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_zone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_time) = self.end_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", end_time)?;
            }
            if let Some(ref event_filter) = self.event_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventFilter", event_filter)?;
            }
            if let Some(ref frequency) = self.frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Frequency", frequency)?;
            }
            if let Some(ref is_local_time) = self.is_local_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLocalTime", is_local_time)?;
            }
            if let Some(ref quiet_time) = self.quiet_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuietTime", quiet_time)?;
            }
            if let Some(ref start_time) = self.start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
            }
            if let Some(ref time_zone) = self.time_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", time_zone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time: Option<::Value<String>> = None;
                    let mut event_filter: Option<::Value<CampaignEventFilter>> = None;
                    let mut frequency: Option<::Value<String>> = None;
                    let mut is_local_time: Option<::Value<bool>> = None;
                    let mut quiet_time: Option<::Value<QuietTime>> = None;
                    let mut start_time: Option<::Value<String>> = None;
                    let mut time_zone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventFilter" => {
                                event_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Frequency" => {
                                frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsLocalTime" => {
                                is_local_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuietTime" => {
                                quiet_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeZone" => {
                                time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        end_time: end_time,
                        event_filter: event_filter,
                        frequency: frequency,
                        is_local_time: is_local_time,
                        quiet_time: quiet_time,
                        start_time: start_time,
                        time_zone: time_zone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.SetDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-setdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct SetDimension {
        /// Property [`DimensionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-setdimension.html#cfn-pinpoint-campaign-setdimension-dimensiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_type: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-setdimension.html#cfn-pinpoint-campaign-setdimension-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SetDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimension_type) = self.dimension_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionType", dimension_type)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SetDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SetDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SetDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SetDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_type: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionType" => {
                                dimension_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SetDimension {
                        dimension_type: dimension_type,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Campaign.WriteTreatmentResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html) property type.
    #[derive(Debug, Default)]
    pub struct WriteTreatmentResource {
        /// Property [`MessageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html#cfn-pinpoint-campaign-writetreatmentresource-messageconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_configuration: Option<::Value<MessageConfiguration>>,
        /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html#cfn-pinpoint-campaign-writetreatmentresource-schedule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule: Option<::Value<Schedule>>,
        /// Property [`SizePercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html#cfn-pinpoint-campaign-writetreatmentresource-sizepercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_percent: Option<::Value<u32>>,
        /// Property [`TreatmentDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html#cfn-pinpoint-campaign-writetreatmentresource-treatmentdescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub treatment_description: Option<::Value<String>>,
        /// Property [`TreatmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html#cfn-pinpoint-campaign-writetreatmentresource-treatmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub treatment_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WriteTreatmentResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_configuration) = self.message_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageConfiguration", message_configuration)?;
            }
            if let Some(ref schedule) = self.schedule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
            }
            if let Some(ref size_percent) = self.size_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizePercent", size_percent)?;
            }
            if let Some(ref treatment_description) = self.treatment_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentDescription", treatment_description)?;
            }
            if let Some(ref treatment_name) = self.treatment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentName", treatment_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WriteTreatmentResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WriteTreatmentResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WriteTreatmentResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WriteTreatmentResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_configuration: Option<::Value<MessageConfiguration>> = None;
                    let mut schedule: Option<::Value<Schedule>> = None;
                    let mut size_percent: Option<::Value<u32>> = None;
                    let mut treatment_description: Option<::Value<String>> = None;
                    let mut treatment_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageConfiguration" => {
                                message_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schedule" => {
                                schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizePercent" => {
                                size_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TreatmentDescription" => {
                                treatment_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TreatmentName" => {
                                treatment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WriteTreatmentResource {
                        message_configuration: message_configuration,
                        schedule: schedule,
                        size_percent: size_percent,
                        treatment_description: treatment_description,
                        treatment_name: treatment_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod in_app_template {
    //! Property types for the `InAppTemplate` resource.

    /// The [`AWS::Pinpoint::InAppTemplate.BodyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-bodyconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct BodyConfig {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-bodyconfig.html#cfn-pinpoint-inapptemplate-bodyconfig-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-bodyconfig.html#cfn-pinpoint-inapptemplate-bodyconfig-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-bodyconfig.html#cfn-pinpoint-inapptemplate-bodyconfig-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BodyConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BodyConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BodyConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BodyConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BodyConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BodyConfig {
                        alignment: alignment,
                        body: body,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::InAppTemplate.ButtonConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ButtonConfig {
        /// Property [`Android`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html#cfn-pinpoint-inapptemplate-buttonconfig-android).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub android: Option<::Value<OverrideButtonConfiguration>>,
        /// Property [`DefaultConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html#cfn-pinpoint-inapptemplate-buttonconfig-defaultconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_config: Option<::Value<DefaultButtonConfiguration>>,
        /// Property [`IOS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html#cfn-pinpoint-inapptemplate-buttonconfig-ios).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ios: Option<::Value<OverrideButtonConfiguration>>,
        /// Property [`Web`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html#cfn-pinpoint-inapptemplate-buttonconfig-web).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web: Option<::Value<OverrideButtonConfiguration>>,
    }

    impl ::codec::SerializeValue for ButtonConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref android) = self.android {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Android", android)?;
            }
            if let Some(ref default_config) = self.default_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultConfig", default_config)?;
            }
            if let Some(ref ios) = self.ios {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IOS", ios)?;
            }
            if let Some(ref web) = self.web {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Web", web)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ButtonConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ButtonConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ButtonConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ButtonConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut android: Option<::Value<OverrideButtonConfiguration>> = None;
                    let mut default_config: Option<::Value<DefaultButtonConfiguration>> = None;
                    let mut ios: Option<::Value<OverrideButtonConfiguration>> = None;
                    let mut web: Option<::Value<OverrideButtonConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Android" => {
                                android = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultConfig" => {
                                default_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IOS" => {
                                ios = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Web" => {
                                web = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ButtonConfig {
                        android: android,
                        default_config: default_config,
                        ios: ios,
                        web: web,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::InAppTemplate.DefaultButtonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultButtonConfiguration {
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BorderRadius`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-borderradius).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub border_radius: Option<::Value<u32>>,
        /// Property [`ButtonAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-buttonaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub button_action: Option<::Value<String>>,
        /// Property [`Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-link).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub link: Option<::Value<String>>,
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html#cfn-pinpoint-inapptemplate-defaultbuttonconfiguration-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DefaultButtonConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref border_radius) = self.border_radius {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BorderRadius", border_radius)?;
            }
            if let Some(ref button_action) = self.button_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ButtonAction", button_action)?;
            }
            if let Some(ref link) = self.link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Link", link)?;
            }
            if let Some(ref text) = self.text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", text)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultButtonConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultButtonConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultButtonConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultButtonConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut background_color: Option<::Value<String>> = None;
                    let mut border_radius: Option<::Value<u32>> = None;
                    let mut button_action: Option<::Value<String>> = None;
                    let mut link: Option<::Value<String>> = None;
                    let mut text: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BorderRadius" => {
                                border_radius = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ButtonAction" => {
                                button_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Link" => {
                                link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultButtonConfiguration {
                        background_color: background_color,
                        border_radius: border_radius,
                        button_action: button_action,
                        link: link,
                        text: text,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::InAppTemplate.HeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-headerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HeaderConfig {
        /// Property [`Alignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-headerconfig.html#cfn-pinpoint-inapptemplate-headerconfig-alignment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alignment: Option<::Value<String>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-headerconfig.html#cfn-pinpoint-inapptemplate-headerconfig-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::Value<String>>,
        /// Property [`TextColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-headerconfig.html#cfn-pinpoint-inapptemplate-headerconfig-textcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_color: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HeaderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alignment) = self.alignment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alignment", alignment)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref text_color) = self.text_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextColor", text_color)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeaderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeaderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeaderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeaderConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alignment: Option<::Value<String>> = None;
                    let mut header: Option<::Value<String>> = None;
                    let mut text_color: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alignment" => {
                                alignment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextColor" => {
                                text_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeaderConfig {
                        alignment: alignment,
                        header: header,
                        text_color: text_color,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::InAppTemplate.InAppMessageContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html) property type.
    #[derive(Debug, Default)]
    pub struct InAppMessageContent {
        /// Property [`BackgroundColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-backgroundcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub background_color: Option<::Value<String>>,
        /// Property [`BodyConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-bodyconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body_config: Option<::Value<BodyConfig>>,
        /// Property [`HeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-headerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_config: Option<::Value<HeaderConfig>>,
        /// Property [`ImageUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-imageurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_url: Option<::Value<String>>,
        /// Property [`PrimaryBtn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-primarybtn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_btn: Option<::Value<ButtonConfig>>,
        /// Property [`SecondaryBtn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html#cfn-pinpoint-inapptemplate-inappmessagecontent-secondarybtn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_btn: Option<::Value<ButtonConfig>>,
    }

    impl ::codec::SerializeValue for InAppMessageContent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref background_color) = self.background_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackgroundColor", background_color)?;
            }
            if let Some(ref body_config) = self.body_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyConfig", body_config)?;
            }
            if let Some(ref header_config) = self.header_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderConfig", header_config)?;
            }
            if let Some(ref image_url) = self.image_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUrl", image_url)?;
            }
            if let Some(ref primary_btn) = self.primary_btn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryBtn", primary_btn)?;
            }
            if let Some(ref secondary_btn) = self.secondary_btn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryBtn", secondary_btn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InAppMessageContent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InAppMessageContent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InAppMessageContent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InAppMessageContent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut background_color: Option<::Value<String>> = None;
                    let mut body_config: Option<::Value<BodyConfig>> = None;
                    let mut header_config: Option<::Value<HeaderConfig>> = None;
                    let mut image_url: Option<::Value<String>> = None;
                    let mut primary_btn: Option<::Value<ButtonConfig>> = None;
                    let mut secondary_btn: Option<::Value<ButtonConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BackgroundColor" => {
                                background_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BodyConfig" => {
                                body_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderConfig" => {
                                header_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUrl" => {
                                image_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryBtn" => {
                                primary_btn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryBtn" => {
                                secondary_btn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InAppMessageContent {
                        background_color: background_color,
                        body_config: body_config,
                        header_config: header_config,
                        image_url: image_url,
                        primary_btn: primary_btn,
                        secondary_btn: secondary_btn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::InAppTemplate.OverrideButtonConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-overridebuttonconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OverrideButtonConfiguration {
        /// Property [`ButtonAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-overridebuttonconfiguration.html#cfn-pinpoint-inapptemplate-overridebuttonconfiguration-buttonaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub button_action: Option<::Value<String>>,
        /// Property [`Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-overridebuttonconfiguration.html#cfn-pinpoint-inapptemplate-overridebuttonconfiguration-link).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub link: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OverrideButtonConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref button_action) = self.button_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ButtonAction", button_action)?;
            }
            if let Some(ref link) = self.link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Link", link)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OverrideButtonConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OverrideButtonConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OverrideButtonConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OverrideButtonConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut button_action: Option<::Value<String>> = None;
                    let mut link: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ButtonAction" => {
                                button_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Link" => {
                                link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OverrideButtonConfiguration {
                        button_action: button_action,
                        link: link,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod push_template {
    //! Property types for the `PushTemplate` resource.

    /// The [`AWS::Pinpoint::PushTemplate.APNSPushNotificationTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct APNSPushNotificationTemplate {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`MediaUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-mediaurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub media_url: Option<::Value<String>>,
        /// Property [`Sound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-sound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sound: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html#cfn-pinpoint-pushtemplate-apnspushnotificationtemplate-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for APNSPushNotificationTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref media_url) = self.media_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaUrl", media_url)?;
            }
            if let Some(ref sound) = self.sound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sound", sound)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for APNSPushNotificationTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<APNSPushNotificationTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = APNSPushNotificationTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type APNSPushNotificationTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut media_url: Option<::Value<String>> = None;
                    let mut sound: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediaUrl" => {
                                media_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sound" => {
                                sound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(APNSPushNotificationTemplate {
                        action: action,
                        body: body,
                        media_url: media_url,
                        sound: sound,
                        title: title,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::PushTemplate.AndroidPushNotificationTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct AndroidPushNotificationTemplate {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`ImageIconUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-imageiconurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_icon_url: Option<::Value<String>>,
        /// Property [`ImageUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-imageurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_url: Option<::Value<String>>,
        /// Property [`SmallImageIconUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-smallimageiconurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub small_image_icon_url: Option<::Value<String>>,
        /// Property [`Sound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-sound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sound: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-androidpushnotificationtemplate-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AndroidPushNotificationTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref image_icon_url) = self.image_icon_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageIconUrl", image_icon_url)?;
            }
            if let Some(ref image_url) = self.image_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUrl", image_url)?;
            }
            if let Some(ref small_image_icon_url) = self.small_image_icon_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmallImageIconUrl", small_image_icon_url)?;
            }
            if let Some(ref sound) = self.sound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sound", sound)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AndroidPushNotificationTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AndroidPushNotificationTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AndroidPushNotificationTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AndroidPushNotificationTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut image_icon_url: Option<::Value<String>> = None;
                    let mut image_url: Option<::Value<String>> = None;
                    let mut small_image_icon_url: Option<::Value<String>> = None;
                    let mut sound: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageIconUrl" => {
                                image_icon_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUrl" => {
                                image_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmallImageIconUrl" => {
                                small_image_icon_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sound" => {
                                sound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AndroidPushNotificationTemplate {
                        action: action,
                        body: body,
                        image_icon_url: image_icon_url,
                        image_url: image_url,
                        small_image_icon_url: small_image_icon_url,
                        sound: sound,
                        title: title,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::PushTemplate.DefaultPushNotificationTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultPushNotificationTemplate {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-defaultpushnotificationtemplate-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-defaultpushnotificationtemplate-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<String>>,
        /// Property [`Sound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-defaultpushnotificationtemplate-sound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sound: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-defaultpushnotificationtemplate-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html#cfn-pinpoint-pushtemplate-defaultpushnotificationtemplate-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DefaultPushNotificationTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref sound) = self.sound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sound", sound)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultPushNotificationTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultPushNotificationTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultPushNotificationTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultPushNotificationTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut body: Option<::Value<String>> = None;
                    let mut sound: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sound" => {
                                sound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultPushNotificationTemplate {
                        action: action,
                        body: body,
                        sound: sound,
                        title: title,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod segment {
    //! Property types for the `Segment` resource.

    /// The [`AWS::Pinpoint::Segment.AttributeDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-attributedimension.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeDimension {
        /// Property [`AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-attributedimension.html#cfn-pinpoint-segment-attributedimension-attributetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_type: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-attributedimension.html#cfn-pinpoint-segment-attributedimension-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AttributeDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_type) = self.attribute_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeType", attribute_type)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_type: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeType" => {
                                attribute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeDimension {
                        attribute_type: attribute_type,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior.html) property type.
    #[derive(Debug, Default)]
    pub struct Behavior {
        /// Property [`Recency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior.html#cfn-pinpoint-segment-segmentdimensions-behavior-recency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recency: Option<::Value<Recency>>,
    }

    impl ::codec::SerializeValue for Behavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref recency) = self.recency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recency", recency)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Behavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Behavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Behavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Behavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut recency: Option<::Value<Recency>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Recency" => {
                                recency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Behavior {
                        recency: recency,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Coordinates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates.html) property type.
    #[derive(Debug, Default)]
    pub struct Coordinates {
        /// Property [`Latitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates.html#cfn-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates-latitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub latitude: ::Value<f64>,
        /// Property [`Longitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates.html#cfn-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates-longitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub longitude: ::Value<f64>,
    }

    impl ::codec::SerializeValue for Coordinates {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Latitude", &self.latitude)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Longitude", &self.longitude)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Coordinates {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Coordinates, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Coordinates;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Coordinates")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut latitude: Option<::Value<f64>> = None;
                    let mut longitude: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Latitude" => {
                                latitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Longitude" => {
                                longitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Coordinates {
                        latitude: latitude.ok_or(::serde::de::Error::missing_field("Latitude"))?,
                        longitude: longitude.ok_or(::serde::de::Error::missing_field("Longitude"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Demographic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html) property type.
    #[derive(Debug, Default)]
    pub struct Demographic {
        /// Property [`AppVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-appversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_version: Option<::Value<SetDimension>>,
        /// Property [`Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-channel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel: Option<::Value<SetDimension>>,
        /// Property [`DeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-devicetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_type: Option<::Value<SetDimension>>,
        /// Property [`Make`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-make).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub make: Option<::Value<SetDimension>>,
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: Option<::Value<SetDimension>>,
        /// Property [`Platform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html#cfn-pinpoint-segment-segmentdimensions-demographic-platform).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform: Option<::Value<SetDimension>>,
    }

    impl ::codec::SerializeValue for Demographic {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_version) = self.app_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppVersion", app_version)?;
            }
            if let Some(ref channel) = self.channel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Channel", channel)?;
            }
            if let Some(ref device_type) = self.device_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceType", device_type)?;
            }
            if let Some(ref make) = self.make {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Make", make)?;
            }
            if let Some(ref model) = self.model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
            }
            if let Some(ref platform) = self.platform {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Platform", platform)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Demographic {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Demographic, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Demographic;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Demographic")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_version: Option<::Value<SetDimension>> = None;
                    let mut channel: Option<::Value<SetDimension>> = None;
                    let mut device_type: Option<::Value<SetDimension>> = None;
                    let mut make: Option<::Value<SetDimension>> = None;
                    let mut model: Option<::Value<SetDimension>> = None;
                    let mut platform: Option<::Value<SetDimension>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppVersion" => {
                                app_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Channel" => {
                                channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceType" => {
                                device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Make" => {
                                make = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Platform" => {
                                platform = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Demographic {
                        app_version: app_version,
                        channel: channel,
                        device_type: device_type,
                        make: make,
                        model: model,
                        platform: platform,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.GPSPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint.html) property type.
    #[derive(Debug, Default)]
    pub struct GPSPoint {
        /// Property [`Coordinates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint.html#cfn-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub coordinates: ::Value<Coordinates>,
        /// Property [`RangeInKilometers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint.html#cfn-pinpoint-segment-segmentdimensions-location-gpspoint-rangeinkilometers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_in_kilometers: ::Value<f64>,
    }

    impl ::codec::SerializeValue for GPSPoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Coordinates", &self.coordinates)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeInKilometers", &self.range_in_kilometers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GPSPoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GPSPoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GPSPoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GPSPoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut coordinates: Option<::Value<Coordinates>> = None;
                    let mut range_in_kilometers: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Coordinates" => {
                                coordinates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeInKilometers" => {
                                range_in_kilometers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GPSPoint {
                        coordinates: coordinates.ok_or(::serde::de::Error::missing_field("Coordinates"))?,
                        range_in_kilometers: range_in_kilometers.ok_or(::serde::de::Error::missing_field("RangeInKilometers"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html) property type.
    #[derive(Debug, Default)]
    pub struct Groups {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html#cfn-pinpoint-segment-segmentgroups-groups-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<SegmentDimensions>>,
        /// Property [`SourceSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html#cfn-pinpoint-segment-segmentgroups-groups-sourcesegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_segments: Option<::ValueList<SourceSegments>>,
        /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html#cfn-pinpoint-segment-segmentgroups-groups-sourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_type: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html#cfn-pinpoint-segment-segmentgroups-groups-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Groups {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            if let Some(ref source_segments) = self.source_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceSegments", source_segments)?;
            }
            if let Some(ref source_type) = self.source_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Groups {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Groups, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Groups;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Groups")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<::ValueList<SegmentDimensions>> = None;
                    let mut source_segments: Option<::ValueList<SourceSegments>> = None;
                    let mut source_type: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceSegments" => {
                                source_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceType" => {
                                source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Groups {
                        dimensions: dimensions,
                        source_segments: source_segments,
                        source_type: source_type,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location.html) property type.
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`Country`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location.html#cfn-pinpoint-segment-segmentdimensions-location-country).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country: Option<::Value<SetDimension>>,
        /// Property [`GPSPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location.html#cfn-pinpoint-segment-segmentdimensions-location-gpspoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gps_point: Option<::Value<GPSPoint>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref country) = self.country {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Country", country)?;
            }
            if let Some(ref gps_point) = self.gps_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GPSPoint", gps_point)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut country: Option<::Value<SetDimension>> = None;
                    let mut gps_point: Option<::Value<GPSPoint>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Country" => {
                                country = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GPSPoint" => {
                                gps_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        country: country,
                        gps_point: gps_point,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.Recency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior-recency.html) property type.
    #[derive(Debug, Default)]
    pub struct Recency {
        /// Property [`Duration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior-recency.html#cfn-pinpoint-segment-segmentdimensions-behavior-recency-duration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration: ::Value<String>,
        /// Property [`RecencyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior-recency.html#cfn-pinpoint-segment-segmentdimensions-behavior-recency-recencytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recency_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Recency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Duration", &self.duration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecencyType", &self.recency_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Recency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Recency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Recency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Recency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration: Option<::Value<String>> = None;
                    let mut recency_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Duration" => {
                                duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecencyType" => {
                                recency_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Recency {
                        duration: duration.ok_or(::serde::de::Error::missing_field("Duration"))?,
                        recency_type: recency_type.ok_or(::serde::de::Error::missing_field("RecencyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.SegmentDimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html) property type.
    #[derive(Debug, Default)]
    pub struct SegmentDimensions {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::Value<::json::Value>>,
        /// Property [`Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior: Option<::Value<Behavior>>,
        /// Property [`Demographic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-demographic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub demographic: Option<::Value<Demographic>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<Location>>,
        /// Property [`Metrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-metrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics: Option<::Value<::json::Value>>,
        /// Property [`UserAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html#cfn-pinpoint-segment-segmentdimensions-userattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_attributes: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for SegmentDimensions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref behavior) = self.behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Behavior", behavior)?;
            }
            if let Some(ref demographic) = self.demographic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Demographic", demographic)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref metrics) = self.metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metrics", metrics)?;
            }
            if let Some(ref user_attributes) = self.user_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttributes", user_attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SegmentDimensions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentDimensions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SegmentDimensions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SegmentDimensions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::Value<::json::Value>> = None;
                    let mut behavior: Option<::Value<Behavior>> = None;
                    let mut demographic: Option<::Value<Demographic>> = None;
                    let mut location: Option<::Value<Location>> = None;
                    let mut metrics: Option<::Value<::json::Value>> = None;
                    let mut user_attributes: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Behavior" => {
                                behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Demographic" => {
                                demographic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metrics" => {
                                metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAttributes" => {
                                user_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SegmentDimensions {
                        attributes: attributes,
                        behavior: behavior,
                        demographic: demographic,
                        location: location,
                        metrics: metrics,
                        user_attributes: user_attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.SegmentGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups.html) property type.
    #[derive(Debug, Default)]
    pub struct SegmentGroups {
        /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups.html#cfn-pinpoint-segment-segmentgroups-groups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub groups: Option<::ValueList<Groups>>,
        /// Property [`Include`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups.html#cfn-pinpoint-segment-segmentgroups-include).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SegmentGroups {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref groups) = self.groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
            }
            if let Some(ref include) = self.include {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Include", include)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SegmentGroups {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentGroups, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SegmentGroups;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SegmentGroups")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut groups: Option<::ValueList<Groups>> = None;
                    let mut include: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Groups" => {
                                groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Include" => {
                                include = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SegmentGroups {
                        groups: groups,
                        include: include,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.SetDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-setdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct SetDimension {
        /// Property [`DimensionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-setdimension.html#cfn-pinpoint-segment-setdimension-dimensiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_type: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-setdimension.html#cfn-pinpoint-segment-setdimension-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SetDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimension_type) = self.dimension_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionType", dimension_type)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SetDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SetDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SetDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SetDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_type: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionType" => {
                                dimension_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SetDimension {
                        dimension_type: dimension_type,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Pinpoint::Segment.SourceSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups-sourcesegments.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceSegments {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups-sourcesegments.html#cfn-pinpoint-segment-segmentgroups-groups-sourcesegments-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups-sourcesegments.html#cfn-pinpoint-segment-segmentgroups-groups-sourcesegments-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SourceSegments {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceSegments {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceSegments, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceSegments;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceSegments")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut version: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceSegments {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
