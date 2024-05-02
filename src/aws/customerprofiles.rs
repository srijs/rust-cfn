//! Types for the `CustomerProfiles` service.

/// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html) resource type.
#[derive(Debug, Default)]
pub struct CalculatedAttributeDefinition {
    properties: CalculatedAttributeDefinitionProperties
}

/// Properties for the `CalculatedAttributeDefinition` resource.
#[derive(Debug, Default)]
pub struct CalculatedAttributeDefinitionProperties {
    /// Property [`AttributeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-attributedetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attribute_details: ::Value<self::calculated_attribute_definition::AttributeDetails>,
    /// Property [`CalculatedAttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-calculatedattributename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub calculated_attribute_name: ::Value<String>,
    /// Property [`Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-conditions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub conditions: Option<::Value<self::calculated_attribute_definition::Conditions>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-statistic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub statistic: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-calculatedattributedefinition.html#cfn-customerprofiles-calculatedattributedefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CalculatedAttributeDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDetails", &self.attribute_details)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CalculatedAttributeName", &self.calculated_attribute_name)?;
        if let Some(ref conditions) = self.conditions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditions", conditions)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CalculatedAttributeDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CalculatedAttributeDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CalculatedAttributeDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CalculatedAttributeDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_details: Option<::Value<self::calculated_attribute_definition::AttributeDetails>> = None;
                let mut calculated_attribute_name: Option<::Value<String>> = None;
                let mut conditions: Option<::Value<self::calculated_attribute_definition::Conditions>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut statistic: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributeDetails" => {
                            attribute_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CalculatedAttributeName" => {
                            calculated_attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Conditions" => {
                            conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Statistic" => {
                            statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CalculatedAttributeDefinitionProperties {
                    attribute_details: attribute_details.ok_or(::serde::de::Error::missing_field("AttributeDetails"))?,
                    calculated_attribute_name: calculated_attribute_name.ok_or(::serde::de::Error::missing_field("CalculatedAttributeName"))?,
                    conditions: conditions,
                    description: description,
                    display_name: display_name,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    statistic: statistic.ok_or(::serde::de::Error::missing_field("Statistic"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CalculatedAttributeDefinition {
    type Properties = CalculatedAttributeDefinitionProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::CalculatedAttributeDefinition";
    fn properties(&self) -> &CalculatedAttributeDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CalculatedAttributeDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CalculatedAttributeDefinition {}

impl From<CalculatedAttributeDefinitionProperties> for CalculatedAttributeDefinition {
    fn from(properties: CalculatedAttributeDefinitionProperties) -> CalculatedAttributeDefinition {
        CalculatedAttributeDefinition { properties }
    }
}

/// The [`AWS::CustomerProfiles::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`DeadLetterQueueUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-deadletterqueueurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dead_letter_queue_url: Option<::Value<String>>,
    /// Property [`DefaultEncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-defaultencryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_encryption_key: Option<::Value<String>>,
    /// Property [`DefaultExpirationDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-defaultexpirationdays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_expiration_days: Option<::Value<u32>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`Matching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-matching).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub matching: Option<::Value<self::domain::Matching>>,
    /// Property [`RuleBasedMatching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-rulebasedmatching).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_based_matching: Option<::Value<self::domain::RuleBasedMatching>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref dead_letter_queue_url) = self.dead_letter_queue_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterQueueUrl", dead_letter_queue_url)?;
        }
        if let Some(ref default_encryption_key) = self.default_encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultEncryptionKey", default_encryption_key)?;
        }
        if let Some(ref default_expiration_days) = self.default_expiration_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultExpirationDays", default_expiration_days)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref matching) = self.matching {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Matching", matching)?;
        }
        if let Some(ref rule_based_matching) = self.rule_based_matching {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleBasedMatching", rule_based_matching)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dead_letter_queue_url: Option<::Value<String>> = None;
                let mut default_encryption_key: Option<::Value<String>> = None;
                let mut default_expiration_days: Option<::Value<u32>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut matching: Option<::Value<self::domain::Matching>> = None;
                let mut rule_based_matching: Option<::Value<self::domain::RuleBasedMatching>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeadLetterQueueUrl" => {
                            dead_letter_queue_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultEncryptionKey" => {
                            default_encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultExpirationDays" => {
                            default_expiration_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Matching" => {
                            matching = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleBasedMatching" => {
                            rule_based_matching = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    dead_letter_queue_url: dead_letter_queue_url,
                    default_encryption_key: default_encryption_key,
                    default_expiration_days: default_expiration_days,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    matching: matching,
                    rule_based_matching: rule_based_matching,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

/// The [`AWS::CustomerProfiles::EventStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html) resource type.
#[derive(Debug, Default)]
pub struct EventStream {
    properties: EventStreamProperties
}

/// Properties for the `EventStream` resource.
#[derive(Debug, Default)]
pub struct EventStreamProperties {
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html#cfn-customerprofiles-eventstream-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`EventStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html#cfn-customerprofiles-eventstream-eventstreamname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_stream_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html#cfn-customerprofiles-eventstream-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-eventstream.html#cfn-customerprofiles-eventstream-uri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub uri: ::Value<String>,
}

impl ::serde::Serialize for EventStreamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventStreamName", &self.event_stream_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", &self.uri)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventStreamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventStreamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventStreamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventStreamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_name: Option<::Value<String>> = None;
                let mut event_stream_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut uri: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventStreamName" => {
                            event_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Uri" => {
                            uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventStreamProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    event_stream_name: event_stream_name.ok_or(::serde::de::Error::missing_field("EventStreamName"))?,
                    tags: tags,
                    uri: uri.ok_or(::serde::de::Error::missing_field("Uri"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventStream {
    type Properties = EventStreamProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::EventStream";
    fn properties(&self) -> &EventStreamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventStreamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventStream {}

impl From<EventStreamProperties> for EventStream {
    fn from(properties: EventStreamProperties) -> EventStream {
        EventStream { properties }
    }
}

/// The [`AWS::CustomerProfiles::Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html) resource type.
#[derive(Debug, Default)]
pub struct Integration {
    properties: IntegrationProperties
}

/// Properties for the `Integration` resource.
#[derive(Debug, Default)]
pub struct IntegrationProperties {
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`FlowDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-flowdefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_definition: Option<::Value<self::integration::FlowDefinition>>,
    /// Property [`ObjectTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-objecttypename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub object_type_name: Option<::Value<String>>,
    /// Property [`ObjectTypeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-objecttypenames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub object_type_names: Option<::ValueList<self::integration::ObjectTypeMapping>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-uri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub uri: Option<::Value<String>>,
}

impl ::serde::Serialize for IntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref flow_definition) = self.flow_definition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowDefinition", flow_definition)?;
        }
        if let Some(ref object_type_name) = self.object_type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeName", object_type_name)?;
        }
        if let Some(ref object_type_names) = self.object_type_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeNames", object_type_names)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref uri) = self.uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", uri)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_name: Option<::Value<String>> = None;
                let mut flow_definition: Option<::Value<self::integration::FlowDefinition>> = None;
                let mut object_type_name: Option<::Value<String>> = None;
                let mut object_type_names: Option<::ValueList<self::integration::ObjectTypeMapping>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut uri: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowDefinition" => {
                            flow_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectTypeName" => {
                            object_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectTypeNames" => {
                            object_type_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Uri" => {
                            uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    flow_definition: flow_definition,
                    object_type_name: object_type_name,
                    object_type_names: object_type_names,
                    tags: tags,
                    uri: uri,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Integration {
    type Properties = IntegrationProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::Integration";
    fn properties(&self) -> &IntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Integration {}

impl From<IntegrationProperties> for Integration {
    fn from(properties: IntegrationProperties) -> Integration {
        Integration { properties }
    }
}

/// The [`AWS::CustomerProfiles::ObjectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html) resource type.
#[derive(Debug, Default)]
pub struct ObjectType {
    properties: ObjectTypeProperties
}

/// Properties for the `ObjectType` resource.
#[derive(Debug, Default)]
pub struct ObjectTypeProperties {
    /// Property [`AllowProfileCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-allowprofilecreation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_profile_creation: Option<::Value<bool>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-encryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_key: Option<::Value<String>>,
    /// Property [`ExpirationDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-expirationdays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expiration_days: Option<::Value<u32>>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: Option<::ValueList<self::object_type::FieldMap>>,
    /// Property [`Keys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-keys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub keys: Option<::ValueList<self::object_type::KeyMap>>,
    /// Property [`ObjectTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-objecttypename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub object_type_name: Option<::Value<String>>,
    /// Property [`SourceLastUpdatedTimestampFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-sourcelastupdatedtimestampformat).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_last_updated_timestamp_format: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-templateid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ObjectTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_profile_creation) = self.allow_profile_creation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowProfileCreation", allow_profile_creation)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        if let Some(ref expiration_days) = self.expiration_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpirationDays", expiration_days)?;
        }
        if let Some(ref fields) = self.fields {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", fields)?;
        }
        if let Some(ref keys) = self.keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Keys", keys)?;
        }
        if let Some(ref object_type_name) = self.object_type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeName", object_type_name)?;
        }
        if let Some(ref source_last_updated_timestamp_format) = self.source_last_updated_timestamp_format {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLastUpdatedTimestampFormat", source_last_updated_timestamp_format)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_id) = self.template_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateId", template_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ObjectTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ObjectTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_profile_creation: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut expiration_days: Option<::Value<u32>> = None;
                let mut fields: Option<::ValueList<self::object_type::FieldMap>> = None;
                let mut keys: Option<::ValueList<self::object_type::KeyMap>> = None;
                let mut object_type_name: Option<::Value<String>> = None;
                let mut source_last_updated_timestamp_format: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowProfileCreation" => {
                            allow_profile_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKey" => {
                            encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExpirationDays" => {
                            expiration_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Keys" => {
                            keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectTypeName" => {
                            object_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceLastUpdatedTimestampFormat" => {
                            source_last_updated_timestamp_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateId" => {
                            template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ObjectTypeProperties {
                    allow_profile_creation: allow_profile_creation,
                    description: description,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    encryption_key: encryption_key,
                    expiration_days: expiration_days,
                    fields: fields,
                    keys: keys,
                    object_type_name: object_type_name,
                    source_last_updated_timestamp_format: source_last_updated_timestamp_format,
                    tags: tags,
                    template_id: template_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ObjectType {
    type Properties = ObjectTypeProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::ObjectType";
    fn properties(&self) -> &ObjectTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ObjectTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ObjectType {}

impl From<ObjectTypeProperties> for ObjectType {
    fn from(properties: ObjectTypeProperties) -> ObjectType {
        ObjectType { properties }
    }
}

pub mod calculated_attribute_definition {
    //! Property types for the `CalculatedAttributeDefinition` resource.

    /// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition.AttributeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeDetails {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributedetails.html#cfn-customerprofiles-calculatedattributedefinition-attributedetails-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: ::ValueList<AttributeItem>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributedetails.html#cfn-customerprofiles-calculatedattributedefinition-attributedetails-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for AttributeDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<AttributeItem>> = None;
                    let mut expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeDetails {
                        attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition.AttributeItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributeitem.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeItem {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-attributeitem.html#cfn-customerprofiles-calculatedattributedefinition-attributeitem-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AttributeItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeItem {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition.Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-conditions.html) property type.
    #[derive(Debug, Default)]
    pub struct Conditions {
        /// Property [`ObjectCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-conditions.html#cfn-customerprofiles-calculatedattributedefinition-conditions-objectcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_count: Option<::Value<u32>>,
        /// Property [`Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-conditions.html#cfn-customerprofiles-calculatedattributedefinition-conditions-range).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range: Option<::Value<Range>>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-conditions.html#cfn-customerprofiles-calculatedattributedefinition-conditions-threshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold: Option<::Value<Threshold>>,
    }

    impl ::codec::SerializeValue for Conditions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref object_count) = self.object_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectCount", object_count)?;
            }
            if let Some(ref range) = self.range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Range", range)?;
            }
            if let Some(ref threshold) = self.threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Conditions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Conditions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Conditions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Conditions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object_count: Option<::Value<u32>> = None;
                    let mut range: Option<::Value<Range>> = None;
                    let mut threshold: Option<::Value<Threshold>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObjectCount" => {
                                object_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Range" => {
                                range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Conditions {
                        object_count: object_count,
                        range: range,
                        threshold: threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition.Range`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-range.html) property type.
    #[derive(Debug, Default)]
    pub struct Range {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-range.html#cfn-customerprofiles-calculatedattributedefinition-range-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-range.html#cfn-customerprofiles-calculatedattributedefinition-range-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Range {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Range {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Range, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Range;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Range")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Range {
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::CalculatedAttributeDefinition.Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-threshold.html) property type.
    #[derive(Debug, Default)]
    pub struct Threshold {
        /// Property [`Operator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-threshold.html#cfn-customerprofiles-calculatedattributedefinition-threshold-operator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-calculatedattributedefinition-threshold.html#cfn-customerprofiles-calculatedattributedefinition-threshold-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Threshold {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operator", &self.operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Threshold {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Threshold, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Threshold;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Threshold")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut operator: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Operator" => {
                                operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Threshold {
                        operator: operator.ok_or(::serde::de::Error::missing_field("Operator"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::CustomerProfiles::Domain.AttributeTypesSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeTypesSelector {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html#cfn-customerprofiles-domain-attributetypesselector-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::ValueList<String>>,
        /// Property [`AttributeMatchingModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html#cfn-customerprofiles-domain-attributetypesselector-attributematchingmodel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_matching_model: ::Value<String>,
        /// Property [`EmailAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html#cfn-customerprofiles-domain-attributetypesselector-emailaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_address: Option<::ValueList<String>>,
        /// Property [`PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-attributetypesselector.html#cfn-customerprofiles-domain-attributetypesselector-phonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AttributeTypesSelector {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeMatchingModel", &self.attribute_matching_model)?;
            if let Some(ref email_address) = self.email_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailAddress", email_address)?;
            }
            if let Some(ref phone_number) = self.phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumber", phone_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeTypesSelector {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeTypesSelector, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeTypesSelector;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeTypesSelector")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::ValueList<String>> = None;
                    let mut attribute_matching_model: Option<::Value<String>> = None;
                    let mut email_address: Option<::ValueList<String>> = None;
                    let mut phone_number: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeMatchingModel" => {
                                attribute_matching_model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailAddress" => {
                                email_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneNumber" => {
                                phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeTypesSelector {
                        address: address,
                        attribute_matching_model: attribute_matching_model.ok_or(::serde::de::Error::missing_field("AttributeMatchingModel"))?,
                        email_address: email_address,
                        phone_number: phone_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.AutoMerging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoMerging {
        /// Property [`ConflictResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html#cfn-customerprofiles-domain-automerging-conflictresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_resolution: Option<::Value<ConflictResolution>>,
        /// Property [`Consolidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html#cfn-customerprofiles-domain-automerging-consolidation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consolidation: Option<::Value<Consolidation>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html#cfn-customerprofiles-domain-automerging-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`MinAllowedConfidenceScoreForMerging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-automerging.html#cfn-customerprofiles-domain-automerging-minallowedconfidencescoreformerging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_allowed_confidence_score_for_merging: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for AutoMerging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref conflict_resolution) = self.conflict_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictResolution", conflict_resolution)?;
            }
            if let Some(ref consolidation) = self.consolidation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Consolidation", consolidation)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref min_allowed_confidence_score_for_merging) = self.min_allowed_confidence_score_for_merging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinAllowedConfidenceScoreForMerging", min_allowed_confidence_score_for_merging)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoMerging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoMerging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoMerging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoMerging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conflict_resolution: Option<::Value<ConflictResolution>> = None;
                    let mut consolidation: Option<::Value<Consolidation>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut min_allowed_confidence_score_for_merging: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConflictResolution" => {
                                conflict_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Consolidation" => {
                                consolidation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinAllowedConfidenceScoreForMerging" => {
                                min_allowed_confidence_score_for_merging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoMerging {
                        conflict_resolution: conflict_resolution,
                        consolidation: consolidation,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        min_allowed_confidence_score_for_merging: min_allowed_confidence_score_for_merging,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.ConflictResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-conflictresolution.html) property type.
    #[derive(Debug, Default)]
    pub struct ConflictResolution {
        /// Property [`ConflictResolvingModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-conflictresolution.html#cfn-customerprofiles-domain-conflictresolution-conflictresolvingmodel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_resolving_model: ::Value<String>,
        /// Property [`SourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-conflictresolution.html#cfn-customerprofiles-domain-conflictresolution-sourcename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConflictResolution {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictResolvingModel", &self.conflict_resolving_model)?;
            if let Some(ref source_name) = self.source_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceName", source_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConflictResolution {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConflictResolution, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConflictResolution;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConflictResolution")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conflict_resolving_model: Option<::Value<String>> = None;
                    let mut source_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConflictResolvingModel" => {
                                conflict_resolving_model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceName" => {
                                source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConflictResolution {
                        conflict_resolving_model: conflict_resolving_model.ok_or(::serde::de::Error::missing_field("ConflictResolvingModel"))?,
                        source_name: source_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.Consolidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-consolidation.html) property type.
    #[derive(Debug, Default)]
    pub struct Consolidation {
        /// Property [`MatchingAttributesList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-consolidation.html#cfn-customerprofiles-domain-consolidation-matchingattributeslist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub matching_attributes_list: ::Value<::json::Value>,
    }

    impl ::codec::SerializeValue for Consolidation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchingAttributesList", &self.matching_attributes_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Consolidation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Consolidation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Consolidation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Consolidation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut matching_attributes_list: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchingAttributesList" => {
                                matching_attributes_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Consolidation {
                        matching_attributes_list: matching_attributes_list.ok_or(::serde::de::Error::missing_field("MatchingAttributesList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.DomainStats`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainStats {
        /// Property [`MeteringProfileCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html#cfn-customerprofiles-domain-domainstats-meteringprofilecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metering_profile_count: Option<::Value<f64>>,
        /// Property [`ObjectCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html#cfn-customerprofiles-domain-domainstats-objectcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_count: Option<::Value<f64>>,
        /// Property [`ProfileCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html#cfn-customerprofiles-domain-domainstats-profilecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile_count: Option<::Value<f64>>,
        /// Property [`TotalSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-domainstats.html#cfn-customerprofiles-domain-domainstats-totalsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_size: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for DomainStats {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metering_profile_count) = self.metering_profile_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeteringProfileCount", metering_profile_count)?;
            }
            if let Some(ref object_count) = self.object_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectCount", object_count)?;
            }
            if let Some(ref profile_count) = self.profile_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileCount", profile_count)?;
            }
            if let Some(ref total_size) = self.total_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalSize", total_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainStats {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainStats, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainStats;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainStats")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metering_profile_count: Option<::Value<f64>> = None;
                    let mut object_count: Option<::Value<f64>> = None;
                    let mut profile_count: Option<::Value<f64>> = None;
                    let mut total_size: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MeteringProfileCount" => {
                                metering_profile_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectCount" => {
                                object_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProfileCount" => {
                                profile_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TotalSize" => {
                                total_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainStats {
                        metering_profile_count: metering_profile_count,
                        object_count: object_count,
                        profile_count: profile_count,
                        total_size: total_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.ExportingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-exportingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ExportingConfig {
        /// Property [`S3Exporting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-exportingconfig.html#cfn-customerprofiles-domain-exportingconfig-s3exporting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_exporting: Option<::Value<S3ExportingConfig>>,
    }

    impl ::codec::SerializeValue for ExportingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_exporting) = self.s3_exporting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Exporting", s3_exporting)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExportingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExportingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExportingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExportingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_exporting: Option<::Value<S3ExportingConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Exporting" => {
                                s3_exporting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExportingConfig {
                        s3_exporting: s3_exporting,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.JobSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-jobschedule.html) property type.
    #[derive(Debug, Default)]
    pub struct JobSchedule {
        /// Property [`DayOfTheWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-jobschedule.html#cfn-customerprofiles-domain-jobschedule-dayoftheweek).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day_of_the_week: ::Value<String>,
        /// Property [`Time`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-jobschedule.html#cfn-customerprofiles-domain-jobschedule-time).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time: ::Value<String>,
    }

    impl ::codec::SerializeValue for JobSchedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfTheWeek", &self.day_of_the_week)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Time", &self.time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobSchedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobSchedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobSchedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobSchedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day_of_the_week: Option<::Value<String>> = None;
                    let mut time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DayOfTheWeek" => {
                                day_of_the_week = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Time" => {
                                time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobSchedule {
                        day_of_the_week: day_of_the_week.ok_or(::serde::de::Error::missing_field("DayOfTheWeek"))?,
                        time: time.ok_or(::serde::de::Error::missing_field("Time"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.Matching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html) property type.
    #[derive(Debug, Default)]
    pub struct Matching {
        /// Property [`AutoMerging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html#cfn-customerprofiles-domain-matching-automerging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_merging: Option<::Value<AutoMerging>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html#cfn-customerprofiles-domain-matching-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`ExportingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html#cfn-customerprofiles-domain-matching-exportingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exporting_config: Option<::Value<ExportingConfig>>,
        /// Property [`JobSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matching.html#cfn-customerprofiles-domain-matching-jobschedule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_schedule: Option<::Value<JobSchedule>>,
    }

    impl ::codec::SerializeValue for Matching {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_merging) = self.auto_merging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMerging", auto_merging)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref exporting_config) = self.exporting_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportingConfig", exporting_config)?;
            }
            if let Some(ref job_schedule) = self.job_schedule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobSchedule", job_schedule)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Matching {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Matching, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Matching;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Matching")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_merging: Option<::Value<AutoMerging>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut exporting_config: Option<::Value<ExportingConfig>> = None;
                    let mut job_schedule: Option<::Value<JobSchedule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoMerging" => {
                                auto_merging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportingConfig" => {
                                exporting_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobSchedule" => {
                                job_schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Matching {
                        auto_merging: auto_merging,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        exporting_config: exporting_config,
                        job_schedule: job_schedule,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.MatchingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matchingrule.html) property type.
    #[derive(Debug, Default)]
    pub struct MatchingRule {
        /// Property [`Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-matchingrule.html#cfn-customerprofiles-domain-matchingrule-rule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for MatchingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rule", &self.rule)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MatchingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MatchingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MatchingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rule" => {
                                rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MatchingRule {
                        rule: rule.ok_or(::serde::de::Error::missing_field("Rule"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.RuleBasedMatching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleBasedMatching {
        /// Property [`AttributeTypesSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-attributetypesselector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_types_selector: Option<::Value<AttributeTypesSelector>>,
        /// Property [`ConflictResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-conflictresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_resolution: Option<::Value<ConflictResolution>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`ExportingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-exportingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exporting_config: Option<::Value<ExportingConfig>>,
        /// Property [`MatchingRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-matchingrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub matching_rules: Option<::ValueList<MatchingRule>>,
        /// Property [`MaxAllowedRuleLevelForMatching`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-maxallowedrulelevelformatching).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_allowed_rule_level_for_matching: Option<::Value<u32>>,
        /// Property [`MaxAllowedRuleLevelForMerging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-maxallowedrulelevelformerging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_allowed_rule_level_for_merging: Option<::Value<u32>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-rulebasedmatching.html#cfn-customerprofiles-domain-rulebasedmatching-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RuleBasedMatching {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_types_selector) = self.attribute_types_selector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeTypesSelector", attribute_types_selector)?;
            }
            if let Some(ref conflict_resolution) = self.conflict_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictResolution", conflict_resolution)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref exporting_config) = self.exporting_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportingConfig", exporting_config)?;
            }
            if let Some(ref matching_rules) = self.matching_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchingRules", matching_rules)?;
            }
            if let Some(ref max_allowed_rule_level_for_matching) = self.max_allowed_rule_level_for_matching {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAllowedRuleLevelForMatching", max_allowed_rule_level_for_matching)?;
            }
            if let Some(ref max_allowed_rule_level_for_merging) = self.max_allowed_rule_level_for_merging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAllowedRuleLevelForMerging", max_allowed_rule_level_for_merging)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleBasedMatching {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleBasedMatching, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleBasedMatching;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleBasedMatching")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_types_selector: Option<::Value<AttributeTypesSelector>> = None;
                    let mut conflict_resolution: Option<::Value<ConflictResolution>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut exporting_config: Option<::Value<ExportingConfig>> = None;
                    let mut matching_rules: Option<::ValueList<MatchingRule>> = None;
                    let mut max_allowed_rule_level_for_matching: Option<::Value<u32>> = None;
                    let mut max_allowed_rule_level_for_merging: Option<::Value<u32>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeTypesSelector" => {
                                attribute_types_selector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConflictResolution" => {
                                conflict_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportingConfig" => {
                                exporting_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchingRules" => {
                                matching_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxAllowedRuleLevelForMatching" => {
                                max_allowed_rule_level_for_matching = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxAllowedRuleLevelForMerging" => {
                                max_allowed_rule_level_for_merging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleBasedMatching {
                        attribute_types_selector: attribute_types_selector,
                        conflict_resolution: conflict_resolution,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        exporting_config: exporting_config,
                        matching_rules: matching_rules,
                        max_allowed_rule_level_for_matching: max_allowed_rule_level_for_matching,
                        max_allowed_rule_level_for_merging: max_allowed_rule_level_for_merging,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Domain.S3ExportingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-s3exportingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3ExportingConfig {
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-s3exportingconfig.html#cfn-customerprofiles-domain-s3exportingconfig-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: ::Value<String>,
        /// Property [`S3KeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-domain-s3exportingconfig.html#cfn-customerprofiles-domain-s3exportingconfig-s3keyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3ExportingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
            if let Some(ref s3_key_name) = self.s3_key_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyName", s3_key_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3ExportingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3ExportingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3ExportingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3ExportingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_key_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyName" => {
                                s3_key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ExportingConfig {
                        s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                        s3_key_name: s3_key_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_stream {
    //! Property types for the `EventStream` resource.

    /// The [`AWS::CustomerProfiles::EventStream.DestinationDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventstream-destinationdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationDetails {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventstream-destinationdetails.html#cfn-customerprofiles-eventstream-destinationdetails-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
        /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-eventstream-destinationdetails.html#cfn-customerprofiles-eventstream-destinationdetails-uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for DestinationDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", &self.uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status: Option<::Value<String>> = None;
                    let mut uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uri" => {
                                uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationDetails {
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                        uri: uri.ok_or(::serde::de::Error::missing_field("Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod integration {
    //! Property types for the `Integration` resource.

    /// The [`AWS::CustomerProfiles::Integration.ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorOperator {
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<String>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<String>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<String>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<String>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectorOperator {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorOperator {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorOperator, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorOperator;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorOperator")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut marketo: Option<::Value<String>> = None;
                    let mut s3: Option<::Value<String>> = None;
                    let mut salesforce: Option<::Value<String>> = None;
                    let mut service_now: Option<::Value<String>> = None;
                    let mut zendesk: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorOperator {
                        marketo: marketo,
                        s3: s3,
                        salesforce: salesforce,
                        service_now: service_now,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.FlowDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct FlowDefinition {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FlowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-flowname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_name: ::Value<String>,
        /// Property [`KmsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-kmsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_arn: ::Value<String>,
        /// Property [`SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-sourceflowconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_flow_config: ::Value<SourceFlowConfig>,
        /// Property [`Tasks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-tasks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tasks: ::ValueList<Task>,
        /// Property [`TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-triggerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_config: ::Value<TriggerConfig>,
    }

    impl ::codec::SerializeValue for FlowDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowName", &self.flow_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsArn", &self.kms_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFlowConfig", &self.source_flow_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tasks", &self.tasks)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerConfig", &self.trigger_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FlowDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FlowDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FlowDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut flow_name: Option<::Value<String>> = None;
                    let mut kms_arn: Option<::Value<String>> = None;
                    let mut source_flow_config: Option<::Value<SourceFlowConfig>> = None;
                    let mut tasks: Option<::ValueList<Task>> = None;
                    let mut trigger_config: Option<::Value<TriggerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlowName" => {
                                flow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsArn" => {
                                kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFlowConfig" => {
                                source_flow_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tasks" => {
                                tasks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerConfig" => {
                                trigger_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FlowDefinition {
                        description: description,
                        flow_name: flow_name.ok_or(::serde::de::Error::missing_field("FlowName"))?,
                        kms_arn: kms_arn.ok_or(::serde::de::Error::missing_field("KmsArn"))?,
                        source_flow_config: source_flow_config.ok_or(::serde::de::Error::missing_field("SourceFlowConfig"))?,
                        tasks: tasks.ok_or(::serde::de::Error::missing_field("Tasks"))?,
                        trigger_config: trigger_config.ok_or(::serde::de::Error::missing_field("TriggerConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-incrementalpullconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct IncrementalPullConfig {
        /// Property [`DatetimeTypeFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-incrementalpullconfig.html#cfn-customerprofiles-integration-incrementalpullconfig-datetimetypefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datetime_type_field_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IncrementalPullConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref datetime_type_field_name) = self.datetime_type_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatetimeTypeFieldName", datetime_type_field_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IncrementalPullConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IncrementalPullConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IncrementalPullConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IncrementalPullConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut datetime_type_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatetimeTypeFieldName" => {
                                datetime_type_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IncrementalPullConfig {
                        datetime_type_field_name: datetime_type_field_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.MarketoSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-marketosourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-marketosourceproperties.html#cfn-customerprofiles-integration-marketosourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for MarketoSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ObjectTypeMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-objecttypemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectTypeMapping {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-objecttypemapping.html#cfn-customerprofiles-integration-objecttypemapping-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-objecttypemapping.html#cfn-customerprofiles-integration-objecttypemapping-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ObjectTypeMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectTypeMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectTypeMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectTypeMapping")
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

                    Ok(ObjectTypeMapping {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.S3SourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct S3SourceProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html#cfn-customerprofiles-integration-s3sourceproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html#cfn-customerprofiles-integration-s3sourceproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3SourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3SourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3SourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3SourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3SourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3SourceProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SalesforceSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceSourceProperties {
        /// Property [`EnableDynamicFieldUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-enabledynamicfieldupdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_dynamic_field_update: Option<::Value<bool>>,
        /// Property [`IncludeDeletedRecords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-includedeletedrecords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_deleted_records: Option<::Value<bool>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SalesforceSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_dynamic_field_update) = self.enable_dynamic_field_update {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDynamicFieldUpdate", enable_dynamic_field_update)?;
            }
            if let Some(ref include_deleted_records) = self.include_deleted_records {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeDeletedRecords", include_deleted_records)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_dynamic_field_update: Option<::Value<bool>> = None;
                    let mut include_deleted_records: Option<::Value<bool>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableDynamicFieldUpdate" => {
                                enable_dynamic_field_update = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeDeletedRecords" => {
                                include_deleted_records = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceSourceProperties {
                        enable_dynamic_field_update: enable_dynamic_field_update,
                        include_deleted_records: include_deleted_records,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ScheduledTriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduledTriggerProperties {
        /// Property [`DataPullMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-datapullmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_pull_mode: Option<::Value<String>>,
        /// Property [`FirstExecutionFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-firstexecutionfrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_execution_from: Option<::Value<f64>>,
        /// Property [`ScheduleEndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleendtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_end_time: Option<::Value<f64>>,
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
        /// Property [`ScheduleOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_offset: Option<::Value<u32>>,
        /// Property [`ScheduleStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-schedulestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_start_time: Option<::Value<f64>>,
        /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScheduledTriggerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_pull_mode) = self.data_pull_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataPullMode", data_pull_mode)?;
            }
            if let Some(ref first_execution_from) = self.first_execution_from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstExecutionFrom", first_execution_from)?;
            }
            if let Some(ref schedule_end_time) = self.schedule_end_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleEndTime", schedule_end_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            if let Some(ref schedule_offset) = self.schedule_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleOffset", schedule_offset)?;
            }
            if let Some(ref schedule_start_time) = self.schedule_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleStartTime", schedule_start_time)?;
            }
            if let Some(ref timezone) = self.timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduledTriggerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledTriggerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduledTriggerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduledTriggerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_pull_mode: Option<::Value<String>> = None;
                    let mut first_execution_from: Option<::Value<f64>> = None;
                    let mut schedule_end_time: Option<::Value<f64>> = None;
                    let mut schedule_expression: Option<::Value<String>> = None;
                    let mut schedule_offset: Option<::Value<u32>> = None;
                    let mut schedule_start_time: Option<::Value<f64>> = None;
                    let mut timezone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataPullMode" => {
                                data_pull_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstExecutionFrom" => {
                                first_execution_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleEndTime" => {
                                schedule_end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleOffset" => {
                                schedule_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleStartTime" => {
                                schedule_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timezone" => {
                                timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduledTriggerProperties {
                        data_pull_mode: data_pull_mode,
                        first_execution_from: first_execution_from,
                        schedule_end_time: schedule_end_time,
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                        schedule_offset: schedule_offset,
                        schedule_start_time: schedule_start_time,
                        timezone: timezone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ServiceNowSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-servicenowsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-servicenowsourceproperties.html#cfn-customerprofiles-integration-servicenowsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceNowSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConnectorProperties {
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoSourceProperties>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3SourceProperties>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceSourceProperties>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<ServiceNowSourceProperties>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskSourceProperties>>,
    }

    impl ::codec::SerializeValue for SourceConnectorProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConnectorProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConnectorProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConnectorProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConnectorProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut marketo: Option<::Value<MarketoSourceProperties>> = None;
                    let mut s3: Option<::Value<S3SourceProperties>> = None;
                    let mut salesforce: Option<::Value<SalesforceSourceProperties>> = None;
                    let mut service_now: Option<::Value<ServiceNowSourceProperties>> = None;
                    let mut zendesk: Option<::Value<ZendeskSourceProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConnectorProperties {
                        marketo: marketo,
                        s3: s3,
                        salesforce: salesforce,
                        service_now: service_now,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceFlowConfig {
        /// Property [`ConnectorProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-connectorprofilename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_name: Option<::Value<String>>,
        /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-connectortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_type: ::Value<String>,
        /// Property [`IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-incrementalpullconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub incremental_pull_config: Option<::Value<IncrementalPullConfig>>,
        /// Property [`SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-sourceconnectorproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_connector_properties: ::Value<SourceConnectorProperties>,
    }

    impl ::codec::SerializeValue for SourceFlowConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_profile_name) = self.connector_profile_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileName", connector_profile_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
            if let Some(ref incremental_pull_config) = self.incremental_pull_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncrementalPullConfig", incremental_pull_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConnectorProperties", &self.source_connector_properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceFlowConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceFlowConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceFlowConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceFlowConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_profile_name: Option<::Value<String>> = None;
                    let mut connector_type: Option<::Value<String>> = None;
                    let mut incremental_pull_config: Option<::Value<IncrementalPullConfig>> = None;
                    let mut source_connector_properties: Option<::Value<SourceConnectorProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorProfileName" => {
                                connector_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorType" => {
                                connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncrementalPullConfig" => {
                                incremental_pull_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceConnectorProperties" => {
                                source_connector_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceFlowConfig {
                        connector_profile_name: connector_profile_name,
                        connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                        incremental_pull_config: incremental_pull_config,
                        source_connector_properties: source_connector_properties.ok_or(::serde::de::Error::missing_field("SourceConnectorProperties"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.Task`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html) property type.
    #[derive(Debug, Default)]
    pub struct Task {
        /// Property [`ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-connectoroperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_operator: Option<::Value<ConnectorOperator>>,
        /// Property [`DestinationField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-destinationfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_field: Option<::Value<String>>,
        /// Property [`SourceFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-sourcefields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_fields: ::ValueList<String>,
        /// Property [`TaskProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-taskproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_properties: Option<::ValueList<TaskPropertiesMap>>,
        /// Property [`TaskType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-tasktype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Task {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_operator) = self.connector_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOperator", connector_operator)?;
            }
            if let Some(ref destination_field) = self.destination_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationField", destination_field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFields", &self.source_fields)?;
            if let Some(ref task_properties) = self.task_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskProperties", task_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskType", &self.task_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Task {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Task, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Task;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Task")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_operator: Option<::Value<ConnectorOperator>> = None;
                    let mut destination_field: Option<::Value<String>> = None;
                    let mut source_fields: Option<::ValueList<String>> = None;
                    let mut task_properties: Option<::ValueList<TaskPropertiesMap>> = None;
                    let mut task_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorOperator" => {
                                connector_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationField" => {
                                destination_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFields" => {
                                source_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskProperties" => {
                                task_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskType" => {
                                task_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Task {
                        connector_operator: connector_operator,
                        destination_field: destination_field,
                        source_fields: source_fields.ok_or(::serde::de::Error::missing_field("SourceFields"))?,
                        task_properties: task_properties,
                        task_type: task_type.ok_or(::serde::de::Error::missing_field("TaskType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TaskPropertiesMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskPropertiesMap {
        /// Property [`OperatorPropertyKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html#cfn-customerprofiles-integration-taskpropertiesmap-operatorpropertykey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator_property_key: ::Value<String>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html#cfn-customerprofiles-integration-taskpropertiesmap-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskPropertiesMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatorPropertyKey", &self.operator_property_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskPropertiesMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskPropertiesMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskPropertiesMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskPropertiesMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut operator_property_key: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OperatorPropertyKey" => {
                                operator_property_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskPropertiesMap {
                        operator_property_key: operator_property_key.ok_or(::serde::de::Error::missing_field("OperatorPropertyKey"))?,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggerConfig {
        /// Property [`TriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html#cfn-customerprofiles-integration-triggerconfig-triggerproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_properties: Option<::Value<TriggerProperties>>,
        /// Property [`TriggerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html#cfn-customerprofiles-integration-triggerconfig-triggertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TriggerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref trigger_properties) = self.trigger_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerProperties", trigger_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerType", &self.trigger_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut trigger_properties: Option<::Value<TriggerProperties>> = None;
                    let mut trigger_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TriggerProperties" => {
                                trigger_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerType" => {
                                trigger_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerConfig {
                        trigger_properties: trigger_properties,
                        trigger_type: trigger_type.ok_or(::serde::de::Error::missing_field("TriggerType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggerProperties {
        /// Property [`Scheduled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerproperties.html#cfn-customerprofiles-integration-triggerproperties-scheduled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled: Option<::Value<ScheduledTriggerProperties>>,
    }

    impl ::codec::SerializeValue for TriggerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref scheduled) = self.scheduled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheduled", scheduled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scheduled: Option<::Value<ScheduledTriggerProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Scheduled" => {
                                scheduled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerProperties {
                        scheduled: scheduled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ZendeskSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-zendesksourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-zendesksourceproperties.html#cfn-customerprofiles-integration-zendesksourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ZendeskSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod object_type {
    //! Property types for the `ObjectType` resource.

    /// The [`AWS::CustomerProfiles::ObjectType.FieldMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldMap {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html#cfn-customerprofiles-objecttype-fieldmap-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ObjectTypeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html#cfn-customerprofiles-objecttype-fieldmap-objecttypefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_type_field: Option<::Value<ObjectTypeField>>,
    }

    impl ::codec::SerializeValue for FieldMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref object_type_field) = self.object_type_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeField", object_type_field)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut object_type_field: Option<::Value<ObjectTypeField>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectTypeField" => {
                                object_type_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldMap {
                        name: name,
                        object_type_field: object_type_field,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.KeyMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyMap {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html#cfn-customerprofiles-objecttype-keymap-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ObjectTypeKeyList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html#cfn-customerprofiles-objecttype-keymap-objecttypekeylist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_type_key_list: Option<::ValueList<ObjectTypeKey>>,
    }

    impl ::codec::SerializeValue for KeyMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref object_type_key_list) = self.object_type_key_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeKeyList", object_type_key_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut object_type_key_list: Option<::ValueList<ObjectTypeKey>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectTypeKeyList" => {
                                object_type_key_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyMap {
                        name: name,
                        object_type_key_list: object_type_key_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.ObjectTypeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectTypeField {
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<String>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ObjectTypeField {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectTypeField {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeField, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectTypeField;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectTypeField")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_type: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObjectTypeField {
                        content_type: content_type,
                        source: source,
                        target: target,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.ObjectTypeKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectTypeKey {
        /// Property [`FieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html#cfn-customerprofiles-objecttype-objecttypekey-fieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_names: Option<::ValueList<String>>,
        /// Property [`StandardIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html#cfn-customerprofiles-objecttype-objecttypekey-standardidentifiers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_identifiers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ObjectTypeKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field_names) = self.field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldNames", field_names)?;
            }
            if let Some(ref standard_identifiers) = self.standard_identifiers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardIdentifiers", standard_identifiers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectTypeKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectTypeKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectTypeKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_names: Option<::ValueList<String>> = None;
                    let mut standard_identifiers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldNames" => {
                                field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardIdentifiers" => {
                                standard_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObjectTypeKey {
                        field_names: field_names,
                        standard_identifiers: standard_identifiers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
