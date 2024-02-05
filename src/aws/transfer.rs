//! Types for the `Transfer` service.

/// The [`AWS::Transfer::Agreement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html) resource type.
#[derive(Debug, Default)]
pub struct Agreement {
    properties: AgreementProperties
}

/// Properties for the `Agreement` resource.
#[derive(Debug, Default)]
pub struct AgreementProperties {
    /// Property [`AccessRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-accessrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_role: ::Value<String>,
    /// Property [`BaseDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-basedirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub base_directory: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`LocalProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-localprofileid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub local_profile_id: ::Value<String>,
    /// Property [`PartnerProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-partnerprofileid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub partner_profile_id: ::Value<String>,
    /// Property [`ServerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-serverid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_id: ::Value<String>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-agreement.html#cfn-transfer-agreement-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AgreementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessRole", &self.access_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseDirectory", &self.base_directory)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalProfileId", &self.local_profile_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartnerProfileId", &self.partner_profile_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerId", &self.server_id)?;
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AgreementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AgreementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AgreementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AgreementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_role: Option<::Value<String>> = None;
                let mut base_directory: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut local_profile_id: Option<::Value<String>> = None;
                let mut partner_profile_id: Option<::Value<String>> = None;
                let mut server_id: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessRole" => {
                            access_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BaseDirectory" => {
                            base_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LocalProfileId" => {
                            local_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PartnerProfileId" => {
                            partner_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerId" => {
                            server_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AgreementProperties {
                    access_role: access_role.ok_or(::serde::de::Error::missing_field("AccessRole"))?,
                    base_directory: base_directory.ok_or(::serde::de::Error::missing_field("BaseDirectory"))?,
                    description: description,
                    local_profile_id: local_profile_id.ok_or(::serde::de::Error::missing_field("LocalProfileId"))?,
                    partner_profile_id: partner_profile_id.ok_or(::serde::de::Error::missing_field("PartnerProfileId"))?,
                    server_id: server_id.ok_or(::serde::de::Error::missing_field("ServerId"))?,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Agreement {
    type Properties = AgreementProperties;
    const TYPE: &'static str = "AWS::Transfer::Agreement";
    fn properties(&self) -> &AgreementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AgreementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Agreement {}

impl From<AgreementProperties> for Agreement {
    fn from(properties: AgreementProperties) -> Agreement {
        Agreement { properties }
    }
}

/// The [`AWS::Transfer::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`ActiveDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-activedate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub active_date: Option<::Value<String>>,
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-certificate).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate: ::Value<String>,
    /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-certificatechain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_chain: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InactiveDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-inactivedate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub inactive_date: Option<::Value<String>>,
    /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-privatekey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub private_key: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Usage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-certificate.html#cfn-transfer-certificate-usage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub usage: ::Value<String>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref active_date) = self.active_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveDate", active_date)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", &self.certificate)?;
        if let Some(ref certificate_chain) = self.certificate_chain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", certificate_chain)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref inactive_date) = self.inactive_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InactiveDate", inactive_date)?;
        }
        if let Some(ref private_key) = self.private_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Usage", &self.usage)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut active_date: Option<::Value<String>> = None;
                let mut certificate: Option<::Value<String>> = None;
                let mut certificate_chain: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut inactive_date: Option<::Value<String>> = None;
                let mut private_key: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut usage: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActiveDate" => {
                            active_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateChain" => {
                            certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InactiveDate" => {
                            inactive_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrivateKey" => {
                            private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Usage" => {
                            usage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    active_date: active_date,
                    certificate: certificate.ok_or(::serde::de::Error::missing_field("Certificate"))?,
                    certificate_chain: certificate_chain,
                    description: description,
                    inactive_date: inactive_date,
                    private_key: private_key,
                    tags: tags,
                    usage: usage.ok_or(::serde::de::Error::missing_field("Usage"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::Transfer::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::Transfer::Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html) resource type.
#[derive(Debug, Default)]
pub struct Connector {
    properties: ConnectorProperties
}

/// Properties for the `Connector` resource.
#[derive(Debug, Default)]
pub struct ConnectorProperties {
    /// Property [`AccessRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-accessrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_role: ::Value<String>,
    /// Property [`As2Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-as2config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub as2_config: Option<::Value<self::connector::As2Config>>,
    /// Property [`LoggingRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-loggingrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_role: Option<::Value<String>>,
    /// Property [`SftpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-sftpconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sftp_config: Option<::Value<self::connector::SftpConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-connector.html#cfn-transfer-connector-url).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub url: ::Value<String>,
}

impl ::serde::Serialize for ConnectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessRole", &self.access_role)?;
        if let Some(ref as2_config) = self.as2_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "As2Config", as2_config)?;
        }
        if let Some(ref logging_role) = self.logging_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingRole", logging_role)?;
        }
        if let Some(ref sftp_config) = self.sftp_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SftpConfig", sftp_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_role: Option<::Value<String>> = None;
                let mut as2_config: Option<::Value<self::connector::As2Config>> = None;
                let mut logging_role: Option<::Value<String>> = None;
                let mut sftp_config: Option<::Value<self::connector::SftpConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut url: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessRole" => {
                            access_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "As2Config" => {
                            as2_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingRole" => {
                            logging_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SftpConfig" => {
                            sftp_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Url" => {
                            url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorProperties {
                    access_role: access_role.ok_or(::serde::de::Error::missing_field("AccessRole"))?,
                    as2_config: as2_config,
                    logging_role: logging_role,
                    sftp_config: sftp_config,
                    tags: tags,
                    url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connector {
    type Properties = ConnectorProperties;
    const TYPE: &'static str = "AWS::Transfer::Connector";
    fn properties(&self) -> &ConnectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connector {}

impl From<ConnectorProperties> for Connector {
    fn from(properties: ConnectorProperties) -> Connector {
        Connector { properties }
    }
}

/// The [`AWS::Transfer::Profile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html) resource type.
#[derive(Debug, Default)]
pub struct Profile {
    properties: ProfileProperties
}

/// Properties for the `Profile` resource.
#[derive(Debug, Default)]
pub struct ProfileProperties {
    /// Property [`As2Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html#cfn-transfer-profile-as2id).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub as2_id: ::Value<String>,
    /// Property [`CertificateIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html#cfn-transfer-profile-certificateids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_ids: Option<::ValueList<String>>,
    /// Property [`ProfileType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html#cfn-transfer-profile-profiletype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub profile_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-profile.html#cfn-transfer-profile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "As2Id", &self.as2_id)?;
        if let Some(ref certificate_ids) = self.certificate_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateIds", certificate_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileType", &self.profile_type)?;
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
                let mut as2_id: Option<::Value<String>> = None;
                let mut certificate_ids: Option<::ValueList<String>> = None;
                let mut profile_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "As2Id" => {
                            as2_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateIds" => {
                            certificate_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProfileType" => {
                            profile_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProfileProperties {
                    as2_id: as2_id.ok_or(::serde::de::Error::missing_field("As2Id"))?,
                    certificate_ids: certificate_ids,
                    profile_type: profile_type.ok_or(::serde::de::Error::missing_field("ProfileType"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Profile {
    type Properties = ProfileProperties;
    const TYPE: &'static str = "AWS::Transfer::Profile";
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

/// The [`AWS::Transfer::Server`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html) resource type.
#[derive(Debug, Default)]
pub struct Server {
    properties: ServerProperties
}

/// Properties for the `Server` resource.
#[derive(Debug, Default)]
pub struct ServerProperties {
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-endpointdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_details: Option<::Value<self::server::EndpointDetails>>,
    /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-endpointtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_type: Option<::Value<String>>,
    /// Property [`IdentityProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-identityproviderdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_provider_details: Option<::Value<self::server::IdentityProviderDetails>>,
    /// Property [`IdentityProviderType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-identityprovidertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_provider_type: Option<::Value<String>>,
    /// Property [`LoggingRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-loggingrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_role: Option<::Value<String>>,
    /// Property [`PostAuthenticationLoginBanner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-postauthenticationloginbanner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub post_authentication_login_banner: Option<::Value<String>>,
    /// Property [`PreAuthenticationLoginBanner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-preauthenticationloginbanner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pre_authentication_login_banner: Option<::Value<String>>,
    /// Property [`ProtocolDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-protocoldetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol_details: Option<::Value<self::server::ProtocolDetails>>,
    /// Property [`Protocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-protocols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocols: Option<::ValueList<self::server::Protocol>>,
    /// Property [`S3StorageOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-s3storageoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_storage_options: Option<::Value<self::server::S3StorageOptions>>,
    /// Property [`SecurityPolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-securitypolicyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_policy_name: Option<::Value<String>>,
    /// Property [`StructuredLogDestinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-structuredlogdestinations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub structured_log_destinations: Option<::ValueList<self::server::StructuredLogDestination>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkflowDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-workflowdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub workflow_details: Option<::Value<self::server::WorkflowDetails>>,
}

impl ::serde::Serialize for ServerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref endpoint_details) = self.endpoint_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointDetails", endpoint_details)?;
        }
        if let Some(ref endpoint_type) = self.endpoint_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", endpoint_type)?;
        }
        if let Some(ref identity_provider_details) = self.identity_provider_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderDetails", identity_provider_details)?;
        }
        if let Some(ref identity_provider_type) = self.identity_provider_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderType", identity_provider_type)?;
        }
        if let Some(ref logging_role) = self.logging_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingRole", logging_role)?;
        }
        if let Some(ref post_authentication_login_banner) = self.post_authentication_login_banner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAuthenticationLoginBanner", post_authentication_login_banner)?;
        }
        if let Some(ref pre_authentication_login_banner) = self.pre_authentication_login_banner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreAuthenticationLoginBanner", pre_authentication_login_banner)?;
        }
        if let Some(ref protocol_details) = self.protocol_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolDetails", protocol_details)?;
        }
        if let Some(ref protocols) = self.protocols {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocols", protocols)?;
        }
        if let Some(ref s3_storage_options) = self.s3_storage_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3StorageOptions", s3_storage_options)?;
        }
        if let Some(ref security_policy_name) = self.security_policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityPolicyName", security_policy_name)?;
        }
        if let Some(ref structured_log_destinations) = self.structured_log_destinations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StructuredLogDestinations", structured_log_destinations)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref workflow_details) = self.workflow_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowDetails", workflow_details)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate: Option<::Value<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut endpoint_details: Option<::Value<self::server::EndpointDetails>> = None;
                let mut endpoint_type: Option<::Value<String>> = None;
                let mut identity_provider_details: Option<::Value<self::server::IdentityProviderDetails>> = None;
                let mut identity_provider_type: Option<::Value<String>> = None;
                let mut logging_role: Option<::Value<String>> = None;
                let mut post_authentication_login_banner: Option<::Value<String>> = None;
                let mut pre_authentication_login_banner: Option<::Value<String>> = None;
                let mut protocol_details: Option<::Value<self::server::ProtocolDetails>> = None;
                let mut protocols: Option<::ValueList<self::server::Protocol>> = None;
                let mut s3_storage_options: Option<::Value<self::server::S3StorageOptions>> = None;
                let mut security_policy_name: Option<::Value<String>> = None;
                let mut structured_log_destinations: Option<::ValueList<self::server::StructuredLogDestination>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workflow_details: Option<::Value<self::server::WorkflowDetails>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointDetails" => {
                            endpoint_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointType" => {
                            endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderDetails" => {
                            identity_provider_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderType" => {
                            identity_provider_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingRole" => {
                            logging_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PostAuthenticationLoginBanner" => {
                            post_authentication_login_banner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreAuthenticationLoginBanner" => {
                            pre_authentication_login_banner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtocolDetails" => {
                            protocol_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocols" => {
                            protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3StorageOptions" => {
                            s3_storage_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityPolicyName" => {
                            security_policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StructuredLogDestinations" => {
                            structured_log_destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkflowDetails" => {
                            workflow_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServerProperties {
                    certificate: certificate,
                    domain: domain,
                    endpoint_details: endpoint_details,
                    endpoint_type: endpoint_type,
                    identity_provider_details: identity_provider_details,
                    identity_provider_type: identity_provider_type,
                    logging_role: logging_role,
                    post_authentication_login_banner: post_authentication_login_banner,
                    pre_authentication_login_banner: pre_authentication_login_banner,
                    protocol_details: protocol_details,
                    protocols: protocols,
                    s3_storage_options: s3_storage_options,
                    security_policy_name: security_policy_name,
                    structured_log_destinations: structured_log_destinations,
                    tags: tags,
                    workflow_details: workflow_details,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Server {
    type Properties = ServerProperties;
    const TYPE: &'static str = "AWS::Transfer::Server";
    fn properties(&self) -> &ServerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Server {}

impl From<ServerProperties> for Server {
    fn from(properties: ServerProperties) -> Server {
        Server { properties }
    }
}

/// The [`AWS::Transfer::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`HomeDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory: Option<::Value<String>>,
    /// Property [`HomeDirectoryMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectorymappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory_mappings: Option<::ValueList<self::user::HomeDirectoryMapEntry>>,
    /// Property [`HomeDirectoryType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectorytype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory_type: Option<::Value<String>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: Option<::Value<String>>,
    /// Property [`PosixProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-posixprofile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub posix_profile: Option<::Value<self::user::PosixProfile>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`ServerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-serverid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_id: ::Value<String>,
    /// Property [`SshPublicKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-sshpublickeys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssh_public_keys: Option<::ValueList<self::user::SshPublicKey>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref home_directory) = self.home_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectory", home_directory)?;
        }
        if let Some(ref home_directory_mappings) = self.home_directory_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectoryMappings", home_directory_mappings)?;
        }
        if let Some(ref home_directory_type) = self.home_directory_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectoryType", home_directory_type)?;
        }
        if let Some(ref policy) = self.policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", policy)?;
        }
        if let Some(ref posix_profile) = self.posix_profile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PosixProfile", posix_profile)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerId", &self.server_id)?;
        if let Some(ref ssh_public_keys) = self.ssh_public_keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshPublicKeys", ssh_public_keys)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut home_directory: Option<::Value<String>> = None;
                let mut home_directory_mappings: Option<::ValueList<self::user::HomeDirectoryMapEntry>> = None;
                let mut home_directory_type: Option<::Value<String>> = None;
                let mut policy: Option<::Value<String>> = None;
                let mut posix_profile: Option<::Value<self::user::PosixProfile>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut server_id: Option<::Value<String>> = None;
                let mut ssh_public_keys: Option<::ValueList<self::user::SshPublicKey>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HomeDirectory" => {
                            home_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeDirectoryMappings" => {
                            home_directory_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeDirectoryType" => {
                            home_directory_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PosixProfile" => {
                            posix_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerId" => {
                            server_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SshPublicKeys" => {
                            ssh_public_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    home_directory: home_directory,
                    home_directory_mappings: home_directory_mappings,
                    home_directory_type: home_directory_type,
                    policy: policy,
                    posix_profile: posix_profile,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    server_id: server_id.ok_or(::serde::de::Error::missing_field("ServerId"))?,
                    ssh_public_keys: ssh_public_keys,
                    tags: tags,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::Transfer::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::Transfer::Workflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html) resource type.
