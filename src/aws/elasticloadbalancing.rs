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
    #[serde(rename = "AccessLoggingPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_logging_policy: Option<::Value<self::load_balancer::AccessLoggingPolicy>>,
    /// Property `AppCookieStickinessPolicy`.
    #[serde(rename = "AppCookieStickinessPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::AppCookieStickinessPolicy>>,
    /// Property `AvailabilityZones`.
    #[serde(rename = "AvailabilityZones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `ConnectionDrainingPolicy`.
    #[serde(rename = "ConnectionDrainingPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_draining_policy: Option<::Value<self::load_balancer::ConnectionDrainingPolicy>>,
    /// Property `ConnectionSettings`.
    #[serde(rename = "ConnectionSettings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<::Value<self::load_balancer::ConnectionSettings>>,
    /// Property `CrossZone`.
    #[serde(rename = "CrossZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cross_zone: Option<::Value<bool>>,
    /// Property `HealthCheck`.
    #[serde(rename = "HealthCheck")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check: Option<::Value<self::load_balancer::HealthCheck>>,
    /// Property `Instances`.
    #[serde(rename = "Instances")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<::ValueList<String>>,
    /// Property `LBCookieStickinessPolicy`.
    #[serde(rename = "LBCookieStickinessPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lb_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::LBCookieStickinessPolicy>>,
    /// Property `Listeners`.
    #[serde(rename = "Listeners")]
    pub listeners: ::ValueList<self::load_balancer::Listeners>,
    /// Property `LoadBalancerName`.
    #[serde(rename = "LoadBalancerName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<::Value<String>>,
    /// Property `Policies`.
    #[serde(rename = "Policies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<::ValueList<self::load_balancer::Policies>>,
    /// Property `Scheme`.
    #[serde(rename = "Scheme")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    #[serde(rename = "SecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<::ValueList<String>>,
    /// Property `Subnets`.
    #[serde(rename = "Subnets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<::ValueList<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
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
        #[serde(rename = "EmitInterval")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub emit_interval: Option<::Value<u32>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `S3BucketName`.
        #[serde(rename = "S3BucketName")]
        pub s3_bucket_name: ::Value<String>,
        /// Property `S3BucketPrefix`.
        #[serde(rename = "S3BucketPrefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_bucket_prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(AccessLoggingPolicy);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AppCookieStickinessPolicy {
        /// Property `CookieName`.
        #[serde(rename = "CookieName")]
        pub cookie_name: ::Value<String>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        pub policy_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(AppCookieStickinessPolicy);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionDrainingPolicy {
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `Timeout`.
        #[serde(rename = "Timeout")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timeout: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(ConnectionDrainingPolicy);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionSettings {
        /// Property `IdleTimeout`.
        #[serde(rename = "IdleTimeout")]
        pub idle_timeout: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ConnectionSettings);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HealthCheck {
        /// Property `HealthyThreshold`.
        #[serde(rename = "HealthyThreshold")]
        pub healthy_threshold: ::Value<String>,
        /// Property `Interval`.
        #[serde(rename = "Interval")]
        pub interval: ::Value<String>,
        /// Property `Target`.
        #[serde(rename = "Target")]
        pub target: ::Value<String>,
        /// Property `Timeout`.
        #[serde(rename = "Timeout")]
        pub timeout: ::Value<String>,
        /// Property `UnhealthyThreshold`.
        #[serde(rename = "UnhealthyThreshold")]
        pub unhealthy_threshold: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(HealthCheck);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LBCookieStickinessPolicy {
        /// Property `CookieExpirationPeriod`.
        #[serde(rename = "CookieExpirationPeriod")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cookie_expiration_period: Option<::Value<String>>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LBCookieStickinessPolicy);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Listeners {
        /// Property `InstancePort`.
        #[serde(rename = "InstancePort")]
        pub instance_port: ::Value<String>,
        /// Property `InstanceProtocol`.
        #[serde(rename = "InstanceProtocol")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_protocol: Option<::Value<String>>,
        /// Property `LoadBalancerPort`.
        #[serde(rename = "LoadBalancerPort")]
        pub load_balancer_port: ::Value<String>,
        /// Property `PolicyNames`.
        #[serde(rename = "PolicyNames")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy_names: Option<::ValueList<String>>,
        /// Property `Protocol`.
        #[serde(rename = "Protocol")]
        pub protocol: ::Value<String>,
        /// Property `SSLCertificateId`.
        #[serde(rename = "SSLCertificateId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssl_certificate_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Listeners);

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `Attributes`.
        #[serde(rename = "Attributes")]
        pub attributes: ::ValueList<::json::Value>,
        /// Property `InstancePorts`.
        #[serde(rename = "InstancePorts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_ports: Option<::ValueList<String>>,
        /// Property `LoadBalancerPorts`.
        #[serde(rename = "LoadBalancerPorts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub load_balancer_ports: Option<::ValueList<String>>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        pub policy_name: ::Value<String>,
        /// Property `PolicyType`.
        #[serde(rename = "PolicyType")]
        pub policy_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Policies);
}
