/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource type.
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerProperties {
    #[serde(rename="Certificates")]
    pub certificates: Vec<self::listener::Certificate>,
    #[serde(rename="DefaultActions")]
    pub default_actions: Vec<self::listener::Action>,
    #[serde(rename="LoadBalancerArn")]
    pub load_balancer_arn: String,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="Protocol")]
    pub protocol: String,
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
pub struct ListenerCertificate {
    properties: ListenerCertificateProperties
}

/// Properties for the `ListenerCertificate` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerCertificateProperties {
    #[serde(rename="Certificates")]
    pub certificates: Vec<self::listener_certificate::Certificate>,
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
pub struct ListenerRule {
    properties: ListenerRuleProperties
}

/// Properties for the `ListenerRule` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerRuleProperties {
    #[serde(rename="Actions")]
    pub actions: Vec<self::listener_rule::Action>,
    #[serde(rename="Conditions")]
    pub conditions: Vec<self::listener_rule::RuleCondition>,
    #[serde(rename="ListenerArn")]
    pub listener_arn: String,
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
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    #[serde(rename="IpAddressType")]
    pub ip_address_type: String,
    #[serde(rename="LoadBalancerAttributes")]
    pub load_balancer_attributes: Vec<self::load_balancer::LoadBalancerAttribute>,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Scheme")]
    pub scheme: String,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="SubnetMappings")]
    pub subnet_mappings: Vec<self::load_balancer::SubnetMapping>,
    #[serde(rename="Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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
pub struct TargetGroup {
    properties: TargetGroupProperties
}

/// Properties for the `TargetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct TargetGroupProperties {
    #[serde(rename="HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: u32,
    #[serde(rename="HealthCheckPath")]
    pub health_check_path: String,
    #[serde(rename="HealthCheckPort")]
    pub health_check_port: String,
    #[serde(rename="HealthCheckProtocol")]
    pub health_check_protocol: String,
    #[serde(rename="HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: u32,
    #[serde(rename="HealthyThresholdCount")]
    pub healthy_threshold_count: u32,
    #[serde(rename="Matcher")]
    pub matcher: self::target_group::Matcher,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="Protocol")]
    pub protocol: String,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="TargetGroupAttributes")]
    pub target_group_attributes: Vec<self::target_group::TargetGroupAttribute>,
    #[serde(rename="TargetType")]
    pub target_type: String,
    #[serde(rename="Targets")]
    pub targets: Vec<self::target_group::TargetDescription>,
    #[serde(rename="UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: u32,
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
    /// The [`AWS::ElasticLoadBalancingV2::Listener.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-defaultactions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Action {
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::Listener.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Certificate {
        #[serde(rename="CertificateArn")]
        pub certificate_arn: String,
    }

}

pub mod listener_certificate {
    /// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate.Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listener-certificates.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Certificate {
        #[serde(rename="CertificateArn")]
        pub certificate_arn: String,
    }

}

pub mod listener_rule {
    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-actions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Action {
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::ListenerRule.RuleCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-listenerrule-conditions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct RuleCondition {
        #[serde(rename="Field")]
        pub field: String,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

}

pub mod load_balancer {
    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.LoadBalancerAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-loadbalancerattributes.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LoadBalancerAttribute {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::LoadBalancer.SubnetMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-loadbalancer-subnetmapping.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SubnetMapping {
        #[serde(rename="AllocationId")]
        pub allocation_id: String,
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }

}

pub mod target_group {
    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.Matcher`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-matcher.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Matcher {
        #[serde(rename="HttpCode")]
        pub http_code: String,
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetdescription.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TargetDescription {
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Port")]
        pub port: u32,
    }

    /// The [`AWS::ElasticLoadBalancingV2::TargetGroup.TargetGroupAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticloadbalancingv2-targetgroup-targetgroupattribute.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TargetGroupAttribute {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

