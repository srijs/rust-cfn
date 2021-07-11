//! Types for the `FinSpace` service.

/// The [`AWS::FinSpace::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html) resource type.
#[derive(Debug, Default)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html#cfn-finspace-environment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FederationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html#cfn-finspace-environment-federationmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub federation_mode: Option<::Value<String>>,
    /// Property [`FederationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html#cfn-finspace-environment-federationparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub federation_parameters: Option<::Value<self::environment::FederationParameters>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html#cfn-finspace-environment-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html#cfn-finspace-environment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref federation_mode) = self.federation_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationMode", federation_mode)?;
        }
        if let Some(ref federation_parameters) = self.federation_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationParameters", federation_parameters)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut federation_mode: Option<::Value<String>> = None;
                let mut federation_parameters: Option<::Value<self::environment::FederationParameters>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FederationMode" => {
                            federation_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FederationParameters" => {
                            federation_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    description: description,
                    federation_mode: federation_mode,
                    federation_parameters: federation_parameters,
                    kms_key_id: kms_key_id,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::FinSpace::Environment";
    fn properties(&self) -> &EnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Environment {}

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::FinSpace::Environment.FederationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct FederationParameters {
        /// Property [`ApplicationCallBackURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-applicationcallbackurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_call_back_url: Option<::Value<String>>,
        /// Property [`AttributeMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-attributemap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_map: Option<::Value<::json::Value>>,
        /// Property [`FederationProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-federationprovidername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub federation_provider_name: Option<::Value<String>>,
        /// Property [`FederationURN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-federationurn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub federation_urn: Option<::Value<String>>,
        /// Property [`SamlMetadataDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-samlmetadatadocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub saml_metadata_document: Option<::Value<String>>,
        /// Property [`SamlMetadataURL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html#cfn-finspace-environment-federationparameters-samlmetadataurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub saml_metadata_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FederationParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_call_back_url) = self.application_call_back_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationCallBackURL", application_call_back_url)?;
            }
            if let Some(ref attribute_map) = self.attribute_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeMap", attribute_map)?;
            }
            if let Some(ref federation_provider_name) = self.federation_provider_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationProviderName", federation_provider_name)?;
            }
            if let Some(ref federation_urn) = self.federation_urn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederationURN", federation_urn)?;
            }
            if let Some(ref saml_metadata_document) = self.saml_metadata_document {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamlMetadataDocument", saml_metadata_document)?;
            }
            if let Some(ref saml_metadata_url) = self.saml_metadata_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamlMetadataURL", saml_metadata_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FederationParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FederationParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FederationParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FederationParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_call_back_url: Option<::Value<String>> = None;
                    let mut attribute_map: Option<::Value<::json::Value>> = None;
                    let mut federation_provider_name: Option<::Value<String>> = None;
                    let mut federation_urn: Option<::Value<String>> = None;
                    let mut saml_metadata_document: Option<::Value<String>> = None;
                    let mut saml_metadata_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationCallBackURL" => {
                                application_call_back_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeMap" => {
                                attribute_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FederationProviderName" => {
                                federation_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FederationURN" => {
                                federation_urn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SamlMetadataDocument" => {
                                saml_metadata_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SamlMetadataURL" => {
                                saml_metadata_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FederationParameters {
                        application_call_back_url: application_call_back_url,
                        attribute_map: attribute_map,
                        federation_provider_name: federation_provider_name,
                        federation_urn: federation_urn,
                        saml_metadata_document: saml_metadata_document,
                        saml_metadata_url: saml_metadata_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
