//! Types for the `ACMPCA` service.

/// The [`AWS::ACMPCA::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`ApiPassthrough`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-apipassthrough).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_passthrough: Option<::Value<self::certificate::ApiPassthrough>>,
    /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-certificateauthorityarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_authority_arn: ::Value<String>,
    /// Property [`CertificateSigningRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-certificatesigningrequest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_signing_request: ::Value<String>,
    /// Property [`SigningAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-signingalgorithm).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signing_algorithm: ::Value<String>,
    /// Property [`TemplateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-templatearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_arn: Option<::Value<String>>,
    /// Property [`Validity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-validity).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validity: ::Value<self::certificate::Validity>,
    /// Property [`ValidityNotBefore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html#cfn-acmpca-certificate-validitynotbefore).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validity_not_before: Option<::Value<self::certificate::Validity>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref api_passthrough) = self.api_passthrough {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiPassthrough", api_passthrough)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", &self.certificate_authority_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateSigningRequest", &self.certificate_signing_request)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningAlgorithm", &self.signing_algorithm)?;
        if let Some(ref template_arn) = self.template_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateArn", template_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validity", &self.validity)?;
        if let Some(ref validity_not_before) = self.validity_not_before {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidityNotBefore", validity_not_before)?;
        }
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
                let mut api_passthrough: Option<::Value<self::certificate::ApiPassthrough>> = None;
                let mut certificate_authority_arn: Option<::Value<String>> = None;
                let mut certificate_signing_request: Option<::Value<String>> = None;
                let mut signing_algorithm: Option<::Value<String>> = None;
                let mut template_arn: Option<::Value<String>> = None;
                let mut validity: Option<::Value<self::certificate::Validity>> = None;
                let mut validity_not_before: Option<::Value<self::certificate::Validity>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiPassthrough" => {
                            api_passthrough = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateAuthorityArn" => {
                            certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateSigningRequest" => {
                            certificate_signing_request = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SigningAlgorithm" => {
                            signing_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateArn" => {
                            template_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Validity" => {
                            validity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidityNotBefore" => {
                            validity_not_before = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    api_passthrough: api_passthrough,
                    certificate_authority_arn: certificate_authority_arn.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArn"))?,
                    certificate_signing_request: certificate_signing_request.ok_or(::serde::de::Error::missing_field("CertificateSigningRequest"))?,
                    signing_algorithm: signing_algorithm.ok_or(::serde::de::Error::missing_field("SigningAlgorithm"))?,
                    template_arn: template_arn,
                    validity: validity.ok_or(::serde::de::Error::missing_field("Validity"))?,
                    validity_not_before: validity_not_before,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::ACMPCA::Certificate";
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

/// The [`AWS::ACMPCA::CertificateAuthority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html) resource type.
#[derive(Debug, Default)]
pub struct CertificateAuthority {
    properties: CertificateAuthorityProperties
}

