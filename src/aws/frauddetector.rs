//! Types for the `FraudDetector` service.

/// The [`AWS::FraudDetector::Detector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html) resource type.
#[derive(Debug, Default)]
pub struct Detector {
    properties: DetectorProperties
}

/// Properties for the `Detector` resource.
#[derive(Debug, Default)]
pub struct DetectorProperties {
    /// Property [`AssociatedModels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-associatedmodels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associated_models: Option<::ValueList<self::detector::Model>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-detectorid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub detector_id: ::Value<String>,
    /// Property [`DetectorVersionStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-detectorversionstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub detector_version_status: Option<::Value<String>>,
    /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-eventtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_type: ::Value<self::detector::EventType>,
    /// Property [`RuleExecutionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-ruleexecutionmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_execution_mode: Option<::Value<String>>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: ::ValueList<self::detector::Rule>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html#cfn-frauddetector-detector-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DetectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associated_models) = self.associated_models {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedModels", associated_models)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", &self.detector_id)?;
        if let Some(ref detector_version_status) = self.detector_version_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorVersionStatus", detector_version_status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", &self.event_type)?;
        if let Some(ref rule_execution_mode) = self.rule_execution_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleExecutionMode", rule_execution_mode)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DetectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DetectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DetectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DetectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associated_models: Option<::ValueList<self::detector::Model>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut detector_id: Option<::Value<String>> = None;
                let mut detector_version_status: Option<::Value<String>> = None;
                let mut event_type: Option<::Value<self::detector::EventType>> = None;
                let mut rule_execution_mode: Option<::Value<String>> = None;
                let mut rules: Option<::ValueList<self::detector::Rule>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociatedModels" => {
                            associated_models = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorId" => {
                            detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DetectorVersionStatus" => {
                            detector_version_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventType" => {
                            event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleExecutionMode" => {
                            rule_execution_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DetectorProperties {
                    associated_models: associated_models,
                    description: description,
                    detector_id: detector_id.ok_or(::serde::de::Error::missing_field("DetectorId"))?,
                    detector_version_status: detector_version_status,
                    event_type: event_type.ok_or(::serde::de::Error::missing_field("EventType"))?,
                    rule_execution_mode: rule_execution_mode,
                    rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Detector {
    type Properties = DetectorProperties;
    const TYPE: &'static str = "AWS::FraudDetector::Detector";
    fn properties(&self) -> &DetectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DetectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Detector {}

impl From<DetectorProperties> for Detector {
    fn from(properties: DetectorProperties) -> Detector {
        Detector { properties }
    }
}

/// The [`AWS::FraudDetector::EntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-entitytype.html) resource type.
#[derive(Debug, Default)]
pub struct EntityType {
    properties: EntityTypeProperties
}

/// Properties for the `EntityType` resource.
#[derive(Debug, Default)]
pub struct EntityTypeProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-entitytype.html#cfn-frauddetector-entitytype-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-entitytype.html#cfn-frauddetector-entitytype-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-entitytype.html#cfn-frauddetector-entitytype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EntityTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EntityTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EntityTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EntityTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(EntityTypeProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EntityType {
    type Properties = EntityTypeProperties;
    const TYPE: &'static str = "AWS::FraudDetector::EntityType";
    fn properties(&self) -> &EntityTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EntityTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EntityType {}

impl From<EntityTypeProperties> for EntityType {
    fn from(properties: EntityTypeProperties) -> EntityType {
        EntityType { properties }
    }
}

/// The [`AWS::FraudDetector::EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html) resource type.
#[derive(Debug, Default)]
pub struct EventType {
    properties: EventTypeProperties
}

/// Properties for the `EventType` resource.
#[derive(Debug, Default)]
pub struct EventTypeProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EntityTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-entitytypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entity_types: ::ValueList<self::event_type::EntityType>,
    /// Property [`EventVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-eventvariables).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_variables: ::ValueList<self::event_type::EventVariable>,
    /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-labels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub labels: ::ValueList<self::event_type::Label>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html#cfn-frauddetector-eventtype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EventTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityTypes", &self.entity_types)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventVariables", &self.event_variables)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", &self.labels)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut entity_types: Option<::ValueList<self::event_type::EntityType>> = None;
                let mut event_variables: Option<::ValueList<self::event_type::EventVariable>> = None;
                let mut labels: Option<::ValueList<self::event_type::Label>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntityTypes" => {
                            entity_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventVariables" => {
                            event_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Labels" => {
                            labels = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(EventTypeProperties {
                    description: description,
                    entity_types: entity_types.ok_or(::serde::de::Error::missing_field("EntityTypes"))?,
                    event_variables: event_variables.ok_or(::serde::de::Error::missing_field("EventVariables"))?,
                    labels: labels.ok_or(::serde::de::Error::missing_field("Labels"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventType {
    type Properties = EventTypeProperties;
    const TYPE: &'static str = "AWS::FraudDetector::EventType";
    fn properties(&self) -> &EventTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventType {}

impl From<EventTypeProperties> for EventType {
    fn from(properties: EventTypeProperties) -> EventType {
        EventType { properties }
    }
}

/// The [`AWS::FraudDetector::Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-label.html) resource type.
#[derive(Debug, Default)]
pub struct Label {
    properties: LabelProperties
}

/// Properties for the `Label` resource.
#[derive(Debug, Default)]
pub struct LabelProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-label.html#cfn-frauddetector-label-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-label.html#cfn-frauddetector-label-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-label.html#cfn-frauddetector-label-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LabelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LabelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LabelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LabelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LabelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(LabelProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Label {
    type Properties = LabelProperties;
    const TYPE: &'static str = "AWS::FraudDetector::Label";
    fn properties(&self) -> &LabelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LabelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Label {}

impl From<LabelProperties> for Label {
    fn from(properties: LabelProperties) -> Label {
        Label { properties }
    }
}

/// The [`AWS::FraudDetector::List`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html) resource type.
#[derive(Debug, Default)]
pub struct List {
    properties: ListProperties
}

/// Properties for the `List` resource.
#[derive(Debug, Default)]
pub struct ListProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html#cfn-frauddetector-list-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Elements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html#cfn-frauddetector-list-elements).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elements: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html#cfn-frauddetector-list-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html#cfn-frauddetector-list-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VariableType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html#cfn-frauddetector-list-variabletype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub variable_type: Option<::Value<String>>,
}

impl ::serde::Serialize for ListProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref elements) = self.elements {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Elements", elements)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref variable_type) = self.variable_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableType", variable_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ListProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ListProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ListProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut elements: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut variable_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Elements" => {
                            elements = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VariableType" => {
                            variable_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ListProperties {
                    description: description,
                    elements: elements,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    variable_type: variable_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for List {
    type Properties = ListProperties;
    const TYPE: &'static str = "AWS::FraudDetector::List";
    fn properties(&self) -> &ListProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ListProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for List {}

impl From<ListProperties> for List {
    fn from(properties: ListProperties) -> List {
        List { properties }
    }
}

/// The [`AWS::FraudDetector::Outcome`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-outcome.html) resource type.
#[derive(Debug, Default)]
pub struct Outcome {
    properties: OutcomeProperties
}

/// Properties for the `Outcome` resource.
#[derive(Debug, Default)]
pub struct OutcomeProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-outcome.html#cfn-frauddetector-outcome-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-outcome.html#cfn-frauddetector-outcome-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-outcome.html#cfn-frauddetector-outcome-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for OutcomeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OutcomeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OutcomeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OutcomeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OutcomeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(OutcomeProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Outcome {
    type Properties = OutcomeProperties;
    const TYPE: &'static str = "AWS::FraudDetector::Outcome";
    fn properties(&self) -> &OutcomeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OutcomeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Outcome {}

impl From<OutcomeProperties> for Outcome {
    fn from(properties: OutcomeProperties) -> Outcome {
        Outcome { properties }
    }
}

/// The [`AWS::FraudDetector::Variable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html) resource type.
#[derive(Debug, Default)]
pub struct Variable {
    properties: VariableProperties
}

/// Properties for the `Variable` resource.
#[derive(Debug, Default)]
pub struct VariableProperties {
    /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-datasource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source: ::Value<String>,
    /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-datatype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_type: ::Value<String>,
    /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-defaultvalue).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_value: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VariableType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html#cfn-frauddetector-variable-variabletype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub variable_type: Option<::Value<String>>,
}

impl ::serde::Serialize for VariableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", &self.data_source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", &self.default_value)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref variable_type) = self.variable_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableType", variable_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VariableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VariableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VariableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VariableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_source: Option<::Value<String>> = None;
                let mut data_type: Option<::Value<String>> = None;
                let mut default_value: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut variable_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataSource" => {
                            data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataType" => {
                            data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultValue" => {
                            default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VariableType" => {
                            variable_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VariableProperties {
                    data_source: data_source.ok_or(::serde::de::Error::missing_field("DataSource"))?,
                    data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                    default_value: default_value.ok_or(::serde::de::Error::missing_field("DefaultValue"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    variable_type: variable_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Variable {
    type Properties = VariableProperties;
    const TYPE: &'static str = "AWS::FraudDetector::Variable";
    fn properties(&self) -> &VariableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VariableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Variable {}

impl From<VariableProperties> for Variable {
    fn from(properties: VariableProperties) -> Variable {
        Variable { properties }
    }
}

pub mod detector {
    //! Property types for the `Detector` resource.

    /// The [`AWS::FraudDetector::Detector.EntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityType {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html#cfn-frauddetector-detector-entitytype-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for EntityType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(EntityType {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html) property type.
    #[derive(Debug, Default)]
    pub struct EventType {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`EntityTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-entitytypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_types: Option<::ValueList<EntityType>>,
        /// Property [`EventVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-eventvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_variables: Option<::ValueList<EventVariable>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-labels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub labels: Option<::ValueList<Label>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html#cfn-frauddetector-detector-eventtype-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for EventType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref entity_types) = self.entity_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityTypes", entity_types)?;
            }
            if let Some(ref event_variables) = self.event_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventVariables", event_variables)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref labels) = self.labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", labels)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut entity_types: Option<::ValueList<EntityType>> = None;
                    let mut event_variables: Option<::ValueList<EventVariable>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut labels: Option<::ValueList<Label>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityTypes" => {
                                entity_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventVariables" => {
                                event_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Labels" => {
                                labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(EventType {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        entity_types: entity_types,
                        event_variables: event_variables,
                        inline: inline,
                        labels: labels,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.EventVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EventVariable {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-datasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source: Option<::Value<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: Option<::Value<String>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
        /// Property [`VariableType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html#cfn-frauddetector-detector-eventvariable-variabletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref data_source) = self.data_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", data_source)?;
            }
            if let Some(ref data_type) = self.data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            if let Some(ref variable_type) = self.variable_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableType", variable_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut data_source: Option<::Value<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;
                    let mut variable_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSource" => {
                                data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariableType" => {
                                variable_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventVariable {
                        arn: arn,
                        created_time: created_time,
                        data_source: data_source,
                        data_type: data_type,
                        default_value: default_value,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                        variable_type: variable_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html) property type.
    #[derive(Debug, Default)]
    pub struct Label {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html#cfn-frauddetector-detector-label-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for Label {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Label {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Label, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Label;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Label")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(Label {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-model.html) property type.
    #[derive(Debug, Default)]
    pub struct Model {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-model.html#cfn-frauddetector-detector-model-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Model {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Model {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Model, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Model;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Model")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Model {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.Outcome`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html) property type.
    #[derive(Debug, Default)]
    pub struct Outcome {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html#cfn-frauddetector-detector-outcome-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for Outcome {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Outcome {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Outcome, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Outcome;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Outcome")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(Outcome {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::Detector.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DetectorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-detectorid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detector_id: Option<::Value<String>>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`Language`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-language).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub language: Option<::Value<String>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Outcomes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-outcomes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outcomes: Option<::ValueList<Outcome>>,
        /// Property [`RuleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-ruleid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_id: Option<::Value<String>>,
        /// Property [`RuleVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-ruleversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_version: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html#cfn-frauddetector-detector-rule-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref detector_id) = self.detector_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectorId", detector_id)?;
            }
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            if let Some(ref language) = self.language {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Language", language)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref outcomes) = self.outcomes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Outcomes", outcomes)?;
            }
            if let Some(ref rule_id) = self.rule_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleId", rule_id)?;
            }
            if let Some(ref rule_version) = self.rule_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleVersion", rule_version)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut detector_id: Option<::Value<String>> = None;
                    let mut expression: Option<::Value<String>> = None;
                    let mut language: Option<::Value<String>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut outcomes: Option<::ValueList<Outcome>> = None;
                    let mut rule_id: Option<::Value<String>> = None;
                    let mut rule_version: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DetectorId" => {
                                detector_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Language" => {
                                language = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Outcomes" => {
                                outcomes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleId" => {
                                rule_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleVersion" => {
                                rule_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        detector_id: detector_id,
                        expression: expression,
                        language: language,
                        last_updated_time: last_updated_time,
                        outcomes: outcomes,
                        rule_id: rule_id,
                        rule_version: rule_version,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod event_type {
    //! Property types for the `EventType` resource.

    /// The [`AWS::FraudDetector::EventType.EntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityType {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html#cfn-frauddetector-eventtype-entitytype-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for EntityType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(EntityType {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::EventType.EventVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html) property type.
    #[derive(Debug, Default)]
    pub struct EventVariable {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-datasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source: Option<::Value<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: Option<::Value<String>>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
        /// Property [`VariableType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html#cfn-frauddetector-eventtype-eventvariable-variabletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref data_source) = self.data_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", data_source)?;
            }
            if let Some(ref data_type) = self.data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", data_type)?;
            }
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            if let Some(ref variable_type) = self.variable_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableType", variable_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut data_source: Option<::Value<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;
                    let mut variable_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSource" => {
                                data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariableType" => {
                                variable_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventVariable {
                        arn: arn,
                        created_time: created_time,
                        data_source: data_source,
                        data_type: data_type,
                        default_value: default_value,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                        variable_type: variable_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FraudDetector::EventType.Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html) property type.
    #[derive(Debug, Default)]
    pub struct Label {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`CreatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-createdtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_time: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Inline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-inline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline: Option<::Value<bool>>,
        /// Property [`LastUpdatedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-lastupdatedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_updated_time: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html#cfn-frauddetector-eventtype-label-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for Label {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref created_time) = self.created_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedTime", created_time)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref inline) = self.inline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inline", inline)?;
            }
            if let Some(ref last_updated_time) = self.last_updated_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUpdatedTime", last_updated_time)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Label {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Label, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Label;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Label")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut created_time: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut inline: Option<::Value<bool>> = None;
                    let mut last_updated_time: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedTime" => {
                                created_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Inline" => {
                                inline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastUpdatedTime" => {
                                last_updated_time = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(Label {
                        arn: arn,
                        created_time: created_time,
                        description: description,
                        inline: inline,
                        last_updated_time: last_updated_time,
                        name: name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
