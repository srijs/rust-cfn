//! Types for the `MediaStore` service.

/// The [`AWS::MediaStore::Container`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html) resource type.
#[derive(Debug, Default)]
pub struct Container {
    properties: ContainerProperties
}

/// Properties for the `Container` resource.
#[derive(Debug, Default)]
pub struct ContainerProperties {
    /// Property [`AccessLoggingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-accessloggingenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_logging_enabled: Option<::Value<bool>>,
    /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-containername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub container_name: ::Value<String>,
    /// Property [`CorsPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-corspolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cors_policy: Option<::ValueList<self::container::CorsRule>>,
    /// Property [`LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-lifecyclepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_policy: Option<::Value<String>>,
    /// Property [`MetricPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-metricpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_policy: Option<::Value<self::container::MetricPolicy>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediastore-container.html#cfn-mediastore-container-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ContainerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_logging_enabled) = self.access_logging_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessLoggingEnabled", access_logging_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", &self.container_name)?;
        if let Some(ref cors_policy) = self.cors_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorsPolicy", cors_policy)?;
        }
        if let Some(ref lifecycle_policy) = self.lifecycle_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecyclePolicy", lifecycle_policy)?;
        }
        if let Some(ref metric_policy) = self.metric_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricPolicy", metric_policy)?;
        }
        if let Some(ref policy) = self.policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", policy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContainerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContainerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContainerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_logging_enabled: Option<::Value<bool>> = None;
                let mut container_name: Option<::Value<String>> = None;
                let mut cors_policy: Option<::ValueList<self::container::CorsRule>> = None;
                let mut lifecycle_policy: Option<::Value<String>> = None;
                let mut metric_policy: Option<::Value<self::container::MetricPolicy>> = None;
                let mut policy: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessLoggingEnabled" => {
                            access_logging_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContainerName" => {
                            container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CorsPolicy" => {
                            cors_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecyclePolicy" => {
                            lifecycle_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricPolicy" => {
                            metric_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContainerProperties {
                    access_logging_enabled: access_logging_enabled,
                    container_name: container_name.ok_or(::serde::de::Error::missing_field("ContainerName"))?,
                    cors_policy: cors_policy,
                    lifecycle_policy: lifecycle_policy,
                    metric_policy: metric_policy,
                    policy: policy,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Container {
    type Properties = ContainerProperties;
    const TYPE: &'static str = "AWS::MediaStore::Container";
    fn properties(&self) -> &ContainerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContainerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Container {}

impl From<ContainerProperties> for Container {
    fn from(properties: ContainerProperties) -> Container {
        Container { properties }
    }
}

pub mod container {
    //! Property types for the `Container` resource.

    /// The [`AWS::MediaStore::Container.CorsRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html) property type.
    #[derive(Debug, Default)]
    pub struct CorsRule {
        /// Property [`AllowedHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html#cfn-mediastore-container-corsrule-allowedheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_headers: Option<::ValueList<String>>,
        /// Property [`AllowedMethods`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html#cfn-mediastore-container-corsrule-allowedmethods).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property [`AllowedOrigins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html#cfn-mediastore-container-corsrule-allowedorigins).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_origins: Option<::ValueList<String>>,
        /// Property [`ExposeHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html#cfn-mediastore-container-corsrule-exposeheaders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expose_headers: Option<::ValueList<String>>,
        /// Property [`MaxAgeSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-corsrule.html#cfn-mediastore-container-corsrule-maxageseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_age_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CorsRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_headers) = self.allowed_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedHeaders", allowed_headers)?;
            }
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref allowed_origins) = self.allowed_origins {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOrigins", allowed_origins)?;
            }
            if let Some(ref expose_headers) = self.expose_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExposeHeaders", expose_headers)?;
            }
            if let Some(ref max_age_seconds) = self.max_age_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAgeSeconds", max_age_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CorsRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CorsRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CorsRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CorsRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_headers: Option<::ValueList<String>> = None;
                    let mut allowed_methods: Option<::ValueList<String>> = None;
                    let mut allowed_origins: Option<::ValueList<String>> = None;
                    let mut expose_headers: Option<::ValueList<String>> = None;
                    let mut max_age_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedHeaders" => {
                                allowed_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedOrigins" => {
                                allowed_origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExposeHeaders" => {
                                expose_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxAgeSeconds" => {
                                max_age_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CorsRule {
                        allowed_headers: allowed_headers,
                        allowed_methods: allowed_methods,
                        allowed_origins: allowed_origins,
                        expose_headers: expose_headers,
                        max_age_seconds: max_age_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaStore::Container.MetricPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricPolicy {
        /// Property [`ContainerLevelMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicy.html#cfn-mediastore-container-metricpolicy-containerlevelmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_level_metrics: ::Value<String>,
        /// Property [`MetricPolicyRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicy.html#cfn-mediastore-container-metricpolicy-metricpolicyrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_policy_rules: Option<::ValueList<MetricPolicyRule>>,
    }

    impl ::codec::SerializeValue for MetricPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerLevelMetrics", &self.container_level_metrics)?;
            if let Some(ref metric_policy_rules) = self.metric_policy_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricPolicyRules", metric_policy_rules)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_level_metrics: Option<::Value<String>> = None;
                    let mut metric_policy_rules: Option<::ValueList<MetricPolicyRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerLevelMetrics" => {
                                container_level_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricPolicyRules" => {
                                metric_policy_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricPolicy {
                        container_level_metrics: container_level_metrics.ok_or(::serde::de::Error::missing_field("ContainerLevelMetrics"))?,
                        metric_policy_rules: metric_policy_rules,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaStore::Container.MetricPolicyRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicyrule.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricPolicyRule {
        /// Property [`ObjectGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicyrule.html#cfn-mediastore-container-metricpolicyrule-objectgroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_group: ::Value<String>,
        /// Property [`ObjectGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediastore-container-metricpolicyrule.html#cfn-mediastore-container-metricpolicyrule-objectgroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_group_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricPolicyRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectGroup", &self.object_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectGroupName", &self.object_group_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricPolicyRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricPolicyRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricPolicyRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricPolicyRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object_group: Option<::Value<String>> = None;
                    let mut object_group_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObjectGroup" => {
                                object_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectGroupName" => {
                                object_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricPolicyRule {
                        object_group: object_group.ok_or(::serde::de::Error::missing_field("ObjectGroup"))?,
                        object_group_name: object_group_name.ok_or(::serde::de::Error::missing_field("ObjectGroupName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