/// Properties for the `CertificateAuthority` resource.
#[derive(Debug, Default)]
pub struct CertificateAuthorityProperties {
    /// Property [`CsrExtensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-csrextensions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub csr_extensions: Option<::Value<self::certificate_authority::CsrExtensions>>,
    /// Property [`KeyAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-keyalgorithm).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_algorithm: ::Value<String>,
    /// Property [`KeyStorageSecurityStandard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-keystoragesecuritystandard).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_storage_security_standard: Option<::Value<String>>,
    /// Property [`RevocationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-revocationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub revocation_configuration: Option<::Value<self::certificate_authority::RevocationConfiguration>>,
    /// Property [`SigningAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-signingalgorithm).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signing_algorithm: ::Value<String>,
    /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-subject).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subject: ::Value<self::certificate_authority::Subject>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html#cfn-acmpca-certificateauthority-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for CertificateAuthorityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref csr_extensions) = self.csr_extensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsrExtensions", csr_extensions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAlgorithm", &self.key_algorithm)?;
        if let Some(ref key_storage_security_standard) = self.key_storage_security_standard {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyStorageSecurityStandard", key_storage_security_standard)?;
        }
        if let Some(ref revocation_configuration) = self.revocation_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevocationConfiguration", revocation_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningAlgorithm", &self.signing_algorithm)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", &self.subject)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateAuthorityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateAuthorityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateAuthorityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateAuthorityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut csr_extensions: Option<::Value<self::certificate_authority::CsrExtensions>> = None;
                let mut key_algorithm: Option<::Value<String>> = None;
                let mut key_storage_security_standard: Option<::Value<String>> = None;
                let mut revocation_configuration: Option<::Value<self::certificate_authority::RevocationConfiguration>> = None;
                let mut signing_algorithm: Option<::Value<String>> = None;
                let mut subject: Option<::Value<self::certificate_authority::Subject>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CsrExtensions" => {
                            csr_extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyAlgorithm" => {
                            key_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyStorageSecurityStandard" => {
                            key_storage_security_standard = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RevocationConfiguration" => {
                            revocation_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SigningAlgorithm" => {
                            signing_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subject" => {
                            subject = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateAuthorityProperties {
                    csr_extensions: csr_extensions,
                    key_algorithm: key_algorithm.ok_or(::serde::de::Error::missing_field("KeyAlgorithm"))?,
                    key_storage_security_standard: key_storage_security_standard,
                    revocation_configuration: revocation_configuration,
                    signing_algorithm: signing_algorithm.ok_or(::serde::de::Error::missing_field("SigningAlgorithm"))?,
                    subject: subject.ok_or(::serde::de::Error::missing_field("Subject"))?,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CertificateAuthority {
    type Properties = CertificateAuthorityProperties;
    const TYPE: &'static str = "AWS::ACMPCA::CertificateAuthority";
    fn properties(&self) -> &CertificateAuthorityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateAuthorityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CertificateAuthority {}

impl From<CertificateAuthorityProperties> for CertificateAuthority {
    fn from(properties: CertificateAuthorityProperties) -> CertificateAuthority {
        CertificateAuthority { properties }
    }
}

/// The [`AWS::ACMPCA::CertificateAuthorityActivation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html) resource type.
#[derive(Debug, Default)]
pub struct CertificateAuthorityActivation {
    properties: CertificateAuthorityActivationProperties
}

/// Properties for the `CertificateAuthorityActivation` resource.
#[derive(Debug, Default)]
pub struct CertificateAuthorityActivationProperties {
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html#cfn-acmpca-certificateauthorityactivation-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: ::Value<String>,
    /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html#cfn-acmpca-certificateauthorityactivation-certificateauthorityarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_authority_arn: ::Value<String>,
    /// Property [`CertificateChain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html#cfn-acmpca-certificateauthorityactivation-certificatechain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_chain: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html#cfn-acmpca-certificateauthorityactivation-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
}

impl ::serde::Serialize for CertificateAuthorityActivationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", &self.certificate)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", &self.certificate_authority_arn)?;
        if let Some(ref certificate_chain) = self.certificate_chain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateChain", certificate_chain)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateAuthorityActivationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateAuthorityActivationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateAuthorityActivationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateAuthorityActivationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate: Option<::Value<String>> = None;
                let mut certificate_authority_arn: Option<::Value<String>> = None;
                let mut certificate_chain: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateAuthorityArn" => {
                            certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateChain" => {
                            certificate_chain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateAuthorityActivationProperties {
                    certificate: certificate.ok_or(::serde::de::Error::missing_field("Certificate"))?,
                    certificate_authority_arn: certificate_authority_arn.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArn"))?,
                    certificate_chain: certificate_chain,
                    status: status,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CertificateAuthorityActivation {
    type Properties = CertificateAuthorityActivationProperties;
    const TYPE: &'static str = "AWS::ACMPCA::CertificateAuthorityActivation";
    fn properties(&self) -> &CertificateAuthorityActivationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateAuthorityActivationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CertificateAuthorityActivation {}

impl From<CertificateAuthorityActivationProperties> for CertificateAuthorityActivation {
    fn from(properties: CertificateAuthorityActivationProperties) -> CertificateAuthorityActivation {
        CertificateAuthorityActivation { properties }
    }
}

/// The [`AWS::ACMPCA::Permission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html) resource type.
#[derive(Debug, Default)]
pub struct Permission {
    properties: PermissionProperties
}

/// Properties for the `Permission` resource.
#[derive(Debug, Default)]
pub struct PermissionProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html#cfn-acmpca-permission-actions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub actions: ::ValueList<String>,
    /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html#cfn-acmpca-permission-certificateauthorityarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_authority_arn: ::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html#cfn-acmpca-permission-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`SourceAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html#cfn-acmpca-permission-sourceaccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_account: Option<::Value<String>>,
}

impl ::serde::Serialize for PermissionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", &self.certificate_authority_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        if let Some(ref source_account) = self.source_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAccount", source_account)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<String>> = None;
                let mut certificate_authority_arn: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;
                let mut source_account: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateAuthorityArn" => {
                            certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAccount" => {
                            source_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PermissionProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    certificate_authority_arn: certificate_authority_arn.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArn"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    source_account: source_account,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Permission {
    type Properties = PermissionProperties;
    const TYPE: &'static str = "AWS::ACMPCA::Permission";
    fn properties(&self) -> &PermissionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Permission {}

impl From<PermissionProperties> for Permission {
    fn from(properties: PermissionProperties) -> Permission {
        Permission { properties }
    }
}

pub mod certificate {
    //! Property types for the `Certificate` resource.

    /// The [`AWS::ACMPCA::Certificate.ApiPassthrough`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-apipassthrough.html) property type.
    #[derive(Debug, Default)]
    pub struct ApiPassthrough {
        /// Property [`Extensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-apipassthrough.html#cfn-acmpca-certificate-apipassthrough-extensions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub extensions: Option<::Value<Extensions>>,
        /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-apipassthrough.html#cfn-acmpca-certificate-apipassthrough-subject).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subject: Option<::Value<Subject>>,
    }

    impl ::codec::SerializeValue for ApiPassthrough {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref extensions) = self.extensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Extensions", extensions)?;
            }
            if let Some(ref subject) = self.subject {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", subject)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApiPassthrough {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiPassthrough, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApiPassthrough;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApiPassthrough")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut extensions: Option<::Value<Extensions>> = None;
                    let mut subject: Option<::Value<Subject>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Extensions" => {
                                extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subject" => {
                                subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApiPassthrough {
                        extensions: extensions,
                        subject: subject,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.CustomAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomAttribute {
        /// Property [`ObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customattribute.html#cfn-acmpca-certificate-customattribute-objectidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_identifier: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customattribute.html#cfn-acmpca-certificate-customattribute-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectIdentifier", &self.object_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object_identifier: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObjectIdentifier" => {
                                object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomAttribute {
                        object_identifier: object_identifier.ok_or(::serde::de::Error::missing_field("ObjectIdentifier"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.CustomExtension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customextension.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomExtension {
        /// Property [`Critical`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customextension.html#cfn-acmpca-certificate-customextension-critical).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub critical: Option<::Value<bool>>,
        /// Property [`ObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customextension.html#cfn-acmpca-certificate-customextension-objectidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_identifier: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customextension.html#cfn-acmpca-certificate-customextension-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomExtension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref critical) = self.critical {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Critical", critical)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectIdentifier", &self.object_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomExtension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomExtension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomExtension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomExtension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut critical: Option<::Value<bool>> = None;
                    let mut object_identifier: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Critical" => {
                                critical = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectIdentifier" => {
                                object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomExtension {
                        critical: critical,
                        object_identifier: object_identifier.ok_or(::serde::de::Error::missing_field("ObjectIdentifier"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.EdiPartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-edipartyname.html) property type.
    #[derive(Debug, Default)]
    pub struct EdiPartyName {
        /// Property [`NameAssigner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-edipartyname.html#cfn-acmpca-certificate-edipartyname-nameassigner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name_assigner: ::Value<String>,
        /// Property [`PartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-edipartyname.html#cfn-acmpca-certificate-edipartyname-partyname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub party_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for EdiPartyName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameAssigner", &self.name_assigner)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartyName", &self.party_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EdiPartyName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EdiPartyName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EdiPartyName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EdiPartyName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name_assigner: Option<::Value<String>> = None;
                    let mut party_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NameAssigner" => {
                                name_assigner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartyName" => {
                                party_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EdiPartyName {
                        name_assigner: name_assigner.ok_or(::serde::de::Error::missing_field("NameAssigner"))?,
                        party_name: party_name.ok_or(::serde::de::Error::missing_field("PartyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.ExtendedKeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extendedkeyusage.html) property type.
    #[derive(Debug, Default)]
    pub struct ExtendedKeyUsage {
        /// Property [`ExtendedKeyUsageObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extendedkeyusage.html#cfn-acmpca-certificate-extendedkeyusage-extendedkeyusageobjectidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub extended_key_usage_object_identifier: Option<::Value<String>>,
        /// Property [`ExtendedKeyUsageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extendedkeyusage.html#cfn-acmpca-certificate-extendedkeyusage-extendedkeyusagetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub extended_key_usage_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExtendedKeyUsage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref extended_key_usage_object_identifier) = self.extended_key_usage_object_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedKeyUsageObjectIdentifier", extended_key_usage_object_identifier)?;
            }
            if let Some(ref extended_key_usage_type) = self.extended_key_usage_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedKeyUsageType", extended_key_usage_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExtendedKeyUsage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExtendedKeyUsage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExtendedKeyUsage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExtendedKeyUsage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut extended_key_usage_object_identifier: Option<::Value<String>> = None;
                    let mut extended_key_usage_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExtendedKeyUsageObjectIdentifier" => {
                                extended_key_usage_object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtendedKeyUsageType" => {
                                extended_key_usage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExtendedKeyUsage {
                        extended_key_usage_object_identifier: extended_key_usage_object_identifier,
                        extended_key_usage_type: extended_key_usage_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.Extensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html) property type.
    #[derive(Debug, Default)]
    pub struct Extensions {
        /// Property [`CertificatePolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html#cfn-acmpca-certificate-extensions-certificatepolicies).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub certificate_policies: Option<::ValueList<PolicyInformation>>,
        /// Property [`CustomExtensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html#cfn-acmpca-certificate-extensions-customextensions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_extensions: Option<::ValueList<CustomExtension>>,
        /// Property [`ExtendedKeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html#cfn-acmpca-certificate-extensions-extendedkeyusage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub extended_key_usage: Option<::ValueList<ExtendedKeyUsage>>,
        /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html#cfn-acmpca-certificate-extensions-keyusage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_usage: Option<::Value<KeyUsage>>,
        /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html#cfn-acmpca-certificate-extensions-subjectalternativenames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subject_alternative_names: Option<::ValueList<GeneralName>>,
    }

    impl ::codec::SerializeValue for Extensions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_policies) = self.certificate_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificatePolicies", certificate_policies)?;
            }
            if let Some(ref custom_extensions) = self.custom_extensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomExtensions", custom_extensions)?;
            }
            if let Some(ref extended_key_usage) = self.extended_key_usage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtendedKeyUsage", extended_key_usage)?;
            }
            if let Some(ref key_usage) = self.key_usage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", key_usage)?;
            }
            if let Some(ref subject_alternative_names) = self.subject_alternative_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Extensions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Extensions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Extensions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Extensions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_policies: Option<::ValueList<PolicyInformation>> = None;
                    let mut custom_extensions: Option<::ValueList<CustomExtension>> = None;
                    let mut extended_key_usage: Option<::ValueList<ExtendedKeyUsage>> = None;
                    let mut key_usage: Option<::Value<KeyUsage>> = None;
                    let mut subject_alternative_names: Option<::ValueList<GeneralName>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificatePolicies" => {
                                certificate_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomExtensions" => {
                                custom_extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtendedKeyUsage" => {
                                extended_key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsage" => {
                                key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectAlternativeNames" => {
                                subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Extensions {
                        certificate_policies: certificate_policies,
                        custom_extensions: custom_extensions,
                        extended_key_usage: extended_key_usage,
                        key_usage: key_usage,
                        subject_alternative_names: subject_alternative_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.GeneralName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html) property type.
    #[derive(Debug, Default)]
    pub struct GeneralName {
        /// Property [`DirectoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-directoryname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub directory_name: Option<::Value<Subject>>,
        /// Property [`DnsName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-dnsname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub dns_name: Option<::Value<String>>,
        /// Property [`EdiPartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-edipartyname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub edi_party_name: Option<::Value<EdiPartyName>>,
        /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-ipaddress).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ip_address: Option<::Value<String>>,
        /// Property [`OtherName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-othername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub other_name: Option<::Value<OtherName>>,
        /// Property [`RegisteredId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-registeredid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub registered_id: Option<::Value<String>>,
        /// Property [`Rfc822Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-rfc822name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub rfc822_name: Option<::Value<String>>,
        /// Property [`UniformResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html#cfn-acmpca-certificate-generalname-uniformresourceidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub uniform_resource_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeneralName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_name) = self.directory_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryName", directory_name)?;
            }
            if let Some(ref dns_name) = self.dns_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsName", dns_name)?;
            }
            if let Some(ref edi_party_name) = self.edi_party_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EdiPartyName", edi_party_name)?;
            }
            if let Some(ref ip_address) = self.ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", ip_address)?;
            }
            if let Some(ref other_name) = self.other_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OtherName", other_name)?;
            }
            if let Some(ref registered_id) = self.registered_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegisteredId", registered_id)?;
            }
            if let Some(ref rfc822_name) = self.rfc822_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rfc822Name", rfc822_name)?;
            }
            if let Some(ref uniform_resource_identifier) = self.uniform_resource_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UniformResourceIdentifier", uniform_resource_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeneralName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeneralName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeneralName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeneralName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_name: Option<::Value<Subject>> = None;
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut edi_party_name: Option<::Value<EdiPartyName>> = None;
                    let mut ip_address: Option<::Value<String>> = None;
                    let mut other_name: Option<::Value<OtherName>> = None;
                    let mut registered_id: Option<::Value<String>> = None;
                    let mut rfc822_name: Option<::Value<String>> = None;
                    let mut uniform_resource_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryName" => {
                                directory_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EdiPartyName" => {
                                edi_party_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IpAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OtherName" => {
                                other_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegisteredId" => {
                                registered_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rfc822Name" => {
                                rfc822_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UniformResourceIdentifier" => {
                                uniform_resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeneralName {
                        directory_name: directory_name,
                        dns_name: dns_name,
                        edi_party_name: edi_party_name,
                        ip_address: ip_address,
                        other_name: other_name,
                        registered_id: registered_id,
                        rfc822_name: rfc822_name,
                        uniform_resource_identifier: uniform_resource_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsage {
        /// Property [`CRLSign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-crlsign).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub crl_sign: Option<::Value<bool>>,
        /// Property [`DataEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-dataencipherment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_encipherment: Option<::Value<bool>>,
        /// Property [`DecipherOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-decipheronly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub decipher_only: Option<::Value<bool>>,
        /// Property [`DigitalSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-digitalsignature).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub digital_signature: Option<::Value<bool>>,
        /// Property [`EncipherOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-encipheronly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encipher_only: Option<::Value<bool>>,
        /// Property [`KeyAgreement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-keyagreement).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_agreement: Option<::Value<bool>>,
        /// Property [`KeyCertSign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-keycertsign).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_cert_sign: Option<::Value<bool>>,
        /// Property [`KeyEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-keyencipherment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_encipherment: Option<::Value<bool>>,
        /// Property [`NonRepudiation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html#cfn-acmpca-certificate-keyusage-nonrepudiation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub non_repudiation: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for KeyUsage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crl_sign) = self.crl_sign {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CRLSign", crl_sign)?;
            }
            if let Some(ref data_encipherment) = self.data_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataEncipherment", data_encipherment)?;
            }
            if let Some(ref decipher_only) = self.decipher_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecipherOnly", decipher_only)?;
            }
            if let Some(ref digital_signature) = self.digital_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DigitalSignature", digital_signature)?;
            }
            if let Some(ref encipher_only) = self.encipher_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncipherOnly", encipher_only)?;
            }
            if let Some(ref key_agreement) = self.key_agreement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAgreement", key_agreement)?;
            }
            if let Some(ref key_cert_sign) = self.key_cert_sign {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyCertSign", key_cert_sign)?;
            }
            if let Some(ref key_encipherment) = self.key_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyEncipherment", key_encipherment)?;
            }
            if let Some(ref non_repudiation) = self.non_repudiation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonRepudiation", non_repudiation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crl_sign: Option<::Value<bool>> = None;
                    let mut data_encipherment: Option<::Value<bool>> = None;
                    let mut decipher_only: Option<::Value<bool>> = None;
                    let mut digital_signature: Option<::Value<bool>> = None;
                    let mut encipher_only: Option<::Value<bool>> = None;
                    let mut key_agreement: Option<::Value<bool>> = None;
                    let mut key_cert_sign: Option<::Value<bool>> = None;
                    let mut key_encipherment: Option<::Value<bool>> = None;
                    let mut non_repudiation: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CRLSign" => {
                                crl_sign = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataEncipherment" => {
                                data_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DecipherOnly" => {
                                decipher_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DigitalSignature" => {
                                digital_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncipherOnly" => {
                                encipher_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyAgreement" => {
                                key_agreement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyCertSign" => {
                                key_cert_sign = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyEncipherment" => {
                                key_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NonRepudiation" => {
                                non_repudiation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsage {
                        crl_sign: crl_sign,
                        data_encipherment: data_encipherment,
                        decipher_only: decipher_only,
                        digital_signature: digital_signature,
                        encipher_only: encipher_only,
                        key_agreement: key_agreement,
                        key_cert_sign: key_cert_sign,
                        key_encipherment: key_encipherment,
                        non_repudiation: non_repudiation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.OtherName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-othername.html) property type.
    #[derive(Debug, Default)]
    pub struct OtherName {
        /// Property [`TypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-othername.html#cfn-acmpca-certificate-othername-typeid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub type_id: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-othername.html#cfn-acmpca-certificate-othername-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for OtherName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeId", &self.type_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OtherName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OtherName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OtherName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OtherName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_id: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TypeId" => {
                                type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OtherName {
                        type_id: type_id.ok_or(::serde::de::Error::missing_field("TypeId"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.PolicyInformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyinformation.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyInformation {
        /// Property [`CertPolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyinformation.html#cfn-acmpca-certificate-policyinformation-certpolicyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cert_policy_id: ::Value<String>,
        /// Property [`PolicyQualifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyinformation.html#cfn-acmpca-certificate-policyinformation-policyqualifiers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub policy_qualifiers: Option<::ValueList<PolicyQualifierInfo>>,
    }

    impl ::codec::SerializeValue for PolicyInformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertPolicyId", &self.cert_policy_id)?;
            if let Some(ref policy_qualifiers) = self.policy_qualifiers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyQualifiers", policy_qualifiers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyInformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyInformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyInformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyInformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cert_policy_id: Option<::Value<String>> = None;
                    let mut policy_qualifiers: Option<::ValueList<PolicyQualifierInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertPolicyId" => {
                                cert_policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyQualifiers" => {
                                policy_qualifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyInformation {
                        cert_policy_id: cert_policy_id.ok_or(::serde::de::Error::missing_field("CertPolicyId"))?,
                        policy_qualifiers: policy_qualifiers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.PolicyQualifierInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyqualifierinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyQualifierInfo {
        /// Property [`PolicyQualifierId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyqualifierinfo.html#cfn-acmpca-certificate-policyqualifierinfo-policyqualifierid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub policy_qualifier_id: ::Value<String>,
        /// Property [`Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyqualifierinfo.html#cfn-acmpca-certificate-policyqualifierinfo-qualifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub qualifier: ::Value<Qualifier>,
    }

    impl ::codec::SerializeValue for PolicyQualifierInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyQualifierId", &self.policy_qualifier_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", &self.qualifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyQualifierInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyQualifierInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyQualifierInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyQualifierInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_qualifier_id: Option<::Value<String>> = None;
                    let mut qualifier: Option<::Value<Qualifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyQualifierId" => {
                                policy_qualifier_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Qualifier" => {
                                qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyQualifierInfo {
                        policy_qualifier_id: policy_qualifier_id.ok_or(::serde::de::Error::missing_field("PolicyQualifierId"))?,
                        qualifier: qualifier.ok_or(::serde::de::Error::missing_field("Qualifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.Qualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-qualifier.html) property type.
    #[derive(Debug, Default)]
    pub struct Qualifier {
        /// Property [`CpsUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-qualifier.html#cfn-acmpca-certificate-qualifier-cpsuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cps_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for Qualifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpsUri", &self.cps_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Qualifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Qualifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Qualifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Qualifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cps_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpsUri" => {
                                cps_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Qualifier {
                        cps_uri: cps_uri.ok_or(::serde::de::Error::missing_field("CpsUri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html) property type.
    #[derive(Debug, Default)]
    pub struct Subject {
        /// Property [`CommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-commonname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub common_name: Option<::Value<String>>,
        /// Property [`Country`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-country).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub country: Option<::Value<String>>,
        /// Property [`CustomAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-customattributes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_attributes: Option<::ValueList<CustomAttribute>>,
        /// Property [`DistinguishedNameQualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-distinguishednamequalifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub distinguished_name_qualifier: Option<::Value<String>>,
        /// Property [`GenerationQualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-generationqualifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub generation_qualifier: Option<::Value<String>>,
        /// Property [`GivenName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-givenname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub given_name: Option<::Value<String>>,
        /// Property [`Initials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-initials).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub initials: Option<::Value<String>>,
        /// Property [`Locality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-locality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub locality: Option<::Value<String>>,
        /// Property [`Organization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-organization).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organization: Option<::Value<String>>,
        /// Property [`OrganizationalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-organizationalunit).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organizational_unit: Option<::Value<String>>,
        /// Property [`Pseudonym`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-pseudonym).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pseudonym: Option<::Value<String>>,
        /// Property [`SerialNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-serialnumber).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub serial_number: Option<::Value<String>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-state).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub state: Option<::Value<String>>,
        /// Property [`Surname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-surname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub surname: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html#cfn-acmpca-certificate-subject-title).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub title: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Subject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref common_name) = self.common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommonName", common_name)?;
            }
            if let Some(ref country) = self.country {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Country", country)?;
            }
            if let Some(ref custom_attributes) = self.custom_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomAttributes", custom_attributes)?;
            }
            if let Some(ref distinguished_name_qualifier) = self.distinguished_name_qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistinguishedNameQualifier", distinguished_name_qualifier)?;
            }
            if let Some(ref generation_qualifier) = self.generation_qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerationQualifier", generation_qualifier)?;
            }
            if let Some(ref given_name) = self.given_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GivenName", given_name)?;
            }
            if let Some(ref initials) = self.initials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Initials", initials)?;
            }
            if let Some(ref locality) = self.locality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Locality", locality)?;
            }
            if let Some(ref organization) = self.organization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Organization", organization)?;
            }
            if let Some(ref organizational_unit) = self.organizational_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnit", organizational_unit)?;
            }
            if let Some(ref pseudonym) = self.pseudonym {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pseudonym", pseudonym)?;
            }
            if let Some(ref serial_number) = self.serial_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerialNumber", serial_number)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            if let Some(ref surname) = self.surname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Surname", surname)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut common_name: Option<::Value<String>> = None;
                    let mut country: Option<::Value<String>> = None;
                    let mut custom_attributes: Option<::ValueList<CustomAttribute>> = None;
                    let mut distinguished_name_qualifier: Option<::Value<String>> = None;
                    let mut generation_qualifier: Option<::Value<String>> = None;
                    let mut given_name: Option<::Value<String>> = None;
                    let mut initials: Option<::Value<String>> = None;
                    let mut locality: Option<::Value<String>> = None;
                    let mut organization: Option<::Value<String>> = None;
                    let mut organizational_unit: Option<::Value<String>> = None;
                    let mut pseudonym: Option<::Value<String>> = None;
                    let mut serial_number: Option<::Value<String>> = None;
                    let mut state: Option<::Value<String>> = None;
                    let mut surname: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CommonName" => {
                                common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Country" => {
                                country = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomAttributes" => {
                                custom_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DistinguishedNameQualifier" => {
                                distinguished_name_qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GenerationQualifier" => {
                                generation_qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GivenName" => {
                                given_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Initials" => {
                                initials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Locality" => {
                                locality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Organization" => {
                                organization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnit" => {
                                organizational_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pseudonym" => {
                                pseudonym = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerialNumber" => {
                                serial_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Surname" => {
                                surname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subject {
                        common_name: common_name,
                        country: country,
                        custom_attributes: custom_attributes,
                        distinguished_name_qualifier: distinguished_name_qualifier,
                        generation_qualifier: generation_qualifier,
                        given_name: given_name,
                        initials: initials,
                        locality: locality,
                        organization: organization,
                        organizational_unit: organizational_unit,
                        pseudonym: pseudonym,
                        serial_number: serial_number,
                        state: state,
                        surname: surname,
                        title: title,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::Certificate.Validity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-validity.html) property type.
    #[derive(Debug, Default)]
    pub struct Validity {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-validity.html#cfn-acmpca-certificate-validity-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-validity.html#cfn-acmpca-certificate-validity-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for Validity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Validity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Validity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Validity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Validity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

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

                    Ok(Validity {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod certificate_authority {
    //! Property types for the `CertificateAuthority` resource.

    /// The [`AWS::ACMPCA::CertificateAuthority.AccessDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessdescription.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessDescription {
        /// Property [`AccessLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessdescription.html#cfn-acmpca-certificateauthority-accessdescription-accesslocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_location: ::Value<GeneralName>,
        /// Property [`AccessMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessdescription.html#cfn-acmpca-certificateauthority-accessdescription-accessmethod).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_method: ::Value<AccessMethod>,
    }

    impl ::codec::SerializeValue for AccessDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLocation", &self.access_location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessMethod", &self.access_method)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_location: Option<::Value<GeneralName>> = None;
                    let mut access_method: Option<::Value<AccessMethod>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessLocation" => {
                                access_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccessMethod" => {
                                access_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessDescription {
                        access_location: access_location.ok_or(::serde::de::Error::missing_field("AccessLocation"))?,
                        access_method: access_method.ok_or(::serde::de::Error::missing_field("AccessMethod"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.AccessMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessmethod.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessMethod {
        /// Property [`AccessMethodType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessmethod.html#cfn-acmpca-certificateauthority-accessmethod-accessmethodtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_method_type: Option<::Value<String>>,
        /// Property [`CustomObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessmethod.html#cfn-acmpca-certificateauthority-accessmethod-customobjectidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_object_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessMethod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_method_type) = self.access_method_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessMethodType", access_method_type)?;
            }
            if let Some(ref custom_object_identifier) = self.custom_object_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomObjectIdentifier", custom_object_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessMethod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessMethod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessMethod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessMethod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_method_type: Option<::Value<String>> = None;
                    let mut custom_object_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessMethodType" => {
                                access_method_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomObjectIdentifier" => {
                                custom_object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessMethod {
                        access_method_type: access_method_type,
                        custom_object_identifier: custom_object_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.CrlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CrlConfiguration {
        /// Property [`CustomCname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html#cfn-acmpca-certificateauthority-crlconfiguration-customcname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_cname: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html#cfn-acmpca-certificateauthority-crlconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`ExpirationInDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html#cfn-acmpca-certificateauthority-crlconfiguration-expirationindays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expiration_in_days: Option<::Value<u32>>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html#cfn-acmpca-certificateauthority-crlconfiguration-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: Option<::Value<String>>,
        /// Property [`S3ObjectAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html#cfn-acmpca-certificateauthority-crlconfiguration-s3objectacl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_acl: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CrlConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_cname) = self.custom_cname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomCname", custom_cname)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref expiration_in_days) = self.expiration_in_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpirationInDays", expiration_in_days)?;
            }
            if let Some(ref s3_bucket_name) = self.s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", s3_bucket_name)?;
            }
            if let Some(ref s3_object_acl) = self.s3_object_acl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectAcl", s3_object_acl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrlConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrlConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrlConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrlConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_cname: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut expiration_in_days: Option<::Value<u32>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_object_acl: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomCname" => {
                                custom_cname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpirationInDays" => {
                                expiration_in_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectAcl" => {
                                s3_object_acl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrlConfiguration {
                        custom_cname: custom_cname,
                        enabled: enabled,
                        expiration_in_days: expiration_in_days,
                        s3_bucket_name: s3_bucket_name,
                        s3_object_acl: s3_object_acl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.CsrExtensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-csrextensions.html) property type.
    #[derive(Debug, Default)]
    pub struct CsrExtensions {
        /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-csrextensions.html#cfn-acmpca-certificateauthority-csrextensions-keyusage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_usage: Option<::Value<KeyUsage>>,
        /// Property [`SubjectInformationAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-csrextensions.html#cfn-acmpca-certificateauthority-csrextensions-subjectinformationaccess).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subject_information_access: Option<::ValueList<AccessDescription>>,
    }

    impl ::codec::SerializeValue for CsrExtensions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_usage) = self.key_usage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", key_usage)?;
            }
            if let Some(ref subject_information_access) = self.subject_information_access {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectInformationAccess", subject_information_access)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CsrExtensions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CsrExtensions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CsrExtensions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CsrExtensions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_usage: Option<::Value<KeyUsage>> = None;
                    let mut subject_information_access: Option<::ValueList<AccessDescription>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyUsage" => {
                                key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectInformationAccess" => {
                                subject_information_access = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CsrExtensions {
                        key_usage: key_usage,
                        subject_information_access: subject_information_access,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.CustomAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-customattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomAttribute {
        /// Property [`ObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-customattribute.html#cfn-acmpca-certificateauthority-customattribute-objectidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_identifier: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-customattribute.html#cfn-acmpca-certificateauthority-customattribute-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectIdentifier", &self.object_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object_identifier: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObjectIdentifier" => {
                                object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomAttribute {
                        object_identifier: object_identifier.ok_or(::serde::de::Error::missing_field("ObjectIdentifier"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.EdiPartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-edipartyname.html) property type.
    #[derive(Debug, Default)]
    pub struct EdiPartyName {
        /// Property [`NameAssigner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-edipartyname.html#cfn-acmpca-certificateauthority-edipartyname-nameassigner).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name_assigner: ::Value<String>,
        /// Property [`PartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-edipartyname.html#cfn-acmpca-certificateauthority-edipartyname-partyname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub party_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for EdiPartyName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NameAssigner", &self.name_assigner)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartyName", &self.party_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EdiPartyName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EdiPartyName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EdiPartyName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EdiPartyName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name_assigner: Option<::Value<String>> = None;
                    let mut party_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NameAssigner" => {
                                name_assigner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartyName" => {
                                party_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EdiPartyName {
                        name_assigner: name_assigner.ok_or(::serde::de::Error::missing_field("NameAssigner"))?,
                        party_name: party_name.ok_or(::serde::de::Error::missing_field("PartyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.GeneralName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html) property type.
    #[derive(Debug, Default)]
    pub struct GeneralName {
        /// Property [`DirectoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-directoryname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub directory_name: Option<::Value<Subject>>,
        /// Property [`DnsName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-dnsname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub dns_name: Option<::Value<String>>,
        /// Property [`EdiPartyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-edipartyname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub edi_party_name: Option<::Value<EdiPartyName>>,
        /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-ipaddress).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ip_address: Option<::Value<String>>,
        /// Property [`OtherName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-othername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub other_name: Option<::Value<OtherName>>,
        /// Property [`RegisteredId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-registeredid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub registered_id: Option<::Value<String>>,
        /// Property [`Rfc822Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-rfc822name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub rfc822_name: Option<::Value<String>>,
        /// Property [`UniformResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html#cfn-acmpca-certificateauthority-generalname-uniformresourceidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub uniform_resource_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GeneralName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_name) = self.directory_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryName", directory_name)?;
            }
            if let Some(ref dns_name) = self.dns_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsName", dns_name)?;
            }
            if let Some(ref edi_party_name) = self.edi_party_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EdiPartyName", edi_party_name)?;
            }
            if let Some(ref ip_address) = self.ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", ip_address)?;
            }
            if let Some(ref other_name) = self.other_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OtherName", other_name)?;
            }
            if let Some(ref registered_id) = self.registered_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegisteredId", registered_id)?;
            }
            if let Some(ref rfc822_name) = self.rfc822_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rfc822Name", rfc822_name)?;
            }
            if let Some(ref uniform_resource_identifier) = self.uniform_resource_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UniformResourceIdentifier", uniform_resource_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeneralName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeneralName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeneralName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeneralName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_name: Option<::Value<Subject>> = None;
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut edi_party_name: Option<::Value<EdiPartyName>> = None;
                    let mut ip_address: Option<::Value<String>> = None;
                    let mut other_name: Option<::Value<OtherName>> = None;
                    let mut registered_id: Option<::Value<String>> = None;
                    let mut rfc822_name: Option<::Value<String>> = None;
                    let mut uniform_resource_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryName" => {
                                directory_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EdiPartyName" => {
                                edi_party_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IpAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OtherName" => {
                                other_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegisteredId" => {
                                registered_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rfc822Name" => {
                                rfc822_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UniformResourceIdentifier" => {
                                uniform_resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeneralName {
                        directory_name: directory_name,
                        dns_name: dns_name,
                        edi_party_name: edi_party_name,
                        ip_address: ip_address,
                        other_name: other_name,
                        registered_id: registered_id,
                        rfc822_name: rfc822_name,
                        uniform_resource_identifier: uniform_resource_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsage {
        /// Property [`CRLSign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-crlsign).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub crl_sign: Option<::Value<bool>>,
        /// Property [`DataEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-dataencipherment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_encipherment: Option<::Value<bool>>,
        /// Property [`DecipherOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-decipheronly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub decipher_only: Option<::Value<bool>>,
        /// Property [`DigitalSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-digitalsignature).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub digital_signature: Option<::Value<bool>>,
        /// Property [`EncipherOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-encipheronly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encipher_only: Option<::Value<bool>>,
        /// Property [`KeyAgreement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-keyagreement).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_agreement: Option<::Value<bool>>,
        /// Property [`KeyCertSign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-keycertsign).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_cert_sign: Option<::Value<bool>>,
        /// Property [`KeyEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-keyencipherment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_encipherment: Option<::Value<bool>>,
        /// Property [`NonRepudiation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html#cfn-acmpca-certificateauthority-keyusage-nonrepudiation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub non_repudiation: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for KeyUsage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crl_sign) = self.crl_sign {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CRLSign", crl_sign)?;
            }
            if let Some(ref data_encipherment) = self.data_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataEncipherment", data_encipherment)?;
            }
            if let Some(ref decipher_only) = self.decipher_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecipherOnly", decipher_only)?;
            }
            if let Some(ref digital_signature) = self.digital_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DigitalSignature", digital_signature)?;
            }
            if let Some(ref encipher_only) = self.encipher_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncipherOnly", encipher_only)?;
            }
            if let Some(ref key_agreement) = self.key_agreement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAgreement", key_agreement)?;
            }
            if let Some(ref key_cert_sign) = self.key_cert_sign {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyCertSign", key_cert_sign)?;
            }
            if let Some(ref key_encipherment) = self.key_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyEncipherment", key_encipherment)?;
            }
            if let Some(ref non_repudiation) = self.non_repudiation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonRepudiation", non_repudiation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crl_sign: Option<::Value<bool>> = None;
                    let mut data_encipherment: Option<::Value<bool>> = None;
                    let mut decipher_only: Option<::Value<bool>> = None;
                    let mut digital_signature: Option<::Value<bool>> = None;
                    let mut encipher_only: Option<::Value<bool>> = None;
                    let mut key_agreement: Option<::Value<bool>> = None;
                    let mut key_cert_sign: Option<::Value<bool>> = None;
                    let mut key_encipherment: Option<::Value<bool>> = None;
                    let mut non_repudiation: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CRLSign" => {
                                crl_sign = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataEncipherment" => {
                                data_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DecipherOnly" => {
                                decipher_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DigitalSignature" => {
                                digital_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncipherOnly" => {
                                encipher_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyAgreement" => {
                                key_agreement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyCertSign" => {
                                key_cert_sign = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyEncipherment" => {
                                key_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NonRepudiation" => {
                                non_repudiation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsage {
                        crl_sign: crl_sign,
                        data_encipherment: data_encipherment,
                        decipher_only: decipher_only,
                        digital_signature: digital_signature,
                        encipher_only: encipher_only,
                        key_agreement: key_agreement,
                        key_cert_sign: key_cert_sign,
                        key_encipherment: key_encipherment,
                        non_repudiation: non_repudiation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.OcspConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-ocspconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OcspConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-ocspconfiguration.html#cfn-acmpca-certificateauthority-ocspconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`OcspCustomCname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-ocspconfiguration.html#cfn-acmpca-certificateauthority-ocspconfiguration-ocspcustomcname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ocsp_custom_cname: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OcspConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref ocsp_custom_cname) = self.ocsp_custom_cname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OcspCustomCname", ocsp_custom_cname)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OcspConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OcspConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OcspConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OcspConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut ocsp_custom_cname: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OcspCustomCname" => {
                                ocsp_custom_cname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OcspConfiguration {
                        enabled: enabled,
                        ocsp_custom_cname: ocsp_custom_cname,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.OtherName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-othername.html) property type.
    #[derive(Debug, Default)]
    pub struct OtherName {
        /// Property [`TypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-othername.html#cfn-acmpca-certificateauthority-othername-typeid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub type_id: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-othername.html#cfn-acmpca-certificateauthority-othername-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for OtherName {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeId", &self.type_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OtherName {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OtherName, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OtherName;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OtherName")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_id: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TypeId" => {
                                type_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OtherName {
                        type_id: type_id.ok_or(::serde::de::Error::missing_field("TypeId"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.RevocationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-revocationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RevocationConfiguration {
        /// Property [`CrlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-revocationconfiguration.html#cfn-acmpca-certificateauthority-revocationconfiguration-crlconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crl_configuration: Option<::Value<CrlConfiguration>>,
        /// Property [`OcspConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-revocationconfiguration.html#cfn-acmpca-certificateauthority-revocationconfiguration-ocspconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ocsp_configuration: Option<::Value<OcspConfiguration>>,
    }

    impl ::codec::SerializeValue for RevocationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crl_configuration) = self.crl_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrlConfiguration", crl_configuration)?;
            }
            if let Some(ref ocsp_configuration) = self.ocsp_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OcspConfiguration", ocsp_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RevocationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RevocationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RevocationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RevocationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crl_configuration: Option<::Value<CrlConfiguration>> = None;
                    let mut ocsp_configuration: Option<::Value<OcspConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrlConfiguration" => {
                                crl_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OcspConfiguration" => {
                                ocsp_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RevocationConfiguration {
                        crl_configuration: crl_configuration,
                        ocsp_configuration: ocsp_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ACMPCA::CertificateAuthority.Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html) property type.
    #[derive(Debug, Default)]
    pub struct Subject {
        /// Property [`CommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-commonname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub common_name: Option<::Value<String>>,
        /// Property [`Country`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-country).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub country: Option<::Value<String>>,
        /// Property [`CustomAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-customattributes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_attributes: Option<::ValueList<CustomAttribute>>,
        /// Property [`DistinguishedNameQualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-distinguishednamequalifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub distinguished_name_qualifier: Option<::Value<String>>,
        /// Property [`GenerationQualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-generationqualifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub generation_qualifier: Option<::Value<String>>,
        /// Property [`GivenName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-givenname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub given_name: Option<::Value<String>>,
        /// Property [`Initials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-initials).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub initials: Option<::Value<String>>,
        /// Property [`Locality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-locality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub locality: Option<::Value<String>>,
        /// Property [`Organization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-organization).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organization: Option<::Value<String>>,
        /// Property [`OrganizationalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-organizationalunit).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organizational_unit: Option<::Value<String>>,
        /// Property [`Pseudonym`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-pseudonym).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pseudonym: Option<::Value<String>>,
        /// Property [`SerialNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-serialnumber).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub serial_number: Option<::Value<String>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-state).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub state: Option<::Value<String>>,
        /// Property [`Surname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-surname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub surname: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html#cfn-acmpca-certificateauthority-subject-title).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub title: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Subject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref common_name) = self.common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommonName", common_name)?;
            }
            if let Some(ref country) = self.country {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Country", country)?;
            }
            if let Some(ref custom_attributes) = self.custom_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomAttributes", custom_attributes)?;
            }
            if let Some(ref distinguished_name_qualifier) = self.distinguished_name_qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistinguishedNameQualifier", distinguished_name_qualifier)?;
            }
            if let Some(ref generation_qualifier) = self.generation_qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerationQualifier", generation_qualifier)?;
            }
            if let Some(ref given_name) = self.given_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GivenName", given_name)?;
            }
            if let Some(ref initials) = self.initials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Initials", initials)?;
            }
            if let Some(ref locality) = self.locality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Locality", locality)?;
            }
            if let Some(ref organization) = self.organization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Organization", organization)?;
            }
            if let Some(ref organizational_unit) = self.organizational_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnit", organizational_unit)?;
            }
            if let Some(ref pseudonym) = self.pseudonym {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pseudonym", pseudonym)?;
            }
            if let Some(ref serial_number) = self.serial_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerialNumber", serial_number)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            if let Some(ref surname) = self.surname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Surname", surname)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut common_name: Option<::Value<String>> = None;
                    let mut country: Option<::Value<String>> = None;
                    let mut custom_attributes: Option<::ValueList<CustomAttribute>> = None;
                    let mut distinguished_name_qualifier: Option<::Value<String>> = None;
                    let mut generation_qualifier: Option<::Value<String>> = None;
                    let mut given_name: Option<::Value<String>> = None;
                    let mut initials: Option<::Value<String>> = None;
                    let mut locality: Option<::Value<String>> = None;
                    let mut organization: Option<::Value<String>> = None;
                    let mut organizational_unit: Option<::Value<String>> = None;
                    let mut pseudonym: Option<::Value<String>> = None;
                    let mut serial_number: Option<::Value<String>> = None;
                    let mut state: Option<::Value<String>> = None;
                    let mut surname: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CommonName" => {
                                common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Country" => {
                                country = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomAttributes" => {
                                custom_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DistinguishedNameQualifier" => {
                                distinguished_name_qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GenerationQualifier" => {
                                generation_qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GivenName" => {
                                given_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Initials" => {
                                initials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Locality" => {
                                locality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Organization" => {
                                organization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnit" => {
                                organizational_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pseudonym" => {
                                pseudonym = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerialNumber" => {
                                serial_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Surname" => {
                                surname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subject {
                        common_name: common_name,
                        country: country,
                        custom_attributes: custom_attributes,
                        distinguished_name_qualifier: distinguished_name_qualifier,
                        generation_qualifier: generation_qualifier,
                        given_name: given_name,
                        initials: initials,
                        locality: locality,
                        organization: organization,
                        organizational_unit: organizational_unit,
                        pseudonym: pseudonym,
                        serial_number: serial_number,
                        state: state,
                        surname: surname,
                        title: title,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
