//! Types for the `ElasticLoadBalancing` service.

/// The [`AWS::ElasticLoadBalancing::LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html) resource type.
#[derive(Debug, Default)]
pub struct LoadBalancer {
    properties: LoadBalancerProperties
}

/// Properties for the `LoadBalancer` resource.
#[derive(Debug, Default)]
pub struct LoadBalancerProperties {
    /// Property [`AccessLoggingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-accessloggingpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_logging_policy: Option<::Value<self::load_balancer::AccessLoggingPolicy>>,
    /// Property [`AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-appcookiestickinesspolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::AppCookieStickinessPolicy>>,
    /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-availabilityzones).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property [`ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-connectiondrainingpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_draining_policy: Option<::Value<self::load_balancer::ConnectionDrainingPolicy>>,
    /// Property [`ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-connectionsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_settings: Option<::Value<self::load_balancer::ConnectionSettings>>,
    /// Property [`CrossZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-crosszone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cross_zone: Option<::Value<bool>>,
    /// Property [`HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-healthcheck).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub health_check: Option<::Value<self::load_balancer::HealthCheck>>,
    /// Property [`Instances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-instances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instances: Option<::ValueList<String>>,
    /// Property [`LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-lbcookiestickinesspolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lb_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::LBCookieStickinessPolicy>>,
    /// Property [`Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-listeners).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub listeners: ::ValueList<self::load_balancer::Listeners>,
    /// Property [`LoadBalancerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-elbname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub load_balancer_name: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::ValueList<self::load_balancer::Policies>>,
    /// Property [`Scheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-scheme).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scheme: Option<::Value<String>>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-securitygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_groups: Option<::ValueList<String>>,
    /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-ec2-elb-subnets).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub subnets: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb.html#cfn-elasticloadbalancing-loadbalancer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LoadBalancerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_logging_policy) = self.access_logging_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLoggingPolicy", access_logging_policy)?;
        }
        if let Some(ref app_cookie_stickiness_policy) = self.app_cookie_stickiness_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppCookieStickinessPolicy", app_cookie_stickiness_policy)?;
        }
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref connection_draining_policy) = self.connection_draining_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionDrainingPolicy", connection_draining_policy)?;
        }
        if let Some(ref connection_settings) = self.connection_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionSettings", connection_settings)?;
        }
        if let Some(ref cross_zone) = self.cross_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossZone", cross_zone)?;
        }
        if let Some(ref health_check) = self.health_check {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheck", health_check)?;
        }
        if let Some(ref instances) = self.instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instances", instances)?;
        }
        if let Some(ref lb_cookie_stickiness_policy) = self.lb_cookie_stickiness_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LBCookieStickinessPolicy", lb_cookie_stickiness_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Listeners", &self.listeners)?;
        if let Some(ref load_balancer_name) = self.load_balancer_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", load_balancer_name)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        if let Some(ref scheme) = self.scheme {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheme", scheme)?;
        }
        if let Some(ref security_groups) = self.security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
        }
        if let Some(ref subnets) = self.subnets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut access_logging_policy: Option<::Value<self::load_balancer::AccessLoggingPolicy>> = None;
                let mut app_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::AppCookieStickinessPolicy>> = None;
                let mut availability_zones: Option<::ValueList<String>> = None;
                let mut connection_draining_policy: Option<::Value<self::load_balancer::ConnectionDrainingPolicy>> = None;
                let mut connection_settings: Option<::Value<self::load_balancer::ConnectionSettings>> = None;
                let mut cross_zone: Option<::Value<bool>> = None;
                let mut health_check: Option<::Value<self::load_balancer::HealthCheck>> = None;
                let mut instances: Option<::ValueList<String>> = None;
                let mut lb_cookie_stickiness_policy: Option<::ValueList<self::load_balancer::LBCookieStickinessPolicy>> = None;
                let mut listeners: Option<::ValueList<self::load_balancer::Listeners>> = None;
                let mut load_balancer_name: Option<::Value<String>> = None;
                let mut policies: Option<::ValueList<self::load_balancer::Policies>> = None;
                let mut scheme: Option<::Value<String>> = None;
                let mut security_groups: Option<::ValueList<String>> = None;
                let mut subnets: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessLoggingPolicy" => {
                            access_logging_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppCookieStickinessPolicy" => {
                            app_cookie_stickiness_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionDrainingPolicy" => {
                            connection_draining_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionSettings" => {
                            connection_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CrossZone" => {
                            cross_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheck" => {
                            health_check = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Instances" => {
                            instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LBCookieStickinessPolicy" => {
                            lb_cookie_stickiness_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Listeners" => {
                            listeners = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancerName" => {
                            load_balancer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scheme" => {
                            scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subnets" => {
                            subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoadBalancerProperties {
                    access_logging_policy: access_logging_policy,
                    app_cookie_stickiness_policy: app_cookie_stickiness_policy,
                    availability_zones: availability_zones,
                    connection_draining_policy: connection_draining_policy,
                    connection_settings: connection_settings,
                    cross_zone: cross_zone,
                    health_check: health_check,
                    instances: instances,
                    lb_cookie_stickiness_policy: lb_cookie_stickiness_policy,
                    listeners: listeners.ok_or(::serde::de::Error::missing_field("Listeners"))?,
                    load_balancer_name: load_balancer_name,
                    policies: policies,
                    scheme: scheme,
                    security_groups: security_groups,
                    subnets: subnets,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LoadBalancer {
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
    #[derive(Debug, Default)]
    pub struct AccessLoggingPolicy {
        /// Property [`EmitInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html#cfn-elb-accessloggingpolicy-emitinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub emit_interval: Option<::Value<u32>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html#cfn-elb-accessloggingpolicy-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html#cfn-elb-accessloggingpolicy-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: ::Value<String>,
        /// Property [`S3BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-accessloggingpolicy.html#cfn-elb-accessloggingpolicy-s3bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessLoggingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref emit_interval) = self.emit_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmitInterval", emit_interval)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
            if let Some(ref s3_bucket_prefix) = self.s3_bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketPrefix", s3_bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessLoggingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessLoggingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessLoggingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessLoggingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut emit_interval: Option<::Value<u32>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_bucket_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmitInterval" => {
                                emit_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketPrefix" => {
                                s3_bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessLoggingPolicy {
                        emit_interval: emit_interval,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                        s3_bucket_prefix: s3_bucket_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.AppCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct AppCookieStickinessPolicy {
        /// Property [`CookieName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html#cfn-elb-appcookiestickinesspolicy-cookiename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_name: ::Value<String>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-AppCookieStickinessPolicy.html#cfn-elb-appcookiestickinesspolicy-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AppCookieStickinessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieName", &self.cookie_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppCookieStickinessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppCookieStickinessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppCookieStickinessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppCookieStickinessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_name: Option<::Value<String>> = None;
                    let mut policy_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieName" => {
                                cookie_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AppCookieStickinessPolicy {
                        cookie_name: cookie_name.ok_or(::serde::de::Error::missing_field("CookieName"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionDrainingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionDrainingPolicy {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html#cfn-elb-connectiondrainingpolicy-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectiondrainingpolicy.html#cfn-elb-connectiondrainingpolicy-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ConnectionDrainingPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionDrainingPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionDrainingPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionDrainingPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionDrainingPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionDrainingPolicy {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.ConnectionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionSettings {
        /// Property [`IdleTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-connectionsettings.html#cfn-elb-connectionsettings-idletimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle_timeout: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ConnectionSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleTimeout", &self.idle_timeout)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle_timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IdleTimeout" => {
                                idle_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionSettings {
                        idle_timeout: idle_timeout.ok_or(::serde::de::Error::missing_field("IdleTimeout"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheck {
        /// Property [`HealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html#cfn-elb-healthcheck-healthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub healthy_threshold: ::Value<String>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html#cfn-elb-healthcheck-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: ::Value<String>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html#cfn-elb-healthcheck-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<String>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html#cfn-elb-healthcheck-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: ::Value<String>,
        /// Property [`UnhealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-health-check.html#cfn-elb-healthcheck-unhealthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unhealthy_threshold: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheck {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", &self.healthy_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", &self.timeout)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", &self.unhealthy_threshold)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheck {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheck, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheck;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheck")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut healthy_threshold: Option<::Value<String>> = None;
                    let mut interval: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;
                    let mut timeout: Option<::Value<String>> = None;
                    let mut unhealthy_threshold: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheck {
                        healthy_threshold: healthy_threshold.ok_or(::serde::de::Error::missing_field("HealthyThreshold"))?,
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                        timeout: timeout.ok_or(::serde::de::Error::missing_field("Timeout"))?,
                        unhealthy_threshold: unhealthy_threshold.ok_or(::serde::de::Error::missing_field("UnhealthyThreshold"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.LBCookieStickinessPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct LBCookieStickinessPolicy {
        /// Property [`CookieExpirationPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html#cfn-elb-lbcookiestickinesspolicy-cookieexpirationperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cookie_expiration_period: Option<::Value<String>>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-LBCookieStickinessPolicy.html#cfn-elb-lbcookiestickinesspolicy-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LBCookieStickinessPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookie_expiration_period) = self.cookie_expiration_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieExpirationPeriod", cookie_expiration_period)?;
            }
            if let Some(ref policy_name) = self.policy_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", policy_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LBCookieStickinessPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LBCookieStickinessPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LBCookieStickinessPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LBCookieStickinessPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookie_expiration_period: Option<::Value<String>> = None;
                    let mut policy_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CookieExpirationPeriod" => {
                                cookie_expiration_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LBCookieStickinessPolicy {
                        cookie_expiration_period: cookie_expiration_period,
                        policy_name: policy_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Listeners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html) property type.
    #[derive(Debug, Default)]
    pub struct Listeners {
        /// Property [`InstancePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-instanceport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_port: ::Value<String>,
        /// Property [`InstanceProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-instanceprotocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_protocol: Option<::Value<String>>,
        /// Property [`LoadBalancerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-loadbalancerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_balancer_port: ::Value<String>,
        /// Property [`PolicyNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-policynames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_names: Option<::ValueList<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`SSLCertificateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-listener.html#cfn-ec2-elb-listener-sslcertificateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_certificate_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Listeners {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancePort", &self.instance_port)?;
            if let Some(ref instance_protocol) = self.instance_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProtocol", instance_protocol)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerPort", &self.load_balancer_port)?;
            if let Some(ref policy_names) = self.policy_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyNames", policy_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            if let Some(ref ssl_certificate_id) = self.ssl_certificate_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSLCertificateId", ssl_certificate_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Listeners {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Listeners, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Listeners;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Listeners")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_port: Option<::Value<String>> = None;
                    let mut instance_protocol: Option<::Value<String>> = None;
                    let mut load_balancer_port: Option<::Value<String>> = None;
                    let mut policy_names: Option<::ValueList<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut ssl_certificate_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstancePort" => {
                                instance_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceProtocol" => {
                                instance_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadBalancerPort" => {
                                load_balancer_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyNames" => {
                                policy_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSLCertificateId" => {
                                ssl_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Listeners {
                        instance_port: instance_port.ok_or(::serde::de::Error::missing_field("InstancePort"))?,
                        instance_protocol: instance_protocol,
                        load_balancer_port: load_balancer_port.ok_or(::serde::de::Error::missing_field("LoadBalancerPort"))?,
                        policy_names: policy_names,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        ssl_certificate_id: ssl_certificate_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ElasticLoadBalancing::LoadBalancer.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html) property type.
    #[derive(Debug, Default)]
    pub struct Policies {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html#cfn-ec2-elb-policy-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: ::ValueList<::json::Value>,
        /// Property [`InstancePorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html#cfn-ec2-elb-policy-instanceports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_ports: Option<::ValueList<String>>,
        /// Property [`LoadBalancerPorts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html#cfn-ec2-elb-policy-loadbalancerports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_balancer_ports: Option<::ValueList<String>>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html#cfn-ec2-elb-policy-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: ::Value<String>,
        /// Property [`PolicyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-elb-policy.html#cfn-ec2-elb-policy-policytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
            if let Some(ref instance_ports) = self.instance_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstancePorts", instance_ports)?;
            }
            if let Some(ref load_balancer_ports) = self.load_balancer_ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerPorts", load_balancer_ports)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", &self.policy_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<::json::Value>> = None;
                    let mut instance_ports: Option<::ValueList<String>> = None;
                    let mut load_balancer_ports: Option<::ValueList<String>> = None;
                    let mut policy_name: Option<::Value<String>> = None;
                    let mut policy_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstancePorts" => {
                                instance_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadBalancerPorts" => {
                                load_balancer_ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyType" => {
                                policy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Policies {
                        attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                        instance_ports: instance_ports,
                        load_balancer_ports: load_balancer_ports,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                        policy_type: policy_type.ok_or(::serde::de::Error::missing_field("PolicyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
