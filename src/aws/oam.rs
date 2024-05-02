//! Types for the `Oam` service.

/// The [`AWS::Oam::Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html) resource type.
#[derive(Debug, Default)]
pub struct Link {
    properties: LinkProperties
}

/// Properties for the `Link` resource.
#[derive(Debug, Default)]
pub struct LinkProperties {
    /// Property [`LabelTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html#cfn-oam-link-labeltemplate).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub label_template: Option<::Value<String>>,
    /// Property [`ResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html#cfn-oam-link-resourcetypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_types: ::ValueList<String>,
    /// Property [`SinkIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html#cfn-oam-link-sinkidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sink_identifier: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-link.html#cfn-oam-link-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for LinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref label_template) = self.label_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelTemplate", label_template)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypes", &self.resource_types)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SinkIdentifier", &self.sink_identifier)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LinkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LinkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LinkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LinkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut label_template: Option<::Value<String>> = None;
                let mut resource_types: Option<::ValueList<String>> = None;
                let mut sink_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LabelTemplate" => {
                            label_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTypes" => {
                            resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SinkIdentifier" => {
                            sink_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LinkProperties {
                    label_template: label_template,
                    resource_types: resource_types.ok_or(::serde::de::Error::missing_field("ResourceTypes"))?,
                    sink_identifier: sink_identifier.ok_or(::serde::de::Error::missing_field("SinkIdentifier"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Link {
    type Properties = LinkProperties;
    const TYPE: &'static str = "AWS::Oam::Link";
    fn properties(&self) -> &LinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LinkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Link {}

impl From<LinkProperties> for Link {
    fn from(properties: LinkProperties) -> Link {
        Link { properties }
    }
}

/// The [`AWS::Oam::Sink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-sink.html) resource type.
#[derive(Debug, Default)]
pub struct Sink {
    properties: SinkProperties
}

/// Properties for the `Sink` resource.
#[derive(Debug, Default)]
pub struct SinkProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-sink.html#cfn-oam-sink-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-sink.html#cfn-oam-sink-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-oam-sink.html#cfn-oam-sink-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for SinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref policy) = self.policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", policy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SinkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SinkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SinkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SinkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut policy: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SinkProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    policy: policy,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Sink {
    type Properties = SinkProperties;
    const TYPE: &'static str = "AWS::Oam::Sink";
    fn properties(&self) -> &SinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SinkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Sink {}

impl From<SinkProperties> for Sink {
    fn from(properties: SinkProperties) -> Sink {
        Sink { properties }
    }
}
