//! Types for the `CertificateManager` service.

/// The [`AWS::CertificateManager::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html) resource type.
#[derive(Debug)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug)]
pub struct CertificateProperties {
    /// Property `DomainName`.
    pub domain_name: ::Value<String>,
    /// Property `DomainValidationOptions`.
    pub domain_validation_options: Option<::ValueList<self::certificate::DomainValidationOption>>,
    /// Property `SubjectAlternativeNames`.
    pub subject_alternative_names: Option<::ValueList<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainValidationOptions", &self.domain_validation_options)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectAlternativeNames", &self.subject_alternative_names)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
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
                let mut domain_name = None;
                let mut domain_validation_options = None;
                let mut subject_alternative_names = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DomainValidationOptions" => {
                            domain_validation_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubjectAlternativeNames" => {
                            subject_alternative_names = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    domain_validation_options: domain_validation_options,
                    subject_alternative_names: subject_alternative_names,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Certificate {
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

pub mod certificate {
    //! Property types for the `Certificate` resource.

    /// The [`AWS::CertificateManager::Certificate.DomainValidationOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html) property type.
    #[derive(Debug)]
    pub struct DomainValidationOption {
        /// Property `DomainName`.
        pub domain_name: ::Value<String>,
        /// Property `ValidationDomain`.
        pub validation_domain: ::Value<String>,
    }

    impl ::codec::SerializeValue for DomainValidationOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationDomain", &self.validation_domain)?;
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
                    let mut domain_name = None;
                    let mut validation_domain = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ValidationDomain" => {
                                validation_domain = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainValidationOption {
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        validation_domain: validation_domain.ok_or(::serde::de::Error::missing_field("ValidationDomain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
