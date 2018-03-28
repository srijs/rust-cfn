/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerProperties {
    #[serde(rename="Certificates")]
    pub certificates: (),
    #[serde(rename="DefaultActions")]
    pub default_actions: (),
    #[serde(rename="LoadBalancerArn")]
    pub load_balancer_arn: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="Protocol")]
    pub protocol: (),
    #[serde(rename="SslPolicy")]
    pub ssl_policy: (),
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

impl From<ListenerProperties> for Listener {
    fn from(properties: ListenerProperties) -> Listener {
        Listener { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::ListenerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenercertificate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerCertificate {
    properties: ListenerCertificateProperties
}

/// Properties for the `ListenerCertificate` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerCertificateProperties {
    #[serde(rename="Certificates")]
    pub certificates: (),
    #[serde(rename="ListenerArn")]
    pub listener_arn: (),
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

impl From<ListenerCertificateProperties> for ListenerCertificate {
    fn from(properties: ListenerCertificateProperties) -> ListenerCertificate {
        ListenerCertificate { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::ListenerRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listenerrule.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerRule {
    properties: ListenerRuleProperties
}

/// Properties for the `ListenerRule` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerRuleProperties {
    #[serde(rename="Actions")]
    pub actions: (),
    #[serde(rename="Conditions")]
    pub conditions: (),
    #[serde(rename="ListenerArn")]
    pub listener_arn: (),
    #[serde(rename="Priority")]
    pub priority: (),
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

impl From<ListenerRuleProperties> for ListenerRule {
    fn from(properties: ListenerRuleProperties) -> ListenerRule {
        ListenerRule { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-loadbalancer.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    #[serde(rename="IpAddressType")]
    pub ip_address_type: (),
    #[serde(rename="LoadBalancerAttributes")]
    pub load_balancer_attributes: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Scheme")]
    pub scheme: (),
    #[serde(rename="SecurityGroups")]
    pub security_groups: (),
    #[serde(rename="SubnetMappings")]
    pub subnet_mappings: (),
    #[serde(rename="Subnets")]
    pub subnets: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="Type")]
    pub type_: (),
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

impl From<LoadBalancerProperties> for LoadBalancer {
    fn from(properties: LoadBalancerProperties) -> LoadBalancer {
        LoadBalancer { properties }
    }
}

/// The [`AWS::ElasticLoadBalancingV2::TargetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-targetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct TargetGroup {
    properties: TargetGroupProperties
}

/// Properties for the `TargetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct TargetGroupProperties {
    #[serde(rename="HealthCheckIntervalSeconds")]
    pub health_check_interval_seconds: (),
    #[serde(rename="HealthCheckPath")]
    pub health_check_path: (),
    #[serde(rename="HealthCheckPort")]
    pub health_check_port: (),
    #[serde(rename="HealthCheckProtocol")]
    pub health_check_protocol: (),
    #[serde(rename="HealthCheckTimeoutSeconds")]
    pub health_check_timeout_seconds: (),
    #[serde(rename="HealthyThresholdCount")]
    pub healthy_threshold_count: (),
    #[serde(rename="Matcher")]
    pub matcher: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Port")]
    pub port: (),
    #[serde(rename="Protocol")]
    pub protocol: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="TargetGroupAttributes")]
    pub target_group_attributes: (),
    #[serde(rename="TargetType")]
    pub target_type: (),
    #[serde(rename="Targets")]
    pub targets: (),
    #[serde(rename="UnhealthyThresholdCount")]
    pub unhealthy_threshold_count: (),
    #[serde(rename="VpcId")]
    pub vpc_id: (),
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

impl From<TargetGroupProperties> for TargetGroup {
    fn from(properties: TargetGroupProperties) -> TargetGroup {
        TargetGroup { properties }
    }
}

