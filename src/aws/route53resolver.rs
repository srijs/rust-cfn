//! Types for the `Route53Resolver` service.

/// The [`AWS::Route53Resolver::FirewallDomainList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html) resource type.
#[derive(Debug, Default)]
pub struct FirewallDomainList {
    properties: FirewallDomainListProperties
}

/// Properties for the `FirewallDomainList` resource.
#[derive(Debug, Default)]
pub struct FirewallDomainListProperties {
    /// Property [`DomainFileUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html#cfn-route53resolver-firewalldomainlist-domainfileurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_file_url: Option<::Value<String>>,
    /// Property [`Domains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html#cfn-route53resolver-firewalldomainlist-domains).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domains: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html#cfn-route53resolver-firewalldomainlist-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html#cfn-route53resolver-firewalldomainlist-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FirewallDomainListProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref domain_file_url) = self.domain_file_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainFileUrl", domain_file_url)?;
        }
        if let Some(ref domains) = self.domains {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domains", domains)?;
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

impl<'de> ::serde::Deserialize<'de> for FirewallDomainListProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallDomainListProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallDomainListProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FirewallDomainListProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_file_url: Option<::Value<String>> = None;
                let mut domains: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainFileUrl" => {
                            domain_file_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domains" => {
                            domains = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(FirewallDomainListProperties {
                    domain_file_url: domain_file_url,
                    domains: domains,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FirewallDomainList {
    type Properties = FirewallDomainListProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::FirewallDomainList";
    fn properties(&self) -> &FirewallDomainListProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FirewallDomainListProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FirewallDomainList {}

impl From<FirewallDomainListProperties> for FirewallDomainList {
    fn from(properties: FirewallDomainListProperties) -> FirewallDomainList {
        FirewallDomainList { properties }
    }
}

/// The [`AWS::Route53Resolver::FirewallRuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroup.html) resource type.
#[derive(Debug, Default)]
pub struct FirewallRuleGroup {
    properties: FirewallRuleGroupProperties
}

/// Properties for the `FirewallRuleGroup` resource.
#[derive(Debug, Default)]
pub struct FirewallRuleGroupProperties {
    /// Property [`FirewallRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroup.html#cfn-route53resolver-firewallrulegroup-firewallrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firewall_rules: Option<::ValueList<self::firewall_rule_group::FirewallRule>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroup.html#cfn-route53resolver-firewallrulegroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroup.html#cfn-route53resolver-firewallrulegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FirewallRuleGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref firewall_rules) = self.firewall_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallRules", firewall_rules)?;
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

impl<'de> ::serde::Deserialize<'de> for FirewallRuleGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallRuleGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallRuleGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FirewallRuleGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut firewall_rules: Option<::ValueList<self::firewall_rule_group::FirewallRule>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FirewallRules" => {
                            firewall_rules = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(FirewallRuleGroupProperties {
                    firewall_rules: firewall_rules,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FirewallRuleGroup {
    type Properties = FirewallRuleGroupProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::FirewallRuleGroup";
    fn properties(&self) -> &FirewallRuleGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FirewallRuleGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FirewallRuleGroup {}

impl From<FirewallRuleGroupProperties> for FirewallRuleGroup {
    fn from(properties: FirewallRuleGroupProperties) -> FirewallRuleGroup {
        FirewallRuleGroup { properties }
    }
}

/// The [`AWS::Route53Resolver::FirewallRuleGroupAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html) resource type.
#[derive(Debug, Default)]
pub struct FirewallRuleGroupAssociation {
    properties: FirewallRuleGroupAssociationProperties
}

/// Properties for the `FirewallRuleGroupAssociation` resource.
#[derive(Debug, Default)]
pub struct FirewallRuleGroupAssociationProperties {
    /// Property [`FirewallRuleGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-firewallrulegroupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub firewall_rule_group_id: ::Value<String>,
    /// Property [`MutationProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-mutationprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mutation_protection: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-priority).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority: ::Value<u32>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html#cfn-route53resolver-firewallrulegroupassociation-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for FirewallRuleGroupAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallRuleGroupId", &self.firewall_rule_group_id)?;
        if let Some(ref mutation_protection) = self.mutation_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MutationProtection", mutation_protection)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FirewallRuleGroupAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallRuleGroupAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FirewallRuleGroupAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FirewallRuleGroupAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut firewall_rule_group_id: Option<::Value<String>> = None;
                let mut mutation_protection: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut priority: Option<::Value<u32>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FirewallRuleGroupId" => {
                            firewall_rule_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MutationProtection" => {
                            mutation_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(FirewallRuleGroupAssociationProperties {
                    firewall_rule_group_id: firewall_rule_group_id.ok_or(::serde::de::Error::missing_field("FirewallRuleGroupId"))?,
                    mutation_protection: mutation_protection,
                    name: name,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FirewallRuleGroupAssociation {
    type Properties = FirewallRuleGroupAssociationProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::FirewallRuleGroupAssociation";
    fn properties(&self) -> &FirewallRuleGroupAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FirewallRuleGroupAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FirewallRuleGroupAssociation {}

impl From<FirewallRuleGroupAssociationProperties> for FirewallRuleGroupAssociation {
    fn from(properties: FirewallRuleGroupAssociationProperties) -> FirewallRuleGroupAssociation {
        FirewallRuleGroupAssociation { properties }
    }
}

/// The [`AWS::Route53Resolver::OutpostResolver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html) resource type.
#[derive(Debug, Default)]
pub struct OutpostResolver {
    properties: OutpostResolverProperties
}

/// Properties for the `OutpostResolver` resource.
#[derive(Debug, Default)]
pub struct OutpostResolverProperties {
    /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html#cfn-route53resolver-outpostresolver-instancecount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_count: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html#cfn-route53resolver-outpostresolver-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OutpostArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html#cfn-route53resolver-outpostresolver-outpostarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub outpost_arn: ::Value<String>,
    /// Property [`PreferredInstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html#cfn-route53resolver-outpostresolver-preferredinstancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_instance_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html#cfn-route53resolver-outpostresolver-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for OutpostResolverProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref instance_count) = self.instance_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", instance_count)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutpostArn", &self.outpost_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredInstanceType", &self.preferred_instance_type)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OutpostResolverProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OutpostResolverProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OutpostResolverProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OutpostResolverProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_count: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut outpost_arn: Option<::Value<String>> = None;
                let mut preferred_instance_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceCount" => {
                            instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutpostArn" => {
                            outpost_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredInstanceType" => {
                            preferred_instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OutpostResolverProperties {
                    instance_count: instance_count,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    outpost_arn: outpost_arn.ok_or(::serde::de::Error::missing_field("OutpostArn"))?,
                    preferred_instance_type: preferred_instance_type.ok_or(::serde::de::Error::missing_field("PreferredInstanceType"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OutpostResolver {
    type Properties = OutpostResolverProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::OutpostResolver";
    fn properties(&self) -> &OutpostResolverProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OutpostResolverProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OutpostResolver {}

impl From<OutpostResolverProperties> for OutpostResolver {
    fn from(properties: OutpostResolverProperties) -> OutpostResolver {
        OutpostResolver { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverconfig.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverConfig {
    properties: ResolverConfigProperties
}

/// Properties for the `ResolverConfig` resource.
#[derive(Debug, Default)]
pub struct ResolverConfigProperties {
    /// Property [`AutodefinedReverseFlag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverconfig.html#cfn-route53resolver-resolverconfig-autodefinedreverseflag).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub autodefined_reverse_flag: ::Value<String>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverconfig.html#cfn-route53resolver-resolverconfig-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: ::Value<String>,
}

impl ::serde::Serialize for ResolverConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutodefinedReverseFlag", &self.autodefined_reverse_flag)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut autodefined_reverse_flag: Option<::Value<String>> = None;
                let mut resource_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutodefinedReverseFlag" => {
                            autodefined_reverse_flag = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverConfigProperties {
                    autodefined_reverse_flag: autodefined_reverse_flag.ok_or(::serde::de::Error::missing_field("AutodefinedReverseFlag"))?,
                    resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverConfig {
    type Properties = ResolverConfigProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverConfig";
    fn properties(&self) -> &ResolverConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverConfig {}

impl From<ResolverConfigProperties> for ResolverConfig {
    fn from(properties: ResolverConfigProperties) -> ResolverConfig {
        ResolverConfig { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverDNSSECConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverdnssecconfig.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverDNSSECConfig {
    properties: ResolverDNSSECConfigProperties
}

/// Properties for the `ResolverDNSSECConfig` resource.
#[derive(Debug, Default)]
pub struct ResolverDNSSECConfigProperties {
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverdnssecconfig.html#cfn-route53resolver-resolverdnssecconfig-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ResolverDNSSECConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref resource_id) = self.resource_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverDNSSECConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverDNSSECConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverDNSSECConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverDNSSECConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverDNSSECConfigProperties {
                    resource_id: resource_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverDNSSECConfig {
    type Properties = ResolverDNSSECConfigProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverDNSSECConfig";
    fn properties(&self) -> &ResolverDNSSECConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverDNSSECConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverDNSSECConfig {}

impl From<ResolverDNSSECConfigProperties> for ResolverDNSSECConfig {
    fn from(properties: ResolverDNSSECConfigProperties) -> ResolverDNSSECConfig {
        ResolverDNSSECConfig { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverEndpoint {
    properties: ResolverEndpointProperties
}

/// Properties for the `ResolverEndpoint` resource.
#[derive(Debug, Default)]
pub struct ResolverEndpointProperties {
    /// Property [`Direction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-direction).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub direction: ::Value<String>,
    /// Property [`IpAddresses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-ipaddresses).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_addresses: ::ValueList<self::resolver_endpoint::IpAddressRequest>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`OutpostArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-outpostarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub outpost_arn: Option<::Value<String>>,
    /// Property [`PreferredInstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-preferredinstancetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub preferred_instance_type: Option<::Value<String>>,
    /// Property [`Protocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-protocols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocols: Option<::ValueList<String>>,
    /// Property [`ResolverEndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-resolverendpointtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resolver_endpoint_type: Option<::Value<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html#cfn-route53resolver-resolverendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ResolverEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Direction", &self.direction)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddresses", &self.ip_addresses)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref outpost_arn) = self.outpost_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutpostArn", outpost_arn)?;
        }
        if let Some(ref preferred_instance_type) = self.preferred_instance_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredInstanceType", preferred_instance_type)?;
        }
        if let Some(ref protocols) = self.protocols {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocols", protocols)?;
        }
        if let Some(ref resolver_endpoint_type) = self.resolver_endpoint_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolverEndpointType", resolver_endpoint_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut direction: Option<::Value<String>> = None;
                let mut ip_addresses: Option<::ValueList<self::resolver_endpoint::IpAddressRequest>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut outpost_arn: Option<::Value<String>> = None;
                let mut preferred_instance_type: Option<::Value<String>> = None;
                let mut protocols: Option<::ValueList<String>> = None;
                let mut resolver_endpoint_type: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Direction" => {
                            direction = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddresses" => {
                            ip_addresses = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutpostArn" => {
                            outpost_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredInstanceType" => {
                            preferred_instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocols" => {
                            protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResolverEndpointType" => {
                            resolver_endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverEndpointProperties {
                    direction: direction.ok_or(::serde::de::Error::missing_field("Direction"))?,
                    ip_addresses: ip_addresses.ok_or(::serde::de::Error::missing_field("IpAddresses"))?,
                    name: name,
                    outpost_arn: outpost_arn,
                    preferred_instance_type: preferred_instance_type,
                    protocols: protocols,
                    resolver_endpoint_type: resolver_endpoint_type,
                    security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverEndpoint {
    type Properties = ResolverEndpointProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverEndpoint";
    fn properties(&self) -> &ResolverEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverEndpoint {}

impl From<ResolverEndpointProperties> for ResolverEndpoint {
    fn from(properties: ResolverEndpointProperties) -> ResolverEndpoint {
        ResolverEndpoint { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverQueryLoggingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfig.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverQueryLoggingConfig {
    properties: ResolverQueryLoggingConfigProperties
}

/// Properties for the `ResolverQueryLoggingConfig` resource.
#[derive(Debug, Default)]
pub struct ResolverQueryLoggingConfigProperties {
    /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfig.html#cfn-route53resolver-resolverqueryloggingconfig-destinationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub destination_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfig.html#cfn-route53resolver-resolverqueryloggingconfig-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ResolverQueryLoggingConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref destination_arn) = self.destination_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", destination_arn)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverQueryLoggingConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverQueryLoggingConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverQueryLoggingConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverQueryLoggingConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut destination_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DestinationArn" => {
                            destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverQueryLoggingConfigProperties {
                    destination_arn: destination_arn,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverQueryLoggingConfig {
    type Properties = ResolverQueryLoggingConfigProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverQueryLoggingConfig";
    fn properties(&self) -> &ResolverQueryLoggingConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverQueryLoggingConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverQueryLoggingConfig {}

impl From<ResolverQueryLoggingConfigProperties> for ResolverQueryLoggingConfig {
    fn from(properties: ResolverQueryLoggingConfigProperties) -> ResolverQueryLoggingConfig {
        ResolverQueryLoggingConfig { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfigassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverQueryLoggingConfigAssociation {
    properties: ResolverQueryLoggingConfigAssociationProperties
}

/// Properties for the `ResolverQueryLoggingConfigAssociation` resource.
#[derive(Debug, Default)]
pub struct ResolverQueryLoggingConfigAssociationProperties {
    /// Property [`ResolverQueryLogConfigId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfigassociation.html#cfn-route53resolver-resolverqueryloggingconfigassociation-resolverquerylogconfigid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resolver_query_log_config_id: Option<::Value<String>>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfigassociation.html#cfn-route53resolver-resolverqueryloggingconfigassociation-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ResolverQueryLoggingConfigAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref resolver_query_log_config_id) = self.resolver_query_log_config_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolverQueryLogConfigId", resolver_query_log_config_id)?;
        }
        if let Some(ref resource_id) = self.resource_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverQueryLoggingConfigAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverQueryLoggingConfigAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverQueryLoggingConfigAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverQueryLoggingConfigAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resolver_query_log_config_id: Option<::Value<String>> = None;
                let mut resource_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResolverQueryLogConfigId" => {
                            resolver_query_log_config_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverQueryLoggingConfigAssociationProperties {
                    resolver_query_log_config_id: resolver_query_log_config_id,
                    resource_id: resource_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverQueryLoggingConfigAssociation {
    type Properties = ResolverQueryLoggingConfigAssociationProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation";
    fn properties(&self) -> &ResolverQueryLoggingConfigAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverQueryLoggingConfigAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverQueryLoggingConfigAssociation {}

impl From<ResolverQueryLoggingConfigAssociationProperties> for ResolverQueryLoggingConfigAssociation {
    fn from(properties: ResolverQueryLoggingConfigAssociationProperties) -> ResolverQueryLoggingConfigAssociation {
        ResolverQueryLoggingConfigAssociation { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverRule {
    properties: ResolverRuleProperties
}

/// Properties for the `ResolverRule` resource.
#[derive(Debug, Default)]
pub struct ResolverRuleProperties {
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-domainname).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub domain_name: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ResolverEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-resolverendpointid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resolver_endpoint_id: Option<::Value<String>>,
    /// Property [`RuleType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-ruletype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html#cfn-route53resolver-resolverrule-targetips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_ips: Option<::ValueList<self::resolver_rule::TargetAddress>>,
}

impl ::serde::Serialize for ResolverRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref resolver_endpoint_id) = self.resolver_endpoint_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolverEndpointId", resolver_endpoint_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleType", &self.rule_type)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_ips) = self.target_ips {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetIps", target_ips)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut resolver_endpoint_id: Option<::Value<String>> = None;
                let mut rule_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_ips: Option<::ValueList<self::resolver_rule::TargetAddress>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResolverEndpointId" => {
                            resolver_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleType" => {
                            rule_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetIps" => {
                            target_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverRuleProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    name: name,
                    resolver_endpoint_id: resolver_endpoint_id,
                    rule_type: rule_type.ok_or(::serde::de::Error::missing_field("RuleType"))?,
                    tags: tags,
                    target_ips: target_ips,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverRule {
    type Properties = ResolverRuleProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverRule";
    fn properties(&self) -> &ResolverRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverRule {}

impl From<ResolverRuleProperties> for ResolverRule {
    fn from(properties: ResolverRuleProperties) -> ResolverRule {
        ResolverRule { properties }
    }
}

/// The [`AWS::Route53Resolver::ResolverRuleAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverruleassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ResolverRuleAssociation {
    properties: ResolverRuleAssociationProperties
}

/// Properties for the `ResolverRuleAssociation` resource.
#[derive(Debug, Default)]
pub struct ResolverRuleAssociationProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverruleassociation.html#cfn-route53resolver-resolverruleassociation-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ResolverRuleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverruleassociation.html#cfn-route53resolver-resolverruleassociation-resolverruleid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resolver_rule_id: ::Value<String>,
    /// Property [`VPCId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverruleassociation.html#cfn-route53resolver-resolverruleassociation-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for ResolverRuleAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolverRuleId", &self.resolver_rule_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverRuleAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverRuleAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverRuleAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverRuleAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut resolver_rule_id: Option<::Value<String>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResolverRuleId" => {
                            resolver_rule_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverRuleAssociationProperties {
                    name: name,
                    resolver_rule_id: resolver_rule_id.ok_or(::serde::de::Error::missing_field("ResolverRuleId"))?,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VPCId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResolverRuleAssociation {
    type Properties = ResolverRuleAssociationProperties;
    const TYPE: &'static str = "AWS::Route53Resolver::ResolverRuleAssociation";
    fn properties(&self) -> &ResolverRuleAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverRuleAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResolverRuleAssociation {}

impl From<ResolverRuleAssociationProperties> for ResolverRuleAssociation {
    fn from(properties: ResolverRuleAssociationProperties) -> ResolverRuleAssociation {
        ResolverRuleAssociation { properties }
    }
}

pub mod firewall_rule_group {
    //! Property types for the `FirewallRuleGroup` resource.

    /// The [`AWS::Route53Resolver::FirewallRuleGroup.FirewallRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html) property type.
    #[derive(Debug, Default)]
    pub struct FirewallRule {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
        /// Property [`BlockOverrideDnsType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-blockoverridednstype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_override_dns_type: Option<::Value<String>>,
        /// Property [`BlockOverrideDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-blockoverridedomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_override_domain: Option<::Value<String>>,
        /// Property [`BlockOverrideTtl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-blockoverridettl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_override_ttl: Option<::Value<u32>>,
        /// Property [`BlockResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-blockresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_response: Option<::Value<String>>,
        /// Property [`FirewallDomainListId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-firewalldomainlistid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firewall_domain_list_id: ::Value<String>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`Qtype`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html#cfn-route53resolver-firewallrulegroup-firewallrule-qtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub qtype: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirewallRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref block_override_dns_type) = self.block_override_dns_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockOverrideDnsType", block_override_dns_type)?;
            }
            if let Some(ref block_override_domain) = self.block_override_domain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockOverrideDomain", block_override_domain)?;
            }
            if let Some(ref block_override_ttl) = self.block_override_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockOverrideTtl", block_override_ttl)?;
            }
            if let Some(ref block_response) = self.block_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockResponse", block_response)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirewallDomainListId", &self.firewall_domain_list_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            if let Some(ref qtype) = self.qtype {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qtype", qtype)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirewallRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirewallRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirewallRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirewallRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut block_override_dns_type: Option<::Value<String>> = None;
                    let mut block_override_domain: Option<::Value<String>> = None;
                    let mut block_override_ttl: Option<::Value<u32>> = None;
                    let mut block_response: Option<::Value<String>> = None;
                    let mut firewall_domain_list_id: Option<::Value<String>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut qtype: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockOverrideDnsType" => {
                                block_override_dns_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockOverrideDomain" => {
                                block_override_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockOverrideTtl" => {
                                block_override_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockResponse" => {
                                block_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirewallDomainListId" => {
                                firewall_domain_list_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Qtype" => {
                                qtype = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirewallRule {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        block_override_dns_type: block_override_dns_type,
                        block_override_domain: block_override_domain,
                        block_override_ttl: block_override_ttl,
                        block_response: block_response,
                        firewall_domain_list_id: firewall_domain_list_id.ok_or(::serde::de::Error::missing_field("FirewallDomainListId"))?,
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        qtype: qtype,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resolver_endpoint {
    //! Property types for the `ResolverEndpoint` resource.

    /// The [`AWS::Route53Resolver::ResolverEndpoint.IpAddressRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverendpoint-ipaddressrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct IpAddressRequest {
        /// Property [`Ip`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverendpoint-ipaddressrequest.html#cfn-route53resolver-resolverendpoint-ipaddressrequest-ip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip: Option<::Value<String>>,
        /// Property [`Ipv6`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverendpoint-ipaddressrequest.html#cfn-route53resolver-resolverendpoint-ipaddressrequest-ipv6).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6: Option<::Value<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverendpoint-ipaddressrequest.html#cfn-route53resolver-resolverendpoint-ipaddressrequest-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for IpAddressRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip) = self.ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ip", ip)?;
            }
            if let Some(ref ipv6) = self.ipv6 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6", ipv6)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpAddressRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpAddressRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpAddressRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpAddressRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip: Option<::Value<String>> = None;
                    let mut ipv6: Option<::Value<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ip" => {
                                ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ipv6" => {
                                ipv6 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IpAddressRequest {
                        ip: ip,
                        ipv6: ipv6,
                        subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resolver_rule {
    //! Property types for the `ResolverRule` resource.

    /// The [`AWS::Route53Resolver::ResolverRule.TargetAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetAddress {
        /// Property [`Ip`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html#cfn-route53resolver-resolverrule-targetaddress-ip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip: Option<::Value<String>>,
        /// Property [`Ipv6`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html#cfn-route53resolver-resolverrule-targetaddress-ipv6).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html#cfn-route53resolver-resolverrule-targetaddress-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html#cfn-route53resolver-resolverrule-targetaddress-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TargetAddress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ip) = self.ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ip", ip)?;
            }
            if let Some(ref ipv6) = self.ipv6 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6", ipv6)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetAddress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetAddress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetAddress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetAddress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip: Option<::Value<String>> = None;
                    let mut ipv6: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ip" => {
                                ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ipv6" => {
                                ipv6 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetAddress {
                        ip: ip,
                        ipv6: ipv6,
                        port: port,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
