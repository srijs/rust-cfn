//! Types for the `SES` service.

/// The [`AWS::SES::ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html) resource type.
#[derive(Debug)]
pub struct ConfigurationSet {
    properties: ConfigurationSetProperties
}

/// Properties for the `ConfigurationSet` resource.
#[derive(Debug)]
pub struct ConfigurationSetProperties {
    /// Property `Name`.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ConfigurationSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetProperties {
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ConfigurationSet {
    type Properties = ConfigurationSetProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSet";
    fn properties(&self) -> &ConfigurationSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationSet {}

impl From<ConfigurationSetProperties> for ConfigurationSet {
    fn from(properties: ConfigurationSetProperties) -> ConfigurationSet {
        ConfigurationSet { properties }
    }
}

/// The [`AWS::SES::ConfigurationSetEventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html) resource type.
#[derive(Debug)]
pub struct ConfigurationSetEventDestination {
    properties: ConfigurationSetEventDestinationProperties
}

/// Properties for the `ConfigurationSetEventDestination` resource.
#[derive(Debug)]
pub struct ConfigurationSetEventDestinationProperties {
    /// Property `ConfigurationSetName`.
    pub configuration_set_name: ::Value<String>,
    /// Property `EventDestination`.
    pub event_destination: ::Value<self::configuration_set_event_destination::EventDestination>,
}

impl ::serde::Serialize for ConfigurationSetEventDestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSetName", &self.configuration_set_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventDestination", &self.event_destination)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationSetEventDestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationSetEventDestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationSetEventDestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationSetEventDestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration_set_name = None;
                let mut event_destination = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigurationSetName" => {
                            configuration_set_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventDestination" => {
                            event_destination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetEventDestinationProperties {
                    configuration_set_name: configuration_set_name.ok_or(::serde::de::Error::missing_field("ConfigurationSetName"))?,
                    event_destination: event_destination.ok_or(::serde::de::Error::missing_field("EventDestination"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ConfigurationSetEventDestination {
    type Properties = ConfigurationSetEventDestinationProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSetEventDestination";
    fn properties(&self) -> &ConfigurationSetEventDestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetEventDestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationSetEventDestination {}

impl From<ConfigurationSetEventDestinationProperties> for ConfigurationSetEventDestination {
    fn from(properties: ConfigurationSetEventDestinationProperties) -> ConfigurationSetEventDestination {
        ConfigurationSetEventDestination { properties }
    }
}

/// The [`AWS::SES::ReceiptFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html) resource type.
#[derive(Debug)]
pub struct ReceiptFilter {
    properties: ReceiptFilterProperties
}

/// Properties for the `ReceiptFilter` resource.
#[derive(Debug)]
pub struct ReceiptFilterProperties {
    /// Property `Filter`.
    pub filter: ::Value<self::receipt_filter::Filter>,
}

impl ::serde::Serialize for ReceiptFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", &self.filter)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Filter" => {
                            filter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptFilterProperties {
                    filter: filter.ok_or(::serde::de::Error::missing_field("Filter"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReceiptFilter {
    type Properties = ReceiptFilterProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptFilter";
    fn properties(&self) -> &ReceiptFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptFilter {}

impl From<ReceiptFilterProperties> for ReceiptFilter {
    fn from(properties: ReceiptFilterProperties) -> ReceiptFilter {
        ReceiptFilter { properties }
    }
}

/// The [`AWS::SES::ReceiptRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html) resource type.
#[derive(Debug)]
pub struct ReceiptRule {
    properties: ReceiptRuleProperties
}

/// Properties for the `ReceiptRule` resource.
#[derive(Debug)]
pub struct ReceiptRuleProperties {
    /// Property `After`.
    pub after: Option<::Value<String>>,
    /// Property `Rule`.
    pub rule: ::Value<self::receipt_rule::Rule>,
    /// Property `RuleSetName`.
    pub rule_set_name: ::Value<String>,
}

impl ::serde::Serialize for ReceiptRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "After", &self.after)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rule", &self.rule)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetName", &self.rule_set_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut after = None;
                let mut rule = None;
                let mut rule_set_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "After" => {
                            after = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Rule" => {
                            rule = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RuleSetName" => {
                            rule_set_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptRuleProperties {
                    after: after,
                    rule: rule.ok_or(::serde::de::Error::missing_field("Rule"))?,
                    rule_set_name: rule_set_name.ok_or(::serde::de::Error::missing_field("RuleSetName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReceiptRule {
    type Properties = ReceiptRuleProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRule";
    fn properties(&self) -> &ReceiptRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptRule {}

impl From<ReceiptRuleProperties> for ReceiptRule {
    fn from(properties: ReceiptRuleProperties) -> ReceiptRule {
        ReceiptRule { properties }
    }
}

/// The [`AWS::SES::ReceiptRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html) resource type.
#[derive(Debug)]
pub struct ReceiptRuleSet {
    properties: ReceiptRuleSetProperties
}

/// Properties for the `ReceiptRuleSet` resource.
#[derive(Debug)]
pub struct ReceiptRuleSetProperties {
    /// Property `RuleSetName`.
    pub rule_set_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ReceiptRuleSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetName", &self.rule_set_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptRuleSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptRuleSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptRuleSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptRuleSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_set_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleSetName" => {
                            rule_set_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptRuleSetProperties {
                    rule_set_name: rule_set_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for ReceiptRuleSet {
    type Properties = ReceiptRuleSetProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRuleSet";
    fn properties(&self) -> &ReceiptRuleSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptRuleSet {}

impl From<ReceiptRuleSetProperties> for ReceiptRuleSet {
    fn from(properties: ReceiptRuleSetProperties) -> ReceiptRuleSet {
        ReceiptRuleSet { properties }
    }
}

/// The [`AWS::SES::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html) resource type.
#[derive(Debug)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Debug)]
pub struct TemplateProperties {
    /// Property `Template`.
    pub template: Option<::Value<self::template::Template>>,
}

impl ::serde::Serialize for TemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Template", &self.template)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut template = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Template" => {
                            template = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(TemplateProperties {
                    template: template,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Template {
    type Properties = TemplateProperties;
    const TYPE: &'static str = "AWS::SES::Template";
    fn properties(&self) -> &TemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Template {}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

pub mod configuration_set_event_destination {
    //! Property types for the `ConfigurationSetEventDestination` resource.

    /// The [`AWS::SES::ConfigurationSetEventDestination.CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-cloudwatchdestination.html) property type.
    #[derive(Debug)]
    pub struct CloudWatchDestination {
        /// Property `DimensionConfigurations`.
        pub dimension_configurations: Option<::ValueList<DimensionConfiguration>>,
    }

    impl ::codec::SerializeValue for CloudWatchDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionConfigurations", &self.dimension_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_configurations = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionConfigurations" => {
                                dimension_configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchDestination {
                        dimension_configurations: dimension_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.DimensionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html) property type.
    #[derive(Debug)]
    pub struct DimensionConfiguration {
        /// Property `DefaultDimensionValue`.
        pub default_dimension_value: ::Value<String>,
        /// Property `DimensionName`.
        pub dimension_name: ::Value<String>,
        /// Property `DimensionValueSource`.
        pub dimension_value_source: ::Value<String>,
    }

    impl ::codec::SerializeValue for DimensionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultDimensionValue", &self.default_dimension_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionName", &self.dimension_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionValueSource", &self.dimension_value_source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DimensionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DimensionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DimensionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DimensionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_dimension_value = None;
                    let mut dimension_name = None;
                    let mut dimension_value_source = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultDimensionValue" => {
                                default_dimension_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DimensionName" => {
                                dimension_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DimensionValueSource" => {
                                dimension_value_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DimensionConfiguration {
                        default_dimension_value: default_dimension_value.ok_or(::serde::de::Error::missing_field("DefaultDimensionValue"))?,
                        dimension_name: dimension_name.ok_or(::serde::de::Error::missing_field("DimensionName"))?,
                        dimension_value_source: dimension_value_source.ok_or(::serde::de::Error::missing_field("DimensionValueSource"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html) property type.
    #[derive(Debug)]
    pub struct EventDestination {
        /// Property `CloudWatchDestination`.
        pub cloud_watch_destination: Option<::Value<CloudWatchDestination>>,
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `KinesisFirehoseDestination`.
        pub kinesis_firehose_destination: Option<::Value<KinesisFirehoseDestination>>,
        /// Property `MatchingEventTypes`.
        pub matching_event_types: ::ValueList<String>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchDestination", &self.cloud_watch_destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseDestination", &self.kinesis_firehose_destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchingEventTypes", &self.matching_event_types)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_destination = None;
                    let mut enabled = None;
                    let mut kinesis_firehose_destination = None;
                    let mut matching_event_types = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchDestination" => {
                                cloud_watch_destination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KinesisFirehoseDestination" => {
                                kinesis_firehose_destination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MatchingEventTypes" => {
                                matching_event_types = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EventDestination {
                        cloud_watch_destination: cloud_watch_destination,
                        enabled: enabled,
                        kinesis_firehose_destination: kinesis_firehose_destination,
                        matching_event_types: matching_event_types.ok_or(::serde::de::Error::missing_field("MatchingEventTypes"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html) property type.
    #[derive(Debug)]
    pub struct KinesisFirehoseDestination {
        /// Property `DeliveryStreamARN`.
        pub delivery_stream_arn: ::Value<String>,
        /// Property `IAMRoleARN`.
        pub iam_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamARN", &self.delivery_stream_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMRoleARN", &self.iam_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream_arn = None;
                    let mut iam_role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStreamARN" => {
                                delivery_stream_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IAMRoleARN" => {
                                iam_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseDestination {
                        delivery_stream_arn: delivery_stream_arn.ok_or(::serde::de::Error::missing_field("DeliveryStreamARN"))?,
                        iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IAMRoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod receipt_filter {
    //! Property types for the `ReceiptFilter` resource.

    /// The [`AWS::SES::ReceiptFilter.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html) property type.
    #[derive(Debug)]
    pub struct Filter {
        /// Property `IpFilter`.
        pub ip_filter: ::Value<IpFilter>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpFilter", &self.ip_filter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_filter = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IpFilter" => {
                                ip_filter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        ip_filter: ip_filter.ok_or(::serde::de::Error::missing_field("IpFilter"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptFilter.IpFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html) property type.
    #[derive(Debug)]
    pub struct IpFilter {
        /// Property `Cidr`.
        pub cidr: ::Value<String>,
        /// Property `Policy`.
        pub policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for IpFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cidr", &self.cidr)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr = None;
                    let mut policy = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cidr" => {
                                cidr = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Policy" => {
                                policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(IpFilter {
                        cidr: cidr.ok_or(::serde::de::Error::missing_field("Cidr"))?,
                        policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod receipt_rule {
    //! Property types for the `ReceiptRule` resource.

    /// The [`AWS::SES::ReceiptRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html) property type.
    #[derive(Debug)]
    pub struct Action {
        /// Property `AddHeaderAction`.
        pub add_header_action: Option<::Value<AddHeaderAction>>,
        /// Property `BounceAction`.
        pub bounce_action: Option<::Value<BounceAction>>,
        /// Property `LambdaAction`.
        pub lambda_action: Option<::Value<LambdaAction>>,
        /// Property `S3Action`.
        pub s3_action: Option<::Value<S3Action>>,
        /// Property `SNSAction`.
        pub sns_action: Option<::Value<SNSAction>>,
        /// Property `StopAction`.
        pub stop_action: Option<::Value<StopAction>>,
        /// Property `WorkmailAction`.
        pub workmail_action: Option<::Value<WorkmailAction>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddHeaderAction", &self.add_header_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BounceAction", &self.bounce_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaAction", &self.lambda_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Action", &self.s3_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SNSAction", &self.sns_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StopAction", &self.stop_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkmailAction", &self.workmail_action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_header_action = None;
                    let mut bounce_action = None;
                    let mut lambda_action = None;
                    let mut s3_action = None;
                    let mut sns_action = None;
                    let mut stop_action = None;
                    let mut workmail_action = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddHeaderAction" => {
                                add_header_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BounceAction" => {
                                bounce_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LambdaAction" => {
                                lambda_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Action" => {
                                s3_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SNSAction" => {
                                sns_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StopAction" => {
                                stop_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WorkmailAction" => {
                                workmail_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        add_header_action: add_header_action,
                        bounce_action: bounce_action,
                        lambda_action: lambda_action,
                        s3_action: s3_action,
                        sns_action: sns_action,
                        stop_action: stop_action,
                        workmail_action: workmail_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.AddHeaderAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html) property type.
    #[derive(Debug)]
    pub struct AddHeaderAction {
        /// Property `HeaderName`.
        pub header_name: ::Value<String>,
        /// Property `HeaderValue`.
        pub header_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for AddHeaderAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderValue", &self.header_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddHeaderAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddHeaderAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddHeaderAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddHeaderAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_name = None;
                    let mut header_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderName" => {
                                header_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HeaderValue" => {
                                header_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AddHeaderAction {
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        header_value: header_value.ok_or(::serde::de::Error::missing_field("HeaderValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.BounceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html) property type.
    #[derive(Debug)]
    pub struct BounceAction {
        /// Property `Message`.
        pub message: ::Value<String>,
        /// Property `Sender`.
        pub sender: ::Value<String>,
        /// Property `SmtpReplyCode`.
        pub smtp_reply_code: ::Value<String>,
        /// Property `StatusCode`.
        pub status_code: Option<::Value<String>>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BounceAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", &self.message)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sender", &self.sender)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmtpReplyCode", &self.smtp_reply_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BounceAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BounceAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BounceAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BounceAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message = None;
                    let mut sender = None;
                    let mut smtp_reply_code = None;
                    let mut status_code = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Message" => {
                                message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Sender" => {
                                sender = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SmtpReplyCode" => {
                                smtp_reply_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StatusCode" => {
                                status_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BounceAction {
                        message: message.ok_or(::serde::de::Error::missing_field("Message"))?,
                        sender: sender.ok_or(::serde::de::Error::missing_field("Sender"))?,
                        smtp_reply_code: smtp_reply_code.ok_or(::serde::de::Error::missing_field("SmtpReplyCode"))?,
                        status_code: status_code,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html) property type.
    #[derive(Debug)]
    pub struct LambdaAction {
        /// Property `FunctionArn`.
        pub function_arn: ::Value<String>,
        /// Property `InvocationType`.
        pub invocation_type: Option<::Value<String>>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", &self.function_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationType", &self.invocation_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn = None;
                    let mut invocation_type = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InvocationType" => {
                                invocation_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaAction {
                        function_arn: function_arn.ok_or(::serde::de::Error::missing_field("FunctionArn"))?,
                        invocation_type: invocation_type,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html) property type.
    #[derive(Debug)]
    pub struct Rule {
        /// Property `Actions`.
        pub actions: Option<::ValueList<Action>>,
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Recipients`.
        pub recipients: Option<::ValueList<String>>,
        /// Property `ScanEnabled`.
        pub scan_enabled: Option<::Value<bool>>,
        /// Property `TlsPolicy`.
        pub tls_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recipients", &self.recipients)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanEnabled", &self.scan_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsPolicy", &self.tls_policy)?;
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
                    let mut actions = None;
                    let mut enabled = None;
                    let mut name = None;
                    let mut recipients = None;
                    let mut scan_enabled = None;
                    let mut tls_policy = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Recipients" => {
                                recipients = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ScanEnabled" => {
                                scan_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TlsPolicy" => {
                                tls_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        actions: actions,
                        enabled: enabled,
                        name: name,
                        recipients: recipients,
                        scan_enabled: scan_enabled,
                        tls_policy: tls_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html) property type.
    #[derive(Debug)]
    pub struct S3Action {
        /// Property `BucketName`.
        pub bucket_name: ::Value<String>,
        /// Property `KmsKeyArn`.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property `ObjectKeyPrefix`.
        pub object_key_prefix: Option<::Value<String>>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", &self.kms_key_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectKeyPrefix", &self.object_key_prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name = None;
                    let mut kms_key_arn = None;
                    let mut object_key_prefix = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KmsKeyArn" => {
                                kms_key_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ObjectKeyPrefix" => {
                                object_key_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Action {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        kms_key_arn: kms_key_arn,
                        object_key_prefix: object_key_prefix,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.SNSAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html) property type.
    #[derive(Debug)]
    pub struct SNSAction {
        /// Property `Encoding`.
        pub encoding: Option<::Value<String>>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SNSAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encoding", &self.encoding)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SNSAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SNSAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SNSAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SNSAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encoding = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encoding" => {
                                encoding = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SNSAction {
                        encoding: encoding,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.StopAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html) property type.
    #[derive(Debug)]
    pub struct StopAction {
        /// Property `Scope`.
        pub scope: ::Value<String>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StopAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StopAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StopAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StopAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StopAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scope = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Scope" => {
                                scope = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StopAction {
                        scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.WorkmailAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html) property type.
    #[derive(Debug)]
    pub struct WorkmailAction {
        /// Property `OrganizationArn`.
        pub organization_arn: ::Value<String>,
        /// Property `TopicArn`.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WorkmailAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationArn", &self.organization_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkmailAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkmailAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkmailAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkmailAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut organization_arn = None;
                    let mut topic_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OrganizationArn" => {
                                organization_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TopicArn" => {
                                topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkmailAction {
                        organization_arn: organization_arn.ok_or(::serde::de::Error::missing_field("OrganizationArn"))?,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod template {
    //! Property types for the `Template` resource.

    /// The [`AWS::SES::Template.Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html) property type.
    #[derive(Debug)]
    pub struct Template {
        /// Property `HtmlPart`.
        pub html_part: Option<::Value<String>>,
        /// Property `SubjectPart`.
        pub subject_part: Option<::Value<String>>,
        /// Property `TemplateName`.
        pub template_name: Option<::Value<String>>,
        /// Property `TextPart`.
        pub text_part: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Template {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlPart", &self.html_part)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectPart", &self.subject_part)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextPart", &self.text_part)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Template {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Template, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Template;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Template")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut html_part = None;
                    let mut subject_part = None;
                    let mut template_name = None;
                    let mut text_part = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HtmlPart" => {
                                html_part = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SubjectPart" => {
                                subject_part = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TemplateName" => {
                                template_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TextPart" => {
                                text_part = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Template {
                        html_part: html_part,
                        subject_part: subject_part,
                        template_name: template_name,
                        text_part: text_part,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
