/// The [`AWS::ElasticLoadBalancingV2::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticloadbalancingv2-listener.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Serialize, Deserialize)]
pub struct ListenerProperties {
    #[serde(rename="Certificates")]
    pub certificates: Vec<()>,
    #[serde(rename="DefaultActions")]
    pub default_actions: Vec<()>,
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
    pub certificates: Vec<()>,
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
    pub actions: Vec<()>,
    #[serde(rename="Conditions")]
    pub conditions: Vec<()>,
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
    pub ip_address_type: String,
    #[serde(rename="LoadBalancerAttributes")]
    pub load_balancer_attributes: Vec<()>,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Scheme")]
    pub scheme: String,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="SubnetMappings")]
    pub subnet_mappings: Vec<()>,
    #[serde(rename="Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
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
    pub matcher: (),
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Port")]
    pub port: u32,
    #[serde(rename="Protocol")]
    pub protocol: String,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="TargetGroupAttributes")]
    pub target_group_attributes: Vec<()>,
    #[serde(rename="TargetType")]
    pub target_type: String,
    #[serde(rename="Targets")]
    pub targets: Vec<()>,
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

impl From<TargetGroupProperties> for TargetGroup {
    fn from(properties: TargetGroupProperties) -> TargetGroup {
        TargetGroup { properties }
    }
}
