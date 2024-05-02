//! Types for the `NetworkFirewall` service.

/// The [`AWS::NetworkFirewall::Firewall`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html) resource type.
#[derive(Debug, Default)]
pub struct Firewall {
    properties: FirewallProperties
}

/// Properties for the `Firewall` resource.
#[derive(Debug, Default)]
pub struct FirewallProperties {
    /// Property [`DeleteProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-deleteprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_protection: Option<::Value<bool>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FirewallName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-firewallname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub firewall_name: ::Value<String>,
    /// Property [`FirewallPolicyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-firewallpolicyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firewall_policy_arn: ::Value<String>,
    /// Property [`FirewallPolicyChangeProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-firewallpolicychangeprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firewall_policy_change_protection: Option<::Value<bool>>,
    /// Property [`SubnetChangeProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-subnetchangeprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_change_protection: Option<::Value<bool>>,
    /// Property [`SubnetMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-subnetmappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_mappings: ::ValueList<self::firewall::SubnetMapping>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewall.html#cfn-networkfirewall-firewall-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for FirewallProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delete_protection) = self.delete_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteProtection", delete_protection)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallName", &self.firewall_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallPolicyArn", &self.firewall_policy_arn)?;
        if let Some(ref firewall_policy_change_protection) = self.firewall_policy_change_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallPolicyChangeProtection", firewall_policy_change_protection)?;
        }
        if let Some(ref subnet_change_protection) = self.subnet_change_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetChangeProtection", subnet_change_protection)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetMappings", &self.subnet_mappings)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FirewallProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FirewallProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut delete_protection: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut firewall_name: Option<::Value<String>> = None;
                let mut firewall_policy_arn: Option<::Value<String>> = None;
                let mut firewall_policy_change_protection: Option<::Value<bool>> = None;
                let mut subnet_change_protection: Option<::Value<bool>> = None;
                let mut subnet_mappings: Option<::ValueList<self::firewall::SubnetMapping>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeleteProtection" => {
                            delete_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallName" => {
                            firewall_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallPolicyArn" => {
                            firewall_policy_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallPolicyChangeProtection" => {
                            firewall_policy_change_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetChangeProtection" => {
                            subnet_change_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetMappings" => {
                            subnet_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FirewallProperties {
                    delete_protection: delete_protection,
                    description: description,
                    firewall_name: firewall_name.ok_or(::serde::de::Error::missing_field("FirewallName"))?,
                    firewall_policy_arn: firewall_policy_arn.ok_or(::serde::de::Error::missing_field("FirewallPolicyArn"))?,
                    firewall_policy_change_protection: firewall_policy_change_protection,
                    subnet_change_protection: subnet_change_protection,
                    subnet_mappings: subnet_mappings.ok_or(::serde::de::Error::missing_field("SubnetMappings"))?,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Firewall {
    type Properties = FirewallProperties;
    const TYPE: &'static str = "AWS::NetworkFirewall::Firewall";
    fn properties(&self) -> &FirewallProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FirewallProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Firewall {}

impl From<FirewallProperties> for Firewall {
    fn from(properties: FirewallProperties) -> Firewall {
        Firewall { properties }
    }
}

/// The [`AWS::NetworkFirewall::FirewallPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html) resource type.
#[derive(Debug, Default)]
pub struct FirewallPolicy {
    properties: FirewallPolicyProperties
}

/// Properties for the `FirewallPolicy` resource.
#[derive(Debug, Default)]
pub struct FirewallPolicyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FirewallPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firewall_policy: ::Value<self::firewall_policy::FirewallPolicy>,
    /// Property [`FirewallPolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub firewall_policy_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FirewallPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallPolicy", &self.firewall_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallPolicyName", &self.firewall_policy_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FirewallPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FirewallPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut firewall_policy: Option<::Value<self::firewall_policy::FirewallPolicy>> = None;
                let mut firewall_policy_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallPolicy" => {
                            firewall_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallPolicyName" => {
                            firewall_policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FirewallPolicyProperties {
                    description: description,
                    firewall_policy: firewall_policy.ok_or(::serde::de::Error::missing_field("FirewallPolicy"))?,
                    firewall_policy_name: firewall_policy_name.ok_or(::serde::de::Error::missing_field("FirewallPolicyName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FirewallPolicy {
    type Properties = FirewallPolicyProperties;
    const TYPE: &'static str = "AWS::NetworkFirewall::FirewallPolicy";
    fn properties(&self) -> &FirewallPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FirewallPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FirewallPolicy {}

impl From<FirewallPolicyProperties> for FirewallPolicy {
    fn from(properties: FirewallPolicyProperties) -> FirewallPolicy {
        FirewallPolicy { properties }
    }
}

/// The [`AWS::NetworkFirewall::LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-loggingconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct LoggingConfiguration {
    properties: LoggingConfigurationProperties
}

/// Properties for the `LoggingConfiguration` resource.
#[derive(Debug, Default)]
pub struct LoggingConfigurationProperties {
    /// Property [`FirewallArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-loggingconfiguration.html#cfn-networkfirewall-loggingconfiguration-firewallarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub firewall_arn: ::Value<String>,
    /// Property [`FirewallName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-loggingconfiguration.html#cfn-networkfirewall-loggingconfiguration-firewallname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub firewall_name: Option<::Value<String>>,
    /// Property [`LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-loggingconfiguration.html#cfn-networkfirewall-loggingconfiguration-loggingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_configuration: ::Value<self::logging_configuration::LoggingConfiguration>,
}

impl ::serde::Serialize for LoggingConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallArn", &self.firewall_arn)?;
        if let Some(ref firewall_name) = self.firewall_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallName", firewall_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfiguration", &self.logging_configuration)?;
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
                let mut firewall_arn: Option<::Value<String>> = None;
                let mut firewall_name: Option<::Value<String>> = None;
                let mut logging_configuration: Option<::Value<self::logging_configuration::LoggingConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FirewallArn" => {
                            firewall_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirewallName" => {
                            firewall_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfiguration" => {
                            logging_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoggingConfigurationProperties {
                    firewall_arn: firewall_arn.ok_or(::serde::de::Error::missing_field("FirewallArn"))?,
                    firewall_name: firewall_name,
                    logging_configuration: logging_configuration.ok_or(::serde::de::Error::missing_field("LoggingConfiguration"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoggingConfiguration {
    type Properties = LoggingConfigurationProperties;
    const TYPE: &'static str = "AWS::NetworkFirewall::LoggingConfiguration";
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

/// The [`AWS::NetworkFirewall::RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html) resource type.
#[derive(Debug, Default)]
pub struct RuleGroup {
    properties: RuleGroupProperties
}

/// Properties for the `RuleGroup` resource.
#[derive(Debug, Default)]
pub struct RuleGroupProperties {
    /// Property [`Capacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-capacity).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub capacity: ::Value<u32>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_group: Option<::Value<self::rule_group::RuleGroup>>,
    /// Property [`RuleGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_group_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-rulegroup.html#cfn-networkfirewall-rulegroup-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for RuleGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capacity", &self.capacity)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref rule_group) = self.rule_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleGroup", rule_group)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleGroupName", &self.rule_group_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                let mut capacity: Option<::Value<u32>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut rule_group: Option<::Value<self::rule_group::RuleGroup>> = None;
                let mut rule_group_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Capacity" => {
                            capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleGroup" => {
                            rule_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleGroupName" => {
                            rule_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleGroupProperties {
                    capacity: capacity.ok_or(::serde::de::Error::missing_field("Capacity"))?,
                    description: description,
                    rule_group: rule_group,
                    rule_group_name: rule_group_name.ok_or(::serde::de::Error::missing_field("RuleGroupName"))?,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RuleGroup {
    type Properties = RuleGroupProperties;
    const TYPE: &'static str = "AWS::NetworkFirewall::RuleGroup";
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

/// The [`AWS::NetworkFirewall::TLSInspectionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct TLSInspectionConfiguration {
    properties: TLSInspectionConfigurationProperties
}

/// Properties for the `TLSInspectionConfiguration` resource.
#[derive(Debug, Default)]
pub struct TLSInspectionConfigurationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`TLSInspectionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tls_inspection_configuration: ::Value<self::tls_inspection_configuration::TLSInspectionConfiguration>,
    /// Property [`TLSInspectionConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfigurationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tls_inspection_configuration_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkfirewall-tlsinspectionconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TLSInspectionConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLSInspectionConfiguration", &self.tls_inspection_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLSInspectionConfigurationName", &self.tls_inspection_configuration_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TLSInspectionConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TLSInspectionConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TLSInspectionConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TLSInspectionConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut tls_inspection_configuration: Option<::Value<self::tls_inspection_configuration::TLSInspectionConfiguration>> = None;
                let mut tls_inspection_configuration_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TLSInspectionConfiguration" => {
                            tls_inspection_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TLSInspectionConfigurationName" => {
                            tls_inspection_configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TLSInspectionConfigurationProperties {
                    description: description,
                    tls_inspection_configuration: tls_inspection_configuration.ok_or(::serde::de::Error::missing_field("TLSInspectionConfiguration"))?,
                    tls_inspection_configuration_name: tls_inspection_configuration_name.ok_or(::serde::de::Error::missing_field("TLSInspectionConfigurationName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TLSInspectionConfiguration {
    type Properties = TLSInspectionConfigurationProperties;
    const TYPE: &'static str = "AWS::NetworkFirewall::TLSInspectionConfiguration";
    fn properties(&self) -> &TLSInspectionConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TLSInspectionConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TLSInspectionConfiguration {}

impl From<TLSInspectionConfigurationProperties> for TLSInspectionConfiguration {
    fn from(properties: TLSInspectionConfigurationProperties) -> TLSInspectionConfiguration {
        TLSInspectionConfiguration { properties }
    }
}

pub mod firewall {
    //! Property types for the `Firewall` resource.

    /// The [`AWS::NetworkFirewall::Firewall.SubnetMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewall-subnetmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct SubnetMapping {
        /// Property [`IPAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewall-subnetmapping.html#cfn-networkfirewall-firewall-subnetmapping-ipaddresstype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_address_type: Option<::Value<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewall-subnetmapping.html#cfn-networkfirewall-firewall-subnetmapping-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubnetMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip_address_type) = self.ip_address_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPAddressType", ip_address_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubnetMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubnetMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubnetMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubnetMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_address_type: Option<::Value<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IPAddressType" => {
                                ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubnetMapping {
                        ip_address_type: ip_address_type,
                        subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod firewall_policy {
    //! Property types for the `FirewallPolicy` resource.

    /// The [`AWS::NetworkFirewall::FirewallPolicy.ActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-actiondefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionDefinition {
        /// Property [`PublishMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-actiondefinition.html#cfn-networkfirewall-firewallpolicy-actiondefinition-publishmetricaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub publish_metric_action: Option<::Value<PublishMetricAction>>,
    }

    impl ::codec::SerializeValue for ActionDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref publish_metric_action) = self.publish_metric_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishMetricAction", publish_metric_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut publish_metric_action: Option<::Value<PublishMetricAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PublishMetricAction" => {
                                publish_metric_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionDefinition {
                        publish_metric_action: publish_metric_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.CustomAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-customaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomAction {
        /// Property [`ActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-customaction.html#cfn-networkfirewall-firewallpolicy-customaction-actiondefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_definition: ::Value<ActionDefinition>,
        /// Property [`ActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-customaction.html#cfn-networkfirewall-firewallpolicy-customaction-actionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionDefinition", &self.action_definition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionName", &self.action_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_definition: Option<::Value<ActionDefinition>> = None;
                    let mut action_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionDefinition" => {
                                action_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ActionName" => {
                                action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomAction {
                        action_definition: action_definition.ok_or(::serde::de::Error::missing_field("ActionDefinition"))?,
                        action_name: action_name.ok_or(::serde::de::Error::missing_field("ActionName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-dimension.html) property type.
    #[derive(Debug, Default)]
    pub struct Dimension {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-dimension.html#cfn-networkfirewall-firewallpolicy-dimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
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

                    Ok(Dimension {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.FirewallPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct FirewallPolicy {
        /// Property [`PolicyVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-policyvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_variables: Option<::Value<PolicyVariables>>,
        /// Property [`StatefulDefaultActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statefuldefaultactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateful_default_actions: Option<::ValueList<String>>,
        /// Property [`StatefulEngineOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statefulengineoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateful_engine_options: Option<::Value<StatefulEngineOptions>>,
        /// Property [`StatefulRuleGroupReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statefulrulegroupreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateful_rule_group_references: Option<::ValueList<StatefulRuleGroupReference>>,
        /// Property [`StatelessCustomActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statelesscustomactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_custom_actions: Option<::ValueList<CustomAction>>,
        /// Property [`StatelessDefaultActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statelessdefaultactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_default_actions: ::ValueList<String>,
        /// Property [`StatelessFragmentDefaultActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statelessfragmentdefaultactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_fragment_default_actions: ::ValueList<String>,
        /// Property [`StatelessRuleGroupReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-statelessrulegroupreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_rule_group_references: Option<::ValueList<StatelessRuleGroupReference>>,
        /// Property [`TLSInspectionConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-firewallpolicy.html#cfn-networkfirewall-firewallpolicy-firewallpolicy-tlsinspectionconfigurationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_inspection_configuration_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirewallPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref policy_variables) = self.policy_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyVariables", policy_variables)?;
            }
            if let Some(ref stateful_default_actions) = self.stateful_default_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatefulDefaultActions", stateful_default_actions)?;
            }
            if let Some(ref stateful_engine_options) = self.stateful_engine_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatefulEngineOptions", stateful_engine_options)?;
            }
            if let Some(ref stateful_rule_group_references) = self.stateful_rule_group_references {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatefulRuleGroupReferences", stateful_rule_group_references)?;
            }
            if let Some(ref stateless_custom_actions) = self.stateless_custom_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessCustomActions", stateless_custom_actions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessDefaultActions", &self.stateless_default_actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessFragmentDefaultActions", &self.stateless_fragment_default_actions)?;
            if let Some(ref stateless_rule_group_references) = self.stateless_rule_group_references {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessRuleGroupReferences", stateless_rule_group_references)?;
            }
            if let Some(ref tls_inspection_configuration_arn) = self.tls_inspection_configuration_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TLSInspectionConfigurationArn", tls_inspection_configuration_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirewallPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirewallPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirewallPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_variables: Option<::Value<PolicyVariables>> = None;
                    let mut stateful_default_actions: Option<::ValueList<String>> = None;
                    let mut stateful_engine_options: Option<::Value<StatefulEngineOptions>> = None;
                    let mut stateful_rule_group_references: Option<::ValueList<StatefulRuleGroupReference>> = None;
                    let mut stateless_custom_actions: Option<::ValueList<CustomAction>> = None;
                    let mut stateless_default_actions: Option<::ValueList<String>> = None;
                    let mut stateless_fragment_default_actions: Option<::ValueList<String>> = None;
                    let mut stateless_rule_group_references: Option<::ValueList<StatelessRuleGroupReference>> = None;
                    let mut tls_inspection_configuration_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyVariables" => {
                                policy_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatefulDefaultActions" => {
                                stateful_default_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatefulEngineOptions" => {
                                stateful_engine_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatefulRuleGroupReferences" => {
                                stateful_rule_group_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessCustomActions" => {
                                stateless_custom_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessDefaultActions" => {
                                stateless_default_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessFragmentDefaultActions" => {
                                stateless_fragment_default_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessRuleGroupReferences" => {
                                stateless_rule_group_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TLSInspectionConfigurationArn" => {
                                tls_inspection_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirewallPolicy {
                        policy_variables: policy_variables,
                        stateful_default_actions: stateful_default_actions,
                        stateful_engine_options: stateful_engine_options,
                        stateful_rule_group_references: stateful_rule_group_references,
                        stateless_custom_actions: stateless_custom_actions,
                        stateless_default_actions: stateless_default_actions.ok_or(::serde::de::Error::missing_field("StatelessDefaultActions"))?,
                        stateless_fragment_default_actions: stateless_fragment_default_actions.ok_or(::serde::de::Error::missing_field("StatelessFragmentDefaultActions"))?,
                        stateless_rule_group_references: stateless_rule_group_references,
                        tls_inspection_configuration_arn: tls_inspection_configuration_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-ipset.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSet {
        /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-ipset.html#cfn-networkfirewall-firewallpolicy-ipset-definition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub definition: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for IPSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref definition) = self.definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut definition: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Definition" => {
                                definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSet {
                        definition: definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.PolicyVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-policyvariables.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyVariables {
        /// Property [`RuleVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-policyvariables.html#cfn-networkfirewall-firewallpolicy-policyvariables-rulevariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_variables: Option<::ValueMap<IPSet>>,
    }

    impl ::codec::SerializeValue for PolicyVariables {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rule_variables) = self.rule_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleVariables", rule_variables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyVariables {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyVariables, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyVariables;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyVariables")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule_variables: Option<::ValueMap<IPSet>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuleVariables" => {
                                rule_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyVariables {
                        rule_variables: rule_variables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.PublishMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-publishmetricaction.html) property type.
    #[derive(Debug, Default)]
    pub struct PublishMetricAction {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-publishmetricaction.html#cfn-networkfirewall-firewallpolicy-publishmetricaction-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: ::ValueList<Dimension>,
    }

    impl ::codec::SerializeValue for PublishMetricAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublishMetricAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublishMetricAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublishMetricAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublishMetricAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<::ValueList<Dimension>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublishMetricAction {
                        dimensions: dimensions.ok_or(::serde::de::Error::missing_field("Dimensions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.StatefulEngineOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulengineoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct StatefulEngineOptions {
        /// Property [`RuleOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulengineoptions.html#cfn-networkfirewall-firewallpolicy-statefulengineoptions-ruleorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_order: Option<::Value<String>>,
        /// Property [`StreamExceptionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulengineoptions.html#cfn-networkfirewall-firewallpolicy-statefulengineoptions-streamexceptionpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_exception_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatefulEngineOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rule_order) = self.rule_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleOrder", rule_order)?;
            }
            if let Some(ref stream_exception_policy) = self.stream_exception_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamExceptionPolicy", stream_exception_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatefulEngineOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatefulEngineOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatefulEngineOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatefulEngineOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule_order: Option<::Value<String>> = None;
                    let mut stream_exception_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuleOrder" => {
                                rule_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamExceptionPolicy" => {
                                stream_exception_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatefulEngineOptions {
                        rule_order: rule_order,
                        stream_exception_policy: stream_exception_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.StatefulRuleGroupOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct StatefulRuleGroupOverride {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupoverride.html#cfn-networkfirewall-firewallpolicy-statefulrulegroupoverride-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatefulRuleGroupOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref action) = self.action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatefulRuleGroupOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatefulRuleGroupOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatefulRuleGroupOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatefulRuleGroupOverride")
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

                    Ok(StatefulRuleGroupOverride {
                        action: action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.StatefulRuleGroupReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupreference.html) property type.
    #[derive(Debug, Default)]
    pub struct StatefulRuleGroupReference {
        /// Property [`Override`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupreference.html#cfn-networkfirewall-firewallpolicy-statefulrulegroupreference-override).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#override: Option<::Value<StatefulRuleGroupOverride>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupreference.html#cfn-networkfirewall-firewallpolicy-statefulrulegroupreference-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: Option<::Value<u32>>,
        /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statefulrulegroupreference.html#cfn-networkfirewall-firewallpolicy-statefulrulegroupreference-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for StatefulRuleGroupReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#override) = self.r#override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Override", r#override)?;
            }
            if let Some(ref priority) = self.priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatefulRuleGroupReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatefulRuleGroupReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatefulRuleGroupReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatefulRuleGroupReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#override: Option<::Value<StatefulRuleGroupOverride>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Override" => {
                                r#override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceArn" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatefulRuleGroupReference {
                        r#override: r#override,
                        priority: priority,
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::FirewallPolicy.StatelessRuleGroupReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statelessrulegroupreference.html) property type.
    #[derive(Debug, Default)]
    pub struct StatelessRuleGroupReference {
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statelessrulegroupreference.html#cfn-networkfirewall-firewallpolicy-statelessrulegroupreference-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-firewallpolicy-statelessrulegroupreference.html#cfn-networkfirewall-firewallpolicy-statelessrulegroupreference-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for StatelessRuleGroupReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatelessRuleGroupReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatelessRuleGroupReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatelessRuleGroupReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatelessRuleGroupReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut priority: Option<::Value<u32>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceArn" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatelessRuleGroupReference {
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod logging_configuration {
    //! Property types for the `LoggingConfiguration` resource.

    /// The [`AWS::NetworkFirewall::LoggingConfiguration.LogDestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-logdestinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LogDestinationConfig {
        /// Property [`LogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-logdestinationconfig.html#cfn-networkfirewall-loggingconfiguration-logdestinationconfig-logdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_destination: ::ValueMap<String>,
        /// Property [`LogDestinationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-logdestinationconfig.html#cfn-networkfirewall-loggingconfiguration-logdestinationconfig-logdestinationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_destination_type: ::Value<String>,
        /// Property [`LogType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-logdestinationconfig.html#cfn-networkfirewall-loggingconfiguration-logdestinationconfig-logtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for LogDestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDestination", &self.log_destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDestinationType", &self.log_destination_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogType", &self.log_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogDestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogDestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogDestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogDestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_destination: Option<::ValueMap<String>> = None;
                    let mut log_destination_type: Option<::Value<String>> = None;
                    let mut log_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDestination" => {
                                log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogDestinationType" => {
                                log_destination_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogType" => {
                                log_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogDestinationConfig {
                        log_destination: log_destination.ok_or(::serde::de::Error::missing_field("LogDestination"))?,
                        log_destination_type: log_destination_type.ok_or(::serde::de::Error::missing_field("LogDestinationType"))?,
                        log_type: log_type.ok_or(::serde::de::Error::missing_field("LogType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::LoggingConfiguration.LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-loggingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfiguration {
        /// Property [`LogDestinationConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-loggingconfiguration-loggingconfiguration.html#cfn-networkfirewall-loggingconfiguration-loggingconfiguration-logdestinationconfigs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_destination_configs: ::ValueList<LogDestinationConfig>,
    }

    impl ::codec::SerializeValue for LoggingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDestinationConfigs", &self.log_destination_configs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_destination_configs: Option<::ValueList<LogDestinationConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDestinationConfigs" => {
                                log_destination_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfiguration {
                        log_destination_configs: log_destination_configs.ok_or(::serde::de::Error::missing_field("LogDestinationConfigs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rule_group {
    //! Property types for the `RuleGroup` resource.

    /// The [`AWS::NetworkFirewall::RuleGroup.ActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-actiondefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionDefinition {
        /// Property [`PublishMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-actiondefinition.html#cfn-networkfirewall-rulegroup-actiondefinition-publishmetricaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub publish_metric_action: Option<::Value<PublishMetricAction>>,
    }

    impl ::codec::SerializeValue for ActionDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref publish_metric_action) = self.publish_metric_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishMetricAction", publish_metric_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut publish_metric_action: Option<::Value<PublishMetricAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PublishMetricAction" => {
                                publish_metric_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionDefinition {
                        publish_metric_action: publish_metric_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-address.html) property type.
    #[derive(Debug, Default)]
    pub struct Address {
        /// Property [`AddressDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-address.html#cfn-networkfirewall-rulegroup-address-addressdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address_definition: ::Value<String>,
    }

    impl ::codec::SerializeValue for Address {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddressDefinition", &self.address_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Address {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Address;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Address")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address_definition: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddressDefinition" => {
                                address_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Address {
                        address_definition: address_definition.ok_or(::serde::de::Error::missing_field("AddressDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.CustomAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-customaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomAction {
        /// Property [`ActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-customaction.html#cfn-networkfirewall-rulegroup-customaction-actiondefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_definition: ::Value<ActionDefinition>,
        /// Property [`ActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-customaction.html#cfn-networkfirewall-rulegroup-customaction-actionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionDefinition", &self.action_definition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionName", &self.action_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action_definition: Option<::Value<ActionDefinition>> = None;
                    let mut action_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActionDefinition" => {
                                action_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ActionName" => {
                                action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomAction {
                        action_definition: action_definition.ok_or(::serde::de::Error::missing_field("ActionDefinition"))?,
                        action_name: action_name.ok_or(::serde::de::Error::missing_field("ActionName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-dimension.html) property type.
    #[derive(Debug, Default)]
    pub struct Dimension {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-dimension.html#cfn-networkfirewall-rulegroup-dimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Dimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Dimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Dimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Dimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Dimension")
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

                    Ok(Dimension {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html) property type.
    #[derive(Debug, Default)]
    pub struct Header {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<String>,
        /// Property [`DestinationPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-destinationport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_port: ::Value<String>,
        /// Property [`Direction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-direction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub direction: ::Value<String>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<String>,
        /// Property [`SourcePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-header.html#cfn-networkfirewall-rulegroup-header-sourceport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_port: ::Value<String>,
    }

    impl ::codec::SerializeValue for Header {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPort", &self.destination_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Direction", &self.direction)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePort", &self.source_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Header {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Header, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Header;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Header")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;
                    let mut destination_port: Option<::Value<String>> = None;
                    let mut direction: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut source_port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationPort" => {
                                destination_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Direction" => {
                                direction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePort" => {
                                source_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Header {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        destination_port: destination_port.ok_or(::serde::de::Error::missing_field("DestinationPort"))?,
                        direction: direction.ok_or(::serde::de::Error::missing_field("Direction"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                        source_port: source_port.ok_or(::serde::de::Error::missing_field("SourcePort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.IPSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipset.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSet {
        /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipset.html#cfn-networkfirewall-rulegroup-ipset-definition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub definition: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for IPSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref definition) = self.definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut definition: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Definition" => {
                                definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSet {
                        definition: definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.IPSetReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipsetreference.html) property type.
    #[derive(Debug, Default)]
    pub struct IPSetReference {
        /// Property [`ReferenceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ipsetreference.html#cfn-networkfirewall-rulegroup-ipsetreference-referencearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reference_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IPSetReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref reference_arn) = self.reference_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceArn", reference_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IPSetReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IPSetReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IPSetReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IPSetReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reference_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceArn" => {
                                reference_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IPSetReference {
                        reference_arn: reference_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.MatchAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct MatchAttributes {
        /// Property [`DestinationPorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-destinationports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_ports: Option<::ValueList<PortRange>>,
        /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-destinations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destinations: Option<::ValueList<Address>>,
        /// Property [`Protocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-protocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocols: Option<::ValueList<u32>>,
        /// Property [`SourcePorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-sourceports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_ports: Option<::ValueList<PortRange>>,
        /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-sources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sources: Option<::ValueList<Address>>,
        /// Property [`TCPFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-matchattributes.html#cfn-networkfirewall-rulegroup-matchattributes-tcpflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tcp_flags: Option<::ValueList<TCPFlagField>>,
    }

    impl ::codec::SerializeValue for MatchAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_ports) = self.destination_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPorts", destination_ports)?;
            }
            if let Some(ref destinations) = self.destinations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
            }
            if let Some(ref protocols) = self.protocols {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocols", protocols)?;
            }
            if let Some(ref source_ports) = self.source_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePorts", source_ports)?;
            }
            if let Some(ref sources) = self.sources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
            }
            if let Some(ref tcp_flags) = self.tcp_flags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TCPFlags", tcp_flags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MatchAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MatchAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MatchAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_ports: Option<::ValueList<PortRange>> = None;
                    let mut destinations: Option<::ValueList<Address>> = None;
                    let mut protocols: Option<::ValueList<u32>> = None;
                    let mut source_ports: Option<::ValueList<PortRange>> = None;
                    let mut sources: Option<::ValueList<Address>> = None;
                    let mut tcp_flags: Option<::ValueList<TCPFlagField>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPorts" => {
                                destination_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destinations" => {
                                destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocols" => {
                                protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePorts" => {
                                source_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sources" => {
                                sources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TCPFlags" => {
                                tcp_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MatchAttributes {
                        destination_ports: destination_ports,
                        destinations: destinations,
                        protocols: protocols,
                        source_ports: source_ports,
                        sources: sources,
                        tcp_flags: tcp_flags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portrange.html) property type.
    #[derive(Debug, Default)]
    pub struct PortRange {
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portrange.html#cfn-networkfirewall-rulegroup-portrange-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: ::Value<u32>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portrange.html#cfn-networkfirewall-rulegroup-portrange-toport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub to_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for PortRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from_port: Option<::Value<u32>> = None;
                    let mut to_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FromPort" => {
                                from_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ToPort" => {
                                to_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortRange {
                        from_port: from_port.ok_or(::serde::de::Error::missing_field("FromPort"))?,
                        to_port: to_port.ok_or(::serde::de::Error::missing_field("ToPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.PortSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portset.html) property type.
    #[derive(Debug, Default)]
    pub struct PortSet {
        /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-portset.html#cfn-networkfirewall-rulegroup-portset-definition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub definition: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PortSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref definition) = self.definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut definition: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Definition" => {
                                definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortSet {
                        definition: definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.PublishMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-publishmetricaction.html) property type.
    #[derive(Debug, Default)]
    pub struct PublishMetricAction {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-publishmetricaction.html#cfn-networkfirewall-rulegroup-publishmetricaction-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: ::ValueList<Dimension>,
    }

    impl ::codec::SerializeValue for PublishMetricAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublishMetricAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublishMetricAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublishMetricAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublishMetricAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions: Option<::ValueList<Dimension>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublishMetricAction {
                        dimensions: dimensions.ok_or(::serde::de::Error::missing_field("Dimensions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.ReferenceSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-referencesets.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferenceSets {
        /// Property [`IPSetReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-referencesets.html#cfn-networkfirewall-rulegroup-referencesets-ipsetreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_set_references: Option<::ValueMap<IPSetReference>>,
    }

    impl ::codec::SerializeValue for ReferenceSets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip_set_references) = self.ip_set_references {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSetReferences", ip_set_references)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferenceSets {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceSets, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferenceSets;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferenceSets")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_set_references: Option<::ValueMap<IPSetReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IPSetReferences" => {
                                ip_set_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferenceSets {
                        ip_set_references: ip_set_references,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RuleDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruledefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleDefinition {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruledefinition.html#cfn-networkfirewall-rulegroup-ruledefinition-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`MatchAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruledefinition.html#cfn-networkfirewall-rulegroup-ruledefinition-matchattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_attributes: ::Value<MatchAttributes>,
    }

    impl ::codec::SerializeValue for RuleDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchAttributes", &self.match_attributes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut match_attributes: Option<::Value<MatchAttributes>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchAttributes" => {
                                match_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleDefinition {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        match_attributes: match_attributes.ok_or(::serde::de::Error::missing_field("MatchAttributes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleGroup {
        /// Property [`ReferenceSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroup-referencesets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reference_sets: Option<::Value<ReferenceSets>>,
        /// Property [`RuleVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroup-rulevariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_variables: Option<::Value<RuleVariables>>,
        /// Property [`RulesSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroup-rulessource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules_source: ::Value<RulesSource>,
        /// Property [`StatefulRuleOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulegroup.html#cfn-networkfirewall-rulegroup-rulegroup-statefulruleoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateful_rule_options: Option<::Value<StatefulRuleOptions>>,
    }

    impl ::codec::SerializeValue for RuleGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref reference_sets) = self.reference_sets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceSets", reference_sets)?;
            }
            if let Some(ref rule_variables) = self.rule_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleVariables", rule_variables)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesSource", &self.rules_source)?;
            if let Some(ref stateful_rule_options) = self.stateful_rule_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatefulRuleOptions", stateful_rule_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reference_sets: Option<::Value<ReferenceSets>> = None;
                    let mut rule_variables: Option<::Value<RuleVariables>> = None;
                    let mut rules_source: Option<::Value<RulesSource>> = None;
                    let mut stateful_rule_options: Option<::Value<StatefulRuleOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceSets" => {
                                reference_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleVariables" => {
                                rule_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RulesSource" => {
                                rules_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatefulRuleOptions" => {
                                stateful_rule_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleGroup {
                        reference_sets: reference_sets,
                        rule_variables: rule_variables,
                        rules_source: rules_source.ok_or(::serde::de::Error::missing_field("RulesSource"))?,
                        stateful_rule_options: stateful_rule_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RuleOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruleoption.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleOption {
        /// Property [`Keyword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruleoption.html#cfn-networkfirewall-rulegroup-ruleoption-keyword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub keyword: ::Value<String>,
        /// Property [`Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-ruleoption.html#cfn-networkfirewall-rulegroup-ruleoption-settings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub settings: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RuleOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Keyword", &self.keyword)?;
            if let Some(ref settings) = self.settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Settings", settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut keyword: Option<::Value<String>> = None;
                    let mut settings: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Keyword" => {
                                keyword = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Settings" => {
                                settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleOption {
                        keyword: keyword.ok_or(::serde::de::Error::missing_field("Keyword"))?,
                        settings: settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RuleVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulevariables.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleVariables {
        /// Property [`IPSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulevariables.html#cfn-networkfirewall-rulegroup-rulevariables-ipsets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_sets: Option<::ValueMap<IPSet>>,
        /// Property [`PortSets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulevariables.html#cfn-networkfirewall-rulegroup-rulevariables-portsets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_sets: Option<::ValueMap<PortSet>>,
    }

    impl ::codec::SerializeValue for RuleVariables {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip_sets) = self.ip_sets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPSets", ip_sets)?;
            }
            if let Some(ref port_sets) = self.port_sets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortSets", port_sets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleVariables {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleVariables, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleVariables;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleVariables")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_sets: Option<::ValueMap<IPSet>> = None;
                    let mut port_sets: Option<::ValueMap<PortSet>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IPSets" => {
                                ip_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortSets" => {
                                port_sets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleVariables {
                        ip_sets: ip_sets,
                        port_sets: port_sets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RulesSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html) property type.
    #[derive(Debug, Default)]
    pub struct RulesSource {
        /// Property [`RulesSourceList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html#cfn-networkfirewall-rulegroup-rulessource-rulessourcelist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules_source_list: Option<::Value<RulesSourceList>>,
        /// Property [`RulesString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html#cfn-networkfirewall-rulegroup-rulessource-rulesstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules_string: Option<::Value<String>>,
        /// Property [`StatefulRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html#cfn-networkfirewall-rulegroup-rulessource-statefulrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateful_rules: Option<::ValueList<StatefulRule>>,
        /// Property [`StatelessRulesAndCustomActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessource.html#cfn-networkfirewall-rulegroup-rulessource-statelessrulesandcustomactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_rules_and_custom_actions: Option<::Value<StatelessRulesAndCustomActions>>,
    }

    impl ::codec::SerializeValue for RulesSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rules_source_list) = self.rules_source_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesSourceList", rules_source_list)?;
            }
            if let Some(ref rules_string) = self.rules_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesString", rules_string)?;
            }
            if let Some(ref stateful_rules) = self.stateful_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatefulRules", stateful_rules)?;
            }
            if let Some(ref stateless_rules_and_custom_actions) = self.stateless_rules_and_custom_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessRulesAndCustomActions", stateless_rules_and_custom_actions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RulesSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RulesSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RulesSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RulesSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rules_source_list: Option<::Value<RulesSourceList>> = None;
                    let mut rules_string: Option<::Value<String>> = None;
                    let mut stateful_rules: Option<::ValueList<StatefulRule>> = None;
                    let mut stateless_rules_and_custom_actions: Option<::Value<StatelessRulesAndCustomActions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RulesSourceList" => {
                                rules_source_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RulesString" => {
                                rules_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatefulRules" => {
                                stateful_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessRulesAndCustomActions" => {
                                stateless_rules_and_custom_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RulesSource {
                        rules_source_list: rules_source_list,
                        rules_string: rules_string,
                        stateful_rules: stateful_rules,
                        stateless_rules_and_custom_actions: stateless_rules_and_custom_actions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.RulesSourceList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessourcelist.html) property type.
    #[derive(Debug, Default)]
    pub struct RulesSourceList {
        /// Property [`GeneratedRulesType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessourcelist.html#cfn-networkfirewall-rulegroup-rulessourcelist-generatedrulestype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub generated_rules_type: ::Value<String>,
        /// Property [`TargetTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessourcelist.html#cfn-networkfirewall-rulegroup-rulessourcelist-targettypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_types: ::ValueList<String>,
        /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-rulessourcelist.html#cfn-networkfirewall-rulegroup-rulessourcelist-targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub targets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for RulesSourceList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneratedRulesType", &self.generated_rules_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTypes", &self.target_types)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RulesSourceList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RulesSourceList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RulesSourceList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RulesSourceList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut generated_rules_type: Option<::Value<String>> = None;
                    let mut target_types: Option<::ValueList<String>> = None;
                    let mut targets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GeneratedRulesType" => {
                                generated_rules_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTypes" => {
                                target_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Targets" => {
                                targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RulesSourceList {
                        generated_rules_type: generated_rules_type.ok_or(::serde::de::Error::missing_field("GeneratedRulesType"))?,
                        target_types: target_types.ok_or(::serde::de::Error::missing_field("TargetTypes"))?,
                        targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.StatefulRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulrule.html) property type.
    #[derive(Debug, Default)]
    pub struct StatefulRule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulrule.html#cfn-networkfirewall-rulegroup-statefulrule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulrule.html#cfn-networkfirewall-rulegroup-statefulrule-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: ::Value<Header>,
        /// Property [`RuleOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulrule.html#cfn-networkfirewall-rulegroup-statefulrule-ruleoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_options: ::ValueList<RuleOption>,
    }

    impl ::codec::SerializeValue for StatefulRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", &self.header)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleOptions", &self.rule_options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatefulRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatefulRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatefulRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatefulRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut header: Option<::Value<Header>> = None;
                    let mut rule_options: Option<::ValueList<RuleOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleOptions" => {
                                rule_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatefulRule {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        header: header.ok_or(::serde::de::Error::missing_field("Header"))?,
                        rule_options: rule_options.ok_or(::serde::de::Error::missing_field("RuleOptions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.StatefulRuleOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulruleoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct StatefulRuleOptions {
        /// Property [`RuleOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statefulruleoptions.html#cfn-networkfirewall-rulegroup-statefulruleoptions-ruleorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_order: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatefulRuleOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rule_order) = self.rule_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleOrder", rule_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatefulRuleOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatefulRuleOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatefulRuleOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatefulRuleOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule_order: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuleOrder" => {
                                rule_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatefulRuleOptions {
                        rule_order: rule_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.StatelessRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrule.html) property type.
    #[derive(Debug, Default)]
    pub struct StatelessRule {
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrule.html#cfn-networkfirewall-rulegroup-statelessrule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`RuleDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrule.html#cfn-networkfirewall-rulegroup-statelessrule-ruledefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_definition: ::Value<RuleDefinition>,
    }

    impl ::codec::SerializeValue for StatelessRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleDefinition", &self.rule_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatelessRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatelessRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatelessRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatelessRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut priority: Option<::Value<u32>> = None;
                    let mut rule_definition: Option<::Value<RuleDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleDefinition" => {
                                rule_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatelessRule {
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        rule_definition: rule_definition.ok_or(::serde::de::Error::missing_field("RuleDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.StatelessRulesAndCustomActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrulesandcustomactions.html) property type.
    #[derive(Debug, Default)]
    pub struct StatelessRulesAndCustomActions {
        /// Property [`CustomActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrulesandcustomactions.html#cfn-networkfirewall-rulegroup-statelessrulesandcustomactions-customactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_actions: Option<::ValueList<CustomAction>>,
        /// Property [`StatelessRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-statelessrulesandcustomactions.html#cfn-networkfirewall-rulegroup-statelessrulesandcustomactions-statelessrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stateless_rules: ::ValueList<StatelessRule>,
    }

    impl ::codec::SerializeValue for StatelessRulesAndCustomActions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_actions) = self.custom_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomActions", custom_actions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatelessRules", &self.stateless_rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatelessRulesAndCustomActions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatelessRulesAndCustomActions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatelessRulesAndCustomActions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatelessRulesAndCustomActions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_actions: Option<::ValueList<CustomAction>> = None;
                    let mut stateless_rules: Option<::ValueList<StatelessRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomActions" => {
                                custom_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatelessRules" => {
                                stateless_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatelessRulesAndCustomActions {
                        custom_actions: custom_actions,
                        stateless_rules: stateless_rules.ok_or(::serde::de::Error::missing_field("StatelessRules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::RuleGroup.TCPFlagField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-tcpflagfield.html) property type.
    #[derive(Debug, Default)]
    pub struct TCPFlagField {
        /// Property [`Flags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-tcpflagfield.html#cfn-networkfirewall-rulegroup-tcpflagfield-flags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flags: ::ValueList<String>,
        /// Property [`Masks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-rulegroup-tcpflagfield.html#cfn-networkfirewall-rulegroup-tcpflagfield-masks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub masks: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TCPFlagField {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Flags", &self.flags)?;
            if let Some(ref masks) = self.masks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Masks", masks)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TCPFlagField {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TCPFlagField, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TCPFlagField;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TCPFlagField")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flags: Option<::ValueList<String>> = None;
                    let mut masks: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Flags" => {
                                flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Masks" => {
                                masks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TCPFlagField {
                        flags: flags.ok_or(::serde::de::Error::missing_field("Flags"))?,
                        masks: masks,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod tls_inspection_configuration {
    //! Property types for the `TLSInspectionConfiguration` resource.

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-address.html) property type.
    #[derive(Debug, Default)]
    pub struct Address {
        /// Property [`AddressDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-address.html#cfn-networkfirewall-tlsinspectionconfiguration-address-addressdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address_definition: ::Value<String>,
    }

    impl ::codec::SerializeValue for Address {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddressDefinition", &self.address_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Address {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Address;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Address")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address_definition: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddressDefinition" => {
                                address_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Address {
                        address_definition: address_definition.ok_or(::serde::de::Error::missing_field("AddressDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.CheckCertificateRevocationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus.html) property type.
    #[derive(Debug, Default)]
    pub struct CheckCertificateRevocationStatus {
        /// Property [`RevokedStatusAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus.html#cfn-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus-revokedstatusaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revoked_status_action: Option<::Value<String>>,
        /// Property [`UnknownStatusAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus.html#cfn-networkfirewall-tlsinspectionconfiguration-checkcertificaterevocationstatus-unknownstatusaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unknown_status_action: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CheckCertificateRevocationStatus {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref revoked_status_action) = self.revoked_status_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevokedStatusAction", revoked_status_action)?;
            }
            if let Some(ref unknown_status_action) = self.unknown_status_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnknownStatusAction", unknown_status_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CheckCertificateRevocationStatus {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CheckCertificateRevocationStatus, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CheckCertificateRevocationStatus;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CheckCertificateRevocationStatus")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut revoked_status_action: Option<::Value<String>> = None;
                    let mut unknown_status_action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RevokedStatusAction" => {
                                revoked_status_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnknownStatusAction" => {
                                unknown_status_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CheckCertificateRevocationStatus {
                        revoked_status_action: revoked_status_action,
                        unknown_status_action: unknown_status_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-portrange.html) property type.
    #[derive(Debug, Default)]
    pub struct PortRange {
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-portrange.html#cfn-networkfirewall-tlsinspectionconfiguration-portrange-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: ::Value<u32>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-portrange.html#cfn-networkfirewall-tlsinspectionconfiguration-portrange-toport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub to_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for PortRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from_port: Option<::Value<u32>> = None;
                    let mut to_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FromPort" => {
                                from_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ToPort" => {
                                to_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortRange {
                        from_port: from_port.ok_or(::serde::de::Error::missing_field("FromPort"))?,
                        to_port: to_port.ok_or(::serde::de::Error::missing_field("ToPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificate.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerCertificate {
        /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificate.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificate-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServerCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref resource_arn) = self.resource_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", resource_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceArn" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerCertificate {
                        resource_arn: resource_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificateConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerCertificateConfiguration {
        /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration-certificateauthorityarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_authority_arn: Option<::Value<String>>,
        /// Property [`CheckCertificateRevocationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration-checkcertificaterevocationstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub check_certificate_revocation_status: Option<::Value<CheckCertificateRevocationStatus>>,
        /// Property [`Scopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration-scopes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scopes: Option<::ValueList<ServerCertificateScope>>,
        /// Property [`ServerCertificates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificateconfiguration-servercertificates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_certificates: Option<::ValueList<ServerCertificate>>,
    }

    impl ::codec::SerializeValue for ServerCertificateConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_authority_arn) = self.certificate_authority_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", certificate_authority_arn)?;
            }
            if let Some(ref check_certificate_revocation_status) = self.check_certificate_revocation_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CheckCertificateRevocationStatus", check_certificate_revocation_status)?;
            }
            if let Some(ref scopes) = self.scopes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scopes", scopes)?;
            }
            if let Some(ref server_certificates) = self.server_certificates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificates", server_certificates)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerCertificateConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerCertificateConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerCertificateConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerCertificateConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_authority_arn: Option<::Value<String>> = None;
                    let mut check_certificate_revocation_status: Option<::Value<CheckCertificateRevocationStatus>> = None;
                    let mut scopes: Option<::ValueList<ServerCertificateScope>> = None;
                    let mut server_certificates: Option<::ValueList<ServerCertificate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateAuthorityArn" => {
                                certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CheckCertificateRevocationStatus" => {
                                check_certificate_revocation_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scopes" => {
                                scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerCertificates" => {
                                server_certificates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerCertificateConfiguration {
                        certificate_authority_arn: certificate_authority_arn,
                        check_certificate_revocation_status: check_certificate_revocation_status,
                        scopes: scopes,
                        server_certificates: server_certificates,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.ServerCertificateScope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerCertificateScope {
        /// Property [`DestinationPorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificatescope-destinationports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_ports: Option<::ValueList<PortRange>>,
        /// Property [`Destinations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificatescope-destinations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destinations: Option<::ValueList<Address>>,
        /// Property [`Protocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificatescope-protocols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocols: Option<::ValueList<u32>>,
        /// Property [`SourcePorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificatescope-sourceports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_ports: Option<::ValueList<PortRange>>,
        /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-servercertificatescope.html#cfn-networkfirewall-tlsinspectionconfiguration-servercertificatescope-sources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sources: Option<::ValueList<Address>>,
    }

    impl ::codec::SerializeValue for ServerCertificateScope {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination_ports) = self.destination_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPorts", destination_ports)?;
            }
            if let Some(ref destinations) = self.destinations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destinations", destinations)?;
            }
            if let Some(ref protocols) = self.protocols {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocols", protocols)?;
            }
            if let Some(ref source_ports) = self.source_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePorts", source_ports)?;
            }
            if let Some(ref sources) = self.sources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", sources)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerCertificateScope {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerCertificateScope, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerCertificateScope;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerCertificateScope")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_ports: Option<::ValueList<PortRange>> = None;
                    let mut destinations: Option<::ValueList<Address>> = None;
                    let mut protocols: Option<::ValueList<u32>> = None;
                    let mut source_ports: Option<::ValueList<PortRange>> = None;
                    let mut sources: Option<::ValueList<Address>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationPorts" => {
                                destination_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Destinations" => {
                                destinations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocols" => {
                                protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePorts" => {
                                source_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sources" => {
                                sources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerCertificateScope {
                        destination_ports: destination_ports,
                        destinations: destinations,
                        protocols: protocols,
                        source_ports: source_ports,
                        sources: sources,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkFirewall::TLSInspectionConfiguration.TLSInspectionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TLSInspectionConfiguration {
        /// Property [`ServerCertificateConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfiguration.html#cfn-networkfirewall-tlsinspectionconfiguration-tlsinspectionconfiguration-servercertificateconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_certificate_configurations: Option<::ValueList<ServerCertificateConfiguration>>,
    }

    impl ::codec::SerializeValue for TLSInspectionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref server_certificate_configurations) = self.server_certificate_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificateConfigurations", server_certificate_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TLSInspectionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TLSInspectionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TLSInspectionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TLSInspectionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_certificate_configurations: Option<::ValueList<ServerCertificateConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerCertificateConfigurations" => {
                                server_certificate_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TLSInspectionConfiguration {
                        server_certificate_configurations: server_certificate_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
