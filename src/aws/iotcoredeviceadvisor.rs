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
    pub suite_definition_configuration: ::Value<::json::Value>,
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
                let mut suite_definition_configuration: Option<::Value<::json::Value>> = None;
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
