//! Types for the `IoTCoreDeviceAdvisor` service.

/// The [`AWS::IoTCoreDeviceAdvisor::SuiteDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotcoredeviceadvisor-suitedefinition.html) resource type.
#[derive(Debug, Default)]
pub struct SuiteDefinition {
    properties: SuiteDefinitionProperties
}

/// Properties for the `SuiteDefinition` resource.
#[derive(Debug, Default)]
pub struct SuiteDefinitionProperties {
    /// Property [`SuiteDefinitionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotcoredeviceadvisor-suitedefinition.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub suite_definition_configuration: ::Value<self::suite_definition::SuiteDefinitionConfiguration>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotcoredeviceadvisor-suitedefinition.html#cfn-iotcoredeviceadvisor-suitedefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SuiteDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuiteDefinitionConfiguration", &self.suite_definition_configuration)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SuiteDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SuiteDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SuiteDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SuiteDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut suite_definition_configuration: Option<::Value<self::suite_definition::SuiteDefinitionConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SuiteDefinitionConfiguration" => {
                            suite_definition_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SuiteDefinitionProperties {
                    suite_definition_configuration: suite_definition_configuration.ok_or(::serde::de::Error::missing_field("SuiteDefinitionConfiguration"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SuiteDefinition {
    type Properties = SuiteDefinitionProperties;
    const TYPE: &'static str = "AWS::IoTCoreDeviceAdvisor::SuiteDefinition";
    fn properties(&self) -> &SuiteDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SuiteDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SuiteDefinition {}

impl From<SuiteDefinitionProperties> for SuiteDefinition {
    fn from(properties: SuiteDefinitionProperties) -> SuiteDefinition {
        SuiteDefinition { properties }
    }
}

pub mod suite_definition {
    //! Property types for the `SuiteDefinition` resource.

    /// The [`AWS::IoTCoreDeviceAdvisor::SuiteDefinition.DeviceUnderTest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-deviceundertest.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceUnderTest {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-deviceundertest.html#cfn-iotcoredeviceadvisor-suitedefinition-deviceundertest-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-deviceundertest.html#cfn-iotcoredeviceadvisor-suitedefinition-deviceundertest-thingarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thing_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeviceUnderTest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref thing_arn) = self.thing_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", thing_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceUnderTest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceUnderTest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceUnderTest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceUnderTest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut thing_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingArn" => {
                                thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceUnderTest {
                        certificate_arn: certificate_arn,
                        thing_arn: thing_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTCoreDeviceAdvisor::SuiteDefinition.SuiteDefinitionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SuiteDefinitionConfiguration {
        /// Property [`DevicePermissionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration-devicepermissionrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_permission_role_arn: ::Value<String>,
        /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration-devices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub devices: Option<::ValueList<DeviceUnderTest>>,
        /// Property [`IntendedForQualification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration-intendedforqualification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intended_for_qualification: Option<::Value<bool>>,
        /// Property [`RootGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration-rootgroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub root_group: ::Value<String>,
        /// Property [`SuiteDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration.html#cfn-iotcoredeviceadvisor-suitedefinition-suitedefinitionconfiguration-suitedefinitionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suite_definition_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SuiteDefinitionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DevicePermissionRoleArn", &self.device_permission_role_arn)?;
            if let Some(ref devices) = self.devices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", devices)?;
            }
            if let Some(ref intended_for_qualification) = self.intended_for_qualification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntendedForQualification", intended_for_qualification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootGroup", &self.root_group)?;
            if let Some(ref suite_definition_name) = self.suite_definition_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuiteDefinitionName", suite_definition_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SuiteDefinitionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SuiteDefinitionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SuiteDefinitionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SuiteDefinitionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_permission_role_arn: Option<::Value<String>> = None;
                    let mut devices: Option<::ValueList<DeviceUnderTest>> = None;
                    let mut intended_for_qualification: Option<::Value<bool>> = None;
                    let mut root_group: Option<::Value<String>> = None;
                    let mut suite_definition_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DevicePermissionRoleArn" => {
                                device_permission_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Devices" => {
                                devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntendedForQualification" => {
                                intended_for_qualification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RootGroup" => {
                                root_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuiteDefinitionName" => {
                                suite_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SuiteDefinitionConfiguration {
                        device_permission_role_arn: device_permission_role_arn.ok_or(::serde::de::Error::missing_field("DevicePermissionRoleArn"))?,
                        devices: devices,
                        intended_for_qualification: intended_for_qualification,
                        root_group: root_group.ok_or(::serde::de::Error::missing_field("RootGroup"))?,
                        suite_definition_name: suite_definition_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
