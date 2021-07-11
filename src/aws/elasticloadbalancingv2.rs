//! Types for the `ElasticLoadBalancingV2` service.

/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource type.
#[derive(Debug, Default)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Debug, Default)]
pub struct ListenerProperties {
    /// Property [`AlpnPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-alpnpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alpn_policy: Option<::ValueList<String>>,
    /// Property [`Certificates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-certificates).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificates: Option<::ValueList<self::listener::Certificate>>,
    /// Property [`DefaultActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-defaultactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_actions: ::ValueList<self::listener::Action>,
    /// Property [`LoadBalancerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-loadbalancerarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub load_balancer_arn: ::Value<String>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-protocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol: Option<::Value<String>>,
    /// Property [`SslPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html#cfn-elasticloadbalancingv2-listener-sslpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssl_policy: Option<::Value<String>>,
}

impl ::serde::Serialize for ListenerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref alpn_policy) = self.alpn_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlpnPolicy", alpn_policy)?;
        }
        if let Some(ref certificates) = self.certificates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificates", certificates)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultActions", &self.default_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerArn", &self.load_balancer_arn)?;
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        if let Some(ref ssl_policy) = self.ssl_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslPolicy", ssl_policy)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ListenerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListenerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ListenerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alpn_policy: Option<::ValueList<String>> = None;
                let mut certificates: Option<::ValueList<self::listener::Certificate>> = None;
                let mut default_actions: Option<::ValueList<self::listener::Action>> = None;
                let mut load_balancer_arn: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut ssl_policy: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AlpnPolicy" => {
                            alpn_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Certificates" => {
                            certificates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultActions" => {
                            default_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerArn" => {
                            load_balancer_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SslPolicy" => {
                            ssl_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ListenerProperties {
                    alpn_policy: alpn_policy,
                    certificates: certificates,
                    default_actions: default_actions.ok_or(::serde::de::Error::missing_field("DefaultActions"))?,
                    load_balancer_arn: load_balancer_arn.ok_or(::serde::de::Error::missing_field("LoadBalancerArn"))?,
                    port: port,
                    protocol: protocol,
                    ssl_policy: ssl_policy,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Listener {
    type Properties = ListenerProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancingV2::Listener";
    fn properties(&self) -> &ListenerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ListenerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Listener {}

impl From<ListenerProperties> for Listener {
    fn from(properties: ListenerProperties) -> Listener {
        Listener { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenercertificate.html) resource type.
#[derive(Debug, Default)]
pub struct ListenerCertificate {
    properties: ListenerCertificateProperties
}

/// Properties for the `ListenerCertificate` resource.
#[derive(Debug, Default)]
pub struct ListenerCertificateProperties {
    /// Property [`Certificates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenercertificate.html#cfn-elasticloadbalancingv2-listenercertificate-certificates).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificates: ::ValueList<self::listener_certificate::Certificate>,
    /// Property [`ListenerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenercertificate.html#cfn-elasticloadbalancingv2-listenercertificate-listenerarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub listener_arn: ::Value<String>,
}

impl ::serde::Serialize for ListenerCertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificates", &self.certificates)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListenerArn", &self.listener_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ListenerCertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerCertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListenerCertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ListenerCertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificates: Option<::ValueList<self::listener_certificate::Certificate>> = None;
                let mut listener_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Certificates" => {
                            certificates = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ListenerArn" => {
                            listener_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ListenerCertificateProperties {
                    certificates: certificates.ok_or(::serde::de::Error::missing_field("Certificates"))?,
                    listener_arn: listener_arn.ok_or(::serde::de::Error::missing_field("ListenerArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ListenerCertificate {
    type Properties = ListenerCertificateProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancingV2::ListenerCertificate";
    fn properties(&self) -> &ListenerCertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ListenerCertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ListenerCertificate {}

impl From<ListenerCertificateProperties> for ListenerCertificate {
    fn from(properties: ListenerCertificateProperties) -> ListenerCertificate {
        ListenerCertificate { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::ListenerRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html) resource type.
#[derive(Debug, Default)]
pub struct ListenerRule {
    properties: ListenerRuleProperties
}

/// Properties for the `ListenerRule` resource.
#[derive(Debug, Default)]
pub struct ListenerRuleProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html#cfn-elasticloadbalancingv2-listenerrule-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: ::ValueList<self::listener_rule::Action>,
    /// Property [`Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html#cfn-elasticloadbalancingv2-listenerrule-conditions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub conditions: ::ValueList<self::listener_rule::RuleCondition>,
    /// Property [`ListenerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html#cfn-elasticloadbalancingv2-listenerrule-listenerarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub listener_arn: ::Value<String>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html#cfn-elasticloadbalancingv2-listenerrule-priority).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub priority: ::Value<u32>,
}

impl ::serde::Serialize for ListenerRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditions", &self.conditions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListenerArn", &self.listener_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ListenerRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListenerRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ListenerRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<self::listener_rule::Action>> = None;
                let mut conditions: Option<::ValueList<self::listener_rule::RuleCondition>> = None;
                let mut listener_arn: Option<::Value<String>> = None;
                let mut priority: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Conditions" => {
                            conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ListenerArn" => {
                            listener_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ListenerRuleProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    conditions: conditions.ok_or(::serde::de::Error::missing_field("Conditions"))?,
                    listener_arn: listener_arn.ok_or(::serde::de::Error::missing_field("ListenerArn"))?,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ListenerRule {
    type Properties = ListenerRuleProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancingV2::ListenerRule";
    fn properties(&self) -> &ListenerRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ListenerRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ListenerRule {}

impl From<ListenerRuleProperties> for ListenerRule {
    fn from(properties: ListenerRuleProperties) -> ListenerRule {
        ListenerRule { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html) resource type.
#[derive(Debug, Default)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug, Default)]
pub struct LoadBalancerProperties {
    /// Property [`IpAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-ipaddresstype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_address_type: Option<::Value<String>>,
    /// Property [`LoadBalancerAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-loadbalancerattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub load_balancer_attributes: Option<::ValueList<self::load_balancer::LoadBalancerAttribute>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Scheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-scheme).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scheme: Option<::Value<String>>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-securitygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_groups: Option<::ValueList<String>>,
    /// Property [`SubnetMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-subnetmappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_mappings: Option<::ValueList<self::load_balancer::SubnetMapping>>,
    /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-subnets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnets: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html#cfn-elasticloadbalancingv2-loadbalancer-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
}

impl ::serde::Serialize for LoadBalancerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref ip_address_type) = self.ip_address_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddressType", ip_address_type)?;
        }
        if let Some(ref load_balancer_attributes) = self.load_balancer_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerAttributes", load_balancer_attributes)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref scheme) = self.scheme {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheme", scheme)?;
        }
        if let Some(ref security_groups) = self.security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
        }
        if let Some(ref subnet_mappings) = self.subnet_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetMappings", subnet_mappings)?;
        }
        if let Some(ref subnets) = self.subnets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoadBalancerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoadBalancerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoadBalancerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ip_address_type: Option<::Value<String>> = None;
                let mut load_balancer_attributes: Option<::ValueList<self::load_balancer::LoadBalancerAttribute>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut scheme: Option<::Value<String>> = None;
                let mut security_groups: Option<::ValueList<String>> = None;
                let mut subnet_mappings: Option<::ValueList<self::load_balancer::SubnetMapping>> = None;
                let mut subnets: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IpAddressType" => {
                            ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerAttributes" => {
                            load_balancer_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scheme" => {
                            scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetMappings" => {
                            subnet_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subnets" => {
                            subnets = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(LoadBalancerProperties {
                    ip_address_type: ip_address_type,
                    load_balancer_attributes: load_balancer_attributes,
                    name: name,
                    scheme: scheme,
                    security_groups: security_groups,
                    subnet_mappings: subnet_mappings,
                    subnets: subnets,
                    tags: tags,
                    r#type: r#type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoadBalancer {
    type Properties = LoadBalancerProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancingV2::LoadBalancer";
    fn properties(&self) -> &LoadBalancerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoadBalancerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LoadBalancer {}

impl From<LoadBalancerProperties> for LoadBalancer {
    fn from(properties: LoadBalancerProperties) -> LoadBalancer {
        LoadBalancer { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::TargetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct TargetGroup {
    properties: TargetGroupProperties
}

/// Properties for the `TargetGroup` resource.
#[derive(Debug, Default)]
pub struct TargetGroupProperties {
    /// Property [`HealthCheckEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthcheckenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_enabled: Option<::Value<bool>>,
    /// Property [`HealthCheckIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthcheckintervalseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_interval_seconds: Option<::Value<u32>>,
    /// Property [`HealthCheckPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthcheckpath).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_path: Option<::Value<String>>,
    /// Property [`HealthCheckPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthcheckport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_port: Option<::Value<String>>,
    /// Property [`HealthCheckProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthcheckprotocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_protocol: Option<::Value<String>>,
    /// Property [`HealthCheckTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthchecktimeoutseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_timeout_seconds: Option<::Value<u32>>,
    /// Property [`HealthyThresholdCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-healthythresholdcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub healthy_threshold_count: Option<::Value<u32>>,
    /// Property [`Matcher`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-matcher).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub matcher: Option<::Value<self::target_group::Matcher>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-protocol).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol: Option<::Value<String>>,
    /// Property [`ProtocolVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-protocolversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub protocol_version: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetGroupAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-targetgroupattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_group_attributes: Option<::ValueList<self::target_group::TargetGroupAttribute>>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-targettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_type: Option<::Value<String>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: Option<::ValueList<self::target_group::TargetDescription>>,
    /// Property [`UnhealthyThresholdCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-unhealthythresholdcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub unhealthy_threshold_count: Option<::Value<u32>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html#cfn-elasticloadbalancingv2-targetgroup-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for TargetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref health_check_enabled) = self.health_check_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckEnabled", health_check_enabled)?;
        }
        if let Some(ref health_check_interval_seconds) = self.health_check_interval_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckIntervalSeconds", health_check_interval_seconds)?;
        }
        if let Some(ref health_check_path) = self.health_check_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckPath", health_check_path)?;
        }
        if let Some(ref health_check_port) = self.health_check_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckPort", health_check_port)?;
        }
        if let Some(ref health_check_protocol) = self.health_check_protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckProtocol", health_check_protocol)?;
        }
        if let Some(ref health_check_timeout_seconds) = self.health_check_timeout_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckTimeoutSeconds", health_check_timeout_seconds)?;
        }
        if let Some(ref healthy_threshold_count) = self.healthy_threshold_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThresholdCount", healthy_threshold_count)?;
        }
        if let Some(ref matcher) = self.matcher {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Matcher", matcher)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        if let Some(ref protocol_version) = self.protocol_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolVersion", protocol_version)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_group_attributes) = self.target_group_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupAttributes", target_group_attributes)?;
        }
        if let Some(ref target_type) = self.target_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", target_type)?;
        }
        if let Some(ref targets) = self.targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", targets)?;
        }
        if let Some(ref unhealthy_threshold_count) = self.unhealthy_threshold_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThresholdCount", unhealthy_threshold_count)?;
        }
        if let Some(ref vpc_id) = self.vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TargetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TargetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TargetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut health_check_enabled: Option<::Value<bool>> = None;
                let mut health_check_interval_seconds: Option<::Value<u32>> = None;
                let mut health_check_path: Option<::Value<String>> = None;
                let mut health_check_port: Option<::Value<String>> = None;
                let mut health_check_protocol: Option<::Value<String>> = None;
                let mut health_check_timeout_seconds: Option<::Value<u32>> = None;
                let mut healthy_threshold_count: Option<::Value<u32>> = None;
                let mut matcher: Option<::Value<self::target_group::Matcher>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut protocol_version: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_group_attributes: Option<::ValueList<self::target_group::TargetGroupAttribute>> = None;
                let mut target_type: Option<::Value<String>> = None;
                let mut targets: Option<::ValueList<self::target_group::TargetDescription>> = None;
                let mut unhealthy_threshold_count: Option<::Value<u32>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HealthCheckEnabled" => {
                            health_check_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckIntervalSeconds" => {
                            health_check_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckPath" => {
                            health_check_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckPort" => {
                            health_check_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckProtocol" => {
                            health_check_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckTimeoutSeconds" => {
                            health_check_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthyThresholdCount" => {
                            healthy_threshold_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Matcher" => {
                            matcher = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtocolVersion" => {
                            protocol_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetGroupAttributes" => {
                            target_group_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UnhealthyThresholdCount" => {
                            unhealthy_threshold_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TargetGroupProperties {
                    health_check_enabled: health_check_enabled,
                    health_check_interval_seconds: health_check_interval_seconds,
                    health_check_path: health_check_path,
                    health_check_port: health_check_port,
                    health_check_protocol: health_check_protocol,
                    health_check_timeout_seconds: health_check_timeout_seconds,
                    healthy_threshold_count: healthy_threshold_count,
                    matcher: matcher,
                    name: name,
                    port: port,
                    protocol: protocol,
                    protocol_version: protocol_version,
                    tags: tags,
                    target_group_attributes: target_group_attributes,
                    target_type: target_type,
                    targets: targets,
                    unhealthy_threshold_count: unhealthy_threshold_count,
                    vpc_id: vpc_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TargetGroup {
    type Properties = TargetGroupProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancingV2::TargetGroup";
    fn properties(&self) -> &TargetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TargetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TargetGroup {}

impl From<TargetGroupProperties> for TargetGroup {
    fn from(properties: TargetGroupProperties) -> TargetGroup {
        TargetGroup { properties }
    }
}

pub mod listener {
    //! Property types for the `Listener` resource.

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`AuthenticateCognitoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-authenticatecognitoconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authenticate_cognito_config: Option<::Value<AuthenticateCognitoConfig>>,
        /// Property [`AuthenticateOidcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-authenticateoidcconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authenticate_oidc_config: Option<::Value<AuthenticateOidcConfig>>,
        /// Property [`FixedResponseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-fixedresponseconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_response_config: Option<::Value<FixedResponseConfig>>,
        /// Property [`ForwardConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-forwardconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forward_config: Option<::Value<ForwardConfig>>,
        /// Property [`Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-order).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order: Option<::Value<u32>>,
        /// Property [`RedirectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-redirectconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redirect_config: Option<::Value<RedirectConfig>>,
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-targetgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-action.html#cfn-elasticloadbalancingv2-listener-action-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authenticate_cognito_config) = self.authenticate_cognito_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticateCognitoConfig", authenticate_cognito_config)?;
            }
            if let Some(ref authenticate_oidc_config) = self.authenticate_oidc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticateOidcConfig", authenticate_oidc_config)?;
            }
            if let Some(ref fixed_response_config) = self.fixed_response_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedResponseConfig", fixed_response_config)?;
            }
            if let Some(ref forward_config) = self.forward_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardConfig", forward_config)?;
            }
            if let Some(ref order) = self.order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Order", order)?;
            }
            if let Some(ref redirect_config) = self.redirect_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectConfig", redirect_config)?;
            }
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut authenticate_cognito_config: Option<::Value<AuthenticateCognitoConfig>> = None;
                    let mut authenticate_oidc_config: Option<::Value<AuthenticateOidcConfig>> = None;
                    let mut fixed_response_config: Option<::Value<FixedResponseConfig>> = None;
                    let mut forward_config: Option<::Value<ForwardConfig>> = None;
                    let mut order: Option<::Value<u32>> = None;
                    let mut redirect_config: Option<::Value<RedirectConfig>> = None;
                    let mut target_group_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticateCognitoConfig" => {
                                authenticate_cognito_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthenticateOidcConfig" => {
                                authenticate_oidc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedResponseConfig" => {
                                fixed_response_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardConfig" => {
                                forward_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Order" => {
                                order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedirectConfig" => {
                                redirect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        authenticate_cognito_config: authenticate_cognito_config,
                        authenticate_oidc_config: authenticate_oidc_config,
                        fixed_response_config: fixed_response_config,
                        forward_config: forward_config,
                        order: order,
                        redirect_config: redirect_config,
                        target_group_arn: target_group_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.AuthenticateCognitoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticateCognitoConfig {
        /// Property [`AuthenticationRequestExtraParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-authenticationrequestextraparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_request_extra_params: Option<::ValueMap<String>>,
        /// Property [`OnUnauthenticatedRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-onunauthenticatedrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_unauthenticated_request: Option<::Value<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: Option<::Value<String>>,
        /// Property [`SessionCookieName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-sessioncookiename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_cookie_name: Option<::Value<String>>,
        /// Property [`SessionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-sessiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_timeout: Option<::Value<String>>,
        /// Property [`UserPoolArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-userpoolarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_arn: ::Value<String>,
        /// Property [`UserPoolClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-userpoolclientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_client_id: ::Value<String>,
        /// Property [`UserPoolDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listener-authenticatecognitoconfig-userpooldomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_domain: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticateCognitoConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_request_extra_params) = self.authentication_request_extra_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationRequestExtraParams", authentication_request_extra_params)?;
            }
            if let Some(ref on_unauthenticated_request) = self.on_unauthenticated_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUnauthenticatedRequest", on_unauthenticated_request)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            if let Some(ref session_cookie_name) = self.session_cookie_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionCookieName", session_cookie_name)?;
            }
            if let Some(ref session_timeout) = self.session_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionTimeout", session_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolArn", &self.user_pool_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolClientId", &self.user_pool_client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolDomain", &self.user_pool_domain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticateCognitoConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticateCognitoConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticateCognitoConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticateCognitoConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_request_extra_params: Option<::ValueMap<String>> = None;
                    let mut on_unauthenticated_request: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;
                    let mut session_cookie_name: Option<::Value<String>> = None;
                    let mut session_timeout: Option<::Value<String>> = None;
                    let mut user_pool_arn: Option<::Value<String>> = None;
                    let mut user_pool_client_id: Option<::Value<String>> = None;
                    let mut user_pool_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationRequestExtraParams" => {
                                authentication_request_extra_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnUnauthenticatedRequest" => {
                                on_unauthenticated_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionCookieName" => {
                                session_cookie_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionTimeout" => {
                                session_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolArn" => {
                                user_pool_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolClientId" => {
                                user_pool_client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolDomain" => {
                                user_pool_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticateCognitoConfig {
                        authentication_request_extra_params: authentication_request_extra_params,
                        on_unauthenticated_request: on_unauthenticated_request,
                        scope: scope,
                        session_cookie_name: session_cookie_name,
                        session_timeout: session_timeout,
                        user_pool_arn: user_pool_arn.ok_or(::serde::de::Error::missing_field("UserPoolArn"))?,
                        user_pool_client_id: user_pool_client_id.ok_or(::serde::de::Error::missing_field("UserPoolClientId"))?,
                        user_pool_domain: user_pool_domain.ok_or(::serde::de::Error::missing_field("UserPoolDomain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.AuthenticateOidcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticateOidcConfig {
        /// Property [`AuthenticationRequestExtraParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-authenticationrequestextraparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_request_extra_params: Option<::ValueMap<String>>,
        /// Property [`AuthorizationEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-authorizationendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_endpoint: ::Value<String>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-issuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer: ::Value<String>,
        /// Property [`OnUnauthenticatedRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-onunauthenticatedrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_unauthenticated_request: Option<::Value<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: Option<::Value<String>>,
        /// Property [`SessionCookieName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-sessioncookiename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_cookie_name: Option<::Value<String>>,
        /// Property [`SessionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-sessiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_timeout: Option<::Value<String>>,
        /// Property [`TokenEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-tokenendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_endpoint: ::Value<String>,
        /// Property [`UserInfoEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listener-authenticateoidcconfig-userinfoendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_info_endpoint: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticateOidcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_request_extra_params) = self.authentication_request_extra_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationRequestExtraParams", authentication_request_extra_params)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationEndpoint", &self.authorization_endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", &self.issuer)?;
            if let Some(ref on_unauthenticated_request) = self.on_unauthenticated_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUnauthenticatedRequest", on_unauthenticated_request)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            if let Some(ref session_cookie_name) = self.session_cookie_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionCookieName", session_cookie_name)?;
            }
            if let Some(ref session_timeout) = self.session_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionTimeout", session_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenEndpoint", &self.token_endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserInfoEndpoint", &self.user_info_endpoint)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticateOidcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticateOidcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticateOidcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticateOidcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_request_extra_params: Option<::ValueMap<String>> = None;
                    let mut authorization_endpoint: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut issuer: Option<::Value<String>> = None;
                    let mut on_unauthenticated_request: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;
                    let mut session_cookie_name: Option<::Value<String>> = None;
                    let mut session_timeout: Option<::Value<String>> = None;
                    let mut token_endpoint: Option<::Value<String>> = None;
                    let mut user_info_endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationRequestExtraParams" => {
                                authentication_request_extra_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthorizationEndpoint" => {
                                authorization_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Issuer" => {
                                issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnUnauthenticatedRequest" => {
                                on_unauthenticated_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionCookieName" => {
                                session_cookie_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionTimeout" => {
                                session_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenEndpoint" => {
                                token_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserInfoEndpoint" => {
                                user_info_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticateOidcConfig {
                        authentication_request_extra_params: authentication_request_extra_params,
                        authorization_endpoint: authorization_endpoint.ok_or(::serde::de::Error::missing_field("AuthorizationEndpoint"))?,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        issuer: issuer.ok_or(::serde::de::Error::missing_field("Issuer"))?,
                        on_unauthenticated_request: on_unauthenticated_request,
                        scope: scope,
                        session_cookie_name: session_cookie_name,
                        session_timeout: session_timeout,
                        token_endpoint: token_endpoint.ok_or(::serde::de::Error::missing_field("TokenEndpoint"))?,
                        user_info_endpoint: user_info_endpoint.ok_or(::serde::de::Error::missing_field("UserInfoEndpoint"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificate.html) property type.
    #[derive(Debug, Default)]
    pub struct Certificate {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificate.html#cfn-elasticloadbalancingv2-listener-certificate-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Certificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Certificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Certificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Certificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Certificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Certificate {
                        certificate_arn: certificate_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.FixedResponseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-fixedresponseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FixedResponseConfig {
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listener-fixedresponseconfig-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`MessageBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listener-fixedresponseconfig-messagebody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_body: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listener-fixedresponseconfig-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for FixedResponseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            if let Some(ref message_body) = self.message_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageBody", message_body)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FixedResponseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FixedResponseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FixedResponseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FixedResponseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_type: Option<::Value<String>> = None;
                    let mut message_body: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageBody" => {
                                message_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FixedResponseConfig {
                        content_type: content_type,
                        message_body: message_body,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.ForwardConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-forwardconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardConfig {
        /// Property [`TargetGroupStickinessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-forwardconfig.html#cfn-elasticloadbalancingv2-listener-forwardconfig-targetgroupstickinessconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_stickiness_config: Option<::Value<TargetGroupStickinessConfig>>,
        /// Property [`TargetGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-forwardconfig.html#cfn-elasticloadbalancingv2-listener-forwardconfig-targetgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_groups: Option<::ValueList<TargetGroupTuple>>,
    }

    impl ::codec::SerializeValue for ForwardConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_group_stickiness_config) = self.target_group_stickiness_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupStickinessConfig", target_group_stickiness_config)?;
            }
            if let Some(ref target_groups) = self.target_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroups", target_groups)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ForwardConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_group_stickiness_config: Option<::Value<TargetGroupStickinessConfig>> = None;
                    let mut target_groups: Option<::ValueList<TargetGroupTuple>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupStickinessConfig" => {
                                target_group_stickiness_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroups" => {
                                target_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardConfig {
                        target_group_stickiness_config: target_group_stickiness_config,
                        target_groups: target_groups,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.RedirectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RedirectConfig {
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`Query`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-query).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-redirectconfig.html#cfn-elasticloadbalancingv2-listener-redirectconfig-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedirectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref query) = self.query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Query", query)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedirectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedirectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedirectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedirectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut query: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Query" => {
                                query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedirectConfig {
                        host: host,
                        path: path,
                        port: port,
                        protocol: protocol,
                        query: query,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.TargetGroupStickinessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgroupstickinessconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetGroupStickinessConfig {
        /// Property [`DurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgroupstickinessconfig.html#cfn-elasticloadbalancingv2-listener-targetgroupstickinessconfig-durationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_seconds: Option<::Value<u32>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgroupstickinessconfig.html#cfn-elasticloadbalancingv2-listener-targetgroupstickinessconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for TargetGroupStickinessConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_seconds) = self.duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationSeconds", duration_seconds)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupStickinessConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupStickinessConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupStickinessConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupStickinessConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_seconds: Option<::Value<u32>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationSeconds" => {
                                duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetGroupStickinessConfig {
                        duration_seconds: duration_seconds,
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.TargetGroupTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgrouptuple.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetGroupTuple {
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgrouptuple.html#cfn-elasticloadbalancingv2-listener-targetgrouptuple-targetgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-targetgrouptuple.html#cfn-elasticloadbalancingv2-listener-targetgrouptuple-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TargetGroupTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupTuple {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupTuple, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupTuple;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupTuple")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_group_arn: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetGroupTuple {
                        target_group_arn: target_group_arn,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod listener_certificate {
    //! Property types for the `ListenerCertificate` resource.

    /// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Debug, Default)]
    pub struct Certificate {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html#cfn-elasticloadbalancingv2-listener-certificates-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Certificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Certificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Certificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Certificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Certificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Certificate {
                        certificate_arn: certificate_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod listener_rule {
    //! Property types for the `ListenerRule` resource.

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`AuthenticateCognitoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-authenticatecognitoconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authenticate_cognito_config: Option<::Value<AuthenticateCognitoConfig>>,
        /// Property [`AuthenticateOidcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-authenticateoidcconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authenticate_oidc_config: Option<::Value<AuthenticateOidcConfig>>,
        /// Property [`FixedResponseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-fixedresponseconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_response_config: Option<::Value<FixedResponseConfig>>,
        /// Property [`ForwardConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-forwardconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub forward_config: Option<::Value<ForwardConfig>>,
        /// Property [`Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-order).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order: Option<::Value<u32>>,
        /// Property [`RedirectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-redirectconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redirect_config: Option<::Value<RedirectConfig>>,
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-targetgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-action.html#cfn-elasticloadbalancingv2-listenerrule-action-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authenticate_cognito_config) = self.authenticate_cognito_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticateCognitoConfig", authenticate_cognito_config)?;
            }
            if let Some(ref authenticate_oidc_config) = self.authenticate_oidc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticateOidcConfig", authenticate_oidc_config)?;
            }
            if let Some(ref fixed_response_config) = self.fixed_response_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedResponseConfig", fixed_response_config)?;
            }
            if let Some(ref forward_config) = self.forward_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardConfig", forward_config)?;
            }
            if let Some(ref order) = self.order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Order", order)?;
            }
            if let Some(ref redirect_config) = self.redirect_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectConfig", redirect_config)?;
            }
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut authenticate_cognito_config: Option<::Value<AuthenticateCognitoConfig>> = None;
                    let mut authenticate_oidc_config: Option<::Value<AuthenticateOidcConfig>> = None;
                    let mut fixed_response_config: Option<::Value<FixedResponseConfig>> = None;
                    let mut forward_config: Option<::Value<ForwardConfig>> = None;
                    let mut order: Option<::Value<u32>> = None;
                    let mut redirect_config: Option<::Value<RedirectConfig>> = None;
                    let mut target_group_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticateCognitoConfig" => {
                                authenticate_cognito_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthenticateOidcConfig" => {
                                authenticate_oidc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedResponseConfig" => {
                                fixed_response_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardConfig" => {
                                forward_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Order" => {
                                order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedirectConfig" => {
                                redirect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        authenticate_cognito_config: authenticate_cognito_config,
                        authenticate_oidc_config: authenticate_oidc_config,
                        fixed_response_config: fixed_response_config,
                        forward_config: forward_config,
                        order: order,
                        redirect_config: redirect_config,
                        target_group_arn: target_group_arn,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.AuthenticateCognitoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticateCognitoConfig {
        /// Property [`AuthenticationRequestExtraParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-authenticationrequestextraparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_request_extra_params: Option<::ValueMap<String>>,
        /// Property [`OnUnauthenticatedRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-onunauthenticatedrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_unauthenticated_request: Option<::Value<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: Option<::Value<String>>,
        /// Property [`SessionCookieName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-sessioncookiename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_cookie_name: Option<::Value<String>>,
        /// Property [`SessionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-sessiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_timeout: Option<::Value<u32>>,
        /// Property [`UserPoolArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-userpoolarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_arn: ::Value<String>,
        /// Property [`UserPoolClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-userpoolclientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_client_id: ::Value<String>,
        /// Property [`UserPoolDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticatecognitoconfig-userpooldomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_domain: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticateCognitoConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_request_extra_params) = self.authentication_request_extra_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationRequestExtraParams", authentication_request_extra_params)?;
            }
            if let Some(ref on_unauthenticated_request) = self.on_unauthenticated_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUnauthenticatedRequest", on_unauthenticated_request)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            if let Some(ref session_cookie_name) = self.session_cookie_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionCookieName", session_cookie_name)?;
            }
            if let Some(ref session_timeout) = self.session_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionTimeout", session_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolArn", &self.user_pool_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolClientId", &self.user_pool_client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolDomain", &self.user_pool_domain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticateCognitoConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticateCognitoConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticateCognitoConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticateCognitoConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_request_extra_params: Option<::ValueMap<String>> = None;
                    let mut on_unauthenticated_request: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;
                    let mut session_cookie_name: Option<::Value<String>> = None;
                    let mut session_timeout: Option<::Value<u32>> = None;
                    let mut user_pool_arn: Option<::Value<String>> = None;
                    let mut user_pool_client_id: Option<::Value<String>> = None;
                    let mut user_pool_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationRequestExtraParams" => {
                                authentication_request_extra_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnUnauthenticatedRequest" => {
                                on_unauthenticated_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionCookieName" => {
                                session_cookie_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionTimeout" => {
                                session_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolArn" => {
                                user_pool_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolClientId" => {
                                user_pool_client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolDomain" => {
                                user_pool_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticateCognitoConfig {
                        authentication_request_extra_params: authentication_request_extra_params,
                        on_unauthenticated_request: on_unauthenticated_request,
                        scope: scope,
                        session_cookie_name: session_cookie_name,
                        session_timeout: session_timeout,
                        user_pool_arn: user_pool_arn.ok_or(::serde::de::Error::missing_field("UserPoolArn"))?,
                        user_pool_client_id: user_pool_client_id.ok_or(::serde::de::Error::missing_field("UserPoolClientId"))?,
                        user_pool_domain: user_pool_domain.ok_or(::serde::de::Error::missing_field("UserPoolDomain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.AuthenticateOidcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticateOidcConfig {
        /// Property [`AuthenticationRequestExtraParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-authenticationrequestextraparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_request_extra_params: Option<::ValueMap<String>>,
        /// Property [`AuthorizationEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-authorizationendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_endpoint: ::Value<String>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-issuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer: ::Value<String>,
        /// Property [`OnUnauthenticatedRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-onunauthenticatedrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_unauthenticated_request: Option<::Value<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: Option<::Value<String>>,
        /// Property [`SessionCookieName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-sessioncookiename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_cookie_name: Option<::Value<String>>,
        /// Property [`SessionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-sessiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_timeout: Option<::Value<u32>>,
        /// Property [`TokenEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-tokenendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_endpoint: ::Value<String>,
        /// Property [`UseExistingClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-useexistingclientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_existing_client_secret: Option<::Value<bool>>,
        /// Property [`UserInfoEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-authenticateoidcconfig.html#cfn-elasticloadbalancingv2-listenerrule-authenticateoidcconfig-userinfoendpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_info_endpoint: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuthenticateOidcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_request_extra_params) = self.authentication_request_extra_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationRequestExtraParams", authentication_request_extra_params)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationEndpoint", &self.authorization_endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", &self.issuer)?;
            if let Some(ref on_unauthenticated_request) = self.on_unauthenticated_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUnauthenticatedRequest", on_unauthenticated_request)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            if let Some(ref session_cookie_name) = self.session_cookie_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionCookieName", session_cookie_name)?;
            }
            if let Some(ref session_timeout) = self.session_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionTimeout", session_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenEndpoint", &self.token_endpoint)?;
            if let Some(ref use_existing_client_secret) = self.use_existing_client_secret {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseExistingClientSecret", use_existing_client_secret)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserInfoEndpoint", &self.user_info_endpoint)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticateOidcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticateOidcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticateOidcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticateOidcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_request_extra_params: Option<::ValueMap<String>> = None;
                    let mut authorization_endpoint: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut issuer: Option<::Value<String>> = None;
                    let mut on_unauthenticated_request: Option<::Value<String>> = None;
                    let mut scope: Option<::Value<String>> = None;
                    let mut session_cookie_name: Option<::Value<String>> = None;
                    let mut session_timeout: Option<::Value<u32>> = None;
                    let mut token_endpoint: Option<::Value<String>> = None;
                    let mut use_existing_client_secret: Option<::Value<bool>> = None;
                    let mut user_info_endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationRequestExtraParams" => {
                                authentication_request_extra_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthorizationEndpoint" => {
                                authorization_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Issuer" => {
                                issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnUnauthenticatedRequest" => {
                                on_unauthenticated_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionCookieName" => {
                                session_cookie_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionTimeout" => {
                                session_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenEndpoint" => {
                                token_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseExistingClientSecret" => {
                                use_existing_client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserInfoEndpoint" => {
                                user_info_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticateOidcConfig {
                        authentication_request_extra_params: authentication_request_extra_params,
                        authorization_endpoint: authorization_endpoint.ok_or(::serde::de::Error::missing_field("AuthorizationEndpoint"))?,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        issuer: issuer.ok_or(::serde::de::Error::missing_field("Issuer"))?,
                        on_unauthenticated_request: on_unauthenticated_request,
                        scope: scope,
                        session_cookie_name: session_cookie_name,
                        session_timeout: session_timeout,
                        token_endpoint: token_endpoint.ok_or(::serde::de::Error::missing_field("TokenEndpoint"))?,
                        use_existing_client_secret: use_existing_client_secret,
                        user_info_endpoint: user_info_endpoint.ok_or(::serde::de::Error::missing_field("UserInfoEndpoint"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.FixedResponseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-fixedresponseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FixedResponseConfig {
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listenerrule-fixedresponseconfig-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`MessageBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listenerrule-fixedresponseconfig-messagebody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_body: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-fixedresponseconfig.html#cfn-elasticloadbalancingv2-listenerrule-fixedresponseconfig-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for FixedResponseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            if let Some(ref message_body) = self.message_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageBody", message_body)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FixedResponseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FixedResponseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FixedResponseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FixedResponseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_type: Option<::Value<String>> = None;
                    let mut message_body: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageBody" => {
                                message_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FixedResponseConfig {
                        content_type: content_type,
                        message_body: message_body,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.ForwardConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-forwardconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ForwardConfig {
        /// Property [`TargetGroupStickinessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-forwardconfig.html#cfn-elasticloadbalancingv2-listenerrule-forwardconfig-targetgroupstickinessconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_stickiness_config: Option<::Value<TargetGroupStickinessConfig>>,
        /// Property [`TargetGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-forwardconfig.html#cfn-elasticloadbalancingv2-listenerrule-forwardconfig-targetgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_groups: Option<::ValueList<TargetGroupTuple>>,
    }

    impl ::codec::SerializeValue for ForwardConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_group_stickiness_config) = self.target_group_stickiness_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupStickinessConfig", target_group_stickiness_config)?;
            }
            if let Some(ref target_groups) = self.target_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroups", target_groups)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ForwardConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_group_stickiness_config: Option<::Value<TargetGroupStickinessConfig>> = None;
                    let mut target_groups: Option<::ValueList<TargetGroupTuple>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupStickinessConfig" => {
                                target_group_stickiness_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroups" => {
                                target_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardConfig {
                        target_group_stickiness_config: target_group_stickiness_config,
                        target_groups: target_groups,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.HostHeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-hostheaderconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HostHeaderConfig {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-hostheaderconfig.html#cfn-elasticloadbalancingv2-listenerrule-hostheaderconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HostHeaderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostHeaderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostHeaderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostHeaderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostHeaderConfig")
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

                    Ok(HostHeaderConfig {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.HttpHeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httpheaderconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpHeaderConfig {
        /// Property [`HttpHeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httpheaderconfig.html#cfn-elasticloadbalancingv2-listenerrule-httpheaderconfig-httpheadername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_header_name: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httpheaderconfig.html#cfn-elasticloadbalancingv2-listenerrule-httpheaderconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HttpHeaderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_header_name) = self.http_header_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpHeaderName", http_header_name)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpHeaderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpHeaderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpHeaderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpHeaderConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_header_name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpHeaderName" => {
                                http_header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpHeaderConfig {
                        http_header_name: http_header_name,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.HttpRequestMethodConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httprequestmethodconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpRequestMethodConfig {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-httprequestmethodconfig.html#cfn-elasticloadbalancingv2-listenerrule-httprequestmethodconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for HttpRequestMethodConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpRequestMethodConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpRequestMethodConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpRequestMethodConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpRequestMethodConfig")
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

                    Ok(HttpRequestMethodConfig {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.PathPatternConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-pathpatternconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PathPatternConfig {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-pathpatternconfig.html#cfn-elasticloadbalancingv2-listenerrule-pathpatternconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PathPatternConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PathPatternConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PathPatternConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PathPatternConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PathPatternConfig")
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

                    Ok(PathPatternConfig {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.QueryStringConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringConfig {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringconfig.html#cfn-elasticloadbalancingv2-listenerrule-querystringconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<QueryStringKeyValue>>,
    }

    impl ::codec::SerializeValue for QueryStringConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryStringConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut values: Option<::ValueList<QueryStringKeyValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryStringConfig {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.QueryStringKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringkeyvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryStringKeyValue {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringkeyvalue.html#cfn-elasticloadbalancingv2-listenerrule-querystringkeyvalue-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-querystringkeyvalue.html#cfn-elasticloadbalancingv2-listenerrule-querystringkeyvalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for QueryStringKeyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryStringKeyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryStringKeyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryStringKeyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryStringKeyValue")
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

                    Ok(QueryStringKeyValue {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.RedirectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RedirectConfig {
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`Query`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-query).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query: Option<::Value<String>>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-redirectconfig.html#cfn-elasticloadbalancingv2-listenerrule-redirectconfig-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedirectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref query) = self.query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Query", query)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", &self.status_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedirectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedirectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedirectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedirectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut query: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Query" => {
                                query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedirectConfig {
                        host: host,
                        path: path,
                        port: port,
                        protocol: protocol,
                        query: query,
                        status_code: status_code.ok_or(::serde::de::Error::missing_field("StatusCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.RuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleCondition {
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`HostHeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-hostheaderconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_header_config: Option<::Value<HostHeaderConfig>>,
        /// Property [`HttpHeaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-httpheaderconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_header_config: Option<::Value<HttpHeaderConfig>>,
        /// Property [`HttpRequestMethodConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-httprequestmethodconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_request_method_config: Option<::Value<HttpRequestMethodConfig>>,
        /// Property [`PathPatternConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-pathpatternconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_pattern_config: Option<::Value<PathPatternConfig>>,
        /// Property [`QueryStringConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-querystringconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_string_config: Option<::Value<QueryStringConfig>>,
        /// Property [`SourceIpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-sourceipconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_ip_config: Option<::Value<SourceIpConfig>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-rulecondition.html#cfn-elasticloadbalancingv2-listenerrule-rulecondition-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RuleCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref host_header_config) = self.host_header_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostHeaderConfig", host_header_config)?;
            }
            if let Some(ref http_header_config) = self.http_header_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpHeaderConfig", http_header_config)?;
            }
            if let Some(ref http_request_method_config) = self.http_request_method_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpRequestMethodConfig", http_request_method_config)?;
            }
            if let Some(ref path_pattern_config) = self.path_pattern_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPatternConfig", path_pattern_config)?;
            }
            if let Some(ref query_string_config) = self.query_string_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringConfig", query_string_config)?;
            }
            if let Some(ref source_ip_config) = self.source_ip_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIpConfig", source_ip_config)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field: Option<::Value<String>> = None;
                    let mut host_header_config: Option<::Value<HostHeaderConfig>> = None;
                    let mut http_header_config: Option<::Value<HttpHeaderConfig>> = None;
                    let mut http_request_method_config: Option<::Value<HttpRequestMethodConfig>> = None;
                    let mut path_pattern_config: Option<::Value<PathPatternConfig>> = None;
                    let mut query_string_config: Option<::Value<QueryStringConfig>> = None;
                    let mut source_ip_config: Option<::Value<SourceIpConfig>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostHeaderConfig" => {
                                host_header_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpHeaderConfig" => {
                                http_header_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpRequestMethodConfig" => {
                                http_request_method_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathPatternConfig" => {
                                path_pattern_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringConfig" => {
                                query_string_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceIpConfig" => {
                                source_ip_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleCondition {
                        field: field,
                        host_header_config: host_header_config,
                        http_header_config: http_header_config,
                        http_request_method_config: http_request_method_config,
                        path_pattern_config: path_pattern_config,
                        query_string_config: query_string_config,
                        source_ip_config: source_ip_config,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.SourceIpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-sourceipconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceIpConfig {
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-sourceipconfig.html#cfn-elasticloadbalancingv2-listenerrule-sourceipconfig-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SourceIpConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceIpConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceIpConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceIpConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceIpConfig")
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

                    Ok(SourceIpConfig {
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.TargetGroupStickinessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetGroupStickinessConfig {
        /// Property [`DurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig.html#cfn-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig-durationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_seconds: Option<::Value<u32>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig.html#cfn-elasticloadbalancingv2-listenerrule-targetgroupstickinessconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for TargetGroupStickinessConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_seconds) = self.duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationSeconds", duration_seconds)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupStickinessConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupStickinessConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupStickinessConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupStickinessConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_seconds: Option<::Value<u32>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationSeconds" => {
                                duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetGroupStickinessConfig {
                        duration_seconds: duration_seconds,
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.TargetGroupTuple`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgrouptuple.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetGroupTuple {
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgrouptuple.html#cfn-elasticloadbalancingv2-listenerrule-targetgrouptuple-targetgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-targetgrouptuple.html#cfn-elasticloadbalancingv2-listenerrule-targetgrouptuple-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TargetGroupTuple {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupTuple {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupTuple, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupTuple;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupTuple")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_group_arn: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetGroupTuple {
                        target_group_arn: target_group_arn,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod load_balancer {
    //! Property types for the `LoadBalancer` resource.

    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.LoadBalancerAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct LoadBalancerAttribute {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattributes.html#cfn-elasticloadbalancingv2-loadbalancer-loadbalancerattributes-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattributes.html#cfn-elasticloadbalancingv2-loadbalancer-loadbalancerattributes-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoadBalancerAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoadBalancerAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoadBalancerAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoadBalancerAttribute")
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

                    Ok(LoadBalancerAttribute {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.SubnetMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct SubnetMapping {
        /// Property [`AllocationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html#cfn-elasticloadbalancingv2-loadbalancer-subnetmapping-allocationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allocation_id: Option<::Value<String>>,
        /// Property [`IPv6Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html#cfn-elasticloadbalancingv2-loadbalancer-subnetmapping-ipv6address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub i_pv6_address: Option<::Value<String>>,
        /// Property [`PrivateIPv4Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html#cfn-elasticloadbalancingv2-loadbalancer-subnetmapping-privateipv4address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_i_pv4_address: Option<::Value<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html#cfn-elasticloadbalancingv2-loadbalancer-subnetmapping-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubnetMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allocation_id) = self.allocation_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationId", allocation_id)?;
            }
            if let Some(ref i_pv6_address) = self.i_pv6_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPv6Address", i_pv6_address)?;
            }
            if let Some(ref private_i_pv4_address) = self.private_i_pv4_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIPv4Address", private_i_pv4_address)?;
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
                    let mut allocation_id: Option<::Value<String>> = None;
                    let mut i_pv6_address: Option<::Value<String>> = None;
                    let mut private_i_pv4_address: Option<::Value<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllocationId" => {
                                allocation_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPv6Address" => {
                                i_pv6_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateIPv4Address" => {
                                private_i_pv4_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubnetMapping {
                        allocation_id: allocation_id,
                        i_pv6_address: i_pv6_address,
                        private_i_pv4_address: private_i_pv4_address,
                        subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod target_group {
    //! Property types for the `TargetGroup` resource.

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.Matcher`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html) property type.
    #[derive(Debug, Default)]
    pub struct Matcher {
        /// Property [`GrpcCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html#cfn-elasticloadbalancingv2-targetgroup-matcher-grpccode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grpc_code: Option<::Value<String>>,
        /// Property [`HttpCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html#cfn-elasticloadbalancingv2-targetgroup-matcher-httpcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_code: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Matcher {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grpc_code) = self.grpc_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrpcCode", grpc_code)?;
            }
            if let Some(ref http_code) = self.http_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpCode", http_code)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Matcher {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Matcher, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Matcher;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Matcher")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grpc_code: Option<::Value<String>> = None;
                    let mut http_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GrpcCode" => {
                                grpc_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpCode" => {
                                http_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Matcher {
                        grpc_code: grpc_code,
                        http_code: http_code,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetDescription {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html#cfn-elasticloadbalancingv2-targetgroup-targetdescription-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html#cfn-elasticloadbalancingv2-targetgroup-targetdescription-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html#cfn-elasticloadbalancingv2-targetgroup-targetdescription-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TargetDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetDescription {
                        availability_zone: availability_zone,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetGroupAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetGroupAttribute {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html#cfn-elasticloadbalancingv2-targetgroup-targetgroupattribute-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html#cfn-elasticloadbalancingv2-targetgroup-targetgroupattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TargetGroupAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupAttribute")
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

                    Ok(TargetGroupAttribute {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
