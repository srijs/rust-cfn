//! Types for the `SecurityHub` service.

/// The [`AWS::SecurityHub::AutomationRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html) resource type.
#[derive(Debug, Default)]
pub struct AutomationRule {
    properties: AutomationRuleProperties
}

/// Properties for the `AutomationRule` resource.
#[derive(Debug, Default)]
pub struct AutomationRuleProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: Option<::ValueList<self::automation_rule::AutomationRulesAction>>,
    /// Property [`Criteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-criteria).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub criteria: Option<::Value<self::automation_rule::AutomationRulesFindingFilters>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IsTerminal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-isterminal).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub is_terminal: Option<::Value<bool>>,
    /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-rulename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_name: Option<::Value<String>>,
    /// Property [`RuleOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-ruleorder).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_order: Option<::Value<u32>>,
    /// Property [`RuleStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-rulestatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-automationrule.html#cfn-securityhub-automationrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for AutomationRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref actions) = self.actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
        }
        if let Some(ref criteria) = self.criteria {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criteria", criteria)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref is_terminal) = self.is_terminal {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsTerminal", is_terminal)?;
        }
        if let Some(ref rule_name) = self.rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
        }
        if let Some(ref rule_order) = self.rule_order {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleOrder", rule_order)?;
        }
        if let Some(ref rule_status) = self.rule_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleStatus", rule_status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AutomationRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AutomationRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AutomationRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AutomationRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<self::automation_rule::AutomationRulesAction>> = None;
                let mut criteria: Option<::Value<self::automation_rule::AutomationRulesFindingFilters>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut is_terminal: Option<::Value<bool>> = None;
                let mut rule_name: Option<::Value<String>> = None;
                let mut rule_order: Option<::Value<u32>> = None;
                let mut rule_status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Criteria" => {
                            criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IsTerminal" => {
                            is_terminal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleName" => {
                            rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleOrder" => {
                            rule_order = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleStatus" => {
                            rule_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AutomationRuleProperties {
                    actions: actions,
                    criteria: criteria,
                    description: description,
                    is_terminal: is_terminal,
                    rule_name: rule_name,
                    rule_order: rule_order,
                    rule_status: rule_status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AutomationRule {
    type Properties = AutomationRuleProperties;
    const TYPE: &'static str = "AWS::SecurityHub::AutomationRule";
    fn properties(&self) -> &AutomationRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AutomationRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AutomationRule {}

impl From<AutomationRuleProperties> for AutomationRule {
    fn from(properties: AutomationRuleProperties) -> AutomationRule {
        AutomationRule { properties }
    }
}

/// The [`AWS::SecurityHub::Hub`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html) resource type.
#[derive(Debug, Default)]
pub struct Hub {
    properties: HubProperties
}

/// Properties for the `Hub` resource.
#[derive(Debug, Default)]
pub struct HubProperties {
    /// Property [`AutoEnableControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html#cfn-securityhub-hub-autoenablecontrols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_enable_controls: Option<::Value<bool>>,
    /// Property [`ControlFindingGenerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html#cfn-securityhub-hub-controlfindinggenerator).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub control_finding_generator: Option<::Value<String>>,
    /// Property [`EnableDefaultStandards`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html#cfn-securityhub-hub-enabledefaultstandards).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_default_standards: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-hub.html#cfn-securityhub-hub-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for HubProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_enable_controls) = self.auto_enable_controls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnableControls", auto_enable_controls)?;
        }
        if let Some(ref control_finding_generator) = self.control_finding_generator {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlFindingGenerator", control_finding_generator)?;
        }
        if let Some(ref enable_default_standards) = self.enable_default_standards {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDefaultStandards", enable_default_standards)?;
        }
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
                let mut auto_enable_controls: Option<::Value<bool>> = None;
                let mut control_finding_generator: Option<::Value<String>> = None;
                let mut enable_default_standards: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoEnableControls" => {
                            auto_enable_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ControlFindingGenerator" => {
                            control_finding_generator = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableDefaultStandards" => {
                            enable_default_standards = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HubProperties {
                    auto_enable_controls: auto_enable_controls,
                    control_finding_generator: control_finding_generator,
                    enable_default_standards: enable_default_standards,
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

/// The [`AWS::SecurityHub::Standard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-standard.html) resource type.
#[derive(Debug, Default)]
pub struct Standard {
    properties: StandardProperties
}

/// Properties for the `Standard` resource.
#[derive(Debug, Default)]
pub struct StandardProperties {
    /// Property [`DisabledStandardsControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-standard.html#cfn-securityhub-standard-disabledstandardscontrols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disabled_standards_controls: Option<::ValueList<self::standard::StandardsControl>>,
    /// Property [`StandardsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-securityhub-standard.html#cfn-securityhub-standard-standardsarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub standards_arn: ::Value<String>,
}

impl ::serde::Serialize for StandardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref disabled_standards_controls) = self.disabled_standards_controls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisabledStandardsControls", disabled_standards_controls)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardsArn", &self.standards_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StandardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StandardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StandardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StandardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut disabled_standards_controls: Option<::ValueList<self::standard::StandardsControl>> = None;
                let mut standards_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DisabledStandardsControls" => {
                            disabled_standards_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StandardsArn" => {
                            standards_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StandardProperties {
                    disabled_standards_controls: disabled_standards_controls,
                    standards_arn: standards_arn.ok_or(::serde::de::Error::missing_field("StandardsArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Standard {
    type Properties = StandardProperties;
    const TYPE: &'static str = "AWS::SecurityHub::Standard";
    fn properties(&self) -> &StandardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StandardProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Standard {}

impl From<StandardProperties> for Standard {
    fn from(properties: StandardProperties) -> Standard {
        Standard { properties }
    }
}

pub mod automation_rule {
    //! Property types for the `AutomationRule` resource.

    /// The [`AWS::SecurityHub::AutomationRule.AutomationRulesAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesaction.html) property type.
    #[derive(Debug, Default)]
    pub struct AutomationRulesAction {
        /// Property [`FindingFieldsUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesaction.html#cfn-securityhub-automationrule-automationrulesaction-findingfieldsupdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub finding_fields_update: ::Value<AutomationRulesFindingFieldsUpdate>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesaction.html#cfn-securityhub-automationrule-automationrulesaction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AutomationRulesAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingFieldsUpdate", &self.finding_fields_update)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutomationRulesAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutomationRulesAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutomationRulesAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutomationRulesAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut finding_fields_update: Option<::Value<AutomationRulesFindingFieldsUpdate>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FindingFieldsUpdate" => {
                                finding_fields_update = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutomationRulesAction {
                        finding_fields_update: finding_fields_update.ok_or(::serde::de::Error::missing_field("FindingFieldsUpdate"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.AutomationRulesFindingFieldsUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html) property type.
    #[derive(Debug, Default)]
    pub struct AutomationRulesFindingFieldsUpdate {
        /// Property [`Confidence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-confidence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confidence: Option<::Value<u32>>,
        /// Property [`Criticality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-criticality).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub criticality: Option<::Value<u32>>,
        /// Property [`Note`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-note).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub note: Option<::Value<NoteUpdate>>,
        /// Property [`RelatedFindings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-relatedfindings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub related_findings: Option<::ValueList<RelatedFinding>>,
        /// Property [`Severity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-severity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub severity: Option<::Value<SeverityUpdate>>,
        /// Property [`Types`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-types).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub types: Option<::ValueList<String>>,
        /// Property [`UserDefinedFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-userdefinedfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_defined_fields: Option<::ValueMap<String>>,
        /// Property [`VerificationState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-verificationstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub verification_state: Option<::Value<String>>,
        /// Property [`Workflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfieldsupdate.html#cfn-securityhub-automationrule-automationrulesfindingfieldsupdate-workflow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workflow: Option<::Value<WorkflowUpdate>>,
    }

    impl ::codec::SerializeValue for AutomationRulesFindingFieldsUpdate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref confidence) = self.confidence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Confidence", confidence)?;
            }
            if let Some(ref criticality) = self.criticality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criticality", criticality)?;
            }
            if let Some(ref note) = self.note {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Note", note)?;
            }
            if let Some(ref related_findings) = self.related_findings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelatedFindings", related_findings)?;
            }
            if let Some(ref severity) = self.severity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Severity", severity)?;
            }
            if let Some(ref types) = self.types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Types", types)?;
            }
            if let Some(ref user_defined_fields) = self.user_defined_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserDefinedFields", user_defined_fields)?;
            }
            if let Some(ref verification_state) = self.verification_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerificationState", verification_state)?;
            }
            if let Some(ref workflow) = self.workflow {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Workflow", workflow)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutomationRulesFindingFieldsUpdate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutomationRulesFindingFieldsUpdate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutomationRulesFindingFieldsUpdate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutomationRulesFindingFieldsUpdate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut confidence: Option<::Value<u32>> = None;
                    let mut criticality: Option<::Value<u32>> = None;
                    let mut note: Option<::Value<NoteUpdate>> = None;
                    let mut related_findings: Option<::ValueList<RelatedFinding>> = None;
                    let mut severity: Option<::Value<SeverityUpdate>> = None;
                    let mut types: Option<::ValueList<String>> = None;
                    let mut user_defined_fields: Option<::ValueMap<String>> = None;
                    let mut verification_state: Option<::Value<String>> = None;
                    let mut workflow: Option<::Value<WorkflowUpdate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Confidence" => {
                                confidence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Criticality" => {
                                criticality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Note" => {
                                note = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelatedFindings" => {
                                related_findings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Severity" => {
                                severity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Types" => {
                                types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserDefinedFields" => {
                                user_defined_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VerificationState" => {
                                verification_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Workflow" => {
                                workflow = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutomationRulesFindingFieldsUpdate {
                        confidence: confidence,
                        criticality: criticality,
                        note: note,
                        related_findings: related_findings,
                        severity: severity,
                        types: types,
                        user_defined_fields: user_defined_fields,
                        verification_state: verification_state,
                        workflow: workflow,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.AutomationRulesFindingFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html) property type.
    #[derive(Debug, Default)]
    pub struct AutomationRulesFindingFilters {
        /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-awsaccountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_account_id: Option<::ValueList<StringFilter>>,
        /// Property [`CompanyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-companyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub company_name: Option<::ValueList<StringFilter>>,
        /// Property [`ComplianceAssociatedStandardsId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-complianceassociatedstandardsid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_associated_standards_id: Option<::ValueList<StringFilter>>,
        /// Property [`ComplianceSecurityControlId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-compliancesecuritycontrolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_security_control_id: Option<::ValueList<StringFilter>>,
        /// Property [`ComplianceStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-compliancestatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compliance_status: Option<::ValueList<StringFilter>>,
        /// Property [`Confidence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-confidence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confidence: Option<::ValueList<NumberFilter>>,
        /// Property [`CreatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-createdat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_at: Option<::ValueList<DateFilter>>,
        /// Property [`Criticality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-criticality).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub criticality: Option<::ValueList<NumberFilter>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::ValueList<StringFilter>>,
        /// Property [`FirstObservedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-firstobservedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_observed_at: Option<::ValueList<DateFilter>>,
        /// Property [`GeneratorId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-generatorid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub generator_id: Option<::ValueList<StringFilter>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: Option<::ValueList<StringFilter>>,
        /// Property [`LastObservedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-lastobservedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_observed_at: Option<::ValueList<DateFilter>>,
        /// Property [`NoteText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-notetext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub note_text: Option<::ValueList<StringFilter>>,
        /// Property [`NoteUpdatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-noteupdatedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub note_updated_at: Option<::ValueList<DateFilter>>,
        /// Property [`NoteUpdatedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-noteupdatedby).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub note_updated_by: Option<::ValueList<StringFilter>>,
        /// Property [`ProductArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-productarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub product_arn: Option<::ValueList<StringFilter>>,
        /// Property [`ProductName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-productname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub product_name: Option<::ValueList<StringFilter>>,
        /// Property [`RecordState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-recordstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_state: Option<::ValueList<StringFilter>>,
        /// Property [`RelatedFindingsId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-relatedfindingsid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub related_findings_id: Option<::ValueList<StringFilter>>,
        /// Property [`RelatedFindingsProductArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-relatedfindingsproductarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub related_findings_product_arn: Option<::ValueList<StringFilter>>,
        /// Property [`ResourceDetailsOther`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourcedetailsother).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_details_other: Option<::ValueList<MapFilter>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::ValueList<StringFilter>>,
        /// Property [`ResourcePartition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourcepartition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_partition: Option<::ValueList<StringFilter>>,
        /// Property [`ResourceRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourceregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_region: Option<::ValueList<StringFilter>>,
        /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourcetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_tags: Option<::ValueList<MapFilter>>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-resourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_type: Option<::ValueList<StringFilter>>,
        /// Property [`SeverityLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-severitylabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub severity_label: Option<::ValueList<StringFilter>>,
        /// Property [`SourceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-sourceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_url: Option<::ValueList<StringFilter>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::ValueList<StringFilter>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::ValueList<StringFilter>>,
        /// Property [`UpdatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-updatedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub updated_at: Option<::ValueList<DateFilter>>,
        /// Property [`UserDefinedFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-userdefinedfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_defined_fields: Option<::ValueList<MapFilter>>,
        /// Property [`VerificationState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-verificationstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub verification_state: Option<::ValueList<StringFilter>>,
        /// Property [`WorkflowStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-automationrulesfindingfilters.html#cfn-securityhub-automationrule-automationrulesfindingfilters-workflowstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workflow_status: Option<::ValueList<StringFilter>>,
    }

    impl ::codec::SerializeValue for AutomationRulesFindingFilters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_account_id) = self.aws_account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", aws_account_id)?;
            }
            if let Some(ref company_name) = self.company_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompanyName", company_name)?;
            }
            if let Some(ref compliance_associated_standards_id) = self.compliance_associated_standards_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceAssociatedStandardsId", compliance_associated_standards_id)?;
            }
            if let Some(ref compliance_security_control_id) = self.compliance_security_control_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceSecurityControlId", compliance_security_control_id)?;
            }
            if let Some(ref compliance_status) = self.compliance_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceStatus", compliance_status)?;
            }
            if let Some(ref confidence) = self.confidence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Confidence", confidence)?;
            }
            if let Some(ref created_at) = self.created_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedAt", created_at)?;
            }
            if let Some(ref criticality) = self.criticality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criticality", criticality)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref first_observed_at) = self.first_observed_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstObservedAt", first_observed_at)?;
            }
            if let Some(ref generator_id) = self.generator_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneratorId", generator_id)?;
            }
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref last_observed_at) = self.last_observed_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastObservedAt", last_observed_at)?;
            }
            if let Some(ref note_text) = self.note_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoteText", note_text)?;
            }
            if let Some(ref note_updated_at) = self.note_updated_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoteUpdatedAt", note_updated_at)?;
            }
            if let Some(ref note_updated_by) = self.note_updated_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoteUpdatedBy", note_updated_by)?;
            }
            if let Some(ref product_arn) = self.product_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductArn", product_arn)?;
            }
            if let Some(ref product_name) = self.product_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductName", product_name)?;
            }
            if let Some(ref record_state) = self.record_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordState", record_state)?;
            }
            if let Some(ref related_findings_id) = self.related_findings_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelatedFindingsId", related_findings_id)?;
            }
            if let Some(ref related_findings_product_arn) = self.related_findings_product_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelatedFindingsProductArn", related_findings_product_arn)?;
            }
            if let Some(ref resource_details_other) = self.resource_details_other {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceDetailsOther", resource_details_other)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            if let Some(ref resource_partition) = self.resource_partition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePartition", resource_partition)?;
            }
            if let Some(ref resource_region) = self.resource_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRegion", resource_region)?;
            }
            if let Some(ref resource_tags) = self.resource_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
            }
            if let Some(ref resource_type) = self.resource_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", resource_type)?;
            }
            if let Some(ref severity_label) = self.severity_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SeverityLabel", severity_label)?;
            }
            if let Some(ref source_url) = self.source_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceUrl", source_url)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref updated_at) = self.updated_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdatedAt", updated_at)?;
            }
            if let Some(ref user_defined_fields) = self.user_defined_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserDefinedFields", user_defined_fields)?;
            }
            if let Some(ref verification_state) = self.verification_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerificationState", verification_state)?;
            }
            if let Some(ref workflow_status) = self.workflow_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowStatus", workflow_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutomationRulesFindingFilters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutomationRulesFindingFilters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutomationRulesFindingFilters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutomationRulesFindingFilters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_account_id: Option<::ValueList<StringFilter>> = None;
                    let mut company_name: Option<::ValueList<StringFilter>> = None;
                    let mut compliance_associated_standards_id: Option<::ValueList<StringFilter>> = None;
                    let mut compliance_security_control_id: Option<::ValueList<StringFilter>> = None;
                    let mut compliance_status: Option<::ValueList<StringFilter>> = None;
                    let mut confidence: Option<::ValueList<NumberFilter>> = None;
                    let mut created_at: Option<::ValueList<DateFilter>> = None;
                    let mut criticality: Option<::ValueList<NumberFilter>> = None;
                    let mut description: Option<::ValueList<StringFilter>> = None;
                    let mut first_observed_at: Option<::ValueList<DateFilter>> = None;
                    let mut generator_id: Option<::ValueList<StringFilter>> = None;
                    let mut id: Option<::ValueList<StringFilter>> = None;
                    let mut last_observed_at: Option<::ValueList<DateFilter>> = None;
                    let mut note_text: Option<::ValueList<StringFilter>> = None;
                    let mut note_updated_at: Option<::ValueList<DateFilter>> = None;
                    let mut note_updated_by: Option<::ValueList<StringFilter>> = None;
                    let mut product_arn: Option<::ValueList<StringFilter>> = None;
                    let mut product_name: Option<::ValueList<StringFilter>> = None;
                    let mut record_state: Option<::ValueList<StringFilter>> = None;
                    let mut related_findings_id: Option<::ValueList<StringFilter>> = None;
                    let mut related_findings_product_arn: Option<::ValueList<StringFilter>> = None;
                    let mut resource_details_other: Option<::ValueList<MapFilter>> = None;
                    let mut resource_id: Option<::ValueList<StringFilter>> = None;
                    let mut resource_partition: Option<::ValueList<StringFilter>> = None;
                    let mut resource_region: Option<::ValueList<StringFilter>> = None;
                    let mut resource_tags: Option<::ValueList<MapFilter>> = None;
                    let mut resource_type: Option<::ValueList<StringFilter>> = None;
                    let mut severity_label: Option<::ValueList<StringFilter>> = None;
                    let mut source_url: Option<::ValueList<StringFilter>> = None;
                    let mut title: Option<::ValueList<StringFilter>> = None;
                    let mut r#type: Option<::ValueList<StringFilter>> = None;
                    let mut updated_at: Option<::ValueList<DateFilter>> = None;
                    let mut user_defined_fields: Option<::ValueList<MapFilter>> = None;
                    let mut verification_state: Option<::ValueList<StringFilter>> = None;
                    let mut workflow_status: Option<::ValueList<StringFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccountId" => {
                                aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompanyName" => {
                                company_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceAssociatedStandardsId" => {
                                compliance_associated_standards_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceSecurityControlId" => {
                                compliance_security_control_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComplianceStatus" => {
                                compliance_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Confidence" => {
                                confidence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreatedAt" => {
                                created_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Criticality" => {
                                criticality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstObservedAt" => {
                                first_observed_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeneratorId" => {
                                generator_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastObservedAt" => {
                                last_observed_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoteText" => {
                                note_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoteUpdatedAt" => {
                                note_updated_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoteUpdatedBy" => {
                                note_updated_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProductArn" => {
                                product_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProductName" => {
                                product_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordState" => {
                                record_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelatedFindingsId" => {
                                related_findings_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelatedFindingsProductArn" => {
                                related_findings_product_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceDetailsOther" => {
                                resource_details_other = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourcePartition" => {
                                resource_partition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRegion" => {
                                resource_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTags" => {
                                resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SeverityLabel" => {
                                severity_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceUrl" => {
                                source_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdatedAt" => {
                                updated_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserDefinedFields" => {
                                user_defined_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VerificationState" => {
                                verification_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkflowStatus" => {
                                workflow_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutomationRulesFindingFilters {
                        aws_account_id: aws_account_id,
                        company_name: company_name,
                        compliance_associated_standards_id: compliance_associated_standards_id,
                        compliance_security_control_id: compliance_security_control_id,
                        compliance_status: compliance_status,
                        confidence: confidence,
                        created_at: created_at,
                        criticality: criticality,
                        description: description,
                        first_observed_at: first_observed_at,
                        generator_id: generator_id,
                        id: id,
                        last_observed_at: last_observed_at,
                        note_text: note_text,
                        note_updated_at: note_updated_at,
                        note_updated_by: note_updated_by,
                        product_arn: product_arn,
                        product_name: product_name,
                        record_state: record_state,
                        related_findings_id: related_findings_id,
                        related_findings_product_arn: related_findings_product_arn,
                        resource_details_other: resource_details_other,
                        resource_id: resource_id,
                        resource_partition: resource_partition,
                        resource_region: resource_region,
                        resource_tags: resource_tags,
                        resource_type: resource_type,
                        severity_label: severity_label,
                        source_url: source_url,
                        title: title,
                        r#type: r#type,
                        updated_at: updated_at,
                        user_defined_fields: user_defined_fields,
                        verification_state: verification_state,
                        workflow_status: workflow_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.DateFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-datefilter.html) property type.
    #[derive(Debug, Default)]
    pub struct DateFilter {
        /// Property [`DateRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-datefilter.html#cfn-securityhub-automationrule-datefilter-daterange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_range: Option<::Value<DateRange>>,
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-datefilter.html#cfn-securityhub-automationrule-datefilter-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: Option<::Value<String>>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-datefilter.html#cfn-securityhub-automationrule-datefilter-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DateFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref date_range) = self.date_range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateRange", date_range)?;
            }
            if let Some(ref end) = self.end {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", end)?;
            }
            if let Some(ref start) = self.start {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", start)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DateFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DateFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DateFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DateFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut date_range: Option<::Value<DateRange>> = None;
                    let mut end: Option<::Value<String>> = None;
                    let mut start: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DateRange" => {
                                date_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DateFilter {
                        date_range: date_range,
                        end: end,
                        start: start,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.DateRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-daterange.html) property type.
    #[derive(Debug, Default)]
    pub struct DateRange {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-daterange.html#cfn-securityhub-automationrule-daterange-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-daterange.html#cfn-securityhub-automationrule-daterange-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for DateRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DateRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DateRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DateRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DateRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

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

                    Ok(DateRange {
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.MapFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-mapfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct MapFilter {
        /// Property [`Comparison`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-mapfilter.html#cfn-securityhub-automationrule-mapfilter-comparison).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-mapfilter.html#cfn-securityhub-automationrule-mapfilter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-mapfilter.html#cfn-securityhub-automationrule-mapfilter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MapFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comparison", &self.comparison)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MapFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MapFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MapFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MapFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comparison" => {
                                comparison = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MapFilter {
                        comparison: comparison.ok_or(::serde::de::Error::missing_field("Comparison"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.NoteUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-noteupdate.html) property type.
    #[derive(Debug, Default)]
    pub struct NoteUpdate {
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-noteupdate.html#cfn-securityhub-automationrule-noteupdate-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: ::Value<String>,
        /// Property [`UpdatedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-noteupdate.html#cfn-securityhub-automationrule-noteupdate-updatedby).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub updated_by: ::Value<::json::Value>,
    }

    impl ::codec::SerializeValue for NoteUpdate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", &self.text)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdatedBy", &self.updated_by)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NoteUpdate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NoteUpdate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NoteUpdate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NoteUpdate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text: Option<::Value<String>> = None;
                    let mut updated_by: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdatedBy" => {
                                updated_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NoteUpdate {
                        text: text.ok_or(::serde::de::Error::missing_field("Text"))?,
                        updated_by: updated_by.ok_or(::serde::de::Error::missing_field("UpdatedBy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.NumberFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-numberfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct NumberFilter {
        /// Property [`Eq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-numberfilter.html#cfn-securityhub-automationrule-numberfilter-eq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub eq: Option<::Value<f64>>,
        /// Property [`Gte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-numberfilter.html#cfn-securityhub-automationrule-numberfilter-gte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gte: Option<::Value<f64>>,
        /// Property [`Lte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-numberfilter.html#cfn-securityhub-automationrule-numberfilter-lte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lte: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for NumberFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref eq) = self.eq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Eq", eq)?;
            }
            if let Some(ref gte) = self.gte {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gte", gte)?;
            }
            if let Some(ref lte) = self.lte {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lte", lte)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NumberFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NumberFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NumberFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NumberFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut eq: Option<::Value<f64>> = None;
                    let mut gte: Option<::Value<f64>> = None;
                    let mut lte: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Eq" => {
                                eq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Gte" => {
                                gte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lte" => {
                                lte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NumberFilter {
                        eq: eq,
                        gte: gte,
                        lte: lte,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.RelatedFinding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-relatedfinding.html) property type.
    #[derive(Debug, Default)]
    pub struct RelatedFinding {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-relatedfinding.html#cfn-securityhub-automationrule-relatedfinding-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<::json::Value>,
        /// Property [`ProductArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-relatedfinding.html#cfn-securityhub-automationrule-relatedfinding-productarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub product_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for RelatedFinding {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductArn", &self.product_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelatedFinding {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelatedFinding, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelatedFinding;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelatedFinding")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<::json::Value>> = None;
                    let mut product_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProductArn" => {
                                product_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelatedFinding {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        product_arn: product_arn.ok_or(::serde::de::Error::missing_field("ProductArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.SeverityUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-severityupdate.html) property type.
    #[derive(Debug, Default)]
    pub struct SeverityUpdate {
        /// Property [`Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-severityupdate.html#cfn-securityhub-automationrule-severityupdate-label).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label: Option<::Value<String>>,
        /// Property [`Normalized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-severityupdate.html#cfn-securityhub-automationrule-severityupdate-normalized).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub normalized: Option<::Value<u32>>,
        /// Property [`Product`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-severityupdate.html#cfn-securityhub-automationrule-severityupdate-product).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub product: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for SeverityUpdate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref label) = self.label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Label", label)?;
            }
            if let Some(ref normalized) = self.normalized {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Normalized", normalized)?;
            }
            if let Some(ref product) = self.product {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Product", product)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SeverityUpdate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SeverityUpdate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SeverityUpdate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SeverityUpdate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut label: Option<::Value<String>> = None;
                    let mut normalized: Option<::Value<u32>> = None;
                    let mut product: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Label" => {
                                label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Normalized" => {
                                normalized = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Product" => {
                                product = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SeverityUpdate {
                        label: label,
                        normalized: normalized,
                        product: product,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.StringFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-stringfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct StringFilter {
        /// Property [`Comparison`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-stringfilter.html#cfn-securityhub-automationrule-stringfilter-comparison).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-stringfilter.html#cfn-securityhub-automationrule-stringfilter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for StringFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comparison", &self.comparison)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StringFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StringFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StringFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StringFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comparison" => {
                                comparison = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StringFilter {
                        comparison: comparison.ok_or(::serde::de::Error::missing_field("Comparison"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecurityHub::AutomationRule.WorkflowUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-workflowupdate.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowUpdate {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-automationrule-workflowupdate.html#cfn-securityhub-automationrule-workflowupdate-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkflowUpdate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowUpdate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowUpdate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowUpdate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowUpdate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowUpdate {
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod standard {
    //! Property types for the `Standard` resource.

    /// The [`AWS::SecurityHub::Standard.StandardsControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-standard-standardscontrol.html) property type.
    #[derive(Debug, Default)]
    pub struct StandardsControl {
        /// Property [`Reason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-standard-standardscontrol.html#cfn-securityhub-standard-standardscontrol-reason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reason: Option<::Value<String>>,
        /// Property [`StandardsControlArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-securityhub-standard-standardscontrol.html#cfn-securityhub-standard-standardscontrol-standardscontrolarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standards_control_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for StandardsControl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref reason) = self.reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reason", reason)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardsControlArn", &self.standards_control_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StandardsControl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StandardsControl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StandardsControl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StandardsControl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reason: Option<::Value<String>> = None;
                    let mut standards_control_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Reason" => {
                                reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardsControlArn" => {
                                standards_control_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StandardsControl {
                        reason: reason,
                        standards_control_arn: standards_control_arn.ok_or(::serde::de::Error::missing_field("StandardsControlArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
