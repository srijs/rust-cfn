//! Types for the `Route53RecoveryControl` service.

/// The [`AWS::Route53RecoveryControl::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-cluster.html#cfn-route53recoverycontrol-cluster-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-cluster.html#cfn-route53recoverycontrol-cluster-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryControl::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::Route53RecoveryControl::ControlPanel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-controlpanel.html) resource type.
#[derive(Debug, Default)]
pub struct ControlPanel {
    properties: ControlPanelProperties
}

/// Properties for the `ControlPanel` resource.
#[derive(Debug, Default)]
pub struct ControlPanelProperties {
    /// Property [`ClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-controlpanel.html#cfn-route53recoverycontrol-controlpanel-clusterarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-controlpanel.html#cfn-route53recoverycontrol-controlpanel-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-controlpanel.html#cfn-route53recoverycontrol-controlpanel-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ControlPanelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cluster_arn) = self.cluster_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterArn", cluster_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ControlPanelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ControlPanelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ControlPanelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ControlPanelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterArn" => {
                            cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ControlPanelProperties {
                    cluster_arn: cluster_arn,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ControlPanel {
    type Properties = ControlPanelProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryControl::ControlPanel";
    fn properties(&self) -> &ControlPanelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ControlPanelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ControlPanel {}

impl From<ControlPanelProperties> for ControlPanel {
    fn from(properties: ControlPanelProperties) -> ControlPanel {
        ControlPanel { properties }
    }
}

/// The [`AWS::Route53RecoveryControl::RoutingControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-routingcontrol.html) resource type.
#[derive(Debug, Default)]
pub struct RoutingControl {
    properties: RoutingControlProperties
}

/// Properties for the `RoutingControl` resource.
#[derive(Debug, Default)]
pub struct RoutingControlProperties {
    /// Property [`ClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-routingcontrol.html#cfn-route53recoverycontrol-routingcontrol-clusterarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_arn: Option<::Value<String>>,
    /// Property [`ControlPanelArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-routingcontrol.html#cfn-route53recoverycontrol-routingcontrol-controlpanelarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub control_panel_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-routingcontrol.html#cfn-route53recoverycontrol-routingcontrol-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for RoutingControlProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cluster_arn) = self.cluster_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterArn", cluster_arn)?;
        }
        if let Some(ref control_panel_arn) = self.control_panel_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlPanelArn", control_panel_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RoutingControlProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingControlProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoutingControlProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RoutingControlProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_arn: Option<::Value<String>> = None;
                let mut control_panel_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterArn" => {
                            cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ControlPanelArn" => {
                            control_panel_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RoutingControlProperties {
                    cluster_arn: cluster_arn,
                    control_panel_arn: control_panel_arn,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RoutingControl {
    type Properties = RoutingControlProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryControl::RoutingControl";
    fn properties(&self) -> &RoutingControlProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RoutingControlProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RoutingControl {}

impl From<RoutingControlProperties> for RoutingControl {
    fn from(properties: RoutingControlProperties) -> RoutingControl {
        RoutingControl { properties }
    }
}

/// The [`AWS::Route53RecoveryControl::SafetyRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html) resource type.
#[derive(Debug, Default)]
pub struct SafetyRule {
    properties: SafetyRuleProperties
}

/// Properties for the `SafetyRule` resource.
#[derive(Debug, Default)]
pub struct SafetyRuleProperties {
    /// Property [`AssertionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-assertionrule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub assertion_rule: Option<::Value<self::safety_rule::AssertionRule>>,
    /// Property [`ControlPanelArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-controlpanelarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub control_panel_arn: ::Value<String>,
    /// Property [`GatingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-gatingrule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gating_rule: Option<::Value<self::safety_rule::GatingRule>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RuleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-ruleconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_config: ::Value<self::safety_rule::RuleConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoverycontrol-safetyrule.html#cfn-route53recoverycontrol-safetyrule-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SafetyRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref assertion_rule) = self.assertion_rule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssertionRule", assertion_rule)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlPanelArn", &self.control_panel_arn)?;
        if let Some(ref gating_rule) = self.gating_rule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatingRule", gating_rule)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleConfig", &self.rule_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SafetyRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SafetyRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SafetyRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SafetyRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assertion_rule: Option<::Value<self::safety_rule::AssertionRule>> = None;
                let mut control_panel_arn: Option<::Value<String>> = None;
                let mut gating_rule: Option<::Value<self::safety_rule::GatingRule>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rule_config: Option<::Value<self::safety_rule::RuleConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssertionRule" => {
                            assertion_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ControlPanelArn" => {
                            control_panel_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GatingRule" => {
                            gating_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleConfig" => {
                            rule_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SafetyRuleProperties {
                    assertion_rule: assertion_rule,
                    control_panel_arn: control_panel_arn.ok_or(::serde::de::Error::missing_field("ControlPanelArn"))?,
                    gating_rule: gating_rule,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rule_config: rule_config.ok_or(::serde::de::Error::missing_field("RuleConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SafetyRule {
    type Properties = SafetyRuleProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryControl::SafetyRule";
    fn properties(&self) -> &SafetyRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SafetyRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SafetyRule {}

impl From<SafetyRuleProperties> for SafetyRule {
    fn from(properties: SafetyRuleProperties) -> SafetyRule {
        SafetyRule { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::Route53RecoveryControl::Cluster.ClusterEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-cluster-clusterendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterEndpoint {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-cluster-clusterendpoint.html#cfn-route53recoverycontrol-cluster-clusterendpoint-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-cluster-clusterendpoint.html#cfn-route53recoverycontrol-cluster-clusterendpoint-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ClusterEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterEndpoint {
                        endpoint: endpoint,
                        region: region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod safety_rule {
    //! Property types for the `SafetyRule` resource.

    /// The [`AWS::Route53RecoveryControl::SafetyRule.AssertionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-assertionrule.html) property type.
    #[derive(Debug, Default)]
    pub struct AssertionRule {
        /// Property [`AssertedControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-assertionrule.html#cfn-route53recoverycontrol-safetyrule-assertionrule-assertedcontrols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asserted_controls: ::ValueList<String>,
        /// Property [`WaitPeriodMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-assertionrule.html#cfn-route53recoverycontrol-safetyrule-assertionrule-waitperiodms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wait_period_ms: ::Value<u32>,
    }

    impl ::codec::SerializeValue for AssertionRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssertedControls", &self.asserted_controls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitPeriodMs", &self.wait_period_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssertionRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssertionRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssertionRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssertionRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut asserted_controls: Option<::ValueList<String>> = None;
                    let mut wait_period_ms: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssertedControls" => {
                                asserted_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitPeriodMs" => {
                                wait_period_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssertionRule {
                        asserted_controls: asserted_controls.ok_or(::serde::de::Error::missing_field("AssertedControls"))?,
                        wait_period_ms: wait_period_ms.ok_or(::serde::de::Error::missing_field("WaitPeriodMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryControl::SafetyRule.GatingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-gatingrule.html) property type.
    #[derive(Debug, Default)]
    pub struct GatingRule {
        /// Property [`GatingControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-gatingrule.html#cfn-route53recoverycontrol-safetyrule-gatingrule-gatingcontrols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gating_controls: ::ValueList<String>,
        /// Property [`TargetControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-gatingrule.html#cfn-route53recoverycontrol-safetyrule-gatingrule-targetcontrols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_controls: ::ValueList<String>,
        /// Property [`WaitPeriodMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-gatingrule.html#cfn-route53recoverycontrol-safetyrule-gatingrule-waitperiodms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wait_period_ms: ::Value<u32>,
    }

    impl ::codec::SerializeValue for GatingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatingControls", &self.gating_controls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetControls", &self.target_controls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitPeriodMs", &self.wait_period_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gating_controls: Option<::ValueList<String>> = None;
                    let mut target_controls: Option<::ValueList<String>> = None;
                    let mut wait_period_ms: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GatingControls" => {
                                gating_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetControls" => {
                                target_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitPeriodMs" => {
                                wait_period_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatingRule {
                        gating_controls: gating_controls.ok_or(::serde::de::Error::missing_field("GatingControls"))?,
                        target_controls: target_controls.ok_or(::serde::de::Error::missing_field("TargetControls"))?,
                        wait_period_ms: wait_period_ms.ok_or(::serde::de::Error::missing_field("WaitPeriodMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryControl::SafetyRule.RuleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-ruleconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleConfig {
        /// Property [`Inverted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-ruleconfig.html#cfn-route53recoverycontrol-safetyrule-ruleconfig-inverted).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inverted: ::Value<bool>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-ruleconfig.html#cfn-route53recoverycontrol-safetyrule-ruleconfig-threshold).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub threshold: ::Value<u32>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoverycontrol-safetyrule-ruleconfig.html#cfn-route53recoverycontrol-safetyrule-ruleconfig-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RuleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inverted", &self.inverted)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut inverted: Option<::Value<bool>> = None;
                    let mut threshold: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Inverted" => {
                                inverted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleConfig {
                        inverted: inverted.ok_or(::serde::de::Error::missing_field("Inverted"))?,
                        threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
