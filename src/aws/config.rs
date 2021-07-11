//! Types for the `Config` service.

/// The [`AWS::Config::AggregationAuthorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-aggregationauthorization.html) resource type.
#[derive(Debug, Default)]
pub struct AggregationAuthorization {
    properties: AggregationAuthorizationProperties
}

/// Properties for the `AggregationAuthorization` resource.
#[derive(Debug, Default)]
pub struct AggregationAuthorizationProperties {
    /// Property [`AuthorizedAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-aggregationauthorization.html#cfn-config-aggregationauthorization-authorizedaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authorized_account_id: ::Value<String>,
    /// Property [`AuthorizedAwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-aggregationauthorization.html#cfn-config-aggregationauthorization-authorizedawsregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authorized_aws_region: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-aggregationauthorization.html#cfn-config-aggregationauthorization-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AggregationAuthorizationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizedAccountId", &self.authorized_account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizedAwsRegion", &self.authorized_aws_region)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AggregationAuthorizationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AggregationAuthorizationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AggregationAuthorizationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AggregationAuthorizationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authorized_account_id: Option<::Value<String>> = None;
                let mut authorized_aws_region: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthorizedAccountId" => {
                            authorized_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizedAwsRegion" => {
                            authorized_aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AggregationAuthorizationProperties {
                    authorized_account_id: authorized_account_id.ok_or(::serde::de::Error::missing_field("AuthorizedAccountId"))?,
                    authorized_aws_region: authorized_aws_region.ok_or(::serde::de::Error::missing_field("AuthorizedAwsRegion"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AggregationAuthorization {
    type Properties = AggregationAuthorizationProperties;
    const TYPE: &'static str = "AWS::Config::AggregationAuthorization";
    fn properties(&self) -> &AggregationAuthorizationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AggregationAuthorizationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AggregationAuthorization {}

impl From<AggregationAuthorizationProperties> for AggregationAuthorization {
    fn from(properties: AggregationAuthorizationProperties) -> AggregationAuthorization {
        AggregationAuthorization { properties }
    }
}

/// The [`AWS::Config::ConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigRule {
    properties: ConfigRuleProperties
}

/// Properties for the `ConfigRule` resource.
#[derive(Debug, Default)]
pub struct ConfigRuleProperties {
    /// Property [`ConfigRuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-configrulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub config_rule_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-inputparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_parameters: Option<::Value<::json::Value>>,
    /// Property [`MaximumExecutionFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-maximumexecutionfrequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_execution_frequency: Option<::Value<String>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-scope).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scope: Option<::Value<self::config_rule::Scope>>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configrule.html#cfn-config-configrule-source).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source: ::Value<self::config_rule::Source>,
}

impl ::serde::Serialize for ConfigRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref config_rule_name) = self.config_rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigRuleName", config_rule_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref input_parameters) = self.input_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParameters", input_parameters)?;
        }
        if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
        }
        if let Some(ref scope) = self.scope {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config_rule_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut input_parameters: Option<::Value<::json::Value>> = None;
                let mut maximum_execution_frequency: Option<::Value<String>> = None;
                let mut scope: Option<::Value<self::config_rule::Scope>> = None;
                let mut source: Option<::Value<self::config_rule::Source>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigRuleName" => {
                            config_rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputParameters" => {
                            input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumExecutionFrequency" => {
                            maximum_execution_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigRuleProperties {
                    config_rule_name: config_rule_name,
                    description: description,
                    input_parameters: input_parameters,
                    maximum_execution_frequency: maximum_execution_frequency,
                    scope: scope,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigRule {
    type Properties = ConfigRuleProperties;
    const TYPE: &'static str = "AWS::Config::ConfigRule";
    fn properties(&self) -> &ConfigRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigRule {}

impl From<ConfigRuleProperties> for ConfigRule {
    fn from(properties: ConfigRuleProperties) -> ConfigRule {
        ConfigRule { properties }
    }
}

/// The [`AWS::Config::ConfigurationAggregator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationAggregator {
    properties: ConfigurationAggregatorProperties
}

/// Properties for the `ConfigurationAggregator` resource.
#[derive(Debug, Default)]
pub struct ConfigurationAggregatorProperties {
    /// Property [`AccountAggregationSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html#cfn-config-configurationaggregator-accountaggregationsources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_aggregation_sources: Option<::ValueList<self::configuration_aggregator::AccountAggregationSource>>,
    /// Property [`ConfigurationAggregatorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html#cfn-config-configurationaggregator-configurationaggregatorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration_aggregator_name: Option<::Value<String>>,
    /// Property [`OrganizationAggregationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html#cfn-config-configurationaggregator-organizationaggregationsource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organization_aggregation_source: Option<::Value<self::configuration_aggregator::OrganizationAggregationSource>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationaggregator.html#cfn-config-configurationaggregator-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConfigurationAggregatorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref account_aggregation_sources) = self.account_aggregation_sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountAggregationSources", account_aggregation_sources)?;
        }
        if let Some(ref configuration_aggregator_name) = self.configuration_aggregator_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationAggregatorName", configuration_aggregator_name)?;
        }
        if let Some(ref organization_aggregation_source) = self.organization_aggregation_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationAggregationSource", organization_aggregation_source)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationAggregatorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationAggregatorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationAggregatorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationAggregatorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_aggregation_sources: Option<::ValueList<self::configuration_aggregator::AccountAggregationSource>> = None;
                let mut configuration_aggregator_name: Option<::Value<String>> = None;
                let mut organization_aggregation_source: Option<::Value<self::configuration_aggregator::OrganizationAggregationSource>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountAggregationSources" => {
                            account_aggregation_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationAggregatorName" => {
                            configuration_aggregator_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationAggregationSource" => {
                            organization_aggregation_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationAggregatorProperties {
                    account_aggregation_sources: account_aggregation_sources,
                    configuration_aggregator_name: configuration_aggregator_name,
                    organization_aggregation_source: organization_aggregation_source,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationAggregator {
    type Properties = ConfigurationAggregatorProperties;
    const TYPE: &'static str = "AWS::Config::ConfigurationAggregator";
    fn properties(&self) -> &ConfigurationAggregatorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationAggregatorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationAggregator {}

impl From<ConfigurationAggregatorProperties> for ConfigurationAggregator {
    fn from(properties: ConfigurationAggregatorProperties) -> ConfigurationAggregator {
        ConfigurationAggregator { properties }
    }
}

/// The [`AWS::Config::ConfigurationRecorder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationRecorder {
    properties: ConfigurationRecorderProperties
}

/// Properties for the `ConfigurationRecorder` resource.
#[derive(Debug, Default)]
pub struct ConfigurationRecorderProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html#cfn-config-configurationrecorder-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html#cfn-config-configurationrecorder-recordinggroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recording_group: Option<::Value<self::configuration_recorder::RecordingGroup>>,
    /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-configurationrecorder.html#cfn-config-configurationrecorder-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for ConfigurationRecorderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref recording_group) = self.recording_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordingGroup", recording_group)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationRecorderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationRecorderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationRecorderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationRecorderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut recording_group: Option<::Value<self::configuration_recorder::RecordingGroup>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecordingGroup" => {
                            recording_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleARN" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationRecorderProperties {
                    name: name,
                    recording_group: recording_group,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationRecorder {
    type Properties = ConfigurationRecorderProperties;
    const TYPE: &'static str = "AWS::Config::ConfigurationRecorder";
    fn properties(&self) -> &ConfigurationRecorderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationRecorderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationRecorder {}

impl From<ConfigurationRecorderProperties> for ConfigurationRecorder {
    fn from(properties: ConfigurationRecorderProperties) -> ConfigurationRecorder {
        ConfigurationRecorder { properties }
    }
}

/// The [`AWS::Config::ConformancePack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html) resource type.
#[derive(Debug, Default)]
pub struct ConformancePack {
    properties: ConformancePackProperties
}

/// Properties for the `ConformancePack` resource.
#[derive(Debug, Default)]
pub struct ConformancePackProperties {
    /// Property [`ConformancePackInputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-conformancepackinputparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub conformance_pack_input_parameters: Option<::ValueList<self::conformance_pack::ConformancePackInputParameter>>,
    /// Property [`ConformancePackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-conformancepackname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub conformance_pack_name: ::Value<String>,
    /// Property [`DeliveryS3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-deliverys3bucket).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_s3_bucket: Option<::Value<String>>,
    /// Property [`DeliveryS3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-deliverys3keyprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_s3_key_prefix: Option<::Value<String>>,
    /// Property [`TemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-templatebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_body: Option<::Value<String>>,
    /// Property [`TemplateS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-conformancepack.html#cfn-config-conformancepack-templates3uri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_s3_uri: Option<::Value<String>>,
}

impl ::serde::Serialize for ConformancePackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref conformance_pack_input_parameters) = self.conformance_pack_input_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConformancePackInputParameters", conformance_pack_input_parameters)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConformancePackName", &self.conformance_pack_name)?;
        if let Some(ref delivery_s3_bucket) = self.delivery_s3_bucket {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryS3Bucket", delivery_s3_bucket)?;
        }
        if let Some(ref delivery_s3_key_prefix) = self.delivery_s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryS3KeyPrefix", delivery_s3_key_prefix)?;
        }
        if let Some(ref template_body) = self.template_body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateBody", template_body)?;
        }
        if let Some(ref template_s3_uri) = self.template_s3_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateS3Uri", template_s3_uri)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConformancePackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConformancePackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConformancePackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConformancePackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut conformance_pack_input_parameters: Option<::ValueList<self::conformance_pack::ConformancePackInputParameter>> = None;
                let mut conformance_pack_name: Option<::Value<String>> = None;
                let mut delivery_s3_bucket: Option<::Value<String>> = None;
                let mut delivery_s3_key_prefix: Option<::Value<String>> = None;
                let mut template_body: Option<::Value<String>> = None;
                let mut template_s3_uri: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConformancePackInputParameters" => {
                            conformance_pack_input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConformancePackName" => {
                            conformance_pack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryS3Bucket" => {
                            delivery_s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryS3KeyPrefix" => {
                            delivery_s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateBody" => {
                            template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateS3Uri" => {
                            template_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConformancePackProperties {
                    conformance_pack_input_parameters: conformance_pack_input_parameters,
                    conformance_pack_name: conformance_pack_name.ok_or(::serde::de::Error::missing_field("ConformancePackName"))?,
                    delivery_s3_bucket: delivery_s3_bucket,
                    delivery_s3_key_prefix: delivery_s3_key_prefix,
                    template_body: template_body,
                    template_s3_uri: template_s3_uri,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConformancePack {
    type Properties = ConformancePackProperties;
    const TYPE: &'static str = "AWS::Config::ConformancePack";
    fn properties(&self) -> &ConformancePackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConformancePackProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConformancePack {}

impl From<ConformancePackProperties> for ConformancePack {
    fn from(properties: ConformancePackProperties) -> ConformancePack {
        ConformancePack { properties }
    }
}

/// The [`AWS::Config::DeliveryChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html) resource type.
#[derive(Debug, Default)]
pub struct DeliveryChannel {
    properties: DeliveryChannelProperties
}

/// Properties for the `DeliveryChannel` resource.
#[derive(Debug, Default)]
pub struct DeliveryChannelProperties {
    /// Property [`ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-configsnapshotdeliveryproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config_snapshot_delivery_properties: Option<::Value<self::delivery_channel::ConfigSnapshotDeliveryProperties>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-s3bucketname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_bucket_name: ::Value<String>,
    /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-s3keyprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_key_prefix: Option<::Value<String>>,
    /// Property [`S3KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-s3kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_kms_key_arn: Option<::Value<String>>,
    /// Property [`SnsTopicARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-deliverychannel.html#cfn-config-deliverychannel-snstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DeliveryChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref config_snapshot_delivery_properties) = self.config_snapshot_delivery_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigSnapshotDeliveryProperties", config_snapshot_delivery_properties)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
        if let Some(ref s3_key_prefix) = self.s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
        }
        if let Some(ref s3_kms_key_arn) = self.s3_kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KmsKeyArn", s3_kms_key_arn)?;
        }
        if let Some(ref sns_topic_arn) = self.sns_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicARN", sns_topic_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeliveryChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeliveryChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeliveryChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeliveryChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config_snapshot_delivery_properties: Option<::Value<self::delivery_channel::ConfigSnapshotDeliveryProperties>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut s3_bucket_name: Option<::Value<String>> = None;
                let mut s3_key_prefix: Option<::Value<String>> = None;
                let mut s3_kms_key_arn: Option<::Value<String>> = None;
                let mut sns_topic_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigSnapshotDeliveryProperties" => {
                            config_snapshot_delivery_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3BucketName" => {
                            s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3KeyPrefix" => {
                            s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3KmsKeyArn" => {
                            s3_kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicARN" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeliveryChannelProperties {
                    config_snapshot_delivery_properties: config_snapshot_delivery_properties,
                    name: name,
                    s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                    s3_key_prefix: s3_key_prefix,
                    s3_kms_key_arn: s3_kms_key_arn,
                    sns_topic_arn: sns_topic_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeliveryChannel {
    type Properties = DeliveryChannelProperties;
    const TYPE: &'static str = "AWS::Config::DeliveryChannel";
    fn properties(&self) -> &DeliveryChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeliveryChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeliveryChannel {}

impl From<DeliveryChannelProperties> for DeliveryChannel {
    fn from(properties: DeliveryChannelProperties) -> DeliveryChannel {
        DeliveryChannel { properties }
    }
}

/// The [`AWS::Config::OrganizationConfigRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html) resource type.
#[derive(Debug, Default)]
pub struct OrganizationConfigRule {
    properties: OrganizationConfigRuleProperties
}

/// Properties for the `OrganizationConfigRule` resource.
#[derive(Debug, Default)]
pub struct OrganizationConfigRuleProperties {
    /// Property [`ExcludedAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html#cfn-config-organizationconfigrule-excludedaccounts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub excluded_accounts: Option<::ValueList<String>>,
    /// Property [`OrganizationConfigRuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html#cfn-config-organizationconfigrule-organizationconfigrulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub organization_config_rule_name: ::Value<String>,
    /// Property [`OrganizationCustomRuleMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organization_custom_rule_metadata: Option<::Value<self::organization_config_rule::OrganizationCustomRuleMetadata>>,
    /// Property [`OrganizationManagedRuleMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconfigrule.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub organization_managed_rule_metadata: Option<::Value<self::organization_config_rule::OrganizationManagedRuleMetadata>>,
}

impl ::serde::Serialize for OrganizationConfigRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref excluded_accounts) = self.excluded_accounts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedAccounts", excluded_accounts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationConfigRuleName", &self.organization_config_rule_name)?;
        if let Some(ref organization_custom_rule_metadata) = self.organization_custom_rule_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationCustomRuleMetadata", organization_custom_rule_metadata)?;
        }
        if let Some(ref organization_managed_rule_metadata) = self.organization_managed_rule_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationManagedRuleMetadata", organization_managed_rule_metadata)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OrganizationConfigRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OrganizationConfigRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OrganizationConfigRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OrganizationConfigRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut excluded_accounts: Option<::ValueList<String>> = None;
                let mut organization_config_rule_name: Option<::Value<String>> = None;
                let mut organization_custom_rule_metadata: Option<::Value<self::organization_config_rule::OrganizationCustomRuleMetadata>> = None;
                let mut organization_managed_rule_metadata: Option<::Value<self::organization_config_rule::OrganizationManagedRuleMetadata>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ExcludedAccounts" => {
                            excluded_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationConfigRuleName" => {
                            organization_config_rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationCustomRuleMetadata" => {
                            organization_custom_rule_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationManagedRuleMetadata" => {
                            organization_managed_rule_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OrganizationConfigRuleProperties {
                    excluded_accounts: excluded_accounts,
                    organization_config_rule_name: organization_config_rule_name.ok_or(::serde::de::Error::missing_field("OrganizationConfigRuleName"))?,
                    organization_custom_rule_metadata: organization_custom_rule_metadata,
                    organization_managed_rule_metadata: organization_managed_rule_metadata,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OrganizationConfigRule {
    type Properties = OrganizationConfigRuleProperties;
    const TYPE: &'static str = "AWS::Config::OrganizationConfigRule";
    fn properties(&self) -> &OrganizationConfigRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OrganizationConfigRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OrganizationConfigRule {}

impl From<OrganizationConfigRuleProperties> for OrganizationConfigRule {
    fn from(properties: OrganizationConfigRuleProperties) -> OrganizationConfigRule {
        OrganizationConfigRule { properties }
    }
}

/// The [`AWS::Config::OrganizationConformancePack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html) resource type.
#[derive(Debug, Default)]
pub struct OrganizationConformancePack {
    properties: OrganizationConformancePackProperties
}

/// Properties for the `OrganizationConformancePack` resource.
#[derive(Debug, Default)]
pub struct OrganizationConformancePackProperties {
    /// Property [`ConformancePackInputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-conformancepackinputparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub conformance_pack_input_parameters: Option<::ValueList<self::organization_conformance_pack::ConformancePackInputParameter>>,
    /// Property [`DeliveryS3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-deliverys3bucket).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_s3_bucket: Option<::Value<String>>,
    /// Property [`DeliveryS3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-deliverys3keyprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_s3_key_prefix: Option<::Value<String>>,
    /// Property [`ExcludedAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-excludedaccounts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub excluded_accounts: Option<::ValueList<String>>,
    /// Property [`OrganizationConformancePackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-organizationconformancepackname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub organization_conformance_pack_name: ::Value<String>,
    /// Property [`TemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-templatebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_body: Option<::Value<String>>,
    /// Property [`TemplateS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-organizationconformancepack.html#cfn-config-organizationconformancepack-templates3uri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_s3_uri: Option<::Value<String>>,
}

impl ::serde::Serialize for OrganizationConformancePackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref conformance_pack_input_parameters) = self.conformance_pack_input_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConformancePackInputParameters", conformance_pack_input_parameters)?;
        }
        if let Some(ref delivery_s3_bucket) = self.delivery_s3_bucket {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryS3Bucket", delivery_s3_bucket)?;
        }
        if let Some(ref delivery_s3_key_prefix) = self.delivery_s3_key_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryS3KeyPrefix", delivery_s3_key_prefix)?;
        }
        if let Some(ref excluded_accounts) = self.excluded_accounts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedAccounts", excluded_accounts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationConformancePackName", &self.organization_conformance_pack_name)?;
        if let Some(ref template_body) = self.template_body {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateBody", template_body)?;
        }
        if let Some(ref template_s3_uri) = self.template_s3_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateS3Uri", template_s3_uri)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OrganizationConformancePackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OrganizationConformancePackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OrganizationConformancePackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OrganizationConformancePackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut conformance_pack_input_parameters: Option<::ValueList<self::organization_conformance_pack::ConformancePackInputParameter>> = None;
                let mut delivery_s3_bucket: Option<::Value<String>> = None;
                let mut delivery_s3_key_prefix: Option<::Value<String>> = None;
                let mut excluded_accounts: Option<::ValueList<String>> = None;
                let mut organization_conformance_pack_name: Option<::Value<String>> = None;
                let mut template_body: Option<::Value<String>> = None;
                let mut template_s3_uri: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConformancePackInputParameters" => {
                            conformance_pack_input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryS3Bucket" => {
                            delivery_s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeliveryS3KeyPrefix" => {
                            delivery_s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExcludedAccounts" => {
                            excluded_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OrganizationConformancePackName" => {
                            organization_conformance_pack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateBody" => {
                            template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateS3Uri" => {
                            template_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OrganizationConformancePackProperties {
                    conformance_pack_input_parameters: conformance_pack_input_parameters,
                    delivery_s3_bucket: delivery_s3_bucket,
                    delivery_s3_key_prefix: delivery_s3_key_prefix,
                    excluded_accounts: excluded_accounts,
                    organization_conformance_pack_name: organization_conformance_pack_name.ok_or(::serde::de::Error::missing_field("OrganizationConformancePackName"))?,
                    template_body: template_body,
                    template_s3_uri: template_s3_uri,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OrganizationConformancePack {
    type Properties = OrganizationConformancePackProperties;
    const TYPE: &'static str = "AWS::Config::OrganizationConformancePack";
    fn properties(&self) -> &OrganizationConformancePackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OrganizationConformancePackProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OrganizationConformancePack {}

impl From<OrganizationConformancePackProperties> for OrganizationConformancePack {
    fn from(properties: OrganizationConformancePackProperties) -> OrganizationConformancePack {
        OrganizationConformancePack { properties }
    }
}

/// The [`AWS::Config::RemediationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct RemediationConfiguration {
    properties: RemediationConfigurationProperties
}

/// Properties for the `RemediationConfiguration` resource.
#[derive(Debug, Default)]
pub struct RemediationConfigurationProperties {
    /// Property [`Automatic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-automatic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automatic: Option<::Value<bool>>,
    /// Property [`ConfigRuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-configrulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub config_rule_name: ::Value<String>,
    /// Property [`ExecutionControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-executioncontrols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_controls: Option<::Value<self::remediation_configuration::ExecutionControls>>,
    /// Property [`MaximumAutomaticAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-maximumautomaticattempts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_automatic_attempts: Option<::Value<u32>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::Value<::json::Value>>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-resourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_type: Option<::Value<String>>,
    /// Property [`RetryAttemptSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-retryattemptseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retry_attempt_seconds: Option<::Value<u32>>,
    /// Property [`TargetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-targetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_id: ::Value<String>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-targettype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_type: ::Value<String>,
    /// Property [`TargetVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-remediationconfiguration.html#cfn-config-remediationconfiguration-targetversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_version: Option<::Value<String>>,
}

impl ::serde::Serialize for RemediationConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref automatic) = self.automatic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Automatic", automatic)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigRuleName", &self.config_rule_name)?;
        if let Some(ref execution_controls) = self.execution_controls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionControls", execution_controls)?;
        }
        if let Some(ref maximum_automatic_attempts) = self.maximum_automatic_attempts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumAutomaticAttempts", maximum_automatic_attempts)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref resource_type) = self.resource_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", resource_type)?;
        }
        if let Some(ref retry_attempt_seconds) = self.retry_attempt_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryAttemptSeconds", retry_attempt_seconds)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetId", &self.target_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", &self.target_type)?;
        if let Some(ref target_version) = self.target_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetVersion", target_version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RemediationConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RemediationConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RemediationConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RemediationConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut automatic: Option<::Value<bool>> = None;
                let mut config_rule_name: Option<::Value<String>> = None;
                let mut execution_controls: Option<::Value<self::remediation_configuration::ExecutionControls>> = None;
                let mut maximum_automatic_attempts: Option<::Value<u32>> = None;
                let mut parameters: Option<::Value<::json::Value>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut retry_attempt_seconds: Option<::Value<u32>> = None;
                let mut target_id: Option<::Value<String>> = None;
                let mut target_type: Option<::Value<String>> = None;
                let mut target_version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Automatic" => {
                            automatic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigRuleName" => {
                            config_rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionControls" => {
                            execution_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumAutomaticAttempts" => {
                            maximum_automatic_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetryAttemptSeconds" => {
                            retry_attempt_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetId" => {
                            target_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetVersion" => {
                            target_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RemediationConfigurationProperties {
                    automatic: automatic,
                    config_rule_name: config_rule_name.ok_or(::serde::de::Error::missing_field("ConfigRuleName"))?,
                    execution_controls: execution_controls,
                    maximum_automatic_attempts: maximum_automatic_attempts,
                    parameters: parameters,
                    resource_type: resource_type,
                    retry_attempt_seconds: retry_attempt_seconds,
                    target_id: target_id.ok_or(::serde::de::Error::missing_field("TargetId"))?,
                    target_type: target_type.ok_or(::serde::de::Error::missing_field("TargetType"))?,
                    target_version: target_version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RemediationConfiguration {
    type Properties = RemediationConfigurationProperties;
    const TYPE: &'static str = "AWS::Config::RemediationConfiguration";
    fn properties(&self) -> &RemediationConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RemediationConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RemediationConfiguration {}

impl From<RemediationConfigurationProperties> for RemediationConfiguration {
    fn from(properties: RemediationConfigurationProperties) -> RemediationConfiguration {
        RemediationConfiguration { properties }
    }
}

/// The [`AWS::Config::StoredQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html) resource type.
#[derive(Debug, Default)]
pub struct StoredQuery {
    properties: StoredQueryProperties
}

/// Properties for the `StoredQuery` resource.
#[derive(Debug, Default)]
pub struct StoredQueryProperties {
    /// Property [`QueryDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html#cfn-config-storedquery-querydescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_description: Option<::Value<String>>,
    /// Property [`QueryExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html#cfn-config-storedquery-queryexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_expression: ::Value<String>,
    /// Property [`QueryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html#cfn-config-storedquery-queryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub query_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-config-storedquery.html#cfn-config-storedquery-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StoredQueryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref query_description) = self.query_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryDescription", query_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryExpression", &self.query_expression)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryName", &self.query_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StoredQueryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StoredQueryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StoredQueryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StoredQueryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut query_description: Option<::Value<String>> = None;
                let mut query_expression: Option<::Value<String>> = None;
                let mut query_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "QueryDescription" => {
                            query_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryExpression" => {
                            query_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryName" => {
                            query_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StoredQueryProperties {
                    query_description: query_description,
                    query_expression: query_expression.ok_or(::serde::de::Error::missing_field("QueryExpression"))?,
                    query_name: query_name.ok_or(::serde::de::Error::missing_field("QueryName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StoredQuery {
    type Properties = StoredQueryProperties;
    const TYPE: &'static str = "AWS::Config::StoredQuery";
    fn properties(&self) -> &StoredQueryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StoredQueryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StoredQuery {}

impl From<StoredQueryProperties> for StoredQuery {
    fn from(properties: StoredQueryProperties) -> StoredQuery {
        StoredQuery { properties }
    }
}

pub mod config_rule {
    //! Property types for the `ConfigRule` resource.

    /// The [`AWS::Config::ConfigRule.Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html) property type.
    #[derive(Debug, Default)]
    pub struct Scope {
        /// Property [`ComplianceResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html#cfn-config-configrule-scope-complianceresourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_resource_id: Option<::Value<String>>,
        /// Property [`ComplianceResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html#cfn-config-configrule-scope-complianceresourcetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_resource_types: Option<::ValueList<String>>,
        /// Property [`TagKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html#cfn-config-configrule-scope-tagkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_key: Option<::Value<String>>,
        /// Property [`TagValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-scope.html#cfn-config-configrule-scope-tagvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Scope {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compliance_resource_id) = self.compliance_resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceId", compliance_resource_id)?;
            }
            if let Some(ref compliance_resource_types) = self.compliance_resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceResourceTypes", compliance_resource_types)?;
            }
            if let Some(ref tag_key) = self.tag_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", tag_key)?;
            }
            if let Some(ref tag_value) = self.tag_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValue", tag_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scope {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scope, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scope;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scope")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compliance_resource_id: Option<::Value<String>> = None;
                    let mut compliance_resource_types: Option<::ValueList<String>> = None;
                    let mut tag_key: Option<::Value<String>> = None;
                    let mut tag_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComplianceResourceId" => {
                                compliance_resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceResourceTypes" => {
                                compliance_resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagKey" => {
                                tag_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValue" => {
                                tag_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scope {
                        compliance_resource_id: compliance_resource_id,
                        compliance_resource_types: compliance_resource_types,
                        tag_key: tag_key,
                        tag_value: tag_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::ConfigRule.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html) property type.
    #[derive(Debug, Default)]
    pub struct Source {
        /// Property [`Owner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html#cfn-config-configrule-source-owner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub owner: ::Value<String>,
        /// Property [`SourceDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html#cfn-config-configrule-source-sourcedetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_details: Option<::ValueList<SourceDetail>>,
        /// Property [`SourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source.html#cfn-config-configrule-source-sourceidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", &self.owner)?;
            if let Some(ref source_details) = self.source_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDetails", source_details)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIdentifier", &self.source_identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut owner: Option<::Value<String>> = None;
                    let mut source_details: Option<::ValueList<SourceDetail>> = None;
                    let mut source_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Owner" => {
                                owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceDetails" => {
                                source_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceIdentifier" => {
                                source_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        owner: owner.ok_or(::serde::de::Error::missing_field("Owner"))?,
                        source_details: source_details,
                        source_identifier: source_identifier.ok_or(::serde::de::Error::missing_field("SourceIdentifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::ConfigRule.SourceDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceDetail {
        /// Property [`EventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html#cfn-config-configrule-source-sourcedetail-eventsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_source: ::Value<String>,
        /// Property [`MaximumExecutionFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html#cfn-config-configrule-sourcedetail-maximumexecutionfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_execution_frequency: Option<::Value<String>>,
        /// Property [`MessageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configrule-source-sourcedetails.html#cfn-config-configrule-source-sourcedetail-messagetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceDetail {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSource", &self.event_source)?;
            if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageType", &self.message_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceDetail {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceDetail, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceDetail;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceDetail")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_source: Option<::Value<String>> = None;
                    let mut maximum_execution_frequency: Option<::Value<String>> = None;
                    let mut message_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventSource" => {
                                event_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumExecutionFrequency" => {
                                maximum_execution_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageType" => {
                                message_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceDetail {
                        event_source: event_source.ok_or(::serde::de::Error::missing_field("EventSource"))?,
                        maximum_execution_frequency: maximum_execution_frequency,
                        message_type: message_type.ok_or(::serde::de::Error::missing_field("MessageType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_aggregator {
    //! Property types for the `ConfigurationAggregator` resource.

    /// The [`AWS::Config::ConfigurationAggregator.AccountAggregationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-accountaggregationsource.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountAggregationSource {
        /// Property [`AccountIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-accountaggregationsource.html#cfn-config-configurationaggregator-accountaggregationsource-accountids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_ids: ::ValueList<String>,
        /// Property [`AllAwsRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-accountaggregationsource.html#cfn-config-configurationaggregator-accountaggregationsource-allawsregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all_aws_regions: Option<::Value<bool>>,
        /// Property [`AwsRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-accountaggregationsource.html#cfn-config-configurationaggregator-accountaggregationsource-awsregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_regions: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AccountAggregationSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountIds", &self.account_ids)?;
            if let Some(ref all_aws_regions) = self.all_aws_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllAwsRegions", all_aws_regions)?;
            }
            if let Some(ref aws_regions) = self.aws_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegions", aws_regions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountAggregationSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountAggregationSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountAggregationSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountAggregationSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_ids: Option<::ValueList<String>> = None;
                    let mut all_aws_regions: Option<::Value<bool>> = None;
                    let mut aws_regions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountIds" => {
                                account_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllAwsRegions" => {
                                all_aws_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsRegions" => {
                                aws_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountAggregationSource {
                        account_ids: account_ids.ok_or(::serde::de::Error::missing_field("AccountIds"))?,
                        all_aws_regions: all_aws_regions,
                        aws_regions: aws_regions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::ConfigurationAggregator.OrganizationAggregationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-organizationaggregationsource.html) property type.
    #[derive(Debug, Default)]
    pub struct OrganizationAggregationSource {
        /// Property [`AllAwsRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-organizationaggregationsource.html#cfn-config-configurationaggregator-organizationaggregationsource-allawsregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all_aws_regions: Option<::Value<bool>>,
        /// Property [`AwsRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-organizationaggregationsource.html#cfn-config-configurationaggregator-organizationaggregationsource-awsregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_regions: Option<::ValueList<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationaggregator-organizationaggregationsource.html#cfn-config-configurationaggregator-organizationaggregationsource-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for OrganizationAggregationSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_aws_regions) = self.all_aws_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllAwsRegions", all_aws_regions)?;
            }
            if let Some(ref aws_regions) = self.aws_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegions", aws_regions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrganizationAggregationSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrganizationAggregationSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrganizationAggregationSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrganizationAggregationSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_aws_regions: Option<::Value<bool>> = None;
                    let mut aws_regions: Option<::ValueList<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllAwsRegions" => {
                                all_aws_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsRegions" => {
                                aws_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrganizationAggregationSource {
                        all_aws_regions: all_aws_regions,
                        aws_regions: aws_regions,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_recorder {
    //! Property types for the `ConfigurationRecorder` resource.

    /// The [`AWS::Config::ConfigurationRecorder.RecordingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordingGroup {
        /// Property [`AllSupported`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html#cfn-config-configurationrecorder-recordinggroup-allsupported).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub all_supported: Option<::Value<bool>>,
        /// Property [`IncludeGlobalResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html#cfn-config-configurationrecorder-recordinggroup-includeglobalresourcetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_global_resource_types: Option<::Value<bool>>,
        /// Property [`ResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-configurationrecorder-recordinggroup.html#cfn-config-configurationrecorder-recordinggroup-resourcetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RecordingGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_supported) = self.all_supported {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllSupported", all_supported)?;
            }
            if let Some(ref include_global_resource_types) = self.include_global_resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeGlobalResourceTypes", include_global_resource_types)?;
            }
            if let Some(ref resource_types) = self.resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypes", resource_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordingGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordingGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordingGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordingGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_supported: Option<::Value<bool>> = None;
                    let mut include_global_resource_types: Option<::Value<bool>> = None;
                    let mut resource_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllSupported" => {
                                all_supported = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeGlobalResourceTypes" => {
                                include_global_resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTypes" => {
                                resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordingGroup {
                        all_supported: all_supported,
                        include_global_resource_types: include_global_resource_types,
                        resource_types: resource_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod conformance_pack {
    //! Property types for the `ConformancePack` resource.

    /// The [`AWS::Config::ConformancePack.ConformancePackInputParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-conformancepack-conformancepackinputparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ConformancePackInputParameter {
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-conformancepack-conformancepackinputparameter.html#cfn-config-conformancepack-conformancepackinputparameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-conformancepack-conformancepackinputparameter.html#cfn-config-conformancepack-conformancepackinputparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConformancePackInputParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConformancePackInputParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConformancePackInputParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConformancePackInputParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConformancePackInputParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConformancePackInputParameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod delivery_channel {
    //! Property types for the `DeliveryChannel` resource.

    /// The [`AWS::Config::DeliveryChannel.ConfigSnapshotDeliveryProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigSnapshotDeliveryProperties {
        /// Property [`DeliveryFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-deliverychannel-configsnapshotdeliveryproperties.html#cfn-config-deliverychannel-configsnapshotdeliveryproperties-deliveryfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_frequency: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigSnapshotDeliveryProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delivery_frequency) = self.delivery_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryFrequency", delivery_frequency)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigSnapshotDeliveryProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigSnapshotDeliveryProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigSnapshotDeliveryProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigSnapshotDeliveryProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_frequency: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryFrequency" => {
                                delivery_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigSnapshotDeliveryProperties {
                        delivery_frequency: delivery_frequency,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod organization_config_rule {
    //! Property types for the `OrganizationConfigRule` resource.

    /// The [`AWS::Config::OrganizationConfigRule.OrganizationCustomRuleMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct OrganizationCustomRuleMetadata {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`InputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-inputparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_parameters: Option<::Value<String>>,
        /// Property [`LambdaFunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-lambdafunctionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_arn: ::Value<String>,
        /// Property [`MaximumExecutionFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-maximumexecutionfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_execution_frequency: Option<::Value<String>>,
        /// Property [`OrganizationConfigRuleTriggerTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-organizationconfigruletriggertypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organization_config_rule_trigger_types: ::ValueList<String>,
        /// Property [`ResourceIdScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-resourceidscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id_scope: Option<::Value<String>>,
        /// Property [`ResourceTypesScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-resourcetypesscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_types_scope: Option<::ValueList<String>>,
        /// Property [`TagKeyScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-tagkeyscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_key_scope: Option<::Value<String>>,
        /// Property [`TagValueScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationcustomrulemetadata.html#cfn-config-organizationconfigrule-organizationcustomrulemetadata-tagvaluescope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_value_scope: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OrganizationCustomRuleMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref input_parameters) = self.input_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParameters", input_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionArn", &self.lambda_function_arn)?;
            if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationConfigRuleTriggerTypes", &self.organization_config_rule_trigger_types)?;
            if let Some(ref resource_id_scope) = self.resource_id_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdScope", resource_id_scope)?;
            }
            if let Some(ref resource_types_scope) = self.resource_types_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypesScope", resource_types_scope)?;
            }
            if let Some(ref tag_key_scope) = self.tag_key_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKeyScope", tag_key_scope)?;
            }
            if let Some(ref tag_value_scope) = self.tag_value_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValueScope", tag_value_scope)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrganizationCustomRuleMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrganizationCustomRuleMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrganizationCustomRuleMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrganizationCustomRuleMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut input_parameters: Option<::Value<String>> = None;
                    let mut lambda_function_arn: Option<::Value<String>> = None;
                    let mut maximum_execution_frequency: Option<::Value<String>> = None;
                    let mut organization_config_rule_trigger_types: Option<::ValueList<String>> = None;
                    let mut resource_id_scope: Option<::Value<String>> = None;
                    let mut resource_types_scope: Option<::ValueList<String>> = None;
                    let mut tag_key_scope: Option<::Value<String>> = None;
                    let mut tag_value_scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputParameters" => {
                                input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionArn" => {
                                lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumExecutionFrequency" => {
                                maximum_execution_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationConfigRuleTriggerTypes" => {
                                organization_config_rule_trigger_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceIdScope" => {
                                resource_id_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTypesScope" => {
                                resource_types_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagKeyScope" => {
                                tag_key_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValueScope" => {
                                tag_value_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrganizationCustomRuleMetadata {
                        description: description,
                        input_parameters: input_parameters,
                        lambda_function_arn: lambda_function_arn.ok_or(::serde::de::Error::missing_field("LambdaFunctionArn"))?,
                        maximum_execution_frequency: maximum_execution_frequency,
                        organization_config_rule_trigger_types: organization_config_rule_trigger_types.ok_or(::serde::de::Error::missing_field("OrganizationConfigRuleTriggerTypes"))?,
                        resource_id_scope: resource_id_scope,
                        resource_types_scope: resource_types_scope,
                        tag_key_scope: tag_key_scope,
                        tag_value_scope: tag_value_scope,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::OrganizationConfigRule.OrganizationManagedRuleMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct OrganizationManagedRuleMetadata {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`InputParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-inputparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_parameters: Option<::Value<String>>,
        /// Property [`MaximumExecutionFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-maximumexecutionfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_execution_frequency: Option<::Value<String>>,
        /// Property [`ResourceIdScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-resourceidscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id_scope: Option<::Value<String>>,
        /// Property [`ResourceTypesScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-resourcetypesscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_types_scope: Option<::ValueList<String>>,
        /// Property [`RuleIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-ruleidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_identifier: ::Value<String>,
        /// Property [`TagKeyScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-tagkeyscope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_key_scope: Option<::Value<String>>,
        /// Property [`TagValueScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconfigrule-organizationmanagedrulemetadata.html#cfn-config-organizationconfigrule-organizationmanagedrulemetadata-tagvaluescope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_value_scope: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OrganizationManagedRuleMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref input_parameters) = self.input_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParameters", input_parameters)?;
            }
            if let Some(ref maximum_execution_frequency) = self.maximum_execution_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionFrequency", maximum_execution_frequency)?;
            }
            if let Some(ref resource_id_scope) = self.resource_id_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdScope", resource_id_scope)?;
            }
            if let Some(ref resource_types_scope) = self.resource_types_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypesScope", resource_types_scope)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleIdentifier", &self.rule_identifier)?;
            if let Some(ref tag_key_scope) = self.tag_key_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKeyScope", tag_key_scope)?;
            }
            if let Some(ref tag_value_scope) = self.tag_value_scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValueScope", tag_value_scope)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OrganizationManagedRuleMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OrganizationManagedRuleMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OrganizationManagedRuleMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OrganizationManagedRuleMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut input_parameters: Option<::Value<String>> = None;
                    let mut maximum_execution_frequency: Option<::Value<String>> = None;
                    let mut resource_id_scope: Option<::Value<String>> = None;
                    let mut resource_types_scope: Option<::ValueList<String>> = None;
                    let mut rule_identifier: Option<::Value<String>> = None;
                    let mut tag_key_scope: Option<::Value<String>> = None;
                    let mut tag_value_scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputParameters" => {
                                input_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumExecutionFrequency" => {
                                maximum_execution_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceIdScope" => {
                                resource_id_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTypesScope" => {
                                resource_types_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleIdentifier" => {
                                rule_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagKeyScope" => {
                                tag_key_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValueScope" => {
                                tag_value_scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OrganizationManagedRuleMetadata {
                        description: description,
                        input_parameters: input_parameters,
                        maximum_execution_frequency: maximum_execution_frequency,
                        resource_id_scope: resource_id_scope,
                        resource_types_scope: resource_types_scope,
                        rule_identifier: rule_identifier.ok_or(::serde::de::Error::missing_field("RuleIdentifier"))?,
                        tag_key_scope: tag_key_scope,
                        tag_value_scope: tag_value_scope,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod organization_conformance_pack {
    //! Property types for the `OrganizationConformancePack` resource.

    /// The [`AWS::Config::OrganizationConformancePack.ConformancePackInputParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconformancepack-conformancepackinputparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ConformancePackInputParameter {
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconformancepack-conformancepackinputparameter.html#cfn-config-organizationconformancepack-conformancepackinputparameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-organizationconformancepack-conformancepackinputparameter.html#cfn-config-organizationconformancepack-conformancepackinputparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConformancePackInputParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConformancePackInputParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConformancePackInputParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConformancePackInputParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConformancePackInputParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConformancePackInputParameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod remediation_configuration {
    //! Property types for the `RemediationConfiguration` resource.

    /// The [`AWS::Config::RemediationConfiguration.ExecutionControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-executioncontrols.html) property type.
    #[derive(Debug, Default)]
    pub struct ExecutionControls {
        /// Property [`SsmControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-executioncontrols.html#cfn-config-remediationconfiguration-executioncontrols-ssmcontrols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssm_controls: Option<::Value<SsmControls>>,
    }

    impl ::codec::SerializeValue for ExecutionControls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ssm_controls) = self.ssm_controls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SsmControls", ssm_controls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExecutionControls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExecutionControls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExecutionControls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExecutionControls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ssm_controls: Option<::Value<SsmControls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SsmControls" => {
                                ssm_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExecutionControls {
                        ssm_controls: ssm_controls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::RemediationConfiguration.RemediationParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-remediationparametervalue.html) property type.
    #[derive(Debug, Default)]
    pub struct RemediationParameterValue {
        /// Property [`ResourceValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-remediationparametervalue.html#cfn-config-remediationconfiguration-remediationparametervalue-resourcevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_value: Option<::Value<ResourceValue>>,
        /// Property [`StaticValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-remediationparametervalue.html#cfn-config-remediationconfiguration-remediationparametervalue-staticvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub static_value: Option<::Value<StaticValue>>,
    }

    impl ::codec::SerializeValue for RemediationParameterValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref resource_value) = self.resource_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceValue", resource_value)?;
            }
            if let Some(ref static_value) = self.static_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StaticValue", static_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RemediationParameterValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RemediationParameterValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RemediationParameterValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RemediationParameterValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_value: Option<::Value<ResourceValue>> = None;
                    let mut static_value: Option<::Value<StaticValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceValue" => {
                                resource_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StaticValue" => {
                                static_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RemediationParameterValue {
                        resource_value: resource_value,
                        static_value: static_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::RemediationConfiguration.ResourceValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-resourcevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceValue {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-resourcevalue.html#cfn-config-remediationconfiguration-resourcevalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceValue {
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::RemediationConfiguration.SsmControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-ssmcontrols.html) property type.
    #[derive(Debug, Default)]
    pub struct SsmControls {
        /// Property [`ConcurrentExecutionRatePercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-ssmcontrols.html#cfn-config-remediationconfiguration-ssmcontrols-concurrentexecutionratepercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub concurrent_execution_rate_percentage: Option<::Value<u32>>,
        /// Property [`ErrorPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-ssmcontrols.html#cfn-config-remediationconfiguration-ssmcontrols-errorpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_percentage: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SsmControls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref concurrent_execution_rate_percentage) = self.concurrent_execution_rate_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConcurrentExecutionRatePercentage", concurrent_execution_rate_percentage)?;
            }
            if let Some(ref error_percentage) = self.error_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorPercentage", error_percentage)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SsmControls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SsmControls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SsmControls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SsmControls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut concurrent_execution_rate_percentage: Option<::Value<u32>> = None;
                    let mut error_percentage: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConcurrentExecutionRatePercentage" => {
                                concurrent_execution_rate_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorPercentage" => {
                                error_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SsmControls {
                        concurrent_execution_rate_percentage: concurrent_execution_rate_percentage,
                        error_percentage: error_percentage,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Config::RemediationConfiguration.StaticValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-staticvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct StaticValue {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-config-remediationconfiguration-staticvalue.html#cfn-config-remediationconfiguration-staticvalue-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for StaticValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StaticValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StaticValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StaticValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StaticValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StaticValue {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
