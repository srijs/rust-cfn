//! Types for the `AppIntegrations` service.

/// The [`AWS::AppIntegrations::EventIntegration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html) resource type.
#[derive(Debug, Default)]
pub struct EventIntegration {
    properties: EventIntegrationProperties
}

/// Properties for the `EventIntegration` resource.
#[derive(Debug, Default)]
pub struct EventIntegrationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html#cfn-appintegrations-eventintegration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventBridgeBus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html#cfn-appintegrations-eventintegration-eventbridgebus).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_bridge_bus: ::Value<String>,
    /// Property [`EventFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html#cfn-appintegrations-eventintegration-eventfilter).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_filter: ::Value<self::event_integration::EventFilter>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html#cfn-appintegrations-eventintegration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appintegrations-eventintegration.html#cfn-appintegrations-eventintegration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EventIntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeBus", &self.event_bridge_bus)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventFilter", &self.event_filter)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventIntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventIntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventIntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventIntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut event_bridge_bus: Option<::Value<String>> = None;
                let mut event_filter: Option<::Value<self::event_integration::EventFilter>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventBridgeBus" => {
                            event_bridge_bus = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventFilter" => {
                            event_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventIntegrationProperties {
                    description: description,
                    event_bridge_bus: event_bridge_bus.ok_or(::serde::de::Error::missing_field("EventBridgeBus"))?,
                    event_filter: event_filter.ok_or(::serde::de::Error::missing_field("EventFilter"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventIntegration {
    type Properties = EventIntegrationProperties;
    const TYPE: &'static str = "AWS::AppIntegrations::EventIntegration";
    fn properties(&self) -> &EventIntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventIntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventIntegration {}

impl From<EventIntegrationProperties> for EventIntegration {
    fn from(properties: EventIntegrationProperties) -> EventIntegration {
        EventIntegration { properties }
    }
}

pub mod event_integration {
    //! Property types for the `EventIntegration` resource.

    /// The [`AWS::AppIntegrations::EventIntegration.EventFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct EventFilter {
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventfilter.html#cfn-appintegrations-eventintegration-eventfilter-source).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source: ::Value<String>,
    }

    impl ::codec::SerializeValue for EventFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventFilter {
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppIntegrations::EventIntegration.EventIntegrationAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html) property type.
    #[derive(Debug, Default)]
    pub struct EventIntegrationAssociation {
        /// Property [`ClientAssociationMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html#cfn-appintegrations-eventintegration-eventintegrationassociation-clientassociationmetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_association_metadata: Option<::ValueList<Metadata>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html#cfn-appintegrations-eventintegration-eventintegrationassociation-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: Option<::Value<String>>,
        /// Property [`EventBridgeRuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html#cfn-appintegrations-eventintegration-eventintegrationassociation-eventbridgerulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_bridge_rule_name: Option<::Value<String>>,
        /// Property [`EventIntegrationAssociationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html#cfn-appintegrations-eventintegration-eventintegrationassociation-eventintegrationassociationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_integration_association_arn: Option<::Value<String>>,
        /// Property [`EventIntegrationAssociationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-eventintegrationassociation.html#cfn-appintegrations-eventintegration-eventintegrationassociation-eventintegrationassociationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_integration_association_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventIntegrationAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_association_metadata) = self.client_association_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAssociationMetadata", client_association_metadata)?;
            }
            if let Some(ref client_id) = self.client_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", client_id)?;
            }
            if let Some(ref event_bridge_rule_name) = self.event_bridge_rule_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeRuleName", event_bridge_rule_name)?;
            }
            if let Some(ref event_integration_association_arn) = self.event_integration_association_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventIntegrationAssociationArn", event_integration_association_arn)?;
            }
            if let Some(ref event_integration_association_id) = self.event_integration_association_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventIntegrationAssociationId", event_integration_association_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventIntegrationAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventIntegrationAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventIntegrationAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventIntegrationAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_association_metadata: Option<::ValueList<Metadata>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut event_bridge_rule_name: Option<::Value<String>> = None;
                    let mut event_integration_association_arn: Option<::Value<String>> = None;
                    let mut event_integration_association_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientAssociationMetadata" => {
                                client_association_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventBridgeRuleName" => {
                                event_bridge_rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventIntegrationAssociationArn" => {
                                event_integration_association_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventIntegrationAssociationId" => {
                                event_integration_association_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventIntegrationAssociation {
                        client_association_metadata: client_association_metadata,
                        client_id: client_id,
                        event_bridge_rule_name: event_bridge_rule_name,
                        event_integration_association_arn: event_integration_association_arn,
                        event_integration_association_id: event_integration_association_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppIntegrations::EventIntegration.Metadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-metadata.html) property type.
    #[derive(Debug, Default)]
    pub struct Metadata {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-metadata.html#cfn-appintegrations-eventintegration-metadata-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appintegrations-eventintegration-metadata.html#cfn-appintegrations-eventintegration-metadata-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Metadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Metadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Metadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Metadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Metadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Metadata {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