#[derive(Debug, Default)]
pub struct Workflow {
    properties: WorkflowProperties
}

/// Properties for the `Workflow` resource.
#[derive(Debug, Default)]
pub struct WorkflowProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`OnExceptionSteps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-onexceptionsteps).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub on_exception_steps: Option<::ValueList<self::workflow::WorkflowStep>>,
    /// Property [`Steps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-steps).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub steps: ::ValueList<self::workflow::WorkflowStep>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for WorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref on_exception_steps) = self.on_exception_steps {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnExceptionSteps", on_exception_steps)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Steps", &self.steps)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkflowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkflowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut on_exception_steps: Option<::ValueList<self::workflow::WorkflowStep>> = None;
                let mut steps: Option<::ValueList<self::workflow::WorkflowStep>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnExceptionSteps" => {
                            on_exception_steps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Steps" => {
                            steps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkflowProperties {
                    description: description,
                    on_exception_steps: on_exception_steps,
                    steps: steps.ok_or(::serde::de::Error::missing_field("Steps"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workflow {
    type Properties = WorkflowProperties;
    const TYPE: &'static str = "AWS::Transfer::Workflow";
    fn properties(&self) -> &WorkflowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkflowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workflow {}

impl From<WorkflowProperties> for Workflow {
    fn from(properties: WorkflowProperties) -> Workflow {
        Workflow { properties }
    }
}

pub mod connector {
    //! Property types for the `Connector` resource.

    /// The [`AWS::Transfer::Connector.As2Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html) property type.
    #[derive(Debug, Default)]
    pub struct As2Config {
        /// Property [`BasicAuthSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-basicauthsecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_auth_secret_id: Option<::Value<String>>,
        /// Property [`Compression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-compression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression: Option<::Value<String>>,
        /// Property [`EncryptionAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-encryptionalgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_algorithm: Option<::Value<String>>,
        /// Property [`LocalProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-localprofileid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub local_profile_id: Option<::Value<String>>,
        /// Property [`MdnResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-mdnresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mdn_response: Option<::Value<String>>,
        /// Property [`MdnSigningAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-mdnsigningalgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mdn_signing_algorithm: Option<::Value<String>>,
        /// Property [`MessageSubject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-messagesubject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_subject: Option<::Value<String>>,
        /// Property [`PartnerProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-partnerprofileid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partner_profile_id: Option<::Value<String>>,
        /// Property [`SigningAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-as2config.html#cfn-transfer-connector-as2config-signingalgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_algorithm: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for As2Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref basic_auth_secret_id) = self.basic_auth_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthSecretId", basic_auth_secret_id)?;
            }
            if let Some(ref compression) = self.compression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compression", compression)?;
            }
            if let Some(ref encryption_algorithm) = self.encryption_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAlgorithm", encryption_algorithm)?;
            }
            if let Some(ref local_profile_id) = self.local_profile_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalProfileId", local_profile_id)?;
            }
            if let Some(ref mdn_response) = self.mdn_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MdnResponse", mdn_response)?;
            }
            if let Some(ref mdn_signing_algorithm) = self.mdn_signing_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MdnSigningAlgorithm", mdn_signing_algorithm)?;
            }
            if let Some(ref message_subject) = self.message_subject {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageSubject", message_subject)?;
            }
            if let Some(ref partner_profile_id) = self.partner_profile_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartnerProfileId", partner_profile_id)?;
            }
            if let Some(ref signing_algorithm) = self.signing_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningAlgorithm", signing_algorithm)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for As2Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<As2Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = As2Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type As2Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut basic_auth_secret_id: Option<::Value<String>> = None;
                    let mut compression: Option<::Value<String>> = None;
                    let mut encryption_algorithm: Option<::Value<String>> = None;
                    let mut local_profile_id: Option<::Value<String>> = None;
                    let mut mdn_response: Option<::Value<String>> = None;
                    let mut mdn_signing_algorithm: Option<::Value<String>> = None;
                    let mut message_subject: Option<::Value<String>> = None;
                    let mut partner_profile_id: Option<::Value<String>> = None;
                    let mut signing_algorithm: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasicAuthSecretId" => {
                                basic_auth_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compression" => {
                                compression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionAlgorithm" => {
                                encryption_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalProfileId" => {
                                local_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MdnResponse" => {
                                mdn_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MdnSigningAlgorithm" => {
                                mdn_signing_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageSubject" => {
                                message_subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartnerProfileId" => {
                                partner_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SigningAlgorithm" => {
                                signing_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(As2Config {
                        basic_auth_secret_id: basic_auth_secret_id,
                        compression: compression,
                        encryption_algorithm: encryption_algorithm,
                        local_profile_id: local_profile_id,
                        mdn_response: mdn_response,
                        mdn_signing_algorithm: mdn_signing_algorithm,
                        message_subject: message_subject,
                        partner_profile_id: partner_profile_id,
                        signing_algorithm: signing_algorithm,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Connector.SftpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-sftpconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SftpConfig {
        /// Property [`TrustedHostKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-sftpconfig.html#cfn-transfer-connector-sftpconfig-trustedhostkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trusted_host_keys: Option<::ValueList<String>>,
        /// Property [`UserSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-connector-sftpconfig.html#cfn-transfer-connector-sftpconfig-usersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_secret_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SftpConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref trusted_host_keys) = self.trusted_host_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedHostKeys", trusted_host_keys)?;
            }
            if let Some(ref user_secret_id) = self.user_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSecretId", user_secret_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SftpConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SftpConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SftpConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SftpConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut trusted_host_keys: Option<::ValueList<String>> = None;
                    let mut user_secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TrustedHostKeys" => {
                                trusted_host_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserSecretId" => {
                                user_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SftpConfig {
                        trusted_host_keys: trusted_host_keys,
                        user_secret_id: user_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod server {
    //! Property types for the `Server` resource.

    /// The [`AWS::Transfer::Server.As2Transport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-as2transport.html) property type.
    #[derive(Debug, Default)]
    pub struct As2Transport {
    }

    impl ::codec::SerializeValue for As2Transport {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for As2Transport {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<As2Transport, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = As2Transport;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type As2Transport")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(As2Transport {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointDetails {
        /// Property [`AddressAllocationIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-addressallocationids).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub address_allocation_ids: Option<::ValueList<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
        /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-vpcendpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoint_id: Option<::Value<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address_allocation_ids) = self.address_allocation_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddressAllocationIds", address_allocation_ids)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address_allocation_ids: Option<::ValueList<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;
                    let mut vpc_endpoint_id: Option<::Value<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddressAllocationIds" => {
                                address_allocation_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcEndpointId" => {
                                vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointDetails {
                        address_allocation_ids: address_allocation_ids,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                        vpc_endpoint_id: vpc_endpoint_id,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.IdentityProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct IdentityProviderDetails {
        /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-directoryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_id: Option<::Value<String>>,
        /// Property [`Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-function).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function: Option<::Value<String>>,
        /// Property [`InvocationRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-invocationrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_role: Option<::Value<String>>,
        /// Property [`SftpAuthenticationMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-sftpauthenticationmethods).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sftp_authentication_methods: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IdentityProviderDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_id) = self.directory_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", directory_id)?;
            }
            if let Some(ref function) = self.function {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Function", function)?;
            }
            if let Some(ref invocation_role) = self.invocation_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationRole", invocation_role)?;
            }
            if let Some(ref sftp_authentication_methods) = self.sftp_authentication_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SftpAuthenticationMethods", sftp_authentication_methods)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdentityProviderDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityProviderDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdentityProviderDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdentityProviderDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_id: Option<::Value<String>> = None;
                    let mut function: Option<::Value<String>> = None;
                    let mut invocation_role: Option<::Value<String>> = None;
                    let mut sftp_authentication_methods: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryId" => {
                                directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Function" => {
                                function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvocationRole" => {
                                invocation_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SftpAuthenticationMethods" => {
                                sftp_authentication_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdentityProviderDetails {
                        directory_id: directory_id,
                        function: function,
                        invocation_role: invocation_role,
                        sftp_authentication_methods: sftp_authentication_methods,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocol.html) property type.
    #[derive(Debug, Default)]
    pub struct Protocol {
    }

    impl ::codec::SerializeValue for Protocol {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Protocol {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Protocol, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Protocol;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Protocol")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Protocol {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.ProtocolDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html) property type.
    #[derive(Debug, Default)]
    pub struct ProtocolDetails {
        /// Property [`As2Transports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-as2transports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub as2_transports: Option<::ValueList<As2Transport>>,
        /// Property [`PassiveIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-passiveip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub passive_ip: Option<::Value<String>>,
        /// Property [`SetStatOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-setstatoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_stat_option: Option<::Value<String>>,
        /// Property [`TlsSessionResumptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-tlssessionresumptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_session_resumption_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProtocolDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref as2_transports) = self.as2_transports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "As2Transports", as2_transports)?;
            }
            if let Some(ref passive_ip) = self.passive_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassiveIp", passive_ip)?;
            }
            if let Some(ref set_stat_option) = self.set_stat_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetStatOption", set_stat_option)?;
            }
            if let Some(ref tls_session_resumption_mode) = self.tls_session_resumption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsSessionResumptionMode", tls_session_resumption_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProtocolDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtocolDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProtocolDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProtocolDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut as2_transports: Option<::ValueList<As2Transport>> = None;
                    let mut passive_ip: Option<::Value<String>> = None;
                    let mut set_stat_option: Option<::Value<String>> = None;
                    let mut tls_session_resumption_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "As2Transports" => {
                                as2_transports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PassiveIp" => {
                                passive_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetStatOption" => {
                                set_stat_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TlsSessionResumptionMode" => {
                                tls_session_resumption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProtocolDetails {
                        as2_transports: as2_transports,
                        passive_ip: passive_ip,
                        set_stat_option: set_stat_option,
                        tls_session_resumption_mode: tls_session_resumption_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.S3StorageOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-s3storageoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct S3StorageOptions {
        /// Property [`DirectoryListingOptimization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-s3storageoptions.html#cfn-transfer-server-s3storageoptions-directorylistingoptimization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_listing_optimization: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3StorageOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_listing_optimization) = self.directory_listing_optimization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryListingOptimization", directory_listing_optimization)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3StorageOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3StorageOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3StorageOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3StorageOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_listing_optimization: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryListingOptimization" => {
                                directory_listing_optimization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3StorageOptions {
                        directory_listing_optimization: directory_listing_optimization,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.StructuredLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-structuredlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct StructuredLogDestination {
    }

    impl ::codec::SerializeValue for StructuredLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StructuredLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StructuredLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StructuredLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StructuredLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(StructuredLogDestination {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.WorkflowDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowDetail {
        /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html#cfn-transfer-server-workflowdetail-executionrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role: ::Value<String>,
        /// Property [`WorkflowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html#cfn-transfer-server-workflowdetail-workflowid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workflow_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkflowDetail {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", &self.execution_role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowId", &self.workflow_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowDetail {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowDetail, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowDetail;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowDetail")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role: Option<::Value<String>> = None;
                    let mut workflow_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRole" => {
                                execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkflowId" => {
                                workflow_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowDetail {
                        execution_role: execution_role.ok_or(::serde::de::Error::missing_field("ExecutionRole"))?,
                        workflow_id: workflow_id.ok_or(::serde::de::Error::missing_field("WorkflowId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.WorkflowDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowDetails {
        /// Property [`OnPartialUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html#cfn-transfer-server-workflowdetails-onpartialupload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_partial_upload: Option<::ValueList<WorkflowDetail>>,
        /// Property [`OnUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html#cfn-transfer-server-workflowdetails-onupload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_upload: Option<::ValueList<WorkflowDetail>>,
    }

    impl ::codec::SerializeValue for WorkflowDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref on_partial_upload) = self.on_partial_upload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnPartialUpload", on_partial_upload)?;
            }
            if let Some(ref on_upload) = self.on_upload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUpload", on_upload)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_partial_upload: Option<::ValueList<WorkflowDetail>> = None;
                    let mut on_upload: Option<::ValueList<WorkflowDetail>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnPartialUpload" => {
                                on_partial_upload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnUpload" => {
                                on_upload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowDetails {
                        on_partial_upload: on_partial_upload,
                        on_upload: on_upload,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::Transfer::User.HomeDirectoryMapEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html) property type.
    #[derive(Debug, Default)]
    pub struct HomeDirectoryMapEntry {
        /// Property [`Entry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html#cfn-transfer-user-homedirectorymapentry-entry).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry: ::Value<String>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html#cfn-transfer-user-homedirectorymapentry-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html#cfn-transfer-user-homedirectorymapentry-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HomeDirectoryMapEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Entry", &self.entry)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HomeDirectoryMapEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HomeDirectoryMapEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HomeDirectoryMapEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HomeDirectoryMapEntry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entry: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Entry" => {
                                entry = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HomeDirectoryMapEntry {
                        entry: entry.ok_or(::serde::de::Error::missing_field("Entry"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::User.PosixProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct PosixProfile {
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-gid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gid: ::Value<f64>,
        /// Property [`SecondaryGids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-secondarygids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_gids: Option<::ValueList<f64>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-uid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uid: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PosixProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", &self.gid)?;
            if let Some(ref secondary_gids) = self.secondary_gids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryGids", secondary_gids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", &self.uid)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PosixProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PosixProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PosixProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PosixProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gid: Option<::Value<f64>> = None;
                    let mut secondary_gids: Option<::ValueList<f64>> = None;
                    let mut uid: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryGids" => {
                                secondary_gids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PosixProfile {
                        gid: gid.ok_or(::serde::de::Error::missing_field("Gid"))?,
                        secondary_gids: secondary_gids,
                        uid: uid.ok_or(::serde::de::Error::missing_field("Uid"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::User.SshPublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-sshpublickey.html) property type.
    #[derive(Debug, Default)]
    pub struct SshPublicKey {
    }

    impl ::codec::SerializeValue for SshPublicKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SshPublicKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SshPublicKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SshPublicKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SshPublicKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(SshPublicKey {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workflow {
    //! Property types for the `Workflow` resource.

    /// The [`AWS::Transfer::Workflow.CopyStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CopyStepDetails {
        /// Property [`DestinationFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html#cfn-transfer-workflow-copystepdetails-destinationfilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_file_location: Option<::Value<S3FileLocation>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html#cfn-transfer-workflow-copystepdetails-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`OverwriteExisting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html#cfn-transfer-workflow-copystepdetails-overwriteexisting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub overwrite_existing: Option<::Value<String>>,
        /// Property [`SourceFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-copystepdetails.html#cfn-transfer-workflow-copystepdetails-sourcefilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_file_location: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CopyStepDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_file_location) = self.destination_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationFileLocation", destination_file_location)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref overwrite_existing) = self.overwrite_existing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverwriteExisting", overwrite_existing)?;
            }
            if let Some(ref source_file_location) = self.source_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFileLocation", source_file_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CopyStepDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CopyStepDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CopyStepDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CopyStepDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_file_location: Option<::Value<S3FileLocation>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut overwrite_existing: Option<::Value<String>> = None;
                    let mut source_file_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationFileLocation" => {
                                destination_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OverwriteExisting" => {
                                overwrite_existing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFileLocation" => {
                                source_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CopyStepDetails {
                        destination_file_location: destination_file_location,
                        name: name,
                        overwrite_existing: overwrite_existing,
                        source_file_location: source_file_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.CustomStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomStepDetails {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html#cfn-transfer-workflow-customstepdetails-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SourceFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html#cfn-transfer-workflow-customstepdetails-sourcefilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_file_location: Option<::Value<String>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html#cfn-transfer-workflow-customstepdetails-target).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub target: Option<::Value<String>>,
        /// Property [`TimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-customstepdetails.html#cfn-transfer-workflow-customstepdetails-timeoutseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CustomStepDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref source_file_location) = self.source_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFileLocation", source_file_location)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            if let Some(ref timeout_seconds) = self.timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutSeconds", timeout_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomStepDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomStepDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomStepDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomStepDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut source_file_location: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;
                    let mut timeout_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFileLocation" => {
                                source_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutSeconds" => {
                                timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomStepDetails {
                        name: name,
                        source_file_location: source_file_location,
                        target: target,
                        timeout_seconds: timeout_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.DecryptStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct DecryptStepDetails {
        /// Property [`DestinationFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html#cfn-transfer-workflow-decryptstepdetails-destinationfilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_file_location: Option<::Value<InputFileLocation>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html#cfn-transfer-workflow-decryptstepdetails-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`OverwriteExisting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html#cfn-transfer-workflow-decryptstepdetails-overwriteexisting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub overwrite_existing: Option<::Value<String>>,
        /// Property [`SourceFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html#cfn-transfer-workflow-decryptstepdetails-sourcefilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_file_location: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-decryptstepdetails.html#cfn-transfer-workflow-decryptstepdetails-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DecryptStepDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_file_location) = self.destination_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationFileLocation", destination_file_location)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref overwrite_existing) = self.overwrite_existing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverwriteExisting", overwrite_existing)?;
            }
            if let Some(ref source_file_location) = self.source_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFileLocation", source_file_location)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DecryptStepDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DecryptStepDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DecryptStepDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DecryptStepDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_file_location: Option<::Value<InputFileLocation>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut overwrite_existing: Option<::Value<String>> = None;
                    let mut source_file_location: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationFileLocation" => {
                                destination_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OverwriteExisting" => {
                                overwrite_existing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFileLocation" => {
                                source_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DecryptStepDetails {
                        destination_file_location: destination_file_location,
                        name: name,
                        overwrite_existing: overwrite_existing,
                        source_file_location: source_file_location,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.DeleteStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-deletestepdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct DeleteStepDetails {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-deletestepdetails.html#cfn-transfer-workflow-deletestepdetails-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SourceFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-deletestepdetails.html#cfn-transfer-workflow-deletestepdetails-sourcefilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_file_location: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeleteStepDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref source_file_location) = self.source_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFileLocation", source_file_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeleteStepDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeleteStepDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeleteStepDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeleteStepDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut source_file_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFileLocation" => {
                                source_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeleteStepDetails {
                        name: name,
                        source_file_location: source_file_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.EfsInputFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-efsinputfilelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct EfsInputFileLocation {
        /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-efsinputfilelocation.html#cfn-transfer-workflow-efsinputfilelocation-filesystemid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub file_system_id: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-efsinputfilelocation.html#cfn-transfer-workflow-efsinputfilelocation-path).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EfsInputFileLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file_system_id) = self.file_system_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", file_system_id)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EfsInputFileLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EfsInputFileLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EfsInputFileLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EfsInputFileLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file_system_id: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FileSystemId" => {
                                file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EfsInputFileLocation {
                        file_system_id: file_system_id,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.InputFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-inputfilelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct InputFileLocation {
        /// Property [`EfsFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-inputfilelocation.html#cfn-transfer-workflow-inputfilelocation-efsfilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub efs_file_location: Option<::Value<EfsInputFileLocation>>,
        /// Property [`S3FileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-inputfilelocation.html#cfn-transfer-workflow-inputfilelocation-s3filelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_file_location: Option<::Value<S3InputFileLocation>>,
    }

    impl ::codec::SerializeValue for InputFileLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref efs_file_location) = self.efs_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EfsFileLocation", efs_file_location)?;
            }
            if let Some(ref s3_file_location) = self.s3_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3FileLocation", s3_file_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputFileLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputFileLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputFileLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputFileLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut efs_file_location: Option<::Value<EfsInputFileLocation>> = None;
                    let mut s3_file_location: Option<::Value<S3InputFileLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EfsFileLocation" => {
                                efs_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3FileLocation" => {
                                s3_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputFileLocation {
                        efs_file_location: efs_file_location,
                        s3_file_location: s3_file_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.S3FileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3filelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3FileLocation {
        /// Property [`S3FileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3filelocation.html#cfn-transfer-workflow-s3filelocation-s3filelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_file_location: Option<::Value<S3InputFileLocation>>,
    }

    impl ::codec::SerializeValue for S3FileLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_file_location) = self.s3_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3FileLocation", s3_file_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3FileLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3FileLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3FileLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3FileLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_file_location: Option<::Value<S3InputFileLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3FileLocation" => {
                                s3_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3FileLocation {
                        s3_file_location: s3_file_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.S3InputFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3inputfilelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3InputFileLocation {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3inputfilelocation.html#cfn-transfer-workflow-s3inputfilelocation-bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3inputfilelocation.html#cfn-transfer-workflow-s3inputfilelocation-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3InputFileLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3InputFileLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3InputFileLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3InputFileLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3InputFileLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3InputFileLocation {
                        bucket: bucket,
                        key: key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.S3Tag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3tag.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Tag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3tag.html#cfn-transfer-workflow-s3tag-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-s3tag.html#cfn-transfer-workflow-s3tag-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Tag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Tag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Tag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Tag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Tag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Tag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.TagStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-tagstepdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct TagStepDetails {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-tagstepdetails.html#cfn-transfer-workflow-tagstepdetails-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SourceFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-tagstepdetails.html#cfn-transfer-workflow-tagstepdetails-sourcefilelocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_file_location: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-tagstepdetails.html#cfn-transfer-workflow-tagstepdetails-tags).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tags: Option<::ValueList<S3Tag>>,
    }

    impl ::codec::SerializeValue for TagStepDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref source_file_location) = self.source_file_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFileLocation", source_file_location)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagStepDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagStepDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagStepDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagStepDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut source_file_location: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<S3Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFileLocation" => {
                                source_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagStepDetails {
                        name: name,
                        source_file_location: source_file_location,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Workflow.WorkflowStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowStep {
        /// Property [`CopyStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-copystepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_step_details: Option<::Value<CopyStepDetails>>,
        /// Property [`CustomStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-customstepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_step_details: Option<::Value<CustomStepDetails>>,
        /// Property [`DecryptStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-decryptstepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub decrypt_step_details: Option<::Value<DecryptStepDetails>>,
        /// Property [`DeleteStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-deletestepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub delete_step_details: Option<::Value<DeleteStepDetails>>,
        /// Property [`TagStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-tagstepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_step_details: Option<::Value<TagStepDetails>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WorkflowStep {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_step_details) = self.copy_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyStepDetails", copy_step_details)?;
            }
            if let Some(ref custom_step_details) = self.custom_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomStepDetails", custom_step_details)?;
            }
            if let Some(ref decrypt_step_details) = self.decrypt_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecryptStepDetails", decrypt_step_details)?;
            }
            if let Some(ref delete_step_details) = self.delete_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteStepDetails", delete_step_details)?;
            }
            if let Some(ref tag_step_details) = self.tag_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagStepDetails", tag_step_details)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowStep {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowStep, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowStep;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowStep")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_step_details: Option<::Value<CopyStepDetails>> = None;
                    let mut custom_step_details: Option<::Value<CustomStepDetails>> = None;
                    let mut decrypt_step_details: Option<::Value<DecryptStepDetails>> = None;
                    let mut delete_step_details: Option<::Value<DeleteStepDetails>> = None;
                    let mut tag_step_details: Option<::Value<TagStepDetails>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyStepDetails" => {
                                copy_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomStepDetails" => {
                                custom_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DecryptStepDetails" => {
                                decrypt_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeleteStepDetails" => {
                                delete_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagStepDetails" => {
                                tag_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowStep {
                        copy_step_details: copy_step_details,
                        custom_step_details: custom_step_details,
                        decrypt_step_details: decrypt_step_details,
                        delete_step_details: delete_step_details,
                        tag_step_details: tag_step_details,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
