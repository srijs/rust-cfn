/// The [`AWS::ElasticLoadBalancing::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html) resource type.
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    #[serde(rename="AccessLoggingPolicy")]
    pub access_logging_policy: self::load_balancer::AccessLoggingPolicy,
    #[serde(rename="AppCookieStickinessPolicy")]
    pub app_cookie_stickiness_policy: Vec<self::load_balancer::AppCookieStickinessPolicy>,
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    #[serde(rename="ConnectionDrainingPolicy")]
    pub connection_draining_policy: self::load_balancer::ConnectionDrainingPolicy,
    #[serde(rename="ConnectionSettings")]
    pub connection_settings: self::load_balancer::ConnectionSettings,
    #[serde(rename="CrossZone")]
    pub cross_zone: bool,
    #[serde(rename="HealthCheck")]
    pub health_check: self::load_balancer::HealthCheck,
    #[serde(rename="Instances")]
    pub instances: Vec<String>,
    #[serde(rename="LBCookieStickinessPolicy")]
    pub lb_cookie_stickiness_policy: Vec<self::load_balancer::LBCookieStickinessPolicy>,
    #[serde(rename="Listeners")]
    pub listeners: Vec<self::load_balancer::Listeners>,
    #[serde(rename="LoadBalancerName")]
    pub load_balancer_name: String,
    #[serde(rename="Policies")]
    pub policies: Vec<self::load_balancer::Policies>,
    #[serde(rename="Scheme")]
    pub scheme: String,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="Subnets")]
    pub subnets: Vec<String>,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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

pub mod load_balancer {
    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AccessLoggingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AccessLoggingPolicy {
        #[serde(rename="EmitInterval")]
        pub emit_interval: u32,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="S3BucketName")]
        pub s3_bucket_name: String,
        #[serde(rename="S3BucketPrefix")]
        pub s3_bucket_prefix: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AppCookieStickinessPolicy {
        #[serde(rename="CookieName")]
        pub cookie_name: String,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ConnectionDrainingPolicy {
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Timeout")]
        pub timeout: u32,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ConnectionSettings {
        #[serde(rename="IdleTimeout")]
        pub idle_timeout: u32,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct HealthCheck {
        #[serde(rename="HealthyThreshold")]
        pub healthy_threshold: String,
        #[serde(rename="Interval")]
        pub interval: String,
        #[serde(rename="Target")]
        pub target: String,
        #[serde(rename="Timeout")]
        pub timeout: String,
        #[serde(rename="UnhealthyThreshold")]
        pub unhealthy_threshold: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LBCookieStickinessPolicy {
        #[serde(rename="CookieExpirationPeriod")]
        pub cookie_expiration_period: String,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Listeners {
        #[serde(rename="InstancePort")]
        pub instance_port: String,
        #[serde(rename="InstanceProtocol")]
        pub instance_protocol: String,
        #[serde(rename="LoadBalancerPort")]
        pub load_balancer_port: String,
        #[serde(rename="PolicyNames")]
        pub policy_names: Vec<String>,
        #[serde(rename="Protocol")]
        pub protocol: String,
        #[serde(rename="SSLCertificateId")]
        pub ssl_certificate_id: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Policies {
        #[serde(rename="Attributes")]
        pub attributes: Vec<::json::Value>,
        #[serde(rename="InstancePorts")]
        pub instance_ports: Vec<String>,
        #[serde(rename="LoadBalancerPorts")]
        pub load_balancer_ports: Vec<String>,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
        #[serde(rename="PolicyType")]
        pub policy_type: String,
    }

}

