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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_logging_policy: Option<self::load_balancer::AccessLoggingPolicy>,
    /// Property `AppCookieStickinessPolicy`.
    #[serde(rename="AppCookieStickinessPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_cookie_stickiness_policy: Option<Vec<self::load_balancer::AppCookieStickinessPolicy>>,
    /// Property `AvailabilityZones`.
    #[serde(rename="AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// Property `ConnectionDrainingPolicy`.
    #[serde(rename="ConnectionDrainingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_draining_policy: Option<self::load_balancer::ConnectionDrainingPolicy>,
    /// Property `ConnectionSettings`.
    #[serde(rename="ConnectionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<self::load_balancer::ConnectionSettings>,
    /// Property `CrossZone`.
    #[serde(rename="CrossZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_zone: Option<bool>,
    /// Property `HealthCheck`.
    #[serde(rename="HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<self::load_balancer::HealthCheck>,
    /// Property `Instances`.
    #[serde(rename="Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// Property `LBCookieStickinessPolicy`.
    #[serde(rename="LBCookieStickinessPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb_cookie_stickiness_policy: Option<Vec<self::load_balancer::LBCookieStickinessPolicy>>,
    /// Property `Listeners`.
    #[serde(rename="Listeners")]
    pub listeners: Vec<self::load_balancer::Listeners>,
    /// Property `LoadBalancerName`.
    #[serde(rename="LoadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// Property `Policies`.
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<self::load_balancer::Policies>>,
    /// Property `Scheme`.
    #[serde(rename="Scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// Property `SecurityGroups`.
    #[serde(rename="SecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// Property `Subnets`.
    #[serde(rename="Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub emit_interval: Option<u32>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `S3BucketName`.
        #[serde(rename="S3BucketName")]
        pub s3_bucket_name: String,
        /// Property `S3BucketPrefix`.
        #[serde(rename="S3BucketPrefix")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_bucket_prefix: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<u32>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cookie_expiration_period: Option<String>,
        /// Property `PolicyName`.
        #[serde(rename="PolicyName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policy_name: Option<String>,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Listeners {
        /// Property `InstancePort`.
        #[serde(rename="InstancePort")]
        pub instance_port: String,
        /// Property `InstanceProtocol`.
        #[serde(rename="InstanceProtocol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_protocol: Option<String>,
        /// Property `LoadBalancerPort`.
        #[serde(rename="LoadBalancerPort")]
        pub load_balancer_port: String,
        /// Property `PolicyNames`.
        #[serde(rename="PolicyNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub policy_names: Option<Vec<String>>,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
        /// Property `SSLCertificateId`.
        #[serde(rename="SSLCertificateId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ssl_certificate_id: Option<String>,
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `Attributes`.
        #[serde(rename="Attributes")]
        pub attributes: Vec<::json::Value>,
        /// Property `InstancePorts`.
        #[serde(rename="InstancePorts")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_ports: Option<Vec<String>>,
        /// Property `LoadBalancerPorts`.
        #[serde(rename="LoadBalancerPorts")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub load_balancer_ports: Option<Vec<String>>,
        /// Property `PolicyName`.
        #[serde(rename="PolicyName")]
        pub policy_name: String,
        /// Property `PolicyType`.
        #[serde(rename="PolicyType")]
        pub policy_type: String,
    }
}
