//! Types for the `SystemsManagerSAP` service.

/// The [`AWS::SystemsManagerSAP::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-applicationid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_id: ::Value<String>,
    /// Property [`ApplicationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-applicationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_type: ::Value<String>,
    /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-credentials).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub credentials: Option<::ValueList<self::application::Credential>>,
    /// Property [`Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-instances).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instances: Option<::ValueList<String>>,
    /// Property [`SapInstanceNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-sapinstancenumber).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sap_instance_number: Option<::Value<String>>,
    /// Property [`Sid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-sid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sid: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-systemsmanagersap-application.html#cfn-systemsmanagersap-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationType", &self.application_type)?;
        if let Some(ref credentials) = self.credentials {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
        }
        if let Some(ref instances) = self.instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instances", instances)?;
        }
        if let Some(ref sap_instance_number) = self.sap_instance_number {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SapInstanceNumber", sap_instance_number)?;
        }
        if let Some(ref sid) = self.sid {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sid", sid)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_id: Option<::Value<String>> = None;
                let mut application_type: Option<::Value<String>> = None;
                let mut credentials: Option<::ValueList<self::application::Credential>> = None;
                let mut instances: Option<::ValueList<String>> = None;
                let mut sap_instance_number: Option<::Value<String>> = None;
                let mut sid: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationId" => {
                            application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationType" => {
                            application_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Credentials" => {
                            credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Instances" => {
                            instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SapInstanceNumber" => {
                            sap_instance_number = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sid" => {
                            sid = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                    application_type: application_type.ok_or(::serde::de::Error::missing_field("ApplicationType"))?,
                    credentials: credentials,
                    instances: instances,
                    sap_instance_number: sap_instance_number,
                    sid: sid,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::SystemsManagerSAP::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::SystemsManagerSAP::Application.Credential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-credential.html) property type.
    #[derive(Debug, Default)]
    pub struct Credential {
        /// Property [`CredentialType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-credential.html#cfn-systemsmanagersap-application-credential-credentialtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub credential_type: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-credential.html#cfn-systemsmanagersap-application-credential-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`SecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-systemsmanagersap-application-credential.html#cfn-systemsmanagersap-application-credential-secretid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secret_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Credential {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref credential_type) = self.credential_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialType", credential_type)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref secret_id) = self.secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretId", secret_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Credential {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Credential, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Credential;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Credential")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credential_type: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CredentialType" => {
                                credential_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretId" => {
                                secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Credential {
                        credential_type: credential_type,
                        database_name: database_name,
                        secret_id: secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
