//! Types for the `SecurityHub` service.

/// The [`AWS::SecurityHub::Hub`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html) resource type.
#[derive(Debug, Default)]
pub struct Hub {
    properties: HubProperties
}

/// Properties for the `Hub` resource.
#[derive(Debug, Default)]
pub struct HubProperties {
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html#cfn-securityhub-hub-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for HubProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HubProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HubProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HubProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HubProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HubProperties {
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Hub {
    type Properties = HubProperties;
    const TYPE: &'static str = "AWS::SecurityHub::Hub";
    fn properties(&self) -> &HubProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HubProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Hub {}

impl From<HubProperties> for Hub {
    fn from(properties: HubProperties) -> Hub {
        Hub { properties }
    }
}
