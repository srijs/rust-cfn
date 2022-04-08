//! Types for the `HealthLake` service.

/// The [`AWS::HealthLake::FHIRDatastore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html) resource type.
#[derive(Debug, Default)]
pub struct FHIRDatastore {
    properties: FHIRDatastoreProperties
}

/// Properties for the `FHIRDatastore` resource.
#[derive(Debug, Default)]
pub struct FHIRDatastoreProperties {
    /// Property [`DatastoreName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html#cfn-healthlake-fhirdatastore-datastorename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub datastore_name: Option<::Value<String>>,
    /// Property [`DatastoreTypeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html#cfn-healthlake-fhirdatastore-datastoretypeversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub datastore_type_version: ::Value<String>,
    /// Property [`PreloadDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html#cfn-healthlake-fhirdatastore-preloaddataconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub preload_data_config: Option<::Value<self::fhir_datastore::PreloadDataConfig>>,
    /// Property [`SseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html#cfn-healthlake-fhirdatastore-sseconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sse_configuration: Option<::Value<self::fhir_datastore::SseConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html#cfn-healthlake-fhirdatastore-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FHIRDatastoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref datastore_name) = self.datastore_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreName", datastore_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatastoreTypeVersion", &self.datastore_type_version)?;
        if let Some(ref preload_data_config) = self.preload_data_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreloadDataConfig", preload_data_config)?;
        }
        if let Some(ref sse_configuration) = self.sse_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseConfiguration", sse_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FHIRDatastoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FHIRDatastoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FHIRDatastoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FHIRDatastoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut datastore_name: Option<::Value<String>> = None;
                let mut datastore_type_version: Option<::Value<String>> = None;
                let mut preload_data_config: Option<::Value<self::fhir_datastore::PreloadDataConfig>> = None;
                let mut sse_configuration: Option<::Value<self::fhir_datastore::SseConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatastoreName" => {
                            datastore_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatastoreTypeVersion" => {
                            datastore_type_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreloadDataConfig" => {
                            preload_data_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SseConfiguration" => {
                            sse_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FHIRDatastoreProperties {
                    datastore_name: datastore_name,
                    datastore_type_version: datastore_type_version.ok_or(::serde::de::Error::missing_field("DatastoreTypeVersion"))?,
                    preload_data_config: preload_data_config,
                    sse_configuration: sse_configuration,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FHIRDatastore {
    type Properties = FHIRDatastoreProperties;
    const TYPE: &'static str = "AWS::HealthLake::FHIRDatastore";
    fn properties(&self) -> &FHIRDatastoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FHIRDatastoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FHIRDatastore {}

impl From<FHIRDatastoreProperties> for FHIRDatastore {
    fn from(properties: FHIRDatastoreProperties) -> FHIRDatastore {
        FHIRDatastore { properties }
    }
}

pub mod fhir_datastore {
    //! Property types for the `FHIRDatastore` resource.

    /// The [`AWS::HealthLake::FHIRDatastore.KmsEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-kmsencryptionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KmsEncryptionConfig {
        /// Property [`CmkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-kmsencryptionconfig.html#cfn-healthlake-fhirdatastore-kmsencryptionconfig-cmktype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cmk_type: ::Value<String>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-kmsencryptionconfig.html#cfn-healthlake-fhirdatastore-kmsencryptionconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KmsEncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmkType", &self.cmk_type)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KmsEncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KmsEncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KmsEncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KmsEncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cmk_type: Option<::Value<String>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CmkType" => {
                                cmk_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KmsEncryptionConfig {
                        cmk_type: cmk_type.ok_or(::serde::de::Error::missing_field("CmkType"))?,
                        kms_key_id: kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::HealthLake::FHIRDatastore.PreloadDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-preloaddataconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PreloadDataConfig {
        /// Property [`PreloadDataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-preloaddataconfig.html#cfn-healthlake-fhirdatastore-preloaddataconfig-preloaddatatype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub preload_data_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for PreloadDataConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreloadDataType", &self.preload_data_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PreloadDataConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PreloadDataConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PreloadDataConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PreloadDataConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut preload_data_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PreloadDataType" => {
                                preload_data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PreloadDataConfig {
                        preload_data_type: preload_data_type.ok_or(::serde::de::Error::missing_field("PreloadDataType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::HealthLake::FHIRDatastore.SseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-sseconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SseConfiguration {
        /// Property [`KmsEncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-sseconfiguration.html#cfn-healthlake-fhirdatastore-sseconfiguration-kmsencryptionconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_encryption_config: ::Value<KmsEncryptionConfig>,
    }

    impl ::codec::SerializeValue for SseConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsEncryptionConfig", &self.kms_encryption_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SseConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SseConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SseConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SseConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_encryption_config: Option<::Value<KmsEncryptionConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsEncryptionConfig" => {
                                kms_encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SseConfiguration {
                        kms_encryption_config: kms_encryption_config.ok_or(::serde::de::Error::missing_field("KmsEncryptionConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
