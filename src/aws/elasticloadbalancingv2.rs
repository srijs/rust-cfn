//! Types for the `ElasticLoadBalancingV2` service.

/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource type.
#[derive(Debug)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Debug)]
pub struct ListenerProperties {
    /// Property `Certificates`.
    pub certificates: Option<::ValueList<self::listener::Certificate>>,
    /// Property `DefaultActions`.
    pub default_actions: ::ValueList<self::listener::Action>,
    /// Property `LoadBalancerArn`.
    pub load_balancer_arn: ::Value<String>,
    /// Property `Port`.
    pub port: ::Value<u32>,
    /// Property `Protocol`.
    pub protocol: ::Value<String>,
    /// Property `SslPolicy`.
    pub ssl_policy: Option<::Value<String>>,
}

impl ::serde::Serialize for ListenerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificates) = self.certificates {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificates", certificates)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultActions", &self.default_actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerArn", &self.load_balancer_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
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
                let mut certificates = None;
                let mut default_actions = None;
                let mut load_balancer_arn = None;
                let mut port = None;
                let mut protocol = None;
                let mut ssl_policy = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
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
                    certificates: certificates,
                    default_actions: default_actions.ok_or(::serde::de::Error::missing_field("DefaultActions"))?,
                    load_balancer_arn: load_balancer_arn.ok_or(::serde::de::Error::missing_field("LoadBalancerArn"))?,
                    port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
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
#[derive(Debug)]
pub struct ListenerCertificate {
    properties: ListenerCertificateProperties
}

/// Properties for the `ListenerCertificate` resource.
#[derive(Debug)]
pub struct ListenerCertificateProperties {
    /// Property `Certificates`.
    pub certificates: ::ValueList<self::listener_certificate::Certificate>,
    /// Property `ListenerArn`.
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
                let mut certificates = None;
                let mut listener_arn = None;

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
#[derive(Debug)]
pub struct ListenerRule {
    properties: ListenerRuleProperties
}

/// Properties for the `ListenerRule` resource.
#[derive(Debug)]
pub struct ListenerRuleProperties {
    /// Property `Actions`.
    pub actions: ::ValueList<self::listener_rule::Action>,
    /// Property `Conditions`.
    pub conditions: ::ValueList<self::listener_rule::RuleCondition>,
    /// Property `ListenerArn`.
    pub listener_arn: ::Value<String>,
    /// Property `Priority`.
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
                let mut actions = None;
                let mut conditions = None;
                let mut listener_arn = None;
                let mut priority = None;

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
#[derive(Debug)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug)]
pub struct LoadBalancerProperties {
    /// Property `IpAddressType`.
    pub ip_address_type: Option<::Value<String>>,
    /// Property `LoadBalancerAttributes`.
    pub load_balancer_attributes: Option<::ValueList<self::load_balancer::LoadBalancerAttribute>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `Scheme`.
    pub scheme: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    pub security_groups: Option<::ValueList<String>>,
    /// Property `SubnetMappings`.
    pub subnet_mappings: Option<::ValueList<self::load_balancer::SubnetMapping>>,
    /// Property `Subnets`.
    pub subnets: Option<::ValueList<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    pub type_: Option<::Value<String>>,
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
        if let Some(ref type_) = self.type_ {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
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
                let mut ip_address_type = None;
                let mut load_balancer_attributes = None;
                let mut name = None;
                let mut scheme = None;
                let mut security_groups = None;
                let mut subnet_mappings = None;
                let mut subnets = None;
                let mut tags = None;
                let mut type_ = None;

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
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    type_: type_,
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
#[derive(Debug)]
pub struct TargetGroup {
    properties: TargetGroupProperties
}

/// Properties for the `TargetGroup` resource.
#[derive(Debug)]
pub struct TargetGroupProperties {
    /// Property `HealthCheckIntervalSeconds`.
    pub health_check_interval_seconds: Option<::Value<u32>>,
    /// Property `HealthCheckPath`.
    pub health_check_path: Option<::Value<String>>,
    /// Property `HealthCheckPort`.
    pub health_check_port: Option<::Value<String>>,
    /// Property `HealthCheckProtocol`.
    pub health_check_protocol: Option<::Value<String>>,
    /// Property `HealthCheckTimeoutSeconds`.
    pub health_check_timeout_seconds: Option<::Value<u32>>,
    /// Property `HealthyThresholdCount`.
    pub healthy_threshold_count: Option<::Value<u32>>,
    /// Property `Matcher`.
    pub matcher: Option<::Value<self::target_group::Matcher>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `Port`.
    pub port: ::Value<u32>,
    /// Property `Protocol`.
    pub protocol: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TargetGroupAttributes`.
    pub target_group_attributes: Option<::ValueList<self::target_group::TargetGroupAttribute>>,
    /// Property `TargetType`.
    pub target_type: Option<::Value<String>>,
    /// Property `Targets`.
    pub targets: Option<::ValueList<self::target_group::TargetDescription>>,
    /// Property `UnhealthyThresholdCount`.
    pub unhealthy_threshold_count: Option<::Value<u32>>,
    /// Property `VpcId`.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for TargetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
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
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
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
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
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
                let mut health_check_interval_seconds = None;
                let mut health_check_path = None;
                let mut health_check_port = None;
                let mut health_check_protocol = None;
                let mut health_check_timeout_seconds = None;
                let mut healthy_threshold_count = None;
                let mut matcher = None;
                let mut name = None;
                let mut port = None;
                let mut protocol = None;
                let mut tags = None;
                let mut target_group_attributes = None;
                let mut target_type = None;
                let mut targets = None;
                let mut unhealthy_threshold_count = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
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
                    health_check_interval_seconds: health_check_interval_seconds,
                    health_check_path: health_check_path,
                    health_check_port: health_check_port,
                    health_check_protocol: health_check_protocol,
                    health_check_timeout_seconds: health_check_timeout_seconds,
                    healthy_threshold_count: healthy_threshold_count,
                    matcher: matcher,
                    name: name,
                    port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    tags: tags,
                    target_group_attributes: target_group_attributes,
                    target_type: target_type,
                    targets: targets,
                    unhealthy_threshold_count: unhealthy_threshold_count,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
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

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-defaultactions.html) property type.
    #[derive(Debug)]
    pub struct Action {
        /// Property `TargetGroupArn`.
        pub target_group_arn: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", &self.target_group_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
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
                    let mut target_group_arn = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        target_group_arn: target_group_arn.ok_or(::serde::de::Error::missing_field("TargetGroupArn"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Debug)]
    pub struct Certificate {
        /// Property `CertificateArn`.
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
                    let mut certificate_arn = None;

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

pub mod listener_certificate {
    //! Property types for the `ListenerCertificate` resource.

    /// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Debug)]
    pub struct Certificate {
        /// Property `CertificateArn`.
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
                    let mut certificate_arn = None;

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

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-actions.html) property type.
    #[derive(Debug)]
    pub struct Action {
        /// Property `TargetGroupArn`.
        pub target_group_arn: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", &self.target_group_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
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
                    let mut target_group_arn = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        target_group_arn: target_group_arn.ok_or(::serde::de::Error::missing_field("TargetGroupArn"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.RuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-conditions.html) property type.
    #[derive(Debug)]
    pub struct RuleCondition {
        /// Property `Field`.
        pub field: Option<::Value<String>>,
        /// Property `Values`.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RuleCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
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
                    let mut field = None;
                    let mut values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleCondition {
                        field: field,
                        values: values,
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
    #[derive(Debug)]
    pub struct LoadBalancerAttribute {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Value`.
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
                    let mut key = None;
                    let mut value = None;

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
    #[derive(Debug)]
    pub struct SubnetMapping {
        /// Property `AllocationId`.
        pub allocation_id: ::Value<String>,
        /// Property `SubnetId`.
        pub subnet_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SubnetMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocationId", &self.allocation_id)?;
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
                    let mut allocation_id = None;
                    let mut subnet_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllocationId" => {
                                allocation_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubnetMapping {
                        allocation_id: allocation_id.ok_or(::serde::de::Error::missing_field("AllocationId"))?,
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
    #[derive(Debug)]
    pub struct Matcher {
        /// Property `HttpCode`.
        pub http_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for Matcher {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpCode", &self.http_code)?;
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
                    let mut http_code = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HttpCode" => {
                                http_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Matcher {
                        http_code: http_code.ok_or(::serde::de::Error::missing_field("HttpCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html) property type.
    #[derive(Debug)]
    pub struct TargetDescription {
        /// Property `AvailabilityZone`.
        pub availability_zone: Option<::Value<String>>,
        /// Property `Id`.
        pub id: ::Value<String>,
        /// Property `Port`.
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
                    let mut availability_zone = None;
                    let mut id = None;
                    let mut port = None;

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
    #[derive(Debug)]
    pub struct TargetGroupAttribute {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Value`.
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
                    let mut key = None;
                    let mut value = None;

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
