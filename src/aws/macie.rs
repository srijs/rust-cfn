//! Types for the `Macie` service.

/// The [`AWS::Macie::CustomDataIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html) resource type.
#[derive(Debug, Default)]
pub struct CustomDataIdentifier {
    properties: CustomDataIdentifierProperties
}

/// Properties for the `CustomDataIdentifier` resource.
#[derive(Debug, Default)]
pub struct CustomDataIdentifierProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IgnoreWords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-ignorewords).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ignore_words: Option<::ValueList<String>>,
    /// Property [`Keywords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-keywords).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub keywords: Option<::ValueList<String>>,
    /// Property [`MaximumMatchDistance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-maximummatchdistance).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub maximum_match_distance: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Regex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-customdataidentifier.html#cfn-macie-customdataidentifier-regex).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub regex: ::Value<String>,
}

impl ::serde::Serialize for CustomDataIdentifierProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref ignore_words) = self.ignore_words {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreWords", ignore_words)?;
        }
        if let Some(ref keywords) = self.keywords {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Keywords", keywords)?;
        }
        if let Some(ref maximum_match_distance) = self.maximum_match_distance {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumMatchDistance", maximum_match_distance)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Regex", &self.regex)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomDataIdentifierProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomDataIdentifierProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomDataIdentifierProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomDataIdentifierProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut ignore_words: Option<::ValueList<String>> = None;
                let mut keywords: Option<::ValueList<String>> = None;
                let mut maximum_match_distance: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut regex: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IgnoreWords" => {
                            ignore_words = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Keywords" => {
                            keywords = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumMatchDistance" => {
                            maximum_match_distance = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Regex" => {
                            regex = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomDataIdentifierProperties {
                    description: description,
                    ignore_words: ignore_words,
                    keywords: keywords,
                    maximum_match_distance: maximum_match_distance,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    regex: regex.ok_or(::serde::de::Error::missing_field("Regex"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomDataIdentifier {
    type Properties = CustomDataIdentifierProperties;
    const TYPE: &'static str = "AWS::Macie::CustomDataIdentifier";
    fn properties(&self) -> &CustomDataIdentifierProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomDataIdentifierProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomDataIdentifier {}

impl From<CustomDataIdentifierProperties> for CustomDataIdentifier {
    fn from(properties: CustomDataIdentifierProperties) -> CustomDataIdentifier {
        CustomDataIdentifier { properties }
    }
}

/// The [`AWS::Macie::FindingsFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html) resource type.
#[derive(Debug, Default)]
pub struct FindingsFilter {
    properties: FindingsFilterProperties
}

/// Properties for the `FindingsFilter` resource.
#[derive(Debug, Default)]
pub struct FindingsFilterProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html#cfn-macie-findingsfilter-action).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html#cfn-macie-findingsfilter-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FindingCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html#cfn-macie-findingsfilter-findingcriteria).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub finding_criteria: ::Value<self::findings_filter::FindingCriteria>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html#cfn-macie-findingsfilter-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-findingsfilter.html#cfn-macie-findingsfilter-position).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub position: Option<::Value<u32>>,
}

impl ::serde::Serialize for FindingsFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref action) = self.action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingCriteria", &self.finding_criteria)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref position) = self.position {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FindingsFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FindingsFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FindingsFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FindingsFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut finding_criteria: Option<::Value<self::findings_filter::FindingCriteria>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut position: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FindingCriteria" => {
                            finding_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Position" => {
                            position = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FindingsFilterProperties {
                    action: action,
                    description: description,
                    finding_criteria: finding_criteria.ok_or(::serde::de::Error::missing_field("FindingCriteria"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    position: position,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FindingsFilter {
    type Properties = FindingsFilterProperties;
    const TYPE: &'static str = "AWS::Macie::FindingsFilter";
    fn properties(&self) -> &FindingsFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FindingsFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FindingsFilter {}

impl From<FindingsFilterProperties> for FindingsFilter {
    fn from(properties: FindingsFilterProperties) -> FindingsFilter {
        FindingsFilter { properties }
    }
}

/// The [`AWS::Macie::Session`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-session.html) resource type.
#[derive(Debug, Default)]
pub struct Session {
    properties: SessionProperties
}

/// Properties for the `Session` resource.
#[derive(Debug, Default)]
pub struct SessionProperties {
    /// Property [`FindingPublishingFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-session.html#cfn-macie-session-findingpublishingfrequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub finding_publishing_frequency: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-macie-session.html#cfn-macie-session-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
}

impl ::serde::Serialize for SessionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref finding_publishing_frequency) = self.finding_publishing_frequency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingPublishingFrequency", finding_publishing_frequency)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SessionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SessionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SessionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SessionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut finding_publishing_frequency: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FindingPublishingFrequency" => {
                            finding_publishing_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SessionProperties {
                    finding_publishing_frequency: finding_publishing_frequency,
                    status: status,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Session {
    type Properties = SessionProperties;
    const TYPE: &'static str = "AWS::Macie::Session";
    fn properties(&self) -> &SessionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SessionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Session {}

impl From<SessionProperties> for Session {
    fn from(properties: SessionProperties) -> Session {
        Session { properties }
    }
}

pub mod findings_filter {
    //! Property types for the `FindingsFilter` resource.

    /// The [`AWS::Macie::FindingsFilter.Criterion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-criterion.html) property type.
    #[derive(Debug, Default)]
    pub struct Criterion {
    }

    impl ::codec::SerializeValue for Criterion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Criterion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Criterion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Criterion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Criterion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Criterion {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Macie::FindingsFilter.FindingCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingcriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct FindingCriteria {
        /// Property [`Criterion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingcriteria.html#cfn-macie-findingsfilter-findingcriteria-criterion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub criterion: Option<::Value<Criterion>>,
    }

    impl ::codec::SerializeValue for FindingCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref criterion) = self.criterion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criterion", criterion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FindingCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FindingCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FindingCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FindingCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut criterion: Option<::Value<Criterion>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Criterion" => {
                                criterion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FindingCriteria {
                        criterion: criterion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Macie::FindingsFilter.FindingsFilterListItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingsfilterlistitem.html) property type.
    #[derive(Debug, Default)]
    pub struct FindingsFilterListItem {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingsfilterlistitem.html#cfn-macie-findingsfilter-findingsfilterlistitem-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-macie-findingsfilter-findingsfilterlistitem.html#cfn-macie-findingsfilter-findingsfilterlistitem-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FindingsFilterListItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FindingsFilterListItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FindingsFilterListItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FindingsFilterListItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FindingsFilterListItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FindingsFilterListItem {
                        id: id,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
