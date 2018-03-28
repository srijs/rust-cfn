/// The [`AWS::ElasticLoadBalancing::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html) resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    #[serde(rename="AccessLoggingPolicy")]
    pub access_logging_policy: (),
    #[serde(rename="AppCookieStickinessPolicy")]
    pub app_cookie_stickiness_policy: (),
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: (),
    #[serde(rename="ConnectionDrainingPolicy")]
    pub connection_draining_policy: (),
    #[serde(rename="ConnectionSettings")]
    pub connection_settings: (),
    #[serde(rename="CrossZone")]
    pub cross_zone: (),
    #[serde(rename="HealthCheck")]
    pub health_check: (),
    #[serde(rename="Instances")]
    pub instances: (),
    #[serde(rename="LBCookieStickinessPolicy")]
    pub lb_cookie_stickiness_policy: (),
    #[serde(rename="Listeners")]
    pub listeners: (),
    #[serde(rename="LoadBalancerName")]
    pub load_balancer_name: (),
    #[serde(rename="Policies")]
    pub policies: (),
    #[serde(rename="Scheme")]
    pub scheme: (),
    #[serde(rename="SecurityGroups")]
    pub security_groups: (),
    #[serde(rename="Subnets")]
    pub subnets: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for LoadBalancer {
    type Properties = LoadBalancerProperties;
    const TYPE: &'static str = "AWS::ElasticLoadBalancing::LoadBalancer";
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

