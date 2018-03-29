//! Types for the `ElasticLoadBalancing` service.

/// The [`AWS::ElasticLoadBalancing::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html) resource type.
#[derive(Debug)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoadBalancerProperties {
    /// Property `AccessLoggingPolicy`.
    #[serde(rename="AccessLoggingPolicy")]
    pub access_logging_policy: self::load_balancer::AccessLoggingPolicy,
    /// Property `AppCookieStickinessPolicy`.
    #[serde(rename="AppCookieStickinessPolicy")]
    pub app_cookie_stickiness_policy: Vec<self::load_balancer::AppCookieStickinessPolicy>,
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: Vec<String>,
    /// Property `ConnectionDrainingPolicy`.
    #[serde(rename="ConnectionDrainingPolicy")]
    pub connection_draining_policy: self::load_balancer::ConnectionDrainingPolicy,
    /// Property `ConnectionSettings`.
    #[serde(rename="ConnectionSettings")]
    pub connection_settings: self::load_balancer::ConnectionSettings,
    /// Property `CrossZone`.
    #[serde(rename="CrossZone")]
    pub cross_zone: bool,
    /// Property `HealthCheck`.
    #[serde(rename="HealthCheck")]
    pub health_check: self::load_balancer::HealthCheck,
    /// Property `Instances`.
    #[serde(rename="Instances")]
    pub instances: Vec<String>,
    /// Property `LBCookieStickinessPolicy`.
    #[serde(rename="LBCookieStickinessPolicy")]
    pub lb_cookie_stickiness_policy: Vec<self::load_balancer::LBCookieStickinessPolicy>,
    /// Property `Listeners`.
    #[serde(rename="Listeners")]
    pub listeners: Vec<self::load_balancer::Listeners>,
    /// Property `LoadBalancerName`.
    #[serde(rename="LoadBalancerName")]
    pub load_balancer_name: String,
    /// Property `Policies`.
    #[serde(rename="Policies")]
    pub policies: Vec<self::load_balancer::Policies>,
    /// Property `Scheme`.
    #[serde(rename="Scheme")]
    pub scheme: String,
    /// Property `SecurityGroups`.
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    /// Property `Subnets`.
    #[serde(rename="Subnets")]
    pub subnets: Vec<String>,
    /// Property `Tags`.
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

impl ::private::Sealed for LoadBalancer {}

impl From<LoadBalancerProperties> for LoadBalancer {
    fn from(properties: LoadBalancerProperties) -> LoadBalancer {
        LoadBalancer { properties }
    }
}

pub mod load_balancer {
    //! Property types for the `LoadBalancer` resource.

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AccessLoggingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccessLoggingPolicy {
        /// Property `EmitInterval`.
        #[serde(rename="EmitInterval")]
        pub emit_interval: u32,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `S3BucketName`.
        #[serde(rename="S3BucketName")]
        pub s3_bucket_name: String,
        /// Property `S3BucketPrefix`.
        #[serde(rename="S3BucketPrefix")]
        pub s3_bucket_prefix: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AppCookieStickinessPolicy {
        /// Property `CookieName`.
        #[serde(rename="CookieName")]
        pub cookie_name: String,
        /// Property `PolicyName`.
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionDrainingPolicy {
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `Timeout`.
        #[serde(rename="Timeout")]
        pub timeout: u32,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionSettings {
        /// Property `IdleTimeout`.
        #[serde(rename="IdleTimeout")]
        pub idle_timeout: u32,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HealthCheck {
        /// Property `HealthyThreshold`.
        #[serde(rename="HealthyThreshold")]
        pub healthy_threshold: String,
        /// Property `Interval`.
        #[serde(rename="Interval")]
        pub interval: String,
        /// Property `Target`.
        #[serde(rename="Target")]
        pub target: String,
        /// Property `Timeout`.
        #[serde(rename="Timeout")]
        pub timeout: String,
        /// Property `UnhealthyThreshold`.
        #[serde(rename="UnhealthyThreshold")]
        pub unhealthy_threshold: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LBCookieStickinessPolicy {
        /// Property `CookieExpirationPeriod`.
        #[serde(rename="CookieExpirationPeriod")]
        pub cookie_expiration_period: String,
        /// Property `PolicyName`.
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Listeners {
        /// Property `InstancePort`.
        #[serde(rename="InstancePort")]
        pub instance_port: String,
        /// Property `InstanceProtocol`.
        #[serde(rename="InstanceProtocol")]
        pub instance_protocol: String,
        /// Property `LoadBalancerPort`.
        #[serde(rename="LoadBalancerPort")]
        pub load_balancer_port: String,
        /// Property `PolicyNames`.
        #[serde(rename="PolicyNames")]
        pub policy_names: Vec<String>,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
        /// Property `SSLCertificateId`.
        #[serde(rename="SSLCertificateId")]
        pub ssl_certificate_id: String,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `Attributes`.
        #[serde(rename="Attributes")]
        pub attributes: Vec<::json::Value>,
        /// Property `InstancePorts`.
        #[serde(rename="InstancePorts")]
        pub instance_ports: Vec<String>,
        /// Property `LoadBalancerPorts`.
        #[serde(rename="LoadBalancerPorts")]
        pub load_balancer_ports: Vec<String>,
        /// Property `PolicyName`.
        #[serde(rename="PolicyName")]
        pub policy_name: String,
        /// Property `PolicyType`.
        #[serde(rename="PolicyType")]
        pub policy_type: String,
    }
}
