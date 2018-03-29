//! Types for the `ElasticLoadBalancingV2` service.

/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource type.
#[derive(Debug)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerProperties {
    /// Property `Certificates`.
    #[serde(rename="Certificates")]
    pub certificates: Vec<self::listener::Certificate>,
    /// Property `DefaultActions`.
    #[serde(rename="DefaultActions")]
    pub default_actions: Vec<self::listener::Action>,
    /// Property `LoadBalancerArn`.
    #[serde(rename="LoadBalancerArn")]
    pub load_balancer_arn: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `Protocol`.
    #[serde(rename="Protocol")]
    pub protocol: String,
    /// Property `SslPolicy`.
    #[serde(rename="SslPolicy")]
    pub ssl_policy: String,
}

impl<'a> ::Resource<'a> for Listener {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerCertificateProperties {
    /// Property `Certificates`.
    #[serde(rename="Certificates")]
    pub certificates: Vec<self::listener_certificate::Certificate>,
    /// Property `ListenerArn`.
    #[serde(rename="ListenerArn")]
    pub listener_arn: String,
}

impl<'a> ::Resource<'a> for ListenerCertificate {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ListenerRuleProperties {
    /// Property `Actions`.
    #[serde(rename="Actions")]
    pub actions: Vec<self::listener_rule::Action>,
    /// Property `Conditions`.
    #[serde(rename="Conditions")]
    pub conditions: Vec<self::listener_rule::RuleCondition>,
    /// Property `ListenerArn`.
    #[serde(rename="ListenerArn")]
    pub listener_arn: String,
    /// Property `Priority`.
    #[serde(rename="Priority")]
    pub priority: u32,
}

impl<'a> ::Resource<'a> for ListenerRule {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    /// Property `IpAddressType`.
    #[serde(rename="IpAddressType")]
    pub ip_address_type: String,
    /// Property `LoadBalancerAttributes`.
    #[serde(rename="LoadBalancerAttributes")]
    pub load_balancer_attributes: Vec<self::load_balancer::LoadBalancerAttribute>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Scheme`.
    #[serde(rename="Scheme")]
    pub scheme: String,
    /// Property `SecurityGroups`.
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    /// Property `SubnetMappings`.
    #[serde(rename="SubnetMappings")]
    pub subnet_mappings: Vec<self::load_balancer::SubnetMapping>,
    /// Property `Subnets`.
    #[serde(rename="Subnets")]
    pub subnets: Vec<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for LoadBalancer {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct TargetGroupProperties {
    /// Property `HealthCheckIntervalSeconds`.
    #[serde(rename="HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: u32,
    /// Property `HealthCheckPath`.
    #[serde(rename="HealthCheckPath")]
    pub health_check_path: String,
    /// Property `HealthCheckPort`.
    #[serde(rename="HealthCheckPort")]
    pub health_check_port: String,
    /// Property `HealthCheckProtocol`.
    #[serde(rename="HealthCheckProtocol")]
    pub health_check_protocol: String,
    /// Property `HealthCheckTimeoutSeconds`.
    #[serde(rename="HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: u32,
    /// Property `HealthyThresholdCount`.
    #[serde(rename="HealthyThresholdCount")]
    pub healthy_threshold_count: u32,
    /// Property `Matcher`.
    #[serde(rename="Matcher")]
    pub matcher: self::target_group::Matcher,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Port`.
    #[serde(rename="Port")]
    pub port: u32,
    /// Property `Protocol`.
    #[serde(rename="Protocol")]
    pub protocol: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    /// Property `TargetGroupAttributes`.
    #[serde(rename="TargetGroupAttributes")]
    pub target_group_attributes: Vec<self::target_group::TargetGroupAttribute>,
    /// Property `TargetType`.
    #[serde(rename="TargetType")]
    pub target_type: String,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    pub targets: Vec<self::target_group::TargetDescription>,
    /// Property `UnhealthyThresholdCount`.
    #[serde(rename="UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: u32,
    /// Property `VpcId`.
    #[serde(rename="VpcId")]
    pub vpc_id: String,
}

impl<'a> ::Resource<'a> for TargetGroup {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `TargetGroupArn`.
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Certificate {
        /// Property `CertificateArn`.
        #[serde(rename="CertificateArn")]
        pub certificate_arn: String,
    }
}

pub mod listener_certificate {
    //! Property types for the `ListenerCertificate` resource.

    /// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Certificate {
        /// Property `CertificateArn`.
        #[serde(rename="CertificateArn")]
        pub certificate_arn: String,
    }
}

pub mod listener_rule {
    //! Property types for the `ListenerRule` resource.

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-actions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `TargetGroupArn`.
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.RuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-conditions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RuleCondition {
        /// Property `Field`.
        #[serde(rename="Field")]
        pub field: String,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }
}

pub mod load_balancer {
    //! Property types for the `LoadBalancer` resource.

    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.LoadBalancerAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattributes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancerAttribute {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.SubnetMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubnetMapping {
        /// Property `AllocationId`.
        #[serde(rename="AllocationId")]
        pub allocation_id: String,
        /// Property `SubnetId`.
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }
}

pub mod target_group {
    //! Property types for the `TargetGroup` resource.

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.Matcher`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Matcher {
        /// Property `HttpCode`.
        #[serde(rename="HttpCode")]
        pub http_code: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetDescription {
        /// Property `AvailabilityZone`.
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `Port`.
        #[serde(rename="Port")]
        pub port: u32,
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetGroupAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TargetGroupAttribute {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}
