//! Types for the `Signer` service.

/// The [`AWS::Signer::ProfilePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html) resource type.
#[derive(Debug, Default)]
pub struct ProfilePermission {
    properties: ProfilePermissionProperties
}

/// Properties for the `ProfilePermission` resource.
#[derive(Debug, Default)]
pub struct ProfilePermissionProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html#cfn-signer-profilepermission-action).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action: ::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html#cfn-signer-profilepermission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`ProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html#cfn-signer-profilepermission-profilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub profile_name: ::Value<String>,
    /// Property [`ProfileVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html#cfn-signer-profilepermission-profileversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub profile_version: Option<::Value<String>>,
    /// Property [`StatementId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html#cfn-signer-profilepermission-statementid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub statement_id: ::Value<String>,
}

impl ::serde::Serialize for ProfilePermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileName", &self.profile_name)?;
        if let Some(ref profile_version) = self.profile_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileVersion", profile_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatementId", &self.statement_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProfilePermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProfilePermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProfilePermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProfilePermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;
                let mut profile_name: Option<::Value<String>> = None;
                let mut profile_version: Option<::Value<String>> = None;
                let mut statement_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProfileName" => {
                            profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProfileVersion" => {
                            profile_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StatementId" => {
                            statement_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProfilePermissionProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    profile_name: profile_name.ok_or(::serde::de::Error::missing_field("ProfileName"))?,
                    profile_version: profile_version,
                    statement_id: statement_id.ok_or(::serde::de::Error::missing_field("StatementId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ProfilePermission {
    type Properties = ProfilePermissionProperties;
    const TYPE: &'static str = "AWS::Signer::ProfilePermission";
    fn properties(&self) -> &ProfilePermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProfilePermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ProfilePermission {}

impl From<ProfilePermissionProperties> for ProfilePermission {
    fn from(properties: ProfilePermissionProperties) -> ProfilePermission {
        ProfilePermission { properties }
    }
}

/// The [`AWS::Signer::SigningProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-signingprofile.html) resource type.
#[derive(Debug, Default)]
pub struct SigningProfile {
    properties: SigningProfileProperties
}

/// Properties for the `SigningProfile` resource.
#[derive(Debug, Default)]
pub struct SigningProfileProperties {
    /// Property [`PlatformId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-signingprofile.html#cfn-signer-signingprofile-platformid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platform_id: ::Value<String>,
    /// Property [`SignatureValidityPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-signingprofile.html#cfn-signer-signingprofile-signaturevalidityperiod).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signature_validity_period: Option<::Value<self::signing_profile::SignatureValidityPeriod>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-signingprofile.html#cfn-signer-signingprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SigningProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformId", &self.platform_id)?;
        if let Some(ref signature_validity_period) = self.signature_validity_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignatureValidityPeriod", signature_validity_period)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SigningProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SigningProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SigningProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SigningProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut platform_id: Option<::Value<String>> = None;
                let mut signature_validity_period: Option<::Value<self::signing_profile::SignatureValidityPeriod>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PlatformId" => {
                            platform_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignatureValidityPeriod" => {
                            signature_validity_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SigningProfileProperties {
                    platform_id: platform_id.ok_or(::serde::de::Error::missing_field("PlatformId"))?,
                    signature_validity_period: signature_validity_period,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SigningProfile {
    type Properties = SigningProfileProperties;
    const TYPE: &'static str = "AWS::Signer::SigningProfile";
    fn properties(&self) -> &SigningProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SigningProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SigningProfile {}

impl From<SigningProfileProperties> for SigningProfile {
    fn from(properties: SigningProfileProperties) -> SigningProfile {
        SigningProfile { properties }
    }
}

pub mod signing_profile {
    //! Property types for the `SigningProfile` resource.

    /// The [`AWS::Signer::SigningProfile.SignatureValidityPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-signer-signingprofile-signaturevalidityperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct SignatureValidityPeriod {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-signer-signingprofile-signaturevalidityperiod.html#cfn-signer-signingprofile-signaturevalidityperiod-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-signer-signingprofile-signaturevalidityperiod.html#cfn-signer-signingprofile-signaturevalidityperiod-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SignatureValidityPeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SignatureValidityPeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SignatureValidityPeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SignatureValidityPeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SignatureValidityPeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SignatureValidityPeriod {
                        r#type: r#type,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
