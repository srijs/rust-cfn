//! Types for the `CertificateManager` service.

/// The [`AWS::CertificateManager::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-account.html) resource type.
#[derive(Debug, Default)]
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Debug, Default)]
pub struct AccountProperties {
    /// Property [`ExpiryEventsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-account.html#cfn-certificatemanager-account-expiryeventsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expiry_events_configuration: ::Value<self::account::ExpiryEventsConfiguration>,
}

impl ::serde::Serialize for AccountProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpiryEventsConfiguration", &self.expiry_events_configuration)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccountProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccountProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccountProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut expiry_events_configuration: Option<::Value<self::account::ExpiryEventsConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExpiryEventsConfiguration" => {
                            expiry_events_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccountProperties {
                    expiry_events_configuration: expiry_events_configuration.ok_or(::serde::de::Error::missing_field("ExpiryEventsConfiguration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Account {
    type Properties = AccountProperties;
    const TYPE: &'static str = "AWS::CertificateManager::Account";
    fn properties(&self) -> &AccountProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccountProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Account {}

impl From<AccountProperties> for Account {
    fn from(properties: AccountProperties) -> Account {
        Account { properties }
    }
}

/// The [`AWS::CertificateManager::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-certificateauthorityarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_authority_arn: Option<::Value<String>>,
    /// Property [`CertificateTransparencyLoggingPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-certificatetransparencyloggingpreference).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_transparency_logging_preference: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`DomainValidationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-domainvalidationoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_validation_options: Option<::ValueList<self::certificate::DomainValidationOption>>,
    /// Property [`KeyAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-keyalgorithm).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_algorithm: Option<::Value<String>>,
    /// Property [`SubjectAlternativeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-subjectalternativenames).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subject_alternative_names: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ValidationMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html#cfn-certificatemanager-certificate-validationmethod).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validation_method: Option<::Value<String>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate_authority_arn) = self.certificate_authority_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", certificate_authority_arn)?;
        }
        if let Some(ref certificate_transparency_logging_preference) = self.certificate_transparency_logging_preference {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateTransparencyLoggingPreference", certificate_transparency_logging_preference)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref domain_validation_options) = self.domain_validation_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainValidationOptions", domain_validation_options)?;
        }
        if let Some(ref key_algorithm) = self.key_algorithm {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAlgorithm", key_algorithm)?;
        }
        if let Some(ref subject_alternative_names) = self.subject_alternative_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", subject_alternative_names)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref validation_method) = self.validation_method {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationMethod", validation_method)?;
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
                let mut certificate_authority_arn: Option<::Value<String>> = None;
                let mut certificate_transparency_logging_preference: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut domain_validation_options: Option<::ValueList<self::certificate::DomainValidationOption>> = None;
                let mut key_algorithm: Option<::Value<String>> = None;
                let mut subject_alternative_names: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut validation_method: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateAuthorityArn" => {
                            certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateTransparencyLoggingPreference" => {
                            certificate_transparency_logging_preference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainValidationOptions" => {
                            domain_validation_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyAlgorithm" => {
                            key_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubjectAlternativeNames" => {
                            subject_alternative_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationMethod" => {
                            validation_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    certificate_authority_arn: certificate_authority_arn,
                    certificate_transparency_logging_preference: certificate_transparency_logging_preference,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    domain_validation_options: domain_validation_options,
                    key_algorithm: key_algorithm,
                    subject_alternative_names: subject_alternative_names,
                    tags: tags,
                    validation_method: validation_method,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::CertificateManager::Certificate";
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

pub mod account {
    //! Property types for the `Account` resource.

    /// The [`AWS::CertificateManager::Account.ExpiryEventsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-account-expiryeventsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ExpiryEventsConfiguration {
        /// Property [`DaysBeforeExpiry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-account-expiryeventsconfiguration.html#cfn-certificatemanager-account-expiryeventsconfiguration-daysbeforeexpiry).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub days_before_expiry: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ExpiryEventsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref days_before_expiry) = self.days_before_expiry {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DaysBeforeExpiry", days_before_expiry)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExpiryEventsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExpiryEventsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExpiryEventsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExpiryEventsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut days_before_expiry: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DaysBeforeExpiry" => {
                                days_before_expiry = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExpiryEventsConfiguration {
                        days_before_expiry: days_before_expiry,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod certificate {
    //! Property types for the `Certificate` resource.

    /// The [`AWS::CertificateManager::Certificate.DomainValidationOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainValidationOption {
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html#cfn-certificatemanager-certificate-domainvalidationoptions-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: ::Value<String>,
        /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html#cfn-certificatemanager-certificate-domainvalidationoption-hostedzoneid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_id: Option<::Value<String>>,
        /// Property [`ValidationDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html#cfn-certificatemanager-certificate-domainvalidationoption-validationdomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation_domain: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DomainValidationOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            if let Some(ref hosted_zone_id) = self.hosted_zone_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", hosted_zone_id)?;
            }
            if let Some(ref validation_domain) = self.validation_domain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationDomain", validation_domain)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainValidationOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainValidationOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainValidationOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainValidationOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut hosted_zone_id: Option<::Value<String>> = None;
                    let mut validation_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidationDomain" => {
                                validation_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainValidationOption {
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        hosted_zone_id: hosted_zone_id,
                        validation_domain: validation_domain,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
