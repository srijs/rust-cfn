//! Types for the `WAFv2` service.

/// The [`AWS::WAFv2::IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html) resource type.
#[derive(Debug, Default)]
pub struct IPSet {
    properties: IPSetProperties
}

/// Properties for the `IPSet` resource.
#[derive(Debug, Default)]
pub struct IPSetProperties {
    /// Property [`Addresses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-addresses).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub addresses: ::ValueList<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IPAddressVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-ipaddressversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_address_version: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html#cfn-wafv2-ipset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for IPSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Addresses", &self.addresses)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPAddressVersion", &self.ip_address_version)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IPSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IPSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IPSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut addresses: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut ip_address_version: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Addresses" => {
                            addresses = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IPAddressVersion" => {
                            ip_address_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IPSetProperties {
                    addresses: addresses.ok_or(::serde::de::Error::missing_field("Addresses"))?,
                    description: description,
                    ip_address_version: ip_address_version.ok_or(::serde::de::Error::missing_field("IPAddressVersion"))?,
                    name: name,
                    scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IPSet {
    type Properties = IPSetProperties;
    const TYPE: &'static str = "AWS::WAFv2::IPSet";
    fn properties(&self) -> &IPSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IPSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IPSet {}

impl From<IPSetProperties> for IPSet {
    fn from(properties: IPSetProperties) -> IPSet {
        IPSet { properties }
    }
}

/// The [`AWS::WAFv2::LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct LoggingConfiguration {
    properties: LoggingConfigurationProperties
}

/// Properties for the `LoggingConfiguration` resource.
#[derive(Debug, Default)]
pub struct LoggingConfigurationProperties {
    /// Property [`LogDestinationConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html#cfn-wafv2-loggingconfiguration-logdestinationconfigs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_destination_configs: ::ValueList<String>,
    /// Property [`LoggingFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html#cfn-wafv2-loggingconfiguration-loggingfilter).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_filter: Option<::Value<self::logging_configuration::LoggingFilter>>,
    /// Property [`RedactedFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html#cfn-wafv2-loggingconfiguration-redactedfields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redacted_fields: Option<::ValueList<self::logging_configuration::FieldToMatch>>,
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html#cfn-wafv2-loggingconfiguration-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
}

impl ::serde::Serialize for LoggingConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDestinationConfigs", &self.log_destination_configs)?;
        if let Some(ref logging_filter) = self.logging_filter {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingFilter", logging_filter)?;
        }
        if let Some(ref redacted_fields) = self.redacted_fields {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedactedFields", redacted_fields)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoggingConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoggingConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoggingConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut log_destination_configs: Option<::ValueList<String>> = None;
                let mut logging_filter: Option<::Value<self::logging_configuration::LoggingFilter>> = None;
                let mut redacted_fields: Option<::ValueList<self::logging_configuration::FieldToMatch>> = None;
                let mut resource_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LogDestinationConfigs" => {
                            log_destination_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingFilter" => {
                            logging_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedactedFields" => {
                            redacted_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoggingConfigurationProperties {
                    log_destination_configs: log_destination_configs.ok_or(::serde::de::Error::missing_field("LogDestinationConfigs"))?,
                    logging_filter: logging_filter,
                    redacted_fields: redacted_fields,
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoggingConfiguration {
    type Properties = LoggingConfigurationProperties;
    const TYPE: &'static str = "AWS::WAFv2::LoggingConfiguration";
    fn properties(&self) -> &LoggingConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoggingConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoggingConfiguration {}

impl From<LoggingConfigurationProperties> for LoggingConfiguration {
    fn from(properties: LoggingConfigurationProperties) -> LoggingConfiguration {
        LoggingConfiguration { properties }
    }
}

/// The [`AWS::WAFv2::RegexPatternSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html) resource type.
#[derive(Debug, Default)]
pub struct RegexPatternSet {
    properties: RegexPatternSetProperties
}

/// Properties for the `RegexPatternSet` resource.
#[derive(Debug, Default)]
pub struct RegexPatternSetProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html#cfn-wafv2-regexpatternset-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html#cfn-wafv2-regexpatternset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RegularExpressionList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html#cfn-wafv2-regexpatternset-regularexpressionlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regular_expression_list: ::ValueList<String>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html#cfn-wafv2-regexpatternset-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html#cfn-wafv2-regexpatternset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RegexPatternSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegularExpressionList", &self.regular_expression_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RegexPatternSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RegexPatternSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RegexPatternSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RegexPatternSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut regular_expression_list: Option<::ValueList<String>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegularExpressionList" => {
                            regular_expression_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RegexPatternSetProperties {
                    description: description,
                    name: name,
                    regular_expression_list: regular_expression_list.ok_or(::serde::de::Error::missing_field("RegularExpressionList"))?,
                    scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RegexPatternSet {
    type Properties = RegexPatternSetProperties;
    const TYPE: &'static str = "AWS::WAFv2::RegexPatternSet";
    fn properties(&self) -> &RegexPatternSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RegexPatternSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RegexPatternSet {}

impl From<RegexPatternSetProperties> for RegexPatternSet {
    fn from(properties: RegexPatternSetProperties) -> RegexPatternSet {
        RegexPatternSet { properties }
    }
}

/// The [`AWS::WAFv2::RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html) resource type.
#[derive(Debug, Default)]
pub struct RuleGroup {
    properties: RuleGroupProperties
}

/// Properties for the `RuleGroup` resource.
#[derive(Debug, Default)]
pub struct RuleGroupProperties {
    /// Property [`AvailableLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-availablelabels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub available_labels: Option<::ValueList<self::rule_group::LabelSummary>>,
    /// Property [`Capacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-capacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity: ::Value<u32>,
    /// Property [`ConsumedLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-consumedlabels).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub consumed_labels: Option<::ValueList<self::rule_group::LabelSummary>>,
    /// Property [`CustomResponseBodies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-customresponsebodies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_response_bodies: Option<::ValueMap<self::rule_group::CustomResponseBody>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: Option<::ValueList<self::rule_group::Rule>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html#cfn-wafv2-rulegroup-visibilityconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub visibility_config: ::Value<self::rule_group::VisibilityConfig>,
}

impl ::serde::Serialize for RuleGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref available_labels) = self.available_labels {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailableLabels", available_labels)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capacity", &self.capacity)?;
        if let Some(ref consumed_labels) = self.consumed_labels {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumedLabels", consumed_labels)?;
        }
        if let Some(ref custom_response_bodies) = self.custom_response_bodies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponseBodies", custom_response_bodies)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref rules) = self.rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", rules)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityConfig", &self.visibility_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RuleGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RuleGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RuleGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut available_labels: Option<::ValueList<self::rule_group::LabelSummary>> = None;
                let mut capacity: Option<::Value<u32>> = None;
                let mut consumed_labels: Option<::ValueList<self::rule_group::LabelSummary>> = None;
                let mut custom_response_bodies: Option<::ValueMap<self::rule_group::CustomResponseBody>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rules: Option<::ValueList<self::rule_group::Rule>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut visibility_config: Option<::Value<self::rule_group::VisibilityConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailableLabels" => {
                            available_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Capacity" => {
                            capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConsumedLabels" => {
                            consumed_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomResponseBodies" => {
                            custom_response_bodies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VisibilityConfig" => {
                            visibility_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleGroupProperties {
                    available_labels: available_labels,
                    capacity: capacity.ok_or(::serde::de::Error::missing_field("Capacity"))?,
                    consumed_labels: consumed_labels,
                    custom_response_bodies: custom_response_bodies,
                    description: description,
                    name: name,
                    rules: rules,
                    scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    tags: tags,
                    visibility_config: visibility_config.ok_or(::serde::de::Error::missing_field("VisibilityConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RuleGroup {
    type Properties = RuleGroupProperties;
    const TYPE: &'static str = "AWS::WAFv2::RuleGroup";
    fn properties(&self) -> &RuleGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RuleGroup {}

impl From<RuleGroupProperties> for RuleGroup {
    fn from(properties: RuleGroupProperties) -> RuleGroup {
        RuleGroup { properties }
    }
}

/// The [`AWS::WAFv2::WebACL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html) resource type.
#[derive(Debug, Default)]
pub struct WebACL {
    properties: WebACLProperties
}

/// Properties for the `WebACL` resource.
#[derive(Debug, Default)]
pub struct WebACLProperties {
    /// Property [`AssociationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-associationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub association_config: Option<::Value<self::web_acl::AssociationConfig>>,
    /// Property [`CaptchaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-captchaconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub captcha_config: Option<::Value<self::web_acl::CaptchaConfig>>,
    /// Property [`ChallengeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-challengeconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub challenge_config: Option<::Value<self::web_acl::ChallengeConfig>>,
    /// Property [`CustomResponseBodies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-customresponsebodies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_response_bodies: Option<::ValueMap<self::web_acl::CustomResponseBody>>,
    /// Property [`DefaultAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-defaultaction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_action: ::Value<self::web_acl::DefaultAction>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: Option<::ValueList<self::web_acl::Rule>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TokenDomains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-tokendomains).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_domains: Option<::ValueList<String>>,
    /// Property [`VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html#cfn-wafv2-webacl-visibilityconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub visibility_config: ::Value<self::web_acl::VisibilityConfig>,
}

impl ::serde::Serialize for WebACLProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref association_config) = self.association_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationConfig", association_config)?;
        }
        if let Some(ref captcha_config) = self.captcha_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptchaConfig", captcha_config)?;
        }
        if let Some(ref challenge_config) = self.challenge_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChallengeConfig", challenge_config)?;
        }
        if let Some(ref custom_response_bodies) = self.custom_response_bodies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponseBodies", custom_response_bodies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAction", &self.default_action)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref rules) = self.rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", rules)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref token_domains) = self.token_domains {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenDomains", token_domains)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityConfig", &self.visibility_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WebACLProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WebACLProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WebACLProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WebACLProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut association_config: Option<::Value<self::web_acl::AssociationConfig>> = None;
                let mut captcha_config: Option<::Value<self::web_acl::CaptchaConfig>> = None;
                let mut challenge_config: Option<::Value<self::web_acl::ChallengeConfig>> = None;
                let mut custom_response_bodies: Option<::ValueMap<self::web_acl::CustomResponseBody>> = None;
                let mut default_action: Option<::Value<self::web_acl::DefaultAction>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rules: Option<::ValueList<self::web_acl::Rule>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut token_domains: Option<::ValueList<String>> = None;
                let mut visibility_config: Option<::Value<self::web_acl::VisibilityConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociationConfig" => {
                            association_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CaptchaConfig" => {
                            captcha_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChallengeConfig" => {
                            challenge_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomResponseBodies" => {
                            custom_response_bodies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAction" => {
                            default_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenDomains" => {
                            token_domains = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VisibilityConfig" => {
                            visibility_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WebACLProperties {
                    association_config: association_config,
                    captcha_config: captcha_config,
                    challenge_config: challenge_config,
                    custom_response_bodies: custom_response_bodies,
                    default_action: default_action.ok_or(::serde::de::Error::missing_field("DefaultAction"))?,
                    description: description,
                    name: name,
                    rules: rules,
                    scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    tags: tags,
                    token_domains: token_domains,
                    visibility_config: visibility_config.ok_or(::serde::de::Error::missing_field("VisibilityConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WebACL {
    type Properties = WebACLProperties;
    const TYPE: &'static str = "AWS::WAFv2::WebACL";
    fn properties(&self) -> &WebACLProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACL {}

impl From<WebACLProperties> for WebACL {
    fn from(properties: WebACLProperties) -> WebACL {
        WebACL { properties }
    }
}

/// The [`AWS::WAFv2::WebACLAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webaclassociation.html) resource type.
#[derive(Debug, Default)]
pub struct WebACLAssociation {
    properties: WebACLAssociationProperties
}

/// Properties for the `WebACLAssociation` resource.
#[derive(Debug, Default)]
pub struct WebACLAssociationProperties {
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webaclassociation.html#cfn-wafv2-webaclassociation-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`WebACLArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webaclassociation.html#cfn-wafv2-webaclassociation-webaclarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub web_acl_arn: ::Value<String>,
}

impl ::serde::Serialize for WebACLAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebACLArn", &self.web_acl_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WebACLAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WebACLAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WebACLAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WebACLAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_arn: Option<::Value<String>> = None;
                let mut web_acl_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WebACLArn" => {
                            web_acl_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WebACLAssociationProperties {
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    web_acl_arn: web_acl_arn.ok_or(::serde::de::Error::missing_field("WebACLArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WebACLAssociation {
    type Properties = WebACLAssociationProperties;
    const TYPE: &'static str = "AWS::WAFv2::WebACLAssociation";
    fn properties(&self) -> &WebACLAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WebACLAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WebACLAssociation {}

impl From<WebACLAssociationProperties> for WebACLAssociation {
    fn from(properties: WebACLAssociationProperties) -> WebACLAssociation {
        WebACLAssociation { properties }
    }
}

pub mod logging_configuration {
    //! Property types for the `LoggingConfiguration` resource.

    /// The [`AWS::WAFv2::LoggingConfiguration.ActionCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-actioncondition.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionCondition {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-actioncondition.html#cfn-wafv2-loggingconfiguration-actioncondition-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
    }

    impl ::codec::SerializeValue for ActionCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionCondition {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-condition.html) property type.
    #[derive(Debug, Default)]
    pub struct Condition {
        /// Property [`ActionCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-condition.html#cfn-wafv2-loggingconfiguration-condition-actioncondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_condition: Option<::Value<ActionCondition>>,
        /// Property [`LabelNameCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-condition.html#cfn-wafv2-loggingconfiguration-condition-labelnamecondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_name_condition: Option<::Value<LabelNameCondition>>,
    }

    impl ::codec::SerializeValue for Condition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action_condition) = self.action_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionCondition", action_condition)?;
            }
            if let Some(ref label_name_condition) = self.label_name_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelNameCondition", label_name_condition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Condition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Condition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Condition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Condition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_condition: Option<::Value<ActionCondition>> = None;
                    let mut label_name_condition: Option<::Value<LabelNameCondition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionCondition" => {
                                action_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelNameCondition" => {
                                label_name_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Condition {
                        action_condition: action_condition,
                        label_name_condition: label_name_condition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html#cfn-wafv2-loggingconfiguration-fieldtomatch-jsonbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_body: Option<::Value<JsonBody>>,
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html#cfn-wafv2-loggingconfiguration-fieldtomatch-method).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method: Option<::Value<::json::Value>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html#cfn-wafv2-loggingconfiguration-fieldtomatch-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: Option<::Value<::json::Value>>,
        /// Property [`SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html#cfn-wafv2-loggingconfiguration-fieldtomatch-singleheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_header: Option<::Value<SingleHeader>>,
        /// Property [`UriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html#cfn-wafv2-loggingconfiguration-fieldtomatch-uripath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri_path: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref json_body) = self.json_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonBody", json_body)?;
            }
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref query_string) = self.query_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
            }
            if let Some(ref single_header) = self.single_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleHeader", single_header)?;
            }
            if let Some(ref uri_path) = self.uri_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPath", uri_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut json_body: Option<::Value<JsonBody>> = None;
                    let mut method: Option<::Value<::json::Value>> = None;
                    let mut query_string: Option<::Value<::json::Value>> = None;
                    let mut single_header: Option<::Value<SingleHeader>> = None;
                    let mut uri_path: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JsonBody" => {
                                json_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleHeader" => {
                                single_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UriPath" => {
                                uri_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        json_body: json_body,
                        method: method,
                        query_string: query_string,
                        single_header: single_header,
                        uri_path: uri_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-filter.html#cfn-wafv2-loggingconfiguration-filter-behavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior: ::Value<String>,
        /// Property [`Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-filter.html#cfn-wafv2-loggingconfiguration-filter-conditions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conditions: ::ValueList<Condition>,
        /// Property [`Requirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-filter.html#cfn-wafv2-loggingconfiguration-filter-requirement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub requirement: ::Value<String>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Behavior", &self.behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditions", &self.conditions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Requirement", &self.requirement)?;
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
                    let mut behavior: Option<::Value<String>> = None;
                    let mut conditions: Option<::ValueList<Condition>> = None;
                    let mut requirement: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Behavior" => {
                                behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Conditions" => {
                                conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Requirement" => {
                                requirement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        behavior: behavior.ok_or(::serde::de::Error::missing_field("Behavior"))?,
                        conditions: conditions.ok_or(::serde::de::Error::missing_field("Conditions"))?,
                        requirement: requirement.ok_or(::serde::de::Error::missing_field("Requirement"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-jsonbody.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonBody {
        /// Property [`InvalidFallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-jsonbody.html#cfn-wafv2-loggingconfiguration-jsonbody-invalidfallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invalid_fallback_behavior: Option<::Value<String>>,
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-jsonbody.html#cfn-wafv2-loggingconfiguration-jsonbody-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<MatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-jsonbody.html#cfn-wafv2-loggingconfiguration-jsonbody-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
    }

    impl ::codec::SerializeValue for JsonBody {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invalid_fallback_behavior) = self.invalid_fallback_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvalidFallbackBehavior", invalid_fallback_behavior)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonBody {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonBody, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonBody;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonBody")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invalid_fallback_behavior: Option<::Value<String>> = None;
                    let mut match_pattern: Option<::Value<MatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvalidFallbackBehavior" => {
                                invalid_fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonBody {
                        invalid_fallback_behavior: invalid_fallback_behavior,
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.LabelNameCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-labelnamecondition.html) property type.
    #[derive(Debug, Default)]
    pub struct LabelNameCondition {
        /// Property [`LabelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-labelnamecondition.html#cfn-wafv2-loggingconfiguration-labelnamecondition-labelname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for LabelNameCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelName", &self.label_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LabelNameCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LabelNameCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LabelNameCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LabelNameCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut label_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LabelName" => {
                                label_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LabelNameCondition {
                        label_name: label_name.ok_or(::serde::de::Error::missing_field("LabelName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.LoggingFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-loggingfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingFilter {
        /// Property [`DefaultBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-loggingfilter.html#cfn-wafv2-loggingconfiguration-loggingfilter-defaultbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_behavior: ::Value<String>,
        /// Property [`Filters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-loggingfilter.html#cfn-wafv2-loggingconfiguration-loggingfilter-filters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filters: ::ValueList<Filter>,
    }

    impl ::codec::SerializeValue for LoggingFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultBehavior", &self.default_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filters", &self.filters)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_behavior: Option<::Value<String>> = None;
                    let mut filters: Option<::ValueList<Filter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultBehavior" => {
                                default_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filters" => {
                                filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingFilter {
                        default_behavior: default_behavior.ok_or(::serde::de::Error::missing_field("DefaultBehavior"))?,
                        filters: filters.ok_or(::serde::de::Error::missing_field("Filters"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-matchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct MatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-matchpattern.html#cfn-wafv2-loggingconfiguration-matchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`IncludedPaths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-matchpattern.html#cfn-wafv2-loggingconfiguration-matchpattern-includedpaths).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_paths: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for MatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref included_paths) = self.included_paths {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedPaths", included_paths)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut included_paths: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedPaths" => {
                                included_paths = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MatchPattern {
                        all: all,
                        included_paths: included_paths,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::LoggingConfiguration.SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-singleheader.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-singleheader.html#cfn-wafv2-loggingconfiguration-singleheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleHeader")
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

                    Ok(SingleHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rule_group {
    //! Property types for the `RuleGroup` resource.

    /// The [`AWS::WAFv2::RuleGroup.AllowAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-allowaction.html) property type.
    #[derive(Debug, Default)]
    pub struct AllowAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-allowaction.html#cfn-wafv2-rulegroup-allowaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for AllowAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AllowAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AllowAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AllowAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AllowAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AllowAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.AndStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-andstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct AndStatement {
        /// Property [`Statements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-andstatement.html#cfn-wafv2-rulegroup-andstatement-statements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statements: ::ValueList<Statement>,
    }

    impl ::codec::SerializeValue for AndStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statements", &self.statements)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AndStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AndStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AndStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AndStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statements: Option<::ValueList<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statements" => {
                                statements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AndStatement {
                        statements: statements.ok_or(::serde::de::Error::missing_field("Statements"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.BlockAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-blockaction.html) property type.
    #[derive(Debug, Default)]
    pub struct BlockAction {
        /// Property [`CustomResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-blockaction.html#cfn-wafv2-rulegroup-blockaction-customresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_response: Option<::Value<CustomResponse>>,
    }

    impl ::codec::SerializeValue for BlockAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_response) = self.custom_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponse", custom_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_response: Option<::Value<CustomResponse>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomResponse" => {
                                custom_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockAction {
                        custom_response: custom_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-body.html) property type.
    #[derive(Debug, Default)]
    pub struct Body {
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-body.html#cfn-wafv2-rulegroup-body-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Body {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref oversize_handling) = self.oversize_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", oversize_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Body {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Body, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Body;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Body")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Body {
                        oversize_handling: oversize_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.ByteMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct ByteMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html#cfn-wafv2-rulegroup-bytematchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`PositionalConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html#cfn-wafv2-rulegroup-bytematchstatement-positionalconstraint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub positional_constraint: ::Value<String>,
        /// Property [`SearchString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html#cfn-wafv2-rulegroup-bytematchstatement-searchstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search_string: Option<::Value<String>>,
        /// Property [`SearchStringBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html#cfn-wafv2-rulegroup-bytematchstatement-searchstringbase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search_string_base64: Option<::Value<String>>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html#cfn-wafv2-rulegroup-bytematchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for ByteMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PositionalConstraint", &self.positional_constraint)?;
            if let Some(ref search_string) = self.search_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchString", search_string)?;
            }
            if let Some(ref search_string_base64) = self.search_string_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchStringBase64", search_string_base64)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ByteMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ByteMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ByteMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ByteMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut positional_constraint: Option<::Value<String>> = None;
                    let mut search_string: Option<::Value<String>> = None;
                    let mut search_string_base64: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PositionalConstraint" => {
                                positional_constraint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SearchString" => {
                                search_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SearchStringBase64" => {
                                search_string_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ByteMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        positional_constraint: positional_constraint.ok_or(::serde::de::Error::missing_field("PositionalConstraint"))?,
                        search_string: search_string,
                        search_string_base64: search_string_base64,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CaptchaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptchaAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaaction.html#cfn-wafv2-rulegroup-captchaaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for CaptchaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptchaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptchaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptchaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptchaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptchaAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CaptchaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptchaConfig {
        /// Property [`ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaconfig.html#cfn-wafv2-rulegroup-captchaconfig-immunitytimeproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time_property: Option<::Value<ImmunityTimeProperty>>,
    }

    impl ::codec::SerializeValue for CaptchaConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref immunity_time_property) = self.immunity_time_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTimeProperty", immunity_time_property)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptchaConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptchaConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptchaConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptchaConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time_property: Option<::Value<ImmunityTimeProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTimeProperty" => {
                                immunity_time_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptchaConfig {
                        immunity_time_property: immunity_time_property,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.ChallengeAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeaction.html) property type.
    #[derive(Debug, Default)]
    pub struct ChallengeAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeaction.html#cfn-wafv2-rulegroup-challengeaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for ChallengeAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChallengeAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChallengeAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChallengeAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChallengeAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChallengeAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.ChallengeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ChallengeConfig {
        /// Property [`ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeconfig.html#cfn-wafv2-rulegroup-challengeconfig-immunitytimeproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time_property: Option<::Value<ImmunityTimeProperty>>,
    }

    impl ::codec::SerializeValue for ChallengeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref immunity_time_property) = self.immunity_time_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTimeProperty", immunity_time_property)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChallengeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChallengeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChallengeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChallengeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time_property: Option<::Value<ImmunityTimeProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTimeProperty" => {
                                immunity_time_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChallengeConfig {
                        immunity_time_property: immunity_time_property,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CookieMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookiematchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct CookieMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookiematchpattern.html#cfn-wafv2-rulegroup-cookiematchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`ExcludedCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookiematchpattern.html#cfn-wafv2-rulegroup-cookiematchpattern-excludedcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_cookies: Option<::ValueList<String>>,
        /// Property [`IncludedCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookiematchpattern.html#cfn-wafv2-rulegroup-cookiematchpattern-includedcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_cookies: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CookieMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref excluded_cookies) = self.excluded_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedCookies", excluded_cookies)?;
            }
            if let Some(ref included_cookies) = self.included_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedCookies", included_cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookieMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookieMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookieMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookieMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut excluded_cookies: Option<::ValueList<String>> = None;
                    let mut included_cookies: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedCookies" => {
                                excluded_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedCookies" => {
                                included_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookieMatchPattern {
                        all: all,
                        excluded_cookies: excluded_cookies,
                        included_cookies: included_cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookies.html) property type.
    #[derive(Debug, Default)]
    pub struct Cookies {
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookies.html#cfn-wafv2-rulegroup-cookies-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<CookieMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookies.html#cfn-wafv2-rulegroup-cookies-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookies.html#cfn-wafv2-rulegroup-cookies-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: ::Value<String>,
    }

    impl ::codec::SerializeValue for Cookies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", &self.oversize_handling)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Cookies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Cookies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Cookies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Cookies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut match_pattern: Option<::Value<CookieMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Cookies {
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling.ok_or(::serde::de::Error::missing_field("OversizeHandling"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CountAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-countaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CountAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-countaction.html#cfn-wafv2-rulegroup-countaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for CountAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CountAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CountAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CountAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CountAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CountAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CustomHTTPHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customhttpheader.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomHTTPHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customhttpheader.html#cfn-wafv2-rulegroup-customhttpheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customhttpheader.html#cfn-wafv2-rulegroup-customhttpheader-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomHTTPHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomHTTPHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomHTTPHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomHTTPHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomHTTPHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomHTTPHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customrequesthandling.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomRequestHandling {
        /// Property [`InsertHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customrequesthandling.html#cfn-wafv2-rulegroup-customrequesthandling-insertheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insert_headers: ::ValueList<CustomHTTPHeader>,
    }

    impl ::codec::SerializeValue for CustomRequestHandling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsertHeaders", &self.insert_headers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomRequestHandling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomRequestHandling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomRequestHandling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomRequestHandling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut insert_headers: Option<::ValueList<CustomHTTPHeader>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InsertHeaders" => {
                                insert_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomRequestHandling {
                        insert_headers: insert_headers.ok_or(::serde::de::Error::missing_field("InsertHeaders"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CustomResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponse.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomResponse {
        /// Property [`CustomResponseBodyKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponse.html#cfn-wafv2-rulegroup-customresponse-customresponsebodykey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_response_body_key: Option<::Value<String>>,
        /// Property [`ResponseCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponse.html#cfn-wafv2-rulegroup-customresponse-responsecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_code: ::Value<u32>,
        /// Property [`ResponseHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponse.html#cfn-wafv2-rulegroup-customresponse-responseheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_headers: Option<::ValueList<CustomHTTPHeader>>,
    }

    impl ::codec::SerializeValue for CustomResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_response_body_key) = self.custom_response_body_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponseBodyKey", custom_response_body_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseCode", &self.response_code)?;
            if let Some(ref response_headers) = self.response_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseHeaders", response_headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_response_body_key: Option<::Value<String>> = None;
                    let mut response_code: Option<::Value<u32>> = None;
                    let mut response_headers: Option<::ValueList<CustomHTTPHeader>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomResponseBodyKey" => {
                                custom_response_body_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseCode" => {
                                response_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseHeaders" => {
                                response_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomResponse {
                        custom_response_body_key: custom_response_body_key,
                        response_code: response_code.ok_or(::serde::de::Error::missing_field("ResponseCode"))?,
                        response_headers: response_headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.CustomResponseBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponsebody.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomResponseBody {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponsebody.html#cfn-wafv2-rulegroup-customresponsebody-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: ::Value<String>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponsebody.html#cfn-wafv2-rulegroup-customresponsebody-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomResponseBody {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", &self.content_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomResponseBody {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResponseBody, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomResponseBody;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomResponseBody")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomResponseBody {
                        content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                        content_type: content_type.ok_or(::serde::de::Error::missing_field("ContentType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`AllQueryArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-allqueryarguments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all_query_arguments: Option<::Value<::json::Value>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<Body>>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<::Value<Cookies>>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::Value<Headers>>,
        /// Property [`JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-jsonbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_body: Option<::Value<JsonBody>>,
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-method).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method: Option<::Value<::json::Value>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: Option<::Value<::json::Value>>,
        /// Property [`SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-singleheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_header: Option<::Value<SingleHeader>>,
        /// Property [`SingleQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-singlequeryargument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_query_argument: Option<::Value<SingleQueryArgument>>,
        /// Property [`UriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html#cfn-wafv2-rulegroup-fieldtomatch-uripath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri_path: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_query_arguments) = self.all_query_arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllQueryArguments", all_query_arguments)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref json_body) = self.json_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonBody", json_body)?;
            }
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref query_string) = self.query_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
            }
            if let Some(ref single_header) = self.single_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleHeader", single_header)?;
            }
            if let Some(ref single_query_argument) = self.single_query_argument {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleQueryArgument", single_query_argument)?;
            }
            if let Some(ref uri_path) = self.uri_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPath", uri_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_query_arguments: Option<::Value<::json::Value>> = None;
                    let mut body: Option<::Value<Body>> = None;
                    let mut cookies: Option<::Value<Cookies>> = None;
                    let mut headers: Option<::Value<Headers>> = None;
                    let mut json_body: Option<::Value<JsonBody>> = None;
                    let mut method: Option<::Value<::json::Value>> = None;
                    let mut query_string: Option<::Value<::json::Value>> = None;
                    let mut single_header: Option<::Value<SingleHeader>> = None;
                    let mut single_query_argument: Option<::Value<SingleQueryArgument>> = None;
                    let mut uri_path: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllQueryArguments" => {
                                all_query_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JsonBody" => {
                                json_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleHeader" => {
                                single_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleQueryArgument" => {
                                single_query_argument = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UriPath" => {
                                uri_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        all_query_arguments: all_query_arguments,
                        body: body,
                        cookies: cookies,
                        headers: headers,
                        json_body: json_body,
                        method: method,
                        query_string: query_string,
                        single_header: single_header,
                        single_query_argument: single_query_argument,
                        uri_path: uri_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.ForwardedIPConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-forwardedipconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardedIPConfiguration {
        /// Property [`FallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-forwardedipconfiguration.html#cfn-wafv2-rulegroup-forwardedipconfiguration-fallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fallback_behavior: ::Value<String>,
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-forwardedipconfiguration.html#cfn-wafv2-rulegroup-forwardedipconfiguration-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ForwardedIPConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FallbackBehavior", &self.fallback_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ForwardedIPConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardedIPConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardedIPConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardedIPConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fallback_behavior: Option<::Value<String>> = None;
                    let mut header_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FallbackBehavior" => {
                                fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardedIPConfiguration {
                        fallback_behavior: fallback_behavior.ok_or(::serde::de::Error::missing_field("FallbackBehavior"))?,
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.GeoMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-geomatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoMatchStatement {
        /// Property [`CountryCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-geomatchstatement.html#cfn-wafv2-rulegroup-geomatchstatement-countrycodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country_codes: Option<::ValueList<String>>,
        /// Property [`ForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-geomatchstatement.html#cfn-wafv2-rulegroup-geomatchstatement-forwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>>,
    }

    impl ::codec::SerializeValue for GeoMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref country_codes) = self.country_codes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCodes", country_codes)?;
            }
            if let Some(ref forwarded_ip_config) = self.forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIPConfig", forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut country_codes: Option<::ValueList<String>> = None;
                    let mut forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CountryCodes" => {
                                country_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIPConfig" => {
                                forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoMatchStatement {
                        country_codes: country_codes,
                        forwarded_ip_config: forwarded_ip_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.HeaderMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headermatchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct HeaderMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headermatchpattern.html#cfn-wafv2-rulegroup-headermatchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`ExcludedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headermatchpattern.html#cfn-wafv2-rulegroup-headermatchpattern-excludedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_headers: Option<::ValueList<String>>,
        /// Property [`IncludedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headermatchpattern.html#cfn-wafv2-rulegroup-headermatchpattern-includedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_headers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HeaderMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref excluded_headers) = self.excluded_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedHeaders", excluded_headers)?;
            }
            if let Some(ref included_headers) = self.included_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedHeaders", included_headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeaderMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeaderMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeaderMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeaderMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut excluded_headers: Option<::ValueList<String>> = None;
                    let mut included_headers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedHeaders" => {
                                excluded_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedHeaders" => {
                                included_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeaderMatchPattern {
                        all: all,
                        excluded_headers: excluded_headers,
                        included_headers: included_headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headers.html) property type.
    #[derive(Debug, Default)]
    pub struct Headers {
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headers.html#cfn-wafv2-rulegroup-headers-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<HeaderMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headers.html#cfn-wafv2-rulegroup-headers-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headers.html#cfn-wafv2-rulegroup-headers-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: ::Value<String>,
    }

    impl ::codec::SerializeValue for Headers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", &self.oversize_handling)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Headers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Headers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Headers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Headers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut match_pattern: Option<::Value<HeaderMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Headers {
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling.ok_or(::serde::de::Error::missing_field("OversizeHandling"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.IPSetForwardedIPConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetforwardedipconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetForwardedIPConfiguration {
        /// Property [`FallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetforwardedipconfiguration.html#cfn-wafv2-rulegroup-ipsetforwardedipconfiguration-fallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fallback_behavior: ::Value<String>,
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetforwardedipconfiguration.html#cfn-wafv2-rulegroup-ipsetforwardedipconfiguration-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetforwardedipconfiguration.html#cfn-wafv2-rulegroup-ipsetforwardedipconfiguration-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: ::Value<String>,
    }

    impl ::codec::SerializeValue for IPSetForwardedIPConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FallbackBehavior", &self.fallback_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", &self.position)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetForwardedIPConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetForwardedIPConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetForwardedIPConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetForwardedIPConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fallback_behavior: Option<::Value<String>> = None;
                    let mut header_name: Option<::Value<String>> = None;
                    let mut position: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FallbackBehavior" => {
                                fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetForwardedIPConfiguration {
                        fallback_behavior: fallback_behavior.ok_or(::serde::de::Error::missing_field("FallbackBehavior"))?,
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        position: position.ok_or(::serde::de::Error::missing_field("Position"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.IPSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetreferencestatement.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetReferenceStatement {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetreferencestatement.html#cfn-wafv2-rulegroup-ipsetreferencestatement-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`IPSetForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetreferencestatement.html#cfn-wafv2-rulegroup-ipsetreferencestatement-ipsetforwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_set_forwarded_ip_config: Option<::Value<IPSetForwardedIPConfiguration>>,
    }

    impl ::codec::SerializeValue for IPSetReferenceStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref ip_set_forwarded_ip_config) = self.ip_set_forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetForwardedIPConfig", ip_set_forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetReferenceStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetReferenceStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetReferenceStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetReferenceStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut ip_set_forwarded_ip_config: Option<::Value<IPSetForwardedIPConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPSetForwardedIPConfig" => {
                                ip_set_forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetReferenceStatement {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        ip_set_forwarded_ip_config: ip_set_forwarded_ip_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-immunitytimeproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct ImmunityTimeProperty {
        /// Property [`ImmunityTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-immunitytimeproperty.html#cfn-wafv2-rulegroup-immunitytimeproperty-immunitytime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ImmunityTimeProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTime", &self.immunity_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImmunityTimeProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImmunityTimeProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImmunityTimeProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImmunityTimeProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTime" => {
                                immunity_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImmunityTimeProperty {
                        immunity_time: immunity_time.ok_or(::serde::de::Error::missing_field("ImmunityTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonBody {
        /// Property [`InvalidFallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html#cfn-wafv2-rulegroup-jsonbody-invalidfallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invalid_fallback_behavior: Option<::Value<String>>,
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html#cfn-wafv2-rulegroup-jsonbody-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<JsonMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html#cfn-wafv2-rulegroup-jsonbody-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html#cfn-wafv2-rulegroup-jsonbody-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JsonBody {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invalid_fallback_behavior) = self.invalid_fallback_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvalidFallbackBehavior", invalid_fallback_behavior)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            if let Some(ref oversize_handling) = self.oversize_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", oversize_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonBody {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonBody, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonBody;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonBody")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invalid_fallback_behavior: Option<::Value<String>> = None;
                    let mut match_pattern: Option<::Value<JsonMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvalidFallbackBehavior" => {
                                invalid_fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonBody {
                        invalid_fallback_behavior: invalid_fallback_behavior,
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.JsonMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonmatchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonmatchpattern.html#cfn-wafv2-rulegroup-jsonmatchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`IncludedPaths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonmatchpattern.html#cfn-wafv2-rulegroup-jsonmatchpattern-includedpaths).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_paths: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for JsonMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref included_paths) = self.included_paths {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedPaths", included_paths)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut included_paths: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedPaths" => {
                                included_paths = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonMatchPattern {
                        all: all,
                        included_paths: included_paths,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-label.html) property type.
    #[derive(Debug, Default)]
    pub struct Label {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-label.html#cfn-wafv2-rulegroup-label-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Label {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Label {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.LabelMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct LabelMatchStatement {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelmatchstatement.html#cfn-wafv2-rulegroup-labelmatchstatement-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelmatchstatement.html#cfn-wafv2-rulegroup-labelmatchstatement-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: ::Value<String>,
    }

    impl ::codec::SerializeValue for LabelMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LabelMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LabelMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LabelMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LabelMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LabelMatchStatement {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.LabelSummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelsummary.html) property type.
    #[derive(Debug, Default)]
    pub struct LabelSummary {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelsummary.html#cfn-wafv2-rulegroup-labelsummary-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LabelSummary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LabelSummary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LabelSummary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LabelSummary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LabelSummary")
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

                    Ok(LabelSummary {
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.NotStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-notstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct NotStatement {
        /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-notstatement.html#cfn-wafv2-rulegroup-notstatement-statement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement: ::Value<Statement>,
    }

    impl ::codec::SerializeValue for NotStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statement: Option<::Value<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statement" => {
                                statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotStatement {
                        statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.OrStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-orstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct OrStatement {
        /// Property [`Statements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-orstatement.html#cfn-wafv2-rulegroup-orstatement-statements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statements: ::ValueList<Statement>,
    }

    impl ::codec::SerializeValue for OrStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statements", &self.statements)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statements: Option<::ValueList<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statements" => {
                                statements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrStatement {
                        statements: statements.ok_or(::serde::de::Error::missing_field("Statements"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateBasedStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RateBasedStatement {
        /// Property [`AggregateKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html#cfn-wafv2-rulegroup-ratebasedstatement-aggregatekeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregate_key_type: ::Value<String>,
        /// Property [`CustomKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html#cfn-wafv2-rulegroup-ratebasedstatement-customkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_keys: Option<::ValueList<RateBasedStatementCustomKey>>,
        /// Property [`ForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html#cfn-wafv2-rulegroup-ratebasedstatement-forwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>>,
        /// Property [`Limit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html#cfn-wafv2-rulegroup-ratebasedstatement-limit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub limit: ::Value<u32>,
        /// Property [`ScopeDownStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html#cfn-wafv2-rulegroup-ratebasedstatement-scopedownstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope_down_statement: Option<::Value<Statement>>,
    }

    impl ::codec::SerializeValue for RateBasedStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregateKeyType", &self.aggregate_key_type)?;
            if let Some(ref custom_keys) = self.custom_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomKeys", custom_keys)?;
            }
            if let Some(ref forwarded_ip_config) = self.forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIPConfig", forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limit", &self.limit)?;
            if let Some(ref scope_down_statement) = self.scope_down_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScopeDownStatement", scope_down_statement)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateBasedStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateBasedStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateBasedStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateBasedStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregate_key_type: Option<::Value<String>> = None;
                    let mut custom_keys: Option<::ValueList<RateBasedStatementCustomKey>> = None;
                    let mut forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>> = None;
                    let mut limit: Option<::Value<u32>> = None;
                    let mut scope_down_statement: Option<::Value<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregateKeyType" => {
                                aggregate_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomKeys" => {
                                custom_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIPConfig" => {
                                forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Limit" => {
                                limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScopeDownStatement" => {
                                scope_down_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateBasedStatement {
                        aggregate_key_type: aggregate_key_type.ok_or(::serde::de::Error::missing_field("AggregateKeyType"))?,
                        custom_keys: custom_keys,
                        forwarded_ip_config: forwarded_ip_config,
                        limit: limit.ok_or(::serde::de::Error::missing_field("Limit"))?,
                        scope_down_statement: scope_down_statement,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateBasedStatementCustomKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html) property type.
    #[derive(Debug, Default)]
    pub struct RateBasedStatementCustomKey {
        /// Property [`Cookie`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-cookie).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie: Option<::Value<RateLimitCookie>>,
        /// Property [`ForwardedIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-forwardedip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip: Option<::Value<::json::Value>>,
        /// Property [`HTTPMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<::json::Value>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::Value<RateLimitHeader>>,
        /// Property [`IP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-ip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip: Option<::Value<::json::Value>>,
        /// Property [`LabelNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-labelnamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_namespace: Option<::Value<RateLimitLabelNamespace>>,
        /// Property [`QueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-queryargument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_argument: Option<::Value<RateLimitQueryArgument>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: Option<::Value<RateLimitQueryString>>,
        /// Property [`UriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html#cfn-wafv2-rulegroup-ratebasedstatementcustomkey-uripath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri_path: Option<::Value<RateLimitUriPath>>,
    }

    impl ::codec::SerializeValue for RateBasedStatementCustomKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookie) = self.cookie {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookie", cookie)?;
            }
            if let Some(ref forwarded_ip) = self.forwarded_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIP", forwarded_ip)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPMethod", http_method)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref ip) = self.ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IP", ip)?;
            }
            if let Some(ref label_namespace) = self.label_namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelNamespace", label_namespace)?;
            }
            if let Some(ref query_argument) = self.query_argument {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryArgument", query_argument)?;
            }
            if let Some(ref query_string) = self.query_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
            }
            if let Some(ref uri_path) = self.uri_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPath", uri_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateBasedStatementCustomKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateBasedStatementCustomKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateBasedStatementCustomKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateBasedStatementCustomKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie: Option<::Value<RateLimitCookie>> = None;
                    let mut forwarded_ip: Option<::Value<::json::Value>> = None;
                    let mut http_method: Option<::Value<::json::Value>> = None;
                    let mut header: Option<::Value<RateLimitHeader>> = None;
                    let mut ip: Option<::Value<::json::Value>> = None;
                    let mut label_namespace: Option<::Value<RateLimitLabelNamespace>> = None;
                    let mut query_argument: Option<::Value<RateLimitQueryArgument>> = None;
                    let mut query_string: Option<::Value<RateLimitQueryString>> = None;
                    let mut uri_path: Option<::Value<RateLimitUriPath>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cookie" => {
                                cookie = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIP" => {
                                forwarded_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IP" => {
                                ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelNamespace" => {
                                label_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryArgument" => {
                                query_argument = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UriPath" => {
                                uri_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateBasedStatementCustomKey {
                        cookie: cookie,
                        forwarded_ip: forwarded_ip,
                        http_method: http_method,
                        header: header,
                        ip: ip,
                        label_namespace: label_namespace,
                        query_argument: query_argument,
                        query_string: query_string,
                        uri_path: uri_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitCookie`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitcookie.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitCookie {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitcookie.html#cfn-wafv2-rulegroup-ratelimitcookie-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitcookie.html#cfn-wafv2-rulegroup-ratelimitcookie-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitCookie {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitCookie {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitCookie, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitCookie;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitCookie")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitCookie {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitheader.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitheader.html#cfn-wafv2-rulegroup-ratelimitheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitheader.html#cfn-wafv2-rulegroup-ratelimitheader-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitLabelNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitlabelnamespace.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitLabelNamespace {
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitlabelnamespace.html#cfn-wafv2-rulegroup-ratelimitlabelnamespace-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: ::Value<String>,
    }

    impl ::codec::SerializeValue for RateLimitLabelNamespace {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitLabelNamespace {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitLabelNamespace, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitLabelNamespace;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitLabelNamespace")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitLabelNamespace {
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitqueryargument.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitQueryArgument {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitqueryargument.html#cfn-wafv2-rulegroup-ratelimitqueryargument-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitqueryargument.html#cfn-wafv2-rulegroup-ratelimitqueryargument-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitQueryArgument {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitQueryArgument {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitQueryArgument, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitQueryArgument;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitQueryArgument")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitQueryArgument {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitQueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitquerystring.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitQueryString {
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitquerystring.html#cfn-wafv2-rulegroup-ratelimitquerystring-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitQueryString {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitQueryString {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitQueryString, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitQueryString;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitQueryString")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitQueryString {
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RateLimitUriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimituripath.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitUriPath {
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimituripath.html#cfn-wafv2-rulegroup-ratelimituripath-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitUriPath {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitUriPath {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitUriPath, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitUriPath;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitUriPath")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitUriPath {
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RegexMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RegexMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexmatchstatement.html#cfn-wafv2-rulegroup-regexmatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`RegexString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexmatchstatement.html#cfn-wafv2-rulegroup-regexmatchstatement-regexstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_string: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexmatchstatement.html#cfn-wafv2-rulegroup-regexmatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RegexMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexString", &self.regex_string)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegexMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegexMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegexMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegexMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut regex_string: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexString" => {
                                regex_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegexMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        regex_string: regex_string.ok_or(::serde::de::Error::missing_field("RegexString"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RegexPatternSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexpatternsetreferencestatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RegexPatternSetReferenceStatement {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexpatternsetreferencestatement.html#cfn-wafv2-rulegroup-regexpatternsetreferencestatement-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexpatternsetreferencestatement.html#cfn-wafv2-rulegroup-regexpatternsetreferencestatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexpatternsetreferencestatement.html#cfn-wafv2-rulegroup-regexpatternsetreferencestatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RegexPatternSetReferenceStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegexPatternSetReferenceStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegexPatternSetReferenceStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegexPatternSetReferenceStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegexPatternSetReferenceStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegexPatternSetReferenceStatement {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<RuleAction>>,
        /// Property [`CaptchaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-captchaconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub captcha_config: Option<::Value<CaptchaConfig>>,
        /// Property [`ChallengeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-challengeconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub challenge_config: Option<::Value<ChallengeConfig>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`RuleLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-rulelabels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_labels: Option<::ValueList<Label>>,
        /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-statement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement: ::Value<Statement>,
        /// Property [`VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html#cfn-wafv2-rulegroup-rule-visibilityconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub visibility_config: ::Value<VisibilityConfig>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref captcha_config) = self.captcha_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptchaConfig", captcha_config)?;
            }
            if let Some(ref challenge_config) = self.challenge_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChallengeConfig", challenge_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            if let Some(ref rule_labels) = self.rule_labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleLabels", rule_labels)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityConfig", &self.visibility_config)?;
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
                    let mut action: Option<::Value<RuleAction>> = None;
                    let mut captcha_config: Option<::Value<CaptchaConfig>> = None;
                    let mut challenge_config: Option<::Value<ChallengeConfig>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut rule_labels: Option<::ValueList<Label>> = None;
                    let mut statement: Option<::Value<Statement>> = None;
                    let mut visibility_config: Option<::Value<VisibilityConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptchaConfig" => {
                                captcha_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChallengeConfig" => {
                                challenge_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleLabels" => {
                                rule_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statement" => {
                                statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VisibilityConfig" => {
                                visibility_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        action: action,
                        captcha_config: captcha_config,
                        challenge_config: challenge_config,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        rule_labels: rule_labels,
                        statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                        visibility_config: visibility_config.ok_or(::serde::de::Error::missing_field("VisibilityConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.RuleAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleAction {
        /// Property [`Allow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html#cfn-wafv2-rulegroup-ruleaction-allow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow: Option<::Value<AllowAction>>,
        /// Property [`Block`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html#cfn-wafv2-rulegroup-ruleaction-block).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block: Option<::Value<BlockAction>>,
        /// Property [`Captcha`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html#cfn-wafv2-rulegroup-ruleaction-captcha).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub captcha: Option<::Value<CaptchaAction>>,
        /// Property [`Challenge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html#cfn-wafv2-rulegroup-ruleaction-challenge).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub challenge: Option<::Value<ChallengeAction>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html#cfn-wafv2-rulegroup-ruleaction-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<CountAction>>,
    }

    impl ::codec::SerializeValue for RuleAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow) = self.allow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Allow", allow)?;
            }
            if let Some(ref block) = self.block {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Block", block)?;
            }
            if let Some(ref captcha) = self.captcha {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Captcha", captcha)?;
            }
            if let Some(ref challenge) = self.challenge {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Challenge", challenge)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow: Option<::Value<AllowAction>> = None;
                    let mut block: Option<::Value<BlockAction>> = None;
                    let mut captcha: Option<::Value<CaptchaAction>> = None;
                    let mut challenge: Option<::Value<ChallengeAction>> = None;
                    let mut count: Option<::Value<CountAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Allow" => {
                                allow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Block" => {
                                block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Captcha" => {
                                captcha = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Challenge" => {
                                challenge = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleAction {
                        allow: allow,
                        block: block,
                        captcha: captcha,
                        challenge: challenge,
                        count: count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singleheader.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singleheader.html#cfn-wafv2-rulegroup-singleheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleHeader")
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

                    Ok(SingleHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.SingleQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singlequeryargument.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleQueryArgument {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singlequeryargument.html#cfn-wafv2-rulegroup-singlequeryargument-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleQueryArgument {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleQueryArgument {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleQueryArgument, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleQueryArgument;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleQueryArgument")
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

                    Ok(SingleQueryArgument {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.SizeConstraintStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct SizeConstraintStatement {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html#cfn-wafv2-rulegroup-sizeconstraintstatement-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html#cfn-wafv2-rulegroup-sizeconstraintstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html#cfn-wafv2-rulegroup-sizeconstraintstatement-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: ::Value<f64>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html#cfn-wafv2-rulegroup-sizeconstraintstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for SizeConstraintStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SizeConstraintStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SizeConstraintStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SizeConstraintStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SizeConstraintStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut size: Option<::Value<f64>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SizeConstraintStatement {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.SqliMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sqlimatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct SqliMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sqlimatchstatement.html#cfn-wafv2-rulegroup-sqlimatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`SensitivityLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sqlimatchstatement.html#cfn-wafv2-rulegroup-sqlimatchstatement-sensitivitylevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sensitivity_level: Option<::Value<String>>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sqlimatchstatement.html#cfn-wafv2-rulegroup-sqlimatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for SqliMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            if let Some(ref sensitivity_level) = self.sensitivity_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SensitivityLevel", sensitivity_level)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqliMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqliMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqliMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqliMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut sensitivity_level: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SensitivityLevel" => {
                                sensitivity_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqliMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        sensitivity_level: sensitivity_level,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html) property type.
    #[derive(Debug, Default)]
    pub struct Statement {
        /// Property [`AndStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-andstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub and_statement: Option<::Value<AndStatement>>,
        /// Property [`ByteMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-bytematchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub byte_match_statement: Option<::Value<ByteMatchStatement>>,
        /// Property [`GeoMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-geomatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub geo_match_statement: Option<::Value<GeoMatchStatement>>,
        /// Property [`IPSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-ipsetreferencestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_set_reference_statement: Option<::Value<IPSetReferenceStatement>>,
        /// Property [`LabelMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-labelmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_match_statement: Option<::Value<LabelMatchStatement>>,
        /// Property [`NotStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-notstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_statement: Option<::Value<NotStatement>>,
        /// Property [`OrStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-orstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub or_statement: Option<::Value<OrStatement>>,
        /// Property [`RateBasedStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-ratebasedstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_based_statement: Option<::Value<RateBasedStatement>>,
        /// Property [`RegexMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-regexmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_match_statement: Option<::Value<RegexMatchStatement>>,
        /// Property [`RegexPatternSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-regexpatternsetreferencestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_pattern_set_reference_statement: Option<::Value<RegexPatternSetReferenceStatement>>,
        /// Property [`SizeConstraintStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-sizeconstraintstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_constraint_statement: Option<::Value<SizeConstraintStatement>>,
        /// Property [`SqliMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-sqlimatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqli_match_statement: Option<::Value<SqliMatchStatement>>,
        /// Property [`XssMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html#cfn-wafv2-rulegroup-statement-xssmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub xss_match_statement: Option<::Value<XssMatchStatement>>,
    }

    impl ::codec::SerializeValue for Statement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref and_statement) = self.and_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AndStatement", and_statement)?;
            }
            if let Some(ref byte_match_statement) = self.byte_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ByteMatchStatement", byte_match_statement)?;
            }
            if let Some(ref geo_match_statement) = self.geo_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoMatchStatement", geo_match_statement)?;
            }
            if let Some(ref ip_set_reference_statement) = self.ip_set_reference_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetReferenceStatement", ip_set_reference_statement)?;
            }
            if let Some(ref label_match_statement) = self.label_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelMatchStatement", label_match_statement)?;
            }
            if let Some(ref not_statement) = self.not_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotStatement", not_statement)?;
            }
            if let Some(ref or_statement) = self.or_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrStatement", or_statement)?;
            }
            if let Some(ref rate_based_statement) = self.rate_based_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateBasedStatement", rate_based_statement)?;
            }
            if let Some(ref regex_match_statement) = self.regex_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexMatchStatement", regex_match_statement)?;
            }
            if let Some(ref regex_pattern_set_reference_statement) = self.regex_pattern_set_reference_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexPatternSetReferenceStatement", regex_pattern_set_reference_statement)?;
            }
            if let Some(ref size_constraint_statement) = self.size_constraint_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeConstraintStatement", size_constraint_statement)?;
            }
            if let Some(ref sqli_match_statement) = self.sqli_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqliMatchStatement", sqli_match_statement)?;
            }
            if let Some(ref xss_match_statement) = self.xss_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "XssMatchStatement", xss_match_statement)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Statement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Statement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Statement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Statement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut and_statement: Option<::Value<AndStatement>> = None;
                    let mut byte_match_statement: Option<::Value<ByteMatchStatement>> = None;
                    let mut geo_match_statement: Option<::Value<GeoMatchStatement>> = None;
                    let mut ip_set_reference_statement: Option<::Value<IPSetReferenceStatement>> = None;
                    let mut label_match_statement: Option<::Value<LabelMatchStatement>> = None;
                    let mut not_statement: Option<::Value<NotStatement>> = None;
                    let mut or_statement: Option<::Value<OrStatement>> = None;
                    let mut rate_based_statement: Option<::Value<RateBasedStatement>> = None;
                    let mut regex_match_statement: Option<::Value<RegexMatchStatement>> = None;
                    let mut regex_pattern_set_reference_statement: Option<::Value<RegexPatternSetReferenceStatement>> = None;
                    let mut size_constraint_statement: Option<::Value<SizeConstraintStatement>> = None;
                    let mut sqli_match_statement: Option<::Value<SqliMatchStatement>> = None;
                    let mut xss_match_statement: Option<::Value<XssMatchStatement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AndStatement" => {
                                and_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ByteMatchStatement" => {
                                byte_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeoMatchStatement" => {
                                geo_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPSetReferenceStatement" => {
                                ip_set_reference_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelMatchStatement" => {
                                label_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotStatement" => {
                                not_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrStatement" => {
                                or_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateBasedStatement" => {
                                rate_based_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexMatchStatement" => {
                                regex_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexPatternSetReferenceStatement" => {
                                regex_pattern_set_reference_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeConstraintStatement" => {
                                size_constraint_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqliMatchStatement" => {
                                sqli_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "XssMatchStatement" => {
                                xss_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Statement {
                        and_statement: and_statement,
                        byte_match_statement: byte_match_statement,
                        geo_match_statement: geo_match_statement,
                        ip_set_reference_statement: ip_set_reference_statement,
                        label_match_statement: label_match_statement,
                        not_statement: not_statement,
                        or_statement: or_statement,
                        rate_based_statement: rate_based_statement,
                        regex_match_statement: regex_match_statement,
                        regex_pattern_set_reference_statement: regex_pattern_set_reference_statement,
                        size_constraint_statement: size_constraint_statement,
                        sqli_match_statement: sqli_match_statement,
                        xss_match_statement: xss_match_statement,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-texttransformation.html) property type.
    #[derive(Debug, Default)]
    pub struct TextTransformation {
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-texttransformation.html#cfn-wafv2-rulegroup-texttransformation-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-texttransformation.html#cfn-wafv2-rulegroup-texttransformation-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TextTransformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextTransformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextTransformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextTransformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextTransformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut priority: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextTransformation {
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-visibilityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VisibilityConfig {
        /// Property [`CloudWatchMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-visibilityconfig.html#cfn-wafv2-rulegroup-visibilityconfig-cloudwatchmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_metrics_enabled: ::Value<bool>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-visibilityconfig.html#cfn-wafv2-rulegroup-visibilityconfig-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`SampledRequestsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-visibilityconfig.html#cfn-wafv2-rulegroup-visibilityconfig-sampledrequestsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sampled_requests_enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for VisibilityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchMetricsEnabled", &self.cloud_watch_metrics_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampledRequestsEnabled", &self.sampled_requests_enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VisibilityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VisibilityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VisibilityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VisibilityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_metrics_enabled: Option<::Value<bool>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut sampled_requests_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchMetricsEnabled" => {
                                cloud_watch_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampledRequestsEnabled" => {
                                sampled_requests_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VisibilityConfig {
                        cloud_watch_metrics_enabled: cloud_watch_metrics_enabled.ok_or(::serde::de::Error::missing_field("CloudWatchMetricsEnabled"))?,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        sampled_requests_enabled: sampled_requests_enabled.ok_or(::serde::de::Error::missing_field("SampledRequestsEnabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::RuleGroup.XssMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-xssmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct XssMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-xssmatchstatement.html#cfn-wafv2-rulegroup-xssmatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-xssmatchstatement.html#cfn-wafv2-rulegroup-xssmatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for XssMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for XssMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<XssMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = XssMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type XssMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(XssMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod web_acl {
    //! Property types for the `WebACL` resource.

    /// The [`AWS::WAFv2::WebACL.AWSManagedRulesACFPRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSManagedRulesACFPRuleSet {
        /// Property [`CreationPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html#cfn-wafv2-webacl-awsmanagedrulesacfpruleset-creationpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_path: ::Value<String>,
        /// Property [`EnableRegexInPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html#cfn-wafv2-webacl-awsmanagedrulesacfpruleset-enableregexinpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_regex_in_path: Option<::Value<bool>>,
        /// Property [`RegistrationPagePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html#cfn-wafv2-webacl-awsmanagedrulesacfpruleset-registrationpagepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registration_page_path: ::Value<String>,
        /// Property [`RequestInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html#cfn-wafv2-webacl-awsmanagedrulesacfpruleset-requestinspection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_inspection: ::Value<RequestInspectionACFP>,
        /// Property [`ResponseInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html#cfn-wafv2-webacl-awsmanagedrulesacfpruleset-responseinspection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_inspection: Option<::Value<ResponseInspection>>,
    }

    impl ::codec::SerializeValue for AWSManagedRulesACFPRuleSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationPath", &self.creation_path)?;
            if let Some(ref enable_regex_in_path) = self.enable_regex_in_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableRegexInPath", enable_regex_in_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistrationPagePath", &self.registration_page_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestInspection", &self.request_inspection)?;
            if let Some(ref response_inspection) = self.response_inspection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseInspection", response_inspection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSManagedRulesACFPRuleSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSManagedRulesACFPRuleSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSManagedRulesACFPRuleSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSManagedRulesACFPRuleSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut creation_path: Option<::Value<String>> = None;
                    let mut enable_regex_in_path: Option<::Value<bool>> = None;
                    let mut registration_page_path: Option<::Value<String>> = None;
                    let mut request_inspection: Option<::Value<RequestInspectionACFP>> = None;
                    let mut response_inspection: Option<::Value<ResponseInspection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreationPath" => {
                                creation_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableRegexInPath" => {
                                enable_regex_in_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistrationPagePath" => {
                                registration_page_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestInspection" => {
                                request_inspection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseInspection" => {
                                response_inspection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSManagedRulesACFPRuleSet {
                        creation_path: creation_path.ok_or(::serde::de::Error::missing_field("CreationPath"))?,
                        enable_regex_in_path: enable_regex_in_path,
                        registration_page_path: registration_page_path.ok_or(::serde::de::Error::missing_field("RegistrationPagePath"))?,
                        request_inspection: request_inspection.ok_or(::serde::de::Error::missing_field("RequestInspection"))?,
                        response_inspection: response_inspection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.AWSManagedRulesATPRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSManagedRulesATPRuleSet {
        /// Property [`EnableRegexInPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html#cfn-wafv2-webacl-awsmanagedrulesatpruleset-enableregexinpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_regex_in_path: Option<::Value<bool>>,
        /// Property [`LoginPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html#cfn-wafv2-webacl-awsmanagedrulesatpruleset-loginpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub login_path: ::Value<String>,
        /// Property [`RequestInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html#cfn-wafv2-webacl-awsmanagedrulesatpruleset-requestinspection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_inspection: Option<::Value<RequestInspection>>,
        /// Property [`ResponseInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html#cfn-wafv2-webacl-awsmanagedrulesatpruleset-responseinspection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_inspection: Option<::Value<ResponseInspection>>,
    }

    impl ::codec::SerializeValue for AWSManagedRulesATPRuleSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_regex_in_path) = self.enable_regex_in_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableRegexInPath", enable_regex_in_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoginPath", &self.login_path)?;
            if let Some(ref request_inspection) = self.request_inspection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestInspection", request_inspection)?;
            }
            if let Some(ref response_inspection) = self.response_inspection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseInspection", response_inspection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSManagedRulesATPRuleSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSManagedRulesATPRuleSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSManagedRulesATPRuleSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSManagedRulesATPRuleSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_regex_in_path: Option<::Value<bool>> = None;
                    let mut login_path: Option<::Value<String>> = None;
                    let mut request_inspection: Option<::Value<RequestInspection>> = None;
                    let mut response_inspection: Option<::Value<ResponseInspection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableRegexInPath" => {
                                enable_regex_in_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoginPath" => {
                                login_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestInspection" => {
                                request_inspection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseInspection" => {
                                response_inspection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSManagedRulesATPRuleSet {
                        enable_regex_in_path: enable_regex_in_path,
                        login_path: login_path.ok_or(::serde::de::Error::missing_field("LoginPath"))?,
                        request_inspection: request_inspection,
                        response_inspection: response_inspection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.AWSManagedRulesBotControlRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesbotcontrolruleset.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSManagedRulesBotControlRuleSet {
        /// Property [`EnableMachineLearning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesbotcontrolruleset.html#cfn-wafv2-webacl-awsmanagedrulesbotcontrolruleset-enablemachinelearning).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_machine_learning: Option<::Value<bool>>,
        /// Property [`InspectionLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesbotcontrolruleset.html#cfn-wafv2-webacl-awsmanagedrulesbotcontrolruleset-inspectionlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inspection_level: ::Value<String>,
    }

    impl ::codec::SerializeValue for AWSManagedRulesBotControlRuleSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_machine_learning) = self.enable_machine_learning {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableMachineLearning", enable_machine_learning)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InspectionLevel", &self.inspection_level)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSManagedRulesBotControlRuleSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSManagedRulesBotControlRuleSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSManagedRulesBotControlRuleSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSManagedRulesBotControlRuleSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_machine_learning: Option<::Value<bool>> = None;
                    let mut inspection_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableMachineLearning" => {
                                enable_machine_learning = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InspectionLevel" => {
                                inspection_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSManagedRulesBotControlRuleSet {
                        enable_machine_learning: enable_machine_learning,
                        inspection_level: inspection_level.ok_or(::serde::de::Error::missing_field("InspectionLevel"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.AllowAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-allowaction.html) property type.
    #[derive(Debug, Default)]
    pub struct AllowAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-allowaction.html#cfn-wafv2-webacl-allowaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for AllowAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AllowAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AllowAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AllowAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AllowAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AllowAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.AndStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-andstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct AndStatement {
        /// Property [`Statements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-andstatement.html#cfn-wafv2-webacl-andstatement-statements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statements: ::ValueList<Statement>,
    }

    impl ::codec::SerializeValue for AndStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statements", &self.statements)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AndStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AndStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AndStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AndStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statements: Option<::ValueList<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statements" => {
                                statements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AndStatement {
                        statements: statements.ok_or(::serde::de::Error::missing_field("Statements"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.AssociationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-associationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AssociationConfig {
        /// Property [`RequestBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-associationconfig.html#cfn-wafv2-webacl-associationconfig-requestbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_body: Option<::ValueMap<RequestBodyAssociatedResourceTypeConfig>>,
    }

    impl ::codec::SerializeValue for AssociationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref request_body) = self.request_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestBody", request_body)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssociationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssociationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssociationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssociationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut request_body: Option<::ValueMap<RequestBodyAssociatedResourceTypeConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RequestBody" => {
                                request_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssociationConfig {
                        request_body: request_body,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.BlockAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-blockaction.html) property type.
    #[derive(Debug, Default)]
    pub struct BlockAction {
        /// Property [`CustomResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-blockaction.html#cfn-wafv2-webacl-blockaction-customresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_response: Option<::Value<CustomResponse>>,
    }

    impl ::codec::SerializeValue for BlockAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_response) = self.custom_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponse", custom_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_response: Option<::Value<CustomResponse>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomResponse" => {
                                custom_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockAction {
                        custom_response: custom_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-body.html) property type.
    #[derive(Debug, Default)]
    pub struct Body {
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-body.html#cfn-wafv2-webacl-body-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Body {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref oversize_handling) = self.oversize_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", oversize_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Body {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Body, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Body;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Body")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Body {
                        oversize_handling: oversize_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ByteMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct ByteMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html#cfn-wafv2-webacl-bytematchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`PositionalConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html#cfn-wafv2-webacl-bytematchstatement-positionalconstraint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub positional_constraint: ::Value<String>,
        /// Property [`SearchString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html#cfn-wafv2-webacl-bytematchstatement-searchstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search_string: Option<::Value<String>>,
        /// Property [`SearchStringBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html#cfn-wafv2-webacl-bytematchstatement-searchstringbase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search_string_base64: Option<::Value<String>>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html#cfn-wafv2-webacl-bytematchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for ByteMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PositionalConstraint", &self.positional_constraint)?;
            if let Some(ref search_string) = self.search_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchString", search_string)?;
            }
            if let Some(ref search_string_base64) = self.search_string_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchStringBase64", search_string_base64)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ByteMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ByteMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ByteMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ByteMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut positional_constraint: Option<::Value<String>> = None;
                    let mut search_string: Option<::Value<String>> = None;
                    let mut search_string_base64: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PositionalConstraint" => {
                                positional_constraint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SearchString" => {
                                search_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SearchStringBase64" => {
                                search_string_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ByteMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        positional_constraint: positional_constraint.ok_or(::serde::de::Error::missing_field("PositionalConstraint"))?,
                        search_string: search_string,
                        search_string_base64: search_string_base64,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CaptchaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptchaAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaaction.html#cfn-wafv2-webacl-captchaaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for CaptchaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptchaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptchaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptchaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptchaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptchaAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CaptchaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptchaConfig {
        /// Property [`ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaconfig.html#cfn-wafv2-webacl-captchaconfig-immunitytimeproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time_property: Option<::Value<ImmunityTimeProperty>>,
    }

    impl ::codec::SerializeValue for CaptchaConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref immunity_time_property) = self.immunity_time_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTimeProperty", immunity_time_property)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptchaConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptchaConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptchaConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptchaConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time_property: Option<::Value<ImmunityTimeProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTimeProperty" => {
                                immunity_time_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptchaConfig {
                        immunity_time_property: immunity_time_property,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ChallengeAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeaction.html) property type.
    #[derive(Debug, Default)]
    pub struct ChallengeAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeaction.html#cfn-wafv2-webacl-challengeaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for ChallengeAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChallengeAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChallengeAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChallengeAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChallengeAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChallengeAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ChallengeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ChallengeConfig {
        /// Property [`ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeconfig.html#cfn-wafv2-webacl-challengeconfig-immunitytimeproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time_property: Option<::Value<ImmunityTimeProperty>>,
    }

    impl ::codec::SerializeValue for ChallengeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref immunity_time_property) = self.immunity_time_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTimeProperty", immunity_time_property)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChallengeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChallengeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChallengeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChallengeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time_property: Option<::Value<ImmunityTimeProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTimeProperty" => {
                                immunity_time_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ChallengeConfig {
                        immunity_time_property: immunity_time_property,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CookieMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookiematchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct CookieMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookiematchpattern.html#cfn-wafv2-webacl-cookiematchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`ExcludedCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookiematchpattern.html#cfn-wafv2-webacl-cookiematchpattern-excludedcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_cookies: Option<::ValueList<String>>,
        /// Property [`IncludedCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookiematchpattern.html#cfn-wafv2-webacl-cookiematchpattern-includedcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_cookies: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CookieMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref excluded_cookies) = self.excluded_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedCookies", excluded_cookies)?;
            }
            if let Some(ref included_cookies) = self.included_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedCookies", included_cookies)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookieMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookieMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookieMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookieMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut excluded_cookies: Option<::ValueList<String>> = None;
                    let mut included_cookies: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedCookies" => {
                                excluded_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedCookies" => {
                                included_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookieMatchPattern {
                        all: all,
                        excluded_cookies: excluded_cookies,
                        included_cookies: included_cookies,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookies.html) property type.
    #[derive(Debug, Default)]
    pub struct Cookies {
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookies.html#cfn-wafv2-webacl-cookies-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<CookieMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookies.html#cfn-wafv2-webacl-cookies-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookies.html#cfn-wafv2-webacl-cookies-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: ::Value<String>,
    }

    impl ::codec::SerializeValue for Cookies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", &self.oversize_handling)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Cookies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Cookies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Cookies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Cookies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut match_pattern: Option<::Value<CookieMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Cookies {
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling.ok_or(::serde::de::Error::missing_field("OversizeHandling"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CountAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-countaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CountAction {
        /// Property [`CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-countaction.html#cfn-wafv2-webacl-countaction-customrequesthandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_request_handling: Option<::Value<CustomRequestHandling>>,
    }

    impl ::codec::SerializeValue for CountAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_request_handling) = self.custom_request_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRequestHandling", custom_request_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CountAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CountAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CountAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CountAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_request_handling: Option<::Value<CustomRequestHandling>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRequestHandling" => {
                                custom_request_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CountAction {
                        custom_request_handling: custom_request_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CustomHTTPHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customhttpheader.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomHTTPHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customhttpheader.html#cfn-wafv2-webacl-customhttpheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customhttpheader.html#cfn-wafv2-webacl-customhttpheader-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomHTTPHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomHTTPHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomHTTPHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomHTTPHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomHTTPHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomHTTPHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CustomRequestHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customrequesthandling.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomRequestHandling {
        /// Property [`InsertHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customrequesthandling.html#cfn-wafv2-webacl-customrequesthandling-insertheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insert_headers: ::ValueList<CustomHTTPHeader>,
    }

    impl ::codec::SerializeValue for CustomRequestHandling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsertHeaders", &self.insert_headers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomRequestHandling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomRequestHandling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomRequestHandling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomRequestHandling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut insert_headers: Option<::ValueList<CustomHTTPHeader>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InsertHeaders" => {
                                insert_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomRequestHandling {
                        insert_headers: insert_headers.ok_or(::serde::de::Error::missing_field("InsertHeaders"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CustomResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponse.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomResponse {
        /// Property [`CustomResponseBodyKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponse.html#cfn-wafv2-webacl-customresponse-customresponsebodykey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_response_body_key: Option<::Value<String>>,
        /// Property [`ResponseCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponse.html#cfn-wafv2-webacl-customresponse-responsecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_code: ::Value<u32>,
        /// Property [`ResponseHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponse.html#cfn-wafv2-webacl-customresponse-responseheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_headers: Option<::ValueList<CustomHTTPHeader>>,
    }

    impl ::codec::SerializeValue for CustomResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_response_body_key) = self.custom_response_body_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomResponseBodyKey", custom_response_body_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseCode", &self.response_code)?;
            if let Some(ref response_headers) = self.response_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseHeaders", response_headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_response_body_key: Option<::Value<String>> = None;
                    let mut response_code: Option<::Value<u32>> = None;
                    let mut response_headers: Option<::ValueList<CustomHTTPHeader>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomResponseBodyKey" => {
                                custom_response_body_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseCode" => {
                                response_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseHeaders" => {
                                response_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomResponse {
                        custom_response_body_key: custom_response_body_key,
                        response_code: response_code.ok_or(::serde::de::Error::missing_field("ResponseCode"))?,
                        response_headers: response_headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.CustomResponseBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponsebody.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomResponseBody {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponsebody.html#cfn-wafv2-webacl-customresponsebody-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: ::Value<String>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponsebody.html#cfn-wafv2-webacl-customresponsebody-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomResponseBody {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", &self.content_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomResponseBody {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResponseBody, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomResponseBody;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomResponseBody")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomResponseBody {
                        content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                        content_type: content_type.ok_or(::serde::de::Error::missing_field("ContentType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.DefaultAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-defaultaction.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultAction {
        /// Property [`Allow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-defaultaction.html#cfn-wafv2-webacl-defaultaction-allow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow: Option<::Value<AllowAction>>,
        /// Property [`Block`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-defaultaction.html#cfn-wafv2-webacl-defaultaction-block).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block: Option<::Value<BlockAction>>,
    }

    impl ::codec::SerializeValue for DefaultAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow) = self.allow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Allow", allow)?;
            }
            if let Some(ref block) = self.block {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Block", block)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow: Option<::Value<AllowAction>> = None;
                    let mut block: Option<::Value<BlockAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Allow" => {
                                allow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Block" => {
                                block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultAction {
                        allow: allow,
                        block: block,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ExcludedRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-excludedrule.html) property type.
    #[derive(Debug, Default)]
    pub struct ExcludedRule {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-excludedrule.html#cfn-wafv2-webacl-excludedrule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ExcludedRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExcludedRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExcludedRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExcludedRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExcludedRule")
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

                    Ok(ExcludedRule {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.FieldIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldIdentifier {
        /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldidentifier.html#cfn-wafv2-webacl-fieldidentifier-identifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", &self.identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Identifier" => {
                                identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldIdentifier {
                        identifier: identifier.ok_or(::serde::de::Error::missing_field("Identifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldToMatch {
        /// Property [`AllQueryArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-allqueryarguments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all_query_arguments: Option<::Value<::json::Value>>,
        /// Property [`Body`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-body).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body: Option<::Value<Body>>,
        /// Property [`Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-cookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookies: Option<::Value<Cookies>>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::Value<Headers>>,
        /// Property [`JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-jsonbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_body: Option<::Value<JsonBody>>,
        /// Property [`Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-method).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub method: Option<::Value<::json::Value>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: Option<::Value<::json::Value>>,
        /// Property [`SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-singleheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_header: Option<::Value<SingleHeader>>,
        /// Property [`SingleQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-singlequeryargument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_query_argument: Option<::Value<SingleQueryArgument>>,
        /// Property [`UriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html#cfn-wafv2-webacl-fieldtomatch-uripath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri_path: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for FieldToMatch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_query_arguments) = self.all_query_arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllQueryArguments", all_query_arguments)?;
            }
            if let Some(ref body) = self.body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Body", body)?;
            }
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref json_body) = self.json_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonBody", json_body)?;
            }
            if let Some(ref method) = self.method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Method", method)?;
            }
            if let Some(ref query_string) = self.query_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
            }
            if let Some(ref single_header) = self.single_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleHeader", single_header)?;
            }
            if let Some(ref single_query_argument) = self.single_query_argument {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleQueryArgument", single_query_argument)?;
            }
            if let Some(ref uri_path) = self.uri_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPath", uri_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldToMatch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldToMatch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldToMatch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldToMatch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_query_arguments: Option<::Value<::json::Value>> = None;
                    let mut body: Option<::Value<Body>> = None;
                    let mut cookies: Option<::Value<Cookies>> = None;
                    let mut headers: Option<::Value<Headers>> = None;
                    let mut json_body: Option<::Value<JsonBody>> = None;
                    let mut method: Option<::Value<::json::Value>> = None;
                    let mut query_string: Option<::Value<::json::Value>> = None;
                    let mut single_header: Option<::Value<SingleHeader>> = None;
                    let mut single_query_argument: Option<::Value<SingleQueryArgument>> = None;
                    let mut uri_path: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllQueryArguments" => {
                                all_query_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Body" => {
                                body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JsonBody" => {
                                json_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Method" => {
                                method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleHeader" => {
                                single_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleQueryArgument" => {
                                single_query_argument = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UriPath" => {
                                uri_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldToMatch {
                        all_query_arguments: all_query_arguments,
                        body: body,
                        cookies: cookies,
                        headers: headers,
                        json_body: json_body,
                        method: method,
                        query_string: query_string,
                        single_header: single_header,
                        single_query_argument: single_query_argument,
                        uri_path: uri_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ForwardedIPConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-forwardedipconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardedIPConfiguration {
        /// Property [`FallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-forwardedipconfiguration.html#cfn-wafv2-webacl-forwardedipconfiguration-fallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fallback_behavior: ::Value<String>,
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-forwardedipconfiguration.html#cfn-wafv2-webacl-forwardedipconfiguration-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ForwardedIPConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FallbackBehavior", &self.fallback_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ForwardedIPConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardedIPConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardedIPConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardedIPConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fallback_behavior: Option<::Value<String>> = None;
                    let mut header_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FallbackBehavior" => {
                                fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardedIPConfiguration {
                        fallback_behavior: fallback_behavior.ok_or(::serde::de::Error::missing_field("FallbackBehavior"))?,
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.GeoMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-geomatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoMatchStatement {
        /// Property [`CountryCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-geomatchstatement.html#cfn-wafv2-webacl-geomatchstatement-countrycodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country_codes: Option<::ValueList<String>>,
        /// Property [`ForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-geomatchstatement.html#cfn-wafv2-webacl-geomatchstatement-forwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>>,
    }

    impl ::codec::SerializeValue for GeoMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref country_codes) = self.country_codes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCodes", country_codes)?;
            }
            if let Some(ref forwarded_ip_config) = self.forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIPConfig", forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut country_codes: Option<::ValueList<String>> = None;
                    let mut forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CountryCodes" => {
                                country_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIPConfig" => {
                                forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoMatchStatement {
                        country_codes: country_codes,
                        forwarded_ip_config: forwarded_ip_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.HeaderMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headermatchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct HeaderMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headermatchpattern.html#cfn-wafv2-webacl-headermatchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`ExcludedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headermatchpattern.html#cfn-wafv2-webacl-headermatchpattern-excludedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_headers: Option<::ValueList<String>>,
        /// Property [`IncludedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headermatchpattern.html#cfn-wafv2-webacl-headermatchpattern-includedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_headers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HeaderMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref excluded_headers) = self.excluded_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedHeaders", excluded_headers)?;
            }
            if let Some(ref included_headers) = self.included_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedHeaders", included_headers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HeaderMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HeaderMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HeaderMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HeaderMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut excluded_headers: Option<::ValueList<String>> = None;
                    let mut included_headers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedHeaders" => {
                                excluded_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedHeaders" => {
                                included_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HeaderMatchPattern {
                        all: all,
                        excluded_headers: excluded_headers,
                        included_headers: included_headers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headers.html) property type.
    #[derive(Debug, Default)]
    pub struct Headers {
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headers.html#cfn-wafv2-webacl-headers-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<HeaderMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headers.html#cfn-wafv2-webacl-headers-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headers.html#cfn-wafv2-webacl-headers-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: ::Value<String>,
    }

    impl ::codec::SerializeValue for Headers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", &self.oversize_handling)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Headers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Headers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Headers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Headers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut match_pattern: Option<::Value<HeaderMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Headers {
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling.ok_or(::serde::de::Error::missing_field("OversizeHandling"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.IPSetForwardedIPConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetforwardedipconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetForwardedIPConfiguration {
        /// Property [`FallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetforwardedipconfiguration.html#cfn-wafv2-webacl-ipsetforwardedipconfiguration-fallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fallback_behavior: ::Value<String>,
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetforwardedipconfiguration.html#cfn-wafv2-webacl-ipsetforwardedipconfiguration-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetforwardedipconfiguration.html#cfn-wafv2-webacl-ipsetforwardedipconfiguration-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: ::Value<String>,
    }

    impl ::codec::SerializeValue for IPSetForwardedIPConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FallbackBehavior", &self.fallback_behavior)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", &self.position)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetForwardedIPConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetForwardedIPConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetForwardedIPConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetForwardedIPConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fallback_behavior: Option<::Value<String>> = None;
                    let mut header_name: Option<::Value<String>> = None;
                    let mut position: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FallbackBehavior" => {
                                fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetForwardedIPConfiguration {
                        fallback_behavior: fallback_behavior.ok_or(::serde::de::Error::missing_field("FallbackBehavior"))?,
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        position: position.ok_or(::serde::de::Error::missing_field("Position"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.IPSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetreferencestatement.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetReferenceStatement {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetreferencestatement.html#cfn-wafv2-webacl-ipsetreferencestatement-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`IPSetForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetreferencestatement.html#cfn-wafv2-webacl-ipsetreferencestatement-ipsetforwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_set_forwarded_ip_config: Option<::Value<IPSetForwardedIPConfiguration>>,
    }

    impl ::codec::SerializeValue for IPSetReferenceStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref ip_set_forwarded_ip_config) = self.ip_set_forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetForwardedIPConfig", ip_set_forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetReferenceStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetReferenceStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetReferenceStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetReferenceStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut ip_set_forwarded_ip_config: Option<::Value<IPSetForwardedIPConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPSetForwardedIPConfig" => {
                                ip_set_forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetReferenceStatement {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        ip_set_forwarded_ip_config: ip_set_forwarded_ip_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ImmunityTimeProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-immunitytimeproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct ImmunityTimeProperty {
        /// Property [`ImmunityTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-immunitytimeproperty.html#cfn-wafv2-webacl-immunitytimeproperty-immunitytime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub immunity_time: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ImmunityTimeProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImmunityTime", &self.immunity_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImmunityTimeProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImmunityTimeProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImmunityTimeProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImmunityTimeProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut immunity_time: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImmunityTime" => {
                                immunity_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImmunityTimeProperty {
                        immunity_time: immunity_time.ok_or(::serde::de::Error::missing_field("ImmunityTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.JsonBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonBody {
        /// Property [`InvalidFallbackBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html#cfn-wafv2-webacl-jsonbody-invalidfallbackbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invalid_fallback_behavior: Option<::Value<String>>,
        /// Property [`MatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html#cfn-wafv2-webacl-jsonbody-matchpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_pattern: ::Value<JsonMatchPattern>,
        /// Property [`MatchScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html#cfn-wafv2-webacl-jsonbody-matchscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_scope: ::Value<String>,
        /// Property [`OversizeHandling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html#cfn-wafv2-webacl-jsonbody-oversizehandling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oversize_handling: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JsonBody {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invalid_fallback_behavior) = self.invalid_fallback_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvalidFallbackBehavior", invalid_fallback_behavior)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchPattern", &self.match_pattern)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchScope", &self.match_scope)?;
            if let Some(ref oversize_handling) = self.oversize_handling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OversizeHandling", oversize_handling)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonBody {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonBody, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonBody;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonBody")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invalid_fallback_behavior: Option<::Value<String>> = None;
                    let mut match_pattern: Option<::Value<JsonMatchPattern>> = None;
                    let mut match_scope: Option<::Value<String>> = None;
                    let mut oversize_handling: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvalidFallbackBehavior" => {
                                invalid_fallback_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchPattern" => {
                                match_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchScope" => {
                                match_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OversizeHandling" => {
                                oversize_handling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonBody {
                        invalid_fallback_behavior: invalid_fallback_behavior,
                        match_pattern: match_pattern.ok_or(::serde::de::Error::missing_field("MatchPattern"))?,
                        match_scope: match_scope.ok_or(::serde::de::Error::missing_field("MatchScope"))?,
                        oversize_handling: oversize_handling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.JsonMatchPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonmatchpattern.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonMatchPattern {
        /// Property [`All`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonmatchpattern.html#cfn-wafv2-webacl-jsonmatchpattern-all).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all: Option<::Value<::json::Value>>,
        /// Property [`IncludedPaths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonmatchpattern.html#cfn-wafv2-webacl-jsonmatchpattern-includedpaths).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_paths: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for JsonMatchPattern {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all) = self.all {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "All", all)?;
            }
            if let Some(ref included_paths) = self.included_paths {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedPaths", included_paths)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonMatchPattern {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonMatchPattern, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonMatchPattern;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonMatchPattern")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all: Option<::Value<::json::Value>> = None;
                    let mut included_paths: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "All" => {
                                all = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedPaths" => {
                                included_paths = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonMatchPattern {
                        all: all,
                        included_paths: included_paths,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-label.html) property type.
    #[derive(Debug, Default)]
    pub struct Label {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-label.html#cfn-wafv2-webacl-label-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Label {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Label {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.LabelMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-labelmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct LabelMatchStatement {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-labelmatchstatement.html#cfn-wafv2-webacl-labelmatchstatement-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-labelmatchstatement.html#cfn-wafv2-webacl-labelmatchstatement-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: ::Value<String>,
    }

    impl ::codec::SerializeValue for LabelMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LabelMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LabelMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LabelMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LabelMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LabelMatchStatement {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ManagedRuleGroupConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ManagedRuleGroupConfig {
        /// Property [`AWSManagedRulesACFPRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-awsmanagedrulesacfpruleset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_managed_rules_acfp_rule_set: Option<::Value<AWSManagedRulesACFPRuleSet>>,
        /// Property [`AWSManagedRulesATPRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-awsmanagedrulesatpruleset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_managed_rules_atp_rule_set: Option<::Value<AWSManagedRulesATPRuleSet>>,
        /// Property [`AWSManagedRulesBotControlRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-awsmanagedrulesbotcontrolruleset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_managed_rules_bot_control_rule_set: Option<::Value<AWSManagedRulesBotControlRuleSet>>,
        /// Property [`LoginPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-loginpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub login_path: Option<::Value<String>>,
        /// Property [`PasswordField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-passwordfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_field: Option<::Value<FieldIdentifier>>,
        /// Property [`PayloadType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-payloadtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_type: Option<::Value<String>>,
        /// Property [`UsernameField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html#cfn-wafv2-webacl-managedrulegroupconfig-usernamefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username_field: Option<::Value<FieldIdentifier>>,
    }

    impl ::codec::SerializeValue for ManagedRuleGroupConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_managed_rules_acfp_rule_set) = self.aws_managed_rules_acfp_rule_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSManagedRulesACFPRuleSet", aws_managed_rules_acfp_rule_set)?;
            }
            if let Some(ref aws_managed_rules_atp_rule_set) = self.aws_managed_rules_atp_rule_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSManagedRulesATPRuleSet", aws_managed_rules_atp_rule_set)?;
            }
            if let Some(ref aws_managed_rules_bot_control_rule_set) = self.aws_managed_rules_bot_control_rule_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSManagedRulesBotControlRuleSet", aws_managed_rules_bot_control_rule_set)?;
            }
            if let Some(ref login_path) = self.login_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoginPath", login_path)?;
            }
            if let Some(ref password_field) = self.password_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordField", password_field)?;
            }
            if let Some(ref payload_type) = self.payload_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadType", payload_type)?;
            }
            if let Some(ref username_field) = self.username_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameField", username_field)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManagedRuleGroupConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedRuleGroupConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManagedRuleGroupConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManagedRuleGroupConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_managed_rules_acfp_rule_set: Option<::Value<AWSManagedRulesACFPRuleSet>> = None;
                    let mut aws_managed_rules_atp_rule_set: Option<::Value<AWSManagedRulesATPRuleSet>> = None;
                    let mut aws_managed_rules_bot_control_rule_set: Option<::Value<AWSManagedRulesBotControlRuleSet>> = None;
                    let mut login_path: Option<::Value<String>> = None;
                    let mut password_field: Option<::Value<FieldIdentifier>> = None;
                    let mut payload_type: Option<::Value<String>> = None;
                    let mut username_field: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AWSManagedRulesACFPRuleSet" => {
                                aws_managed_rules_acfp_rule_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AWSManagedRulesATPRuleSet" => {
                                aws_managed_rules_atp_rule_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AWSManagedRulesBotControlRuleSet" => {
                                aws_managed_rules_bot_control_rule_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoginPath" => {
                                login_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PasswordField" => {
                                password_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadType" => {
                                payload_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsernameField" => {
                                username_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManagedRuleGroupConfig {
                        aws_managed_rules_acfp_rule_set: aws_managed_rules_acfp_rule_set,
                        aws_managed_rules_atp_rule_set: aws_managed_rules_atp_rule_set,
                        aws_managed_rules_bot_control_rule_set: aws_managed_rules_bot_control_rule_set,
                        login_path: login_path,
                        password_field: password_field,
                        payload_type: payload_type,
                        username_field: username_field,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ManagedRuleGroupStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct ManagedRuleGroupStatement {
        /// Property [`ExcludedRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-excludedrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_rules: Option<::ValueList<ExcludedRule>>,
        /// Property [`ManagedRuleGroupConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-managedrulegroupconfigs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_rule_group_configs: Option<::ValueList<ManagedRuleGroupConfig>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`RuleActionOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-ruleactionoverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_action_overrides: Option<::ValueList<RuleActionOverride>>,
        /// Property [`ScopeDownStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-scopedownstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope_down_statement: Option<::Value<Statement>>,
        /// Property [`VendorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-vendorname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vendor_name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html#cfn-wafv2-webacl-managedrulegroupstatement-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ManagedRuleGroupStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_rules) = self.excluded_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedRules", excluded_rules)?;
            }
            if let Some(ref managed_rule_group_configs) = self.managed_rule_group_configs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedRuleGroupConfigs", managed_rule_group_configs)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref rule_action_overrides) = self.rule_action_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleActionOverrides", rule_action_overrides)?;
            }
            if let Some(ref scope_down_statement) = self.scope_down_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScopeDownStatement", scope_down_statement)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VendorName", &self.vendor_name)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManagedRuleGroupStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedRuleGroupStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManagedRuleGroupStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManagedRuleGroupStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_rules: Option<::ValueList<ExcludedRule>> = None;
                    let mut managed_rule_group_configs: Option<::ValueList<ManagedRuleGroupConfig>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut rule_action_overrides: Option<::ValueList<RuleActionOverride>> = None;
                    let mut scope_down_statement: Option<::Value<Statement>> = None;
                    let mut vendor_name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedRules" => {
                                excluded_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedRuleGroupConfigs" => {
                                managed_rule_group_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleActionOverrides" => {
                                rule_action_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScopeDownStatement" => {
                                scope_down_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VendorName" => {
                                vendor_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManagedRuleGroupStatement {
                        excluded_rules: excluded_rules,
                        managed_rule_group_configs: managed_rule_group_configs,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        rule_action_overrides: rule_action_overrides,
                        scope_down_statement: scope_down_statement,
                        vendor_name: vendor_name.ok_or(::serde::de::Error::missing_field("VendorName"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.NotStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-notstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct NotStatement {
        /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-notstatement.html#cfn-wafv2-webacl-notstatement-statement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement: ::Value<Statement>,
    }

    impl ::codec::SerializeValue for NotStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statement: Option<::Value<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statement" => {
                                statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotStatement {
                        statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.OrStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-orstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct OrStatement {
        /// Property [`Statements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-orstatement.html#cfn-wafv2-webacl-orstatement-statements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statements: ::ValueList<Statement>,
    }

    impl ::codec::SerializeValue for OrStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statements", &self.statements)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statements: Option<::ValueList<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statements" => {
                                statements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrStatement {
                        statements: statements.ok_or(::serde::de::Error::missing_field("Statements"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.OverrideAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-overrideaction.html) property type.
    #[derive(Debug, Default)]
    pub struct OverrideAction {
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-overrideaction.html#cfn-wafv2-webacl-overrideaction-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<::json::Value>>,
        /// Property [`None`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-overrideaction.html#cfn-wafv2-webacl-overrideaction-none).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub none: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for OverrideAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref none) = self.none {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "None", none)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OverrideAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OverrideAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OverrideAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OverrideAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut count: Option<::Value<::json::Value>> = None;
                    let mut none: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "None" => {
                                none = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OverrideAction {
                        count: count,
                        none: none,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateBasedStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RateBasedStatement {
        /// Property [`AggregateKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html#cfn-wafv2-webacl-ratebasedstatement-aggregatekeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregate_key_type: ::Value<String>,
        /// Property [`CustomKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html#cfn-wafv2-webacl-ratebasedstatement-customkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_keys: Option<::ValueList<RateBasedStatementCustomKey>>,
        /// Property [`ForwardedIPConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html#cfn-wafv2-webacl-ratebasedstatement-forwardedipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>>,
        /// Property [`Limit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html#cfn-wafv2-webacl-ratebasedstatement-limit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub limit: ::Value<u32>,
        /// Property [`ScopeDownStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html#cfn-wafv2-webacl-ratebasedstatement-scopedownstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope_down_statement: Option<::Value<Statement>>,
    }

    impl ::codec::SerializeValue for RateBasedStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregateKeyType", &self.aggregate_key_type)?;
            if let Some(ref custom_keys) = self.custom_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomKeys", custom_keys)?;
            }
            if let Some(ref forwarded_ip_config) = self.forwarded_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIPConfig", forwarded_ip_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limit", &self.limit)?;
            if let Some(ref scope_down_statement) = self.scope_down_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScopeDownStatement", scope_down_statement)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateBasedStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateBasedStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateBasedStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateBasedStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregate_key_type: Option<::Value<String>> = None;
                    let mut custom_keys: Option<::ValueList<RateBasedStatementCustomKey>> = None;
                    let mut forwarded_ip_config: Option<::Value<ForwardedIPConfiguration>> = None;
                    let mut limit: Option<::Value<u32>> = None;
                    let mut scope_down_statement: Option<::Value<Statement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregateKeyType" => {
                                aggregate_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomKeys" => {
                                custom_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIPConfig" => {
                                forwarded_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Limit" => {
                                limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScopeDownStatement" => {
                                scope_down_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateBasedStatement {
                        aggregate_key_type: aggregate_key_type.ok_or(::serde::de::Error::missing_field("AggregateKeyType"))?,
                        custom_keys: custom_keys,
                        forwarded_ip_config: forwarded_ip_config,
                        limit: limit.ok_or(::serde::de::Error::missing_field("Limit"))?,
                        scope_down_statement: scope_down_statement,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateBasedStatementCustomKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html) property type.
    #[derive(Debug, Default)]
    pub struct RateBasedStatementCustomKey {
        /// Property [`Cookie`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-cookie).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie: Option<::Value<RateLimitCookie>>,
        /// Property [`ForwardedIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-forwardedip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forwarded_ip: Option<::Value<::json::Value>>,
        /// Property [`HTTPMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<::json::Value>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::Value<RateLimitHeader>>,
        /// Property [`IP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-ip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip: Option<::Value<::json::Value>>,
        /// Property [`LabelNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-labelnamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_namespace: Option<::Value<RateLimitLabelNamespace>>,
        /// Property [`QueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-queryargument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_argument: Option<::Value<RateLimitQueryArgument>>,
        /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-querystring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string: Option<::Value<RateLimitQueryString>>,
        /// Property [`UriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html#cfn-wafv2-webacl-ratebasedstatementcustomkey-uripath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uri_path: Option<::Value<RateLimitUriPath>>,
    }

    impl ::codec::SerializeValue for RateBasedStatementCustomKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookie) = self.cookie {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookie", cookie)?;
            }
            if let Some(ref forwarded_ip) = self.forwarded_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedIP", forwarded_ip)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPMethod", http_method)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref ip) = self.ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IP", ip)?;
            }
            if let Some(ref label_namespace) = self.label_namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelNamespace", label_namespace)?;
            }
            if let Some(ref query_argument) = self.query_argument {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryArgument", query_argument)?;
            }
            if let Some(ref query_string) = self.query_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
            }
            if let Some(ref uri_path) = self.uri_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UriPath", uri_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateBasedStatementCustomKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateBasedStatementCustomKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateBasedStatementCustomKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateBasedStatementCustomKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie: Option<::Value<RateLimitCookie>> = None;
                    let mut forwarded_ip: Option<::Value<::json::Value>> = None;
                    let mut http_method: Option<::Value<::json::Value>> = None;
                    let mut header: Option<::Value<RateLimitHeader>> = None;
                    let mut ip: Option<::Value<::json::Value>> = None;
                    let mut label_namespace: Option<::Value<RateLimitLabelNamespace>> = None;
                    let mut query_argument: Option<::Value<RateLimitQueryArgument>> = None;
                    let mut query_string: Option<::Value<RateLimitQueryString>> = None;
                    let mut uri_path: Option<::Value<RateLimitUriPath>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cookie" => {
                                cookie = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedIP" => {
                                forwarded_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IP" => {
                                ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelNamespace" => {
                                label_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryArgument" => {
                                query_argument = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UriPath" => {
                                uri_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateBasedStatementCustomKey {
                        cookie: cookie,
                        forwarded_ip: forwarded_ip,
                        http_method: http_method,
                        header: header,
                        ip: ip,
                        label_namespace: label_namespace,
                        query_argument: query_argument,
                        query_string: query_string,
                        uri_path: uri_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitCookie`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitcookie.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitCookie {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitcookie.html#cfn-wafv2-webacl-ratelimitcookie-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitcookie.html#cfn-wafv2-webacl-ratelimitcookie-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitCookie {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitCookie {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitCookie, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitCookie;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitCookie")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitCookie {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitheader.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitheader.html#cfn-wafv2-webacl-ratelimitheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitheader.html#cfn-wafv2-webacl-ratelimitheader-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitLabelNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitlabelnamespace.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitLabelNamespace {
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitlabelnamespace.html#cfn-wafv2-webacl-ratelimitlabelnamespace-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: ::Value<String>,
    }

    impl ::codec::SerializeValue for RateLimitLabelNamespace {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitLabelNamespace {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitLabelNamespace, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitLabelNamespace;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitLabelNamespace")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitLabelNamespace {
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitqueryargument.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitQueryArgument {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitqueryargument.html#cfn-wafv2-webacl-ratelimitqueryargument-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitqueryargument.html#cfn-wafv2-webacl-ratelimitqueryargument-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitQueryArgument {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitQueryArgument {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitQueryArgument, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitQueryArgument;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitQueryArgument")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitQueryArgument {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitQueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitquerystring.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitQueryString {
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitquerystring.html#cfn-wafv2-webacl-ratelimitquerystring-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitQueryString {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitQueryString {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitQueryString, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitQueryString;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitQueryString")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitQueryString {
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RateLimitUriPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimituripath.html) property type.
    #[derive(Debug, Default)]
    pub struct RateLimitUriPath {
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimituripath.html#cfn-wafv2-webacl-ratelimituripath-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RateLimitUriPath {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateLimitUriPath {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateLimitUriPath, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateLimitUriPath;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateLimitUriPath")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateLimitUriPath {
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RegexMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RegexMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexmatchstatement.html#cfn-wafv2-webacl-regexmatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`RegexString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexmatchstatement.html#cfn-wafv2-webacl-regexmatchstatement-regexstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_string: ::Value<String>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexmatchstatement.html#cfn-wafv2-webacl-regexmatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RegexMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexString", &self.regex_string)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegexMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegexMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegexMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegexMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut regex_string: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexString" => {
                                regex_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegexMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        regex_string: regex_string.ok_or(::serde::de::Error::missing_field("RegexString"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RegexPatternSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexpatternsetreferencestatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RegexPatternSetReferenceStatement {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexpatternsetreferencestatement.html#cfn-wafv2-webacl-regexpatternsetreferencestatement-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexpatternsetreferencestatement.html#cfn-wafv2-webacl-regexpatternsetreferencestatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexpatternsetreferencestatement.html#cfn-wafv2-webacl-regexpatternsetreferencestatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for RegexPatternSetReferenceStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegexPatternSetReferenceStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegexPatternSetReferenceStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegexPatternSetReferenceStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegexPatternSetReferenceStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegexPatternSetReferenceStatement {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RequestBodyAssociatedResourceTypeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestbodyassociatedresourcetypeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RequestBodyAssociatedResourceTypeConfig {
        /// Property [`DefaultSizeInspectionLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestbodyassociatedresourcetypeconfig.html#cfn-wafv2-webacl-requestbodyassociatedresourcetypeconfig-defaultsizeinspectionlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_size_inspection_limit: ::Value<String>,
    }

    impl ::codec::SerializeValue for RequestBodyAssociatedResourceTypeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSizeInspectionLimit", &self.default_size_inspection_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequestBodyAssociatedResourceTypeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequestBodyAssociatedResourceTypeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequestBodyAssociatedResourceTypeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequestBodyAssociatedResourceTypeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_size_inspection_limit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultSizeInspectionLimit" => {
                                default_size_inspection_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RequestBodyAssociatedResourceTypeConfig {
                        default_size_inspection_limit: default_size_inspection_limit.ok_or(::serde::de::Error::missing_field("DefaultSizeInspectionLimit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RequestInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspection.html) property type.
    #[derive(Debug, Default)]
    pub struct RequestInspection {
        /// Property [`PasswordField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspection.html#cfn-wafv2-webacl-requestinspection-passwordfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_field: ::Value<FieldIdentifier>,
        /// Property [`PayloadType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspection.html#cfn-wafv2-webacl-requestinspection-payloadtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_type: ::Value<String>,
        /// Property [`UsernameField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspection.html#cfn-wafv2-webacl-requestinspection-usernamefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username_field: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for RequestInspection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordField", &self.password_field)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadType", &self.payload_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameField", &self.username_field)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequestInspection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequestInspection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequestInspection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequestInspection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password_field: Option<::Value<FieldIdentifier>> = None;
                    let mut payload_type: Option<::Value<String>> = None;
                    let mut username_field: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordField" => {
                                password_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadType" => {
                                payload_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsernameField" => {
                                username_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RequestInspection {
                        password_field: password_field.ok_or(::serde::de::Error::missing_field("PasswordField"))?,
                        payload_type: payload_type.ok_or(::serde::de::Error::missing_field("PayloadType"))?,
                        username_field: username_field.ok_or(::serde::de::Error::missing_field("UsernameField"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RequestInspectionACFP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html) property type.
    #[derive(Debug, Default)]
    pub struct RequestInspectionACFP {
        /// Property [`AddressFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-addressfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address_fields: Option<::ValueList<FieldIdentifier>>,
        /// Property [`EmailField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-emailfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_field: Option<::Value<FieldIdentifier>>,
        /// Property [`PasswordField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-passwordfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_field: Option<::Value<FieldIdentifier>>,
        /// Property [`PayloadType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-payloadtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_type: ::Value<String>,
        /// Property [`PhoneNumberFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-phonenumberfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number_fields: Option<::ValueList<FieldIdentifier>>,
        /// Property [`UsernameField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html#cfn-wafv2-webacl-requestinspectionacfp-usernamefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username_field: Option<::Value<FieldIdentifier>>,
    }

    impl ::codec::SerializeValue for RequestInspectionACFP {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address_fields) = self.address_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddressFields", address_fields)?;
            }
            if let Some(ref email_field) = self.email_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailField", email_field)?;
            }
            if let Some(ref password_field) = self.password_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordField", password_field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadType", &self.payload_type)?;
            if let Some(ref phone_number_fields) = self.phone_number_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumberFields", phone_number_fields)?;
            }
            if let Some(ref username_field) = self.username_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameField", username_field)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequestInspectionACFP {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequestInspectionACFP, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequestInspectionACFP;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequestInspectionACFP")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address_fields: Option<::ValueList<FieldIdentifier>> = None;
                    let mut email_field: Option<::Value<FieldIdentifier>> = None;
                    let mut password_field: Option<::Value<FieldIdentifier>> = None;
                    let mut payload_type: Option<::Value<String>> = None;
                    let mut phone_number_fields: Option<::ValueList<FieldIdentifier>> = None;
                    let mut username_field: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddressFields" => {
                                address_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailField" => {
                                email_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PasswordField" => {
                                password_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadType" => {
                                payload_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneNumberFields" => {
                                phone_number_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsernameField" => {
                                username_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RequestInspectionACFP {
                        address_fields: address_fields,
                        email_field: email_field,
                        password_field: password_field,
                        payload_type: payload_type.ok_or(::serde::de::Error::missing_field("PayloadType"))?,
                        phone_number_fields: phone_number_fields,
                        username_field: username_field,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ResponseInspection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseInspection {
        /// Property [`BodyContains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html#cfn-wafv2-webacl-responseinspection-bodycontains).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub body_contains: Option<::Value<ResponseInspectionBodyContains>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html#cfn-wafv2-webacl-responseinspection-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::Value<ResponseInspectionHeader>>,
        /// Property [`Json`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html#cfn-wafv2-webacl-responseinspection-json).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json: Option<::Value<ResponseInspectionJson>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html#cfn-wafv2-webacl-responseinspection-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: Option<::Value<ResponseInspectionStatusCode>>,
    }

    impl ::codec::SerializeValue for ResponseInspection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref body_contains) = self.body_contains {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BodyContains", body_contains)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref json) = self.json {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Json", json)?;
            }
            if let Some(ref status_code) = self.status_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", status_code)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseInspection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseInspection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseInspection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseInspection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut body_contains: Option<::Value<ResponseInspectionBodyContains>> = None;
                    let mut header: Option<::Value<ResponseInspectionHeader>> = None;
                    let mut json: Option<::Value<ResponseInspectionJson>> = None;
                    let mut status_code: Option<::Value<ResponseInspectionStatusCode>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BodyContains" => {
                                body_contains = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Json" => {
                                json = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseInspection {
                        body_contains: body_contains,
                        header: header,
                        json: json,
                        status_code: status_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ResponseInspectionBodyContains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionbodycontains.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseInspectionBodyContains {
        /// Property [`FailureStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionbodycontains.html#cfn-wafv2-webacl-responseinspectionbodycontains-failurestrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_strings: ::ValueList<String>,
        /// Property [`SuccessStrings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionbodycontains.html#cfn-wafv2-webacl-responseinspectionbodycontains-successstrings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_strings: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ResponseInspectionBodyContains {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureStrings", &self.failure_strings)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessStrings", &self.success_strings)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseInspectionBodyContains {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseInspectionBodyContains, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseInspectionBodyContains;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseInspectionBodyContains")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_strings: Option<::ValueList<String>> = None;
                    let mut success_strings: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureStrings" => {
                                failure_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessStrings" => {
                                success_strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseInspectionBodyContains {
                        failure_strings: failure_strings.ok_or(::serde::de::Error::missing_field("FailureStrings"))?,
                        success_strings: success_strings.ok_or(::serde::de::Error::missing_field("SuccessStrings"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ResponseInspectionHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionheader.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseInspectionHeader {
        /// Property [`FailureValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionheader.html#cfn-wafv2-webacl-responseinspectionheader-failurevalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_values: ::ValueList<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionheader.html#cfn-wafv2-webacl-responseinspectionheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SuccessValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionheader.html#cfn-wafv2-webacl-responseinspectionheader-successvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ResponseInspectionHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureValues", &self.failure_values)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessValues", &self.success_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseInspectionHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseInspectionHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseInspectionHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseInspectionHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_values: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut success_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureValues" => {
                                failure_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessValues" => {
                                success_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseInspectionHeader {
                        failure_values: failure_values.ok_or(::serde::de::Error::missing_field("FailureValues"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        success_values: success_values.ok_or(::serde::de::Error::missing_field("SuccessValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ResponseInspectionJson`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionjson.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseInspectionJson {
        /// Property [`FailureValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionjson.html#cfn-wafv2-webacl-responseinspectionjson-failurevalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_values: ::ValueList<String>,
        /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionjson.html#cfn-wafv2-webacl-responseinspectionjson-identifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifier: ::Value<String>,
        /// Property [`SuccessValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionjson.html#cfn-wafv2-webacl-responseinspectionjson-successvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ResponseInspectionJson {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureValues", &self.failure_values)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", &self.identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessValues", &self.success_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseInspectionJson {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseInspectionJson, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseInspectionJson;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseInspectionJson")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_values: Option<::ValueList<String>> = None;
                    let mut identifier: Option<::Value<String>> = None;
                    let mut success_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureValues" => {
                                failure_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Identifier" => {
                                identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessValues" => {
                                success_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseInspectionJson {
                        failure_values: failure_values.ok_or(::serde::de::Error::missing_field("FailureValues"))?,
                        identifier: identifier.ok_or(::serde::de::Error::missing_field("Identifier"))?,
                        success_values: success_values.ok_or(::serde::de::Error::missing_field("SuccessValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.ResponseInspectionStatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionstatuscode.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseInspectionStatusCode {
        /// Property [`FailureCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionstatuscode.html#cfn-wafv2-webacl-responseinspectionstatuscode-failurecodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_codes: ::ValueList<u32>,
        /// Property [`SuccessCodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionstatuscode.html#cfn-wafv2-webacl-responseinspectionstatuscode-successcodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_codes: ::ValueList<u32>,
    }

    impl ::codec::SerializeValue for ResponseInspectionStatusCode {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureCodes", &self.failure_codes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessCodes", &self.success_codes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseInspectionStatusCode {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseInspectionStatusCode, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseInspectionStatusCode;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseInspectionStatusCode")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_codes: Option<::ValueList<u32>> = None;
                    let mut success_codes: Option<::ValueList<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureCodes" => {
                                failure_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessCodes" => {
                                success_codes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseInspectionStatusCode {
                        failure_codes: failure_codes.ok_or(::serde::de::Error::missing_field("FailureCodes"))?,
                        success_codes: success_codes.ok_or(::serde::de::Error::missing_field("SuccessCodes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<RuleAction>>,
        /// Property [`CaptchaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-captchaconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub captcha_config: Option<::Value<CaptchaConfig>>,
        /// Property [`ChallengeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-challengeconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub challenge_config: Option<::Value<ChallengeConfig>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`OverrideAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-overrideaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub override_action: Option<::Value<OverrideAction>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`RuleLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-rulelabels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_labels: Option<::ValueList<Label>>,
        /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-statement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement: ::Value<Statement>,
        /// Property [`VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html#cfn-wafv2-webacl-rule-visibilityconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub visibility_config: ::Value<VisibilityConfig>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            if let Some(ref captcha_config) = self.captcha_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptchaConfig", captcha_config)?;
            }
            if let Some(ref challenge_config) = self.challenge_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChallengeConfig", challenge_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref override_action) = self.override_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverrideAction", override_action)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            if let Some(ref rule_labels) = self.rule_labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleLabels", rule_labels)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityConfig", &self.visibility_config)?;
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
                    let mut action: Option<::Value<RuleAction>> = None;
                    let mut captcha_config: Option<::Value<CaptchaConfig>> = None;
                    let mut challenge_config: Option<::Value<ChallengeConfig>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut override_action: Option<::Value<OverrideAction>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut rule_labels: Option<::ValueList<Label>> = None;
                    let mut statement: Option<::Value<Statement>> = None;
                    let mut visibility_config: Option<::Value<VisibilityConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptchaConfig" => {
                                captcha_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChallengeConfig" => {
                                challenge_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OverrideAction" => {
                                override_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleLabels" => {
                                rule_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statement" => {
                                statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VisibilityConfig" => {
                                visibility_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        action: action,
                        captcha_config: captcha_config,
                        challenge_config: challenge_config,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        override_action: override_action,
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        rule_labels: rule_labels,
                        statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                        visibility_config: visibility_config.ok_or(::serde::de::Error::missing_field("VisibilityConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RuleAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleAction {
        /// Property [`Allow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html#cfn-wafv2-webacl-ruleaction-allow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow: Option<::Value<AllowAction>>,
        /// Property [`Block`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html#cfn-wafv2-webacl-ruleaction-block).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block: Option<::Value<BlockAction>>,
        /// Property [`Captcha`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html#cfn-wafv2-webacl-ruleaction-captcha).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub captcha: Option<::Value<CaptchaAction>>,
        /// Property [`Challenge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html#cfn-wafv2-webacl-ruleaction-challenge).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub challenge: Option<::Value<ChallengeAction>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html#cfn-wafv2-webacl-ruleaction-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<CountAction>>,
    }

    impl ::codec::SerializeValue for RuleAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow) = self.allow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Allow", allow)?;
            }
            if let Some(ref block) = self.block {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Block", block)?;
            }
            if let Some(ref captcha) = self.captcha {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Captcha", captcha)?;
            }
            if let Some(ref challenge) = self.challenge {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Challenge", challenge)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow: Option<::Value<AllowAction>> = None;
                    let mut block: Option<::Value<BlockAction>> = None;
                    let mut captcha: Option<::Value<CaptchaAction>> = None;
                    let mut challenge: Option<::Value<ChallengeAction>> = None;
                    let mut count: Option<::Value<CountAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Allow" => {
                                allow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Block" => {
                                block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Captcha" => {
                                captcha = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Challenge" => {
                                challenge = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleAction {
                        allow: allow,
                        block: block,
                        captcha: captcha,
                        challenge: challenge,
                        count: count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RuleActionOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleactionoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleActionOverride {
        /// Property [`ActionToUse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleactionoverride.html#cfn-wafv2-webacl-ruleactionoverride-actiontouse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_to_use: ::Value<RuleAction>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleactionoverride.html#cfn-wafv2-webacl-ruleactionoverride-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for RuleActionOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionToUse", &self.action_to_use)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleActionOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleActionOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleActionOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleActionOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_to_use: Option<::Value<RuleAction>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionToUse" => {
                                action_to_use = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleActionOverride {
                        action_to_use: action_to_use.ok_or(::serde::de::Error::missing_field("ActionToUse"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.RuleGroupReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rulegroupreferencestatement.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleGroupReferenceStatement {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rulegroupreferencestatement.html#cfn-wafv2-webacl-rulegroupreferencestatement-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`ExcludedRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rulegroupreferencestatement.html#cfn-wafv2-webacl-rulegroupreferencestatement-excludedrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_rules: Option<::ValueList<ExcludedRule>>,
        /// Property [`RuleActionOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rulegroupreferencestatement.html#cfn-wafv2-webacl-rulegroupreferencestatement-ruleactionoverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_action_overrides: Option<::ValueList<RuleActionOverride>>,
    }

    impl ::codec::SerializeValue for RuleGroupReferenceStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref excluded_rules) = self.excluded_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedRules", excluded_rules)?;
            }
            if let Some(ref rule_action_overrides) = self.rule_action_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleActionOverrides", rule_action_overrides)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleGroupReferenceStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleGroupReferenceStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleGroupReferenceStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleGroupReferenceStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut excluded_rules: Option<::ValueList<ExcludedRule>> = None;
                    let mut rule_action_overrides: Option<::ValueList<RuleActionOverride>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedRules" => {
                                excluded_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleActionOverrides" => {
                                rule_action_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleGroupReferenceStatement {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        excluded_rules: excluded_rules,
                        rule_action_overrides: rule_action_overrides,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.SingleHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singleheader.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleHeader {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singleheader.html#cfn-wafv2-webacl-singleheader-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleHeader")
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

                    Ok(SingleHeader {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.SingleQueryArgument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singlequeryargument.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleQueryArgument {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singlequeryargument.html#cfn-wafv2-webacl-singlequeryargument-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleQueryArgument {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleQueryArgument {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleQueryArgument, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleQueryArgument;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleQueryArgument")
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

                    Ok(SingleQueryArgument {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.SizeConstraintStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct SizeConstraintStatement {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html#cfn-wafv2-webacl-sizeconstraintstatement-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html#cfn-wafv2-webacl-sizeconstraintstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html#cfn-wafv2-webacl-sizeconstraintstatement-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: ::Value<f64>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html#cfn-wafv2-webacl-sizeconstraintstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for SizeConstraintStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SizeConstraintStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SizeConstraintStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SizeConstraintStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SizeConstraintStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut size: Option<::Value<f64>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SizeConstraintStatement {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.SqliMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sqlimatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct SqliMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sqlimatchstatement.html#cfn-wafv2-webacl-sqlimatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`SensitivityLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sqlimatchstatement.html#cfn-wafv2-webacl-sqlimatchstatement-sensitivitylevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sensitivity_level: Option<::Value<String>>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sqlimatchstatement.html#cfn-wafv2-webacl-sqlimatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for SqliMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            if let Some(ref sensitivity_level) = self.sensitivity_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SensitivityLevel", sensitivity_level)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqliMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqliMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqliMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqliMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut sensitivity_level: Option<::Value<String>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SensitivityLevel" => {
                                sensitivity_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqliMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        sensitivity_level: sensitivity_level,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html) property type.
    #[derive(Debug, Default)]
    pub struct Statement {
        /// Property [`AndStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-andstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub and_statement: Option<::Value<AndStatement>>,
        /// Property [`ByteMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-bytematchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub byte_match_statement: Option<::Value<ByteMatchStatement>>,
        /// Property [`GeoMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-geomatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub geo_match_statement: Option<::Value<GeoMatchStatement>>,
        /// Property [`IPSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-ipsetreferencestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_set_reference_statement: Option<::Value<IPSetReferenceStatement>>,
        /// Property [`LabelMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-labelmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label_match_statement: Option<::Value<LabelMatchStatement>>,
        /// Property [`ManagedRuleGroupStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-managedrulegroupstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_rule_group_statement: Option<::Value<ManagedRuleGroupStatement>>,
        /// Property [`NotStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-notstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_statement: Option<::Value<NotStatement>>,
        /// Property [`OrStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-orstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub or_statement: Option<::Value<OrStatement>>,
        /// Property [`RateBasedStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-ratebasedstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rate_based_statement: Option<::Value<RateBasedStatement>>,
        /// Property [`RegexMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-regexmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_match_statement: Option<::Value<RegexMatchStatement>>,
        /// Property [`RegexPatternSetReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-regexpatternsetreferencestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_pattern_set_reference_statement: Option<::Value<RegexPatternSetReferenceStatement>>,
        /// Property [`RuleGroupReferenceStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-rulegroupreferencestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_group_reference_statement: Option<::Value<RuleGroupReferenceStatement>>,
        /// Property [`SizeConstraintStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-sizeconstraintstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_constraint_statement: Option<::Value<SizeConstraintStatement>>,
        /// Property [`SqliMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-sqlimatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqli_match_statement: Option<::Value<SqliMatchStatement>>,
        /// Property [`XssMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html#cfn-wafv2-webacl-statement-xssmatchstatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub xss_match_statement: Option<::Value<XssMatchStatement>>,
    }

    impl ::codec::SerializeValue for Statement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref and_statement) = self.and_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AndStatement", and_statement)?;
            }
            if let Some(ref byte_match_statement) = self.byte_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ByteMatchStatement", byte_match_statement)?;
            }
            if let Some(ref geo_match_statement) = self.geo_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoMatchStatement", geo_match_statement)?;
            }
            if let Some(ref ip_set_reference_statement) = self.ip_set_reference_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetReferenceStatement", ip_set_reference_statement)?;
            }
            if let Some(ref label_match_statement) = self.label_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelMatchStatement", label_match_statement)?;
            }
            if let Some(ref managed_rule_group_statement) = self.managed_rule_group_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedRuleGroupStatement", managed_rule_group_statement)?;
            }
            if let Some(ref not_statement) = self.not_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotStatement", not_statement)?;
            }
            if let Some(ref or_statement) = self.or_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrStatement", or_statement)?;
            }
            if let Some(ref rate_based_statement) = self.rate_based_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateBasedStatement", rate_based_statement)?;
            }
            if let Some(ref regex_match_statement) = self.regex_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexMatchStatement", regex_match_statement)?;
            }
            if let Some(ref regex_pattern_set_reference_statement) = self.regex_pattern_set_reference_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexPatternSetReferenceStatement", regex_pattern_set_reference_statement)?;
            }
            if let Some(ref rule_group_reference_statement) = self.rule_group_reference_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleGroupReferenceStatement", rule_group_reference_statement)?;
            }
            if let Some(ref size_constraint_statement) = self.size_constraint_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeConstraintStatement", size_constraint_statement)?;
            }
            if let Some(ref sqli_match_statement) = self.sqli_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqliMatchStatement", sqli_match_statement)?;
            }
            if let Some(ref xss_match_statement) = self.xss_match_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "XssMatchStatement", xss_match_statement)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Statement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Statement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Statement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Statement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut and_statement: Option<::Value<AndStatement>> = None;
                    let mut byte_match_statement: Option<::Value<ByteMatchStatement>> = None;
                    let mut geo_match_statement: Option<::Value<GeoMatchStatement>> = None;
                    let mut ip_set_reference_statement: Option<::Value<IPSetReferenceStatement>> = None;
                    let mut label_match_statement: Option<::Value<LabelMatchStatement>> = None;
                    let mut managed_rule_group_statement: Option<::Value<ManagedRuleGroupStatement>> = None;
                    let mut not_statement: Option<::Value<NotStatement>> = None;
                    let mut or_statement: Option<::Value<OrStatement>> = None;
                    let mut rate_based_statement: Option<::Value<RateBasedStatement>> = None;
                    let mut regex_match_statement: Option<::Value<RegexMatchStatement>> = None;
                    let mut regex_pattern_set_reference_statement: Option<::Value<RegexPatternSetReferenceStatement>> = None;
                    let mut rule_group_reference_statement: Option<::Value<RuleGroupReferenceStatement>> = None;
                    let mut size_constraint_statement: Option<::Value<SizeConstraintStatement>> = None;
                    let mut sqli_match_statement: Option<::Value<SqliMatchStatement>> = None;
                    let mut xss_match_statement: Option<::Value<XssMatchStatement>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AndStatement" => {
                                and_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ByteMatchStatement" => {
                                byte_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeoMatchStatement" => {
                                geo_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPSetReferenceStatement" => {
                                ip_set_reference_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelMatchStatement" => {
                                label_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedRuleGroupStatement" => {
                                managed_rule_group_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotStatement" => {
                                not_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrStatement" => {
                                or_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateBasedStatement" => {
                                rate_based_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexMatchStatement" => {
                                regex_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexPatternSetReferenceStatement" => {
                                regex_pattern_set_reference_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleGroupReferenceStatement" => {
                                rule_group_reference_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeConstraintStatement" => {
                                size_constraint_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqliMatchStatement" => {
                                sqli_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "XssMatchStatement" => {
                                xss_match_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Statement {
                        and_statement: and_statement,
                        byte_match_statement: byte_match_statement,
                        geo_match_statement: geo_match_statement,
                        ip_set_reference_statement: ip_set_reference_statement,
                        label_match_statement: label_match_statement,
                        managed_rule_group_statement: managed_rule_group_statement,
                        not_statement: not_statement,
                        or_statement: or_statement,
                        rate_based_statement: rate_based_statement,
                        regex_match_statement: regex_match_statement,
                        regex_pattern_set_reference_statement: regex_pattern_set_reference_statement,
                        rule_group_reference_statement: rule_group_reference_statement,
                        size_constraint_statement: size_constraint_statement,
                        sqli_match_statement: sqli_match_statement,
                        xss_match_statement: xss_match_statement,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.TextTransformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-texttransformation.html) property type.
    #[derive(Debug, Default)]
    pub struct TextTransformation {
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-texttransformation.html#cfn-wafv2-webacl-texttransformation-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-texttransformation.html#cfn-wafv2-webacl-texttransformation-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TextTransformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextTransformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextTransformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextTransformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextTransformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut priority: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextTransformation {
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.VisibilityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-visibilityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VisibilityConfig {
        /// Property [`CloudWatchMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-visibilityconfig.html#cfn-wafv2-webacl-visibilityconfig-cloudwatchmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_metrics_enabled: ::Value<bool>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-visibilityconfig.html#cfn-wafv2-webacl-visibilityconfig-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`SampledRequestsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-visibilityconfig.html#cfn-wafv2-webacl-visibilityconfig-sampledrequestsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sampled_requests_enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for VisibilityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchMetricsEnabled", &self.cloud_watch_metrics_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampledRequestsEnabled", &self.sampled_requests_enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VisibilityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VisibilityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VisibilityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VisibilityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_metrics_enabled: Option<::Value<bool>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut sampled_requests_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchMetricsEnabled" => {
                                cloud_watch_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampledRequestsEnabled" => {
                                sampled_requests_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VisibilityConfig {
                        cloud_watch_metrics_enabled: cloud_watch_metrics_enabled.ok_or(::serde::de::Error::missing_field("CloudWatchMetricsEnabled"))?,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        sampled_requests_enabled: sampled_requests_enabled.ok_or(::serde::de::Error::missing_field("SampledRequestsEnabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WAFv2::WebACL.XssMatchStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-xssmatchstatement.html) property type.
    #[derive(Debug, Default)]
    pub struct XssMatchStatement {
        /// Property [`FieldToMatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-xssmatchstatement.html#cfn-wafv2-webacl-xssmatchstatement-fieldtomatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_to_match: ::Value<FieldToMatch>,
        /// Property [`TextTransformations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-xssmatchstatement.html#cfn-wafv2-webacl-xssmatchstatement-texttransformations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_transformations: ::ValueList<TextTransformation>,
    }

    impl ::codec::SerializeValue for XssMatchStatement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldToMatch", &self.field_to_match)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextTransformations", &self.text_transformations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for XssMatchStatement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<XssMatchStatement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = XssMatchStatement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type XssMatchStatement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_to_match: Option<::Value<FieldToMatch>> = None;
                    let mut text_transformations: Option<::ValueList<TextTransformation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldToMatch" => {
                                field_to_match = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextTransformations" => {
                                text_transformations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(XssMatchStatement {
                        field_to_match: field_to_match.ok_or(::serde::de::Error::missing_field("FieldToMatch"))?,
                        text_transformations: text_transformations.ok_or(::serde::de::Error::missing_field("TextTransformations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
