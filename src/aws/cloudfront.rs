//! Types for the `CloudFront` service.

/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource type.
#[derive(Debug)]
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Debug)]
pub struct CloudFrontOriginAccessIdentityProperties {
    /// Property `CloudFrontOriginAccessIdentityConfig`.
    pub cloud_front_origin_access_identity_config: ::Value<self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig>,
}

impl ::serde::Serialize for CloudFrontOriginAccessIdentityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFrontOriginAccessIdentityConfig", &self.cloud_front_origin_access_identity_config)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CloudFrontOriginAccessIdentityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFrontOriginAccessIdentityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CloudFrontOriginAccessIdentityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CloudFrontOriginAccessIdentityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cloud_front_origin_access_identity_config: Option<::Value<self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CloudFrontOriginAccessIdentityConfig" => {
                            cloud_front_origin_access_identity_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CloudFrontOriginAccessIdentityProperties {
                    cloud_front_origin_access_identity_config: cloud_front_origin_access_identity_config.ok_or(::serde::de::Error::missing_field("CloudFrontOriginAccessIdentityConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CloudFrontOriginAccessIdentity {
    type Properties = CloudFrontOriginAccessIdentityProperties;
    const TYPE: &'static str = "AWS::CloudFront::CloudFrontOriginAccessIdentity";
    fn properties(&self) -> &CloudFrontOriginAccessIdentityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CloudFrontOriginAccessIdentityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CloudFrontOriginAccessIdentity {}

impl From<CloudFrontOriginAccessIdentityProperties> for CloudFrontOriginAccessIdentity {
    fn from(properties: CloudFrontOriginAccessIdentityProperties) -> CloudFrontOriginAccessIdentity {
        CloudFrontOriginAccessIdentity { properties }
    }
}

/// The [`AWS::CloudFront::Distribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html) resource type.
#[derive(Debug)]
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Debug)]
pub struct DistributionProperties {
    /// Property `DistributionConfig`.
    pub distribution_config: ::Value<self::distribution::DistributionConfig>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DistributionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DistributionConfig", &self.distribution_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DistributionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DistributionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DistributionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut distribution_config: Option<::Value<self::distribution::DistributionConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DistributionConfig" => {
                            distribution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DistributionProperties {
                    distribution_config: distribution_config.ok_or(::serde::de::Error::missing_field("DistributionConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Distribution {
    type Properties = DistributionProperties;
    const TYPE: &'static str = "AWS::CloudFront::Distribution";
    fn properties(&self) -> &DistributionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DistributionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Distribution {}

impl From<DistributionProperties> for Distribution {
    fn from(properties: DistributionProperties) -> Distribution {
        Distribution { properties }
    }
}

/// The [`AWS::CloudFront::StreamingDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html) resource type.
#[derive(Debug)]
pub struct StreamingDistribution {
    properties: StreamingDistributionProperties
}

/// Properties for the `StreamingDistribution` resource.
#[derive(Debug)]
pub struct StreamingDistributionProperties {
    /// Property `StreamingDistributionConfig`.
    pub streaming_distribution_config: ::Value<self::streaming_distribution::StreamingDistributionConfig>,
    /// Property `Tags`.
    pub tags: ::ValueList<::Tag>,
}

impl ::serde::Serialize for StreamingDistributionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingDistributionConfig", &self.streaming_distribution_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StreamingDistributionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingDistributionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StreamingDistributionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StreamingDistributionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut streaming_distribution_config: Option<::Value<self::streaming_distribution::StreamingDistributionConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "StreamingDistributionConfig" => {
                            streaming_distribution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StreamingDistributionProperties {
                    streaming_distribution_config: streaming_distribution_config.ok_or(::serde::de::Error::missing_field("StreamingDistributionConfig"))?,
                    tags: tags.ok_or(::serde::de::Error::missing_field("Tags"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StreamingDistribution {
    type Properties = StreamingDistributionProperties;
    const TYPE: &'static str = "AWS::CloudFront::StreamingDistribution";
    fn properties(&self) -> &StreamingDistributionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StreamingDistributionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StreamingDistribution {}

impl From<StreamingDistributionProperties> for StreamingDistribution {
    fn from(properties: StreamingDistributionProperties) -> StreamingDistribution {
        StreamingDistribution { properties }
    }
}

pub mod cloud_front_origin_access_identity {
    //! Property types for the `CloudFrontOriginAccessIdentity` resource.

    /// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity.CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html) property type.
    #[derive(Debug)]
    pub struct CloudFrontOriginAccessIdentityConfig {
        /// Property `Comment`.
        pub comment: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudFrontOriginAccessIdentityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudFrontOriginAccessIdentityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFrontOriginAccessIdentityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudFrontOriginAccessIdentityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudFrontOriginAccessIdentityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudFrontOriginAccessIdentityConfig {
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod distribution {
    //! Property types for the `Distribution` resource.

    /// The [`AWS::CloudFront::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html) property type.
    #[derive(Debug)]
    pub struct CacheBehavior {
        /// Property `AllowedMethods`.
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property `CachedMethods`.
        pub cached_methods: Option<::ValueList<String>>,
        /// Property `Compress`.
        pub compress: Option<::Value<bool>>,
        /// Property `DefaultTTL`.
        pub default_ttl: Option<::Value<f64>>,
        /// Property `ForwardedValues`.
        pub forwarded_values: ::Value<ForwardedValues>,
        /// Property `LambdaFunctionAssociations`.
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        pub max_ttl: Option<::Value<f64>>,
        /// Property `MinTTL`.
        pub min_ttl: Option<::Value<f64>>,
        /// Property `PathPattern`.
        pub path_pattern: ::Value<String>,
        /// Property `SmoothStreaming`.
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property `TargetOriginId`.
        pub target_origin_id: ::Value<String>,
        /// Property `TrustedSigners`.
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property `ViewerProtocolPolicy`.
        pub viewer_protocol_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for CacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cached_methods) = self.cached_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachedMethods", cached_methods)?;
            }
            if let Some(ref compress) = self.compress {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compress", compress)?;
            }
            if let Some(ref default_ttl) = self.default_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", default_ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", &self.forwarded_values)?;
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPattern", &self.path_pattern)?;
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_signers) = self.trusted_signers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", trusted_signers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerProtocolPolicy", &self.viewer_protocol_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CacheBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CacheBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CacheBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CacheBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_methods: Option<::ValueList<String>> = None;
                    let mut cached_methods: Option<::ValueList<String>> = None;
                    let mut compress: Option<::Value<bool>> = None;
                    let mut default_ttl: Option<::Value<f64>> = None;
                    let mut forwarded_values: Option<::Value<ForwardedValues>> = None;
                    let mut lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<::Value<f64>> = None;
                    let mut min_ttl: Option<::Value<f64>> = None;
                    let mut path_pattern: Option<::Value<String>> = None;
                    let mut smooth_streaming: Option<::Value<bool>> = None;
                    let mut target_origin_id: Option<::Value<String>> = None;
                    let mut trusted_signers: Option<::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachedMethods" => {
                                cached_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compress" => {
                                compress = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionAssociations" => {
                                lambda_function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathPattern" => {
                                path_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerProtocolPolicy" => {
                                viewer_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CacheBehavior {
                        allowed_methods: allowed_methods,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        forwarded_values: forwarded_values.ok_or(::serde::de::Error::missing_field("ForwardedValues"))?,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        path_pattern: path_pattern.ok_or(::serde::de::Error::missing_field("PathPattern"))?,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Debug)]
    pub struct Cookies {
        /// Property `Forward`.
        pub forward: ::Value<String>,
        /// Property `WhitelistedNames`.
        pub whitelisted_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Cookies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Forward", &self.forward)?;
            if let Some(ref whitelisted_names) = self.whitelisted_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WhitelistedNames", whitelisted_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Cookies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Cookies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Cookies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Cookies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut forward: Option<::Value<String>> = None;
                    let mut whitelisted_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Forward" => {
                                forward = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WhitelistedNames" => {
                                whitelisted_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Cookies {
                        forward: forward.ok_or(::serde::de::Error::missing_field("Forward"))?,
                        whitelisted_names: whitelisted_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.CustomErrorResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html) property type.
    #[derive(Debug)]
    pub struct CustomErrorResponse {
        /// Property `ErrorCachingMinTTL`.
        pub error_caching_min_ttl: Option<::Value<f64>>,
        /// Property `ErrorCode`.
        pub error_code: ::Value<u32>,
        /// Property `ResponseCode`.
        pub response_code: Option<::Value<u32>>,
        /// Property `ResponsePagePath`.
        pub response_page_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomErrorResponse {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_caching_min_ttl) = self.error_caching_min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorCachingMinTTL", error_caching_min_ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorCode", &self.error_code)?;
            if let Some(ref response_code) = self.response_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseCode", response_code)?;
            }
            if let Some(ref response_page_path) = self.response_page_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponsePagePath", response_page_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomErrorResponse {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomErrorResponse, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomErrorResponse;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomErrorResponse")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_caching_min_ttl: Option<::Value<f64>> = None;
                    let mut error_code: Option<::Value<u32>> = None;
                    let mut response_code: Option<::Value<u32>> = None;
                    let mut response_page_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorCachingMinTTL" => {
                                error_caching_min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorCode" => {
                                error_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseCode" => {
                                response_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponsePagePath" => {
                                response_page_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomErrorResponse {
                        error_caching_min_ttl: error_caching_min_ttl,
                        error_code: error_code.ok_or(::serde::de::Error::missing_field("ErrorCode"))?,
                        response_code: response_code,
                        response_page_path: response_page_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html) property type.
    #[derive(Debug)]
    pub struct CustomOriginConfig {
        /// Property `HTTPPort`.
        pub http_port: Option<::Value<u32>>,
        /// Property `HTTPSPort`.
        pub https_port: Option<::Value<u32>>,
        /// Property `OriginKeepaliveTimeout`.
        pub origin_keepalive_timeout: Option<::Value<u32>>,
        /// Property `OriginProtocolPolicy`.
        pub origin_protocol_policy: ::Value<String>,
        /// Property `OriginReadTimeout`.
        pub origin_read_timeout: Option<::Value<u32>>,
        /// Property `OriginSSLProtocols`.
        pub origin_ssl_protocols: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CustomOriginConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref http_port) = self.http_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPPort", http_port)?;
            }
            if let Some(ref https_port) = self.https_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPSPort", https_port)?;
            }
            if let Some(ref origin_keepalive_timeout) = self.origin_keepalive_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginKeepaliveTimeout", origin_keepalive_timeout)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginProtocolPolicy", &self.origin_protocol_policy)?;
            if let Some(ref origin_read_timeout) = self.origin_read_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginReadTimeout", origin_read_timeout)?;
            }
            if let Some(ref origin_ssl_protocols) = self.origin_ssl_protocols {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginSSLProtocols", origin_ssl_protocols)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomOriginConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomOriginConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomOriginConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomOriginConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut http_port: Option<::Value<u32>> = None;
                    let mut https_port: Option<::Value<u32>> = None;
                    let mut origin_keepalive_timeout: Option<::Value<u32>> = None;
                    let mut origin_protocol_policy: Option<::Value<String>> = None;
                    let mut origin_read_timeout: Option<::Value<u32>> = None;
                    let mut origin_ssl_protocols: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HTTPPort" => {
                                http_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPSPort" => {
                                https_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginKeepaliveTimeout" => {
                                origin_keepalive_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginProtocolPolicy" => {
                                origin_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginReadTimeout" => {
                                origin_read_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginSSLProtocols" => {
                                origin_ssl_protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomOriginConfig {
                        http_port: http_port,
                        https_port: https_port,
                        origin_keepalive_timeout: origin_keepalive_timeout,
                        origin_protocol_policy: origin_protocol_policy.ok_or(::serde::de::Error::missing_field("OriginProtocolPolicy"))?,
                        origin_read_timeout: origin_read_timeout,
                        origin_ssl_protocols: origin_ssl_protocols,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html) property type.
    #[derive(Debug)]
    pub struct DefaultCacheBehavior {
        /// Property `AllowedMethods`.
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property `CachedMethods`.
        pub cached_methods: Option<::ValueList<String>>,
        /// Property `Compress`.
        pub compress: Option<::Value<bool>>,
        /// Property `DefaultTTL`.
        pub default_ttl: Option<::Value<f64>>,
        /// Property `ForwardedValues`.
        pub forwarded_values: ::Value<ForwardedValues>,
        /// Property `LambdaFunctionAssociations`.
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        pub max_ttl: Option<::Value<f64>>,
        /// Property `MinTTL`.
        pub min_ttl: Option<::Value<f64>>,
        /// Property `SmoothStreaming`.
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property `TargetOriginId`.
        pub target_origin_id: ::Value<String>,
        /// Property `TrustedSigners`.
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property `ViewerProtocolPolicy`.
        pub viewer_protocol_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for DefaultCacheBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_methods) = self.allowed_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedMethods", allowed_methods)?;
            }
            if let Some(ref cached_methods) = self.cached_methods {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachedMethods", cached_methods)?;
            }
            if let Some(ref compress) = self.compress {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compress", compress)?;
            }
            if let Some(ref default_ttl) = self.default_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTTL", default_ttl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForwardedValues", &self.forwarded_values)?;
            if let Some(ref lambda_function_associations) = self.lambda_function_associations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionAssociations", lambda_function_associations)?;
            }
            if let Some(ref max_ttl) = self.max_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTTL", max_ttl)?;
            }
            if let Some(ref min_ttl) = self.min_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinTTL", min_ttl)?;
            }
            if let Some(ref smooth_streaming) = self.smooth_streaming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothStreaming", smooth_streaming)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetOriginId", &self.target_origin_id)?;
            if let Some(ref trusted_signers) = self.trusted_signers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", trusted_signers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerProtocolPolicy", &self.viewer_protocol_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultCacheBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultCacheBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultCacheBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultCacheBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_methods: Option<::ValueList<String>> = None;
                    let mut cached_methods: Option<::ValueList<String>> = None;
                    let mut compress: Option<::Value<bool>> = None;
                    let mut default_ttl: Option<::Value<f64>> = None;
                    let mut forwarded_values: Option<::Value<ForwardedValues>> = None;
                    let mut lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>> = None;
                    let mut max_ttl: Option<::Value<f64>> = None;
                    let mut min_ttl: Option<::Value<f64>> = None;
                    let mut smooth_streaming: Option<::Value<bool>> = None;
                    let mut target_origin_id: Option<::Value<String>> = None;
                    let mut trusted_signers: Option<::ValueList<String>> = None;
                    let mut viewer_protocol_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedMethods" => {
                                allowed_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CachedMethods" => {
                                cached_methods = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compress" => {
                                compress = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultTTL" => {
                                default_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForwardedValues" => {
                                forwarded_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionAssociations" => {
                                lambda_function_associations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTTL" => {
                                max_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinTTL" => {
                                min_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmoothStreaming" => {
                                smooth_streaming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetOriginId" => {
                                target_origin_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerProtocolPolicy" => {
                                viewer_protocol_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultCacheBehavior {
                        allowed_methods: allowed_methods,
                        cached_methods: cached_methods,
                        compress: compress,
                        default_ttl: default_ttl,
                        forwarded_values: forwarded_values.ok_or(::serde::de::Error::missing_field("ForwardedValues"))?,
                        lambda_function_associations: lambda_function_associations,
                        max_ttl: max_ttl,
                        min_ttl: min_ttl,
                        smooth_streaming: smooth_streaming,
                        target_origin_id: target_origin_id.ok_or(::serde::de::Error::missing_field("TargetOriginId"))?,
                        trusted_signers: trusted_signers,
                        viewer_protocol_policy: viewer_protocol_policy.ok_or(::serde::de::Error::missing_field("ViewerProtocolPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Debug)]
    pub struct DistributionConfig {
        /// Property `Aliases`.
        pub aliases: Option<::ValueList<String>>,
        /// Property `CacheBehaviors`.
        pub cache_behaviors: Option<::ValueList<CacheBehavior>>,
        /// Property `Comment`.
        pub comment: Option<::Value<String>>,
        /// Property `CustomErrorResponses`.
        pub custom_error_responses: Option<::ValueList<CustomErrorResponse>>,
        /// Property `DefaultCacheBehavior`.
        pub default_cache_behavior: Option<::Value<DefaultCacheBehavior>>,
        /// Property `DefaultRootObject`.
        pub default_root_object: Option<::Value<String>>,
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `HttpVersion`.
        pub http_version: Option<::Value<String>>,
        /// Property `IPV6Enabled`.
        pub ipv6_enabled: Option<::Value<bool>>,
        /// Property `Logging`.
        pub logging: Option<::Value<Logging>>,
        /// Property `Origins`.
        pub origins: Option<::ValueList<Origin>>,
        /// Property `PriceClass`.
        pub price_class: Option<::Value<String>>,
        /// Property `Restrictions`.
        pub restrictions: Option<::Value<Restrictions>>,
        /// Property `ViewerCertificate`.
        pub viewer_certificate: Option<::Value<ViewerCertificate>>,
        /// Property `WebACLId`.
        pub web_acl_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DistributionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            if let Some(ref cache_behaviors) = self.cache_behaviors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CacheBehaviors", cache_behaviors)?;
            }
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            if let Some(ref custom_error_responses) = self.custom_error_responses {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomErrorResponses", custom_error_responses)?;
            }
            if let Some(ref default_cache_behavior) = self.default_cache_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCacheBehavior", default_cache_behavior)?;
            }
            if let Some(ref default_root_object) = self.default_root_object {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRootObject", default_root_object)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref http_version) = self.http_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpVersion", http_version)?;
            }
            if let Some(ref ipv6_enabled) = self.ipv6_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IPV6Enabled", ipv6_enabled)?;
            }
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            if let Some(ref origins) = self.origins {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Origins", origins)?;
            }
            if let Some(ref price_class) = self.price_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriceClass", price_class)?;
            }
            if let Some(ref restrictions) = self.restrictions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Restrictions", restrictions)?;
            }
            if let Some(ref viewer_certificate) = self.viewer_certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewerCertificate", viewer_certificate)?;
            }
            if let Some(ref web_acl_id) = self.web_acl_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebACLId", web_acl_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DistributionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DistributionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DistributionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DistributionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aliases: Option<::ValueList<String>> = None;
                    let mut cache_behaviors: Option<::ValueList<CacheBehavior>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut custom_error_responses: Option<::ValueList<CustomErrorResponse>> = None;
                    let mut default_cache_behavior: Option<::Value<DefaultCacheBehavior>> = None;
                    let mut default_root_object: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut http_version: Option<::Value<String>> = None;
                    let mut ipv6_enabled: Option<::Value<bool>> = None;
                    let mut logging: Option<::Value<Logging>> = None;
                    let mut origins: Option<::ValueList<Origin>> = None;
                    let mut price_class: Option<::Value<String>> = None;
                    let mut restrictions: Option<::Value<Restrictions>> = None;
                    let mut viewer_certificate: Option<::Value<ViewerCertificate>> = None;
                    let mut web_acl_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CacheBehaviors" => {
                                cache_behaviors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomErrorResponses" => {
                                custom_error_responses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultCacheBehavior" => {
                                default_cache_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultRootObject" => {
                                default_root_object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HttpVersion" => {
                                http_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IPV6Enabled" => {
                                ipv6_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Origins" => {
                                origins = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PriceClass" => {
                                price_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Restrictions" => {
                                restrictions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewerCertificate" => {
                                viewer_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebACLId" => {
                                web_acl_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DistributionConfig {
                        aliases: aliases,
                        cache_behaviors: cache_behaviors,
                        comment: comment,
                        custom_error_responses: custom_error_responses,
                        default_cache_behavior: default_cache_behavior,
                        default_root_object: default_root_object,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        http_version: http_version,
                        ipv6_enabled: ipv6_enabled,
                        logging: logging,
                        origins: origins,
                        price_class: price_class,
                        restrictions: restrictions,
                        viewer_certificate: viewer_certificate,
                        web_acl_id: web_acl_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Debug)]
    pub struct ForwardedValues {
        /// Property `Cookies`.
        pub cookies: Option<::Value<Cookies>>,
        /// Property `Headers`.
        pub headers: Option<::ValueList<String>>,
        /// Property `QueryString`.
        pub query_string: ::Value<bool>,
        /// Property `QueryStringCacheKeys`.
        pub query_string_cache_keys: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ForwardedValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cookies) = self.cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cookies", cookies)?;
            }
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", &self.query_string)?;
            if let Some(ref query_string_cache_keys) = self.query_string_cache_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStringCacheKeys", query_string_cache_keys)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ForwardedValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ForwardedValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ForwardedValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ForwardedValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cookies: Option<::Value<Cookies>> = None;
                    let mut headers: Option<::ValueList<String>> = None;
                    let mut query_string: Option<::Value<bool>> = None;
                    let mut query_string_cache_keys: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cookies" => {
                                cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryString" => {
                                query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryStringCacheKeys" => {
                                query_string_cache_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ForwardedValues {
                        cookies: cookies,
                        headers: headers,
                        query_string: query_string.ok_or(::serde::de::Error::missing_field("QueryString"))?,
                        query_string_cache_keys: query_string_cache_keys,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Debug)]
    pub struct GeoRestriction {
        /// Property `Locations`.
        pub locations: Option<::ValueList<String>>,
        /// Property `RestrictionType`.
        pub restriction_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for GeoRestriction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref locations) = self.locations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Locations", locations)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestrictionType", &self.restriction_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoRestriction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoRestriction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoRestriction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoRestriction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut locations: Option<::ValueList<String>> = None;
                    let mut restriction_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Locations" => {
                                locations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RestrictionType" => {
                                restriction_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoRestriction {
                        locations: locations,
                        restriction_type: restriction_type.ok_or(::serde::de::Error::missing_field("RestrictionType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.LambdaFunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html) property type.
    #[derive(Debug)]
    pub struct LambdaFunctionAssociation {
        /// Property `EventType`.
        pub event_type: Option<::Value<String>>,
        /// Property `LambdaFunctionARN`.
        pub lambda_function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaFunctionAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref event_type) = self.event_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
            }
            if let Some(ref lambda_function_arn) = self.lambda_function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionARN", lambda_function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaFunctionAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaFunctionAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaFunctionAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaFunctionAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_type: Option<::Value<String>> = None;
                    let mut lambda_function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaFunctionARN" => {
                                lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaFunctionAssociation {
                        event_type: event_type,
                        lambda_function_arn: lambda_function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Debug)]
    pub struct Logging {
        /// Property `Bucket`.
        pub bucket: ::Value<String>,
        /// Property `IncludeCookies`.
        pub include_cookies: Option<::Value<bool>>,
        /// Property `Prefix`.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref include_cookies) = self.include_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeCookies", include_cookies)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut include_cookies: Option<::Value<bool>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeCookies" => {
                                include_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        include_cookies: include_cookies,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html) property type.
    #[derive(Debug)]
    pub struct Origin {
        /// Property `CustomOriginConfig`.
        pub custom_origin_config: Option<::Value<CustomOriginConfig>>,
        /// Property `DomainName`.
        pub domain_name: ::Value<String>,
        /// Property `Id`.
        pub id: ::Value<String>,
        /// Property `OriginCustomHeaders`.
        pub origin_custom_headers: Option<::ValueList<OriginCustomHeader>>,
        /// Property `OriginPath`.
        pub origin_path: Option<::Value<String>>,
        /// Property `S3OriginConfig`.
        pub s3_origin_config: Option<::Value<S3OriginConfig>>,
    }

    impl ::codec::SerializeValue for Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_origin_config) = self.custom_origin_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomOriginConfig", custom_origin_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref origin_custom_headers) = self.origin_custom_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginCustomHeaders", origin_custom_headers)?;
            }
            if let Some(ref origin_path) = self.origin_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginPath", origin_path)?;
            }
            if let Some(ref s3_origin_config) = self.s3_origin_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OriginConfig", s3_origin_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_origin_config: Option<::Value<CustomOriginConfig>> = None;
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut origin_custom_headers: Option<::ValueList<OriginCustomHeader>> = None;
                    let mut origin_path: Option<::Value<String>> = None;
                    let mut s3_origin_config: Option<::Value<S3OriginConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomOriginConfig" => {
                                custom_origin_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginCustomHeaders" => {
                                origin_custom_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginPath" => {
                                origin_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OriginConfig" => {
                                s3_origin_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Origin {
                        custom_origin_config: custom_origin_config,
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        origin_custom_headers: origin_custom_headers,
                        origin_path: origin_path,
                        s3_origin_config: s3_origin_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Debug)]
    pub struct OriginCustomHeader {
        /// Property `HeaderName`.
        pub header_name: ::Value<String>,
        /// Property `HeaderValue`.
        pub header_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for OriginCustomHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderValue", &self.header_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginCustomHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginCustomHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginCustomHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginCustomHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_name: Option<::Value<String>> = None;
                    let mut header_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderValue" => {
                                header_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginCustomHeader {
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        header_value: header_value.ok_or(::serde::de::Error::missing_field("HeaderValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Debug)]
    pub struct Restrictions {
        /// Property `GeoRestriction`.
        pub geo_restriction: ::Value<GeoRestriction>,
    }

    impl ::codec::SerializeValue for Restrictions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoRestriction", &self.geo_restriction)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Restrictions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Restrictions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Restrictions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Restrictions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut geo_restriction: Option<::Value<GeoRestriction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GeoRestriction" => {
                                geo_restriction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Restrictions {
                        geo_restriction: geo_restriction.ok_or(::serde::de::Error::missing_field("GeoRestriction"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html) property type.
    #[derive(Debug)]
    pub struct S3OriginConfig {
        /// Property `OriginAccessIdentity`.
        pub origin_access_identity: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3OriginConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref origin_access_identity) = self.origin_access_identity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", origin_access_identity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3OriginConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3OriginConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3OriginConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3OriginConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut origin_access_identity: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3OriginConfig {
                        origin_access_identity: origin_access_identity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Debug)]
    pub struct ViewerCertificate {
        /// Property `AcmCertificateArn`.
        pub acm_certificate_arn: Option<::Value<String>>,
        /// Property `CloudFrontDefaultCertificate`.
        pub cloud_front_default_certificate: Option<::Value<bool>>,
        /// Property `IamCertificateId`.
        pub iam_certificate_id: Option<::Value<String>>,
        /// Property `MinimumProtocolVersion`.
        pub minimum_protocol_version: Option<::Value<String>>,
        /// Property `SslSupportMethod`.
        pub ssl_support_method: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ViewerCertificate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acm_certificate_arn) = self.acm_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcmCertificateArn", acm_certificate_arn)?;
            }
            if let Some(ref cloud_front_default_certificate) = self.cloud_front_default_certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFrontDefaultCertificate", cloud_front_default_certificate)?;
            }
            if let Some(ref iam_certificate_id) = self.iam_certificate_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamCertificateId", iam_certificate_id)?;
            }
            if let Some(ref minimum_protocol_version) = self.minimum_protocol_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumProtocolVersion", minimum_protocol_version)?;
            }
            if let Some(ref ssl_support_method) = self.ssl_support_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslSupportMethod", ssl_support_method)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ViewerCertificate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ViewerCertificate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ViewerCertificate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ViewerCertificate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acm_certificate_arn: Option<::Value<String>> = None;
                    let mut cloud_front_default_certificate: Option<::Value<bool>> = None;
                    let mut iam_certificate_id: Option<::Value<String>> = None;
                    let mut minimum_protocol_version: Option<::Value<String>> = None;
                    let mut ssl_support_method: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcmCertificateArn" => {
                                acm_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudFrontDefaultCertificate" => {
                                cloud_front_default_certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamCertificateId" => {
                                iam_certificate_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumProtocolVersion" => {
                                minimum_protocol_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslSupportMethod" => {
                                ssl_support_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ViewerCertificate {
                        acm_certificate_arn: acm_certificate_arn,
                        cloud_front_default_certificate: cloud_front_default_certificate,
                        iam_certificate_id: iam_certificate_id,
                        minimum_protocol_version: minimum_protocol_version,
                        ssl_support_method: ssl_support_method,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod streaming_distribution {
    //! Property types for the `StreamingDistribution` resource.

    /// The [`AWS::CloudFront::StreamingDistribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html) property type.
    #[derive(Debug)]
    pub struct Logging {
        /// Property `Bucket`.
        pub bucket: ::Value<String>,
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `Prefix`.
        pub prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for Logging {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Logging {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Logging, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Logging;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Logging")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Logging {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html) property type.
    #[derive(Debug)]
    pub struct S3Origin {
        /// Property `DomainName`.
        pub domain_name: ::Value<String>,
        /// Property `OriginAccessIdentity`.
        pub origin_access_identity: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Origin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginAccessIdentity", &self.origin_access_identity)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Origin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Origin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Origin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Origin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut origin_access_identity: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginAccessIdentity" => {
                                origin_access_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Origin {
                        domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                        origin_access_identity: origin_access_identity.ok_or(::serde::de::Error::missing_field("OriginAccessIdentity"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html) property type.
    #[derive(Debug)]
    pub struct StreamingDistributionConfig {
        /// Property `Aliases`.
        pub aliases: Option<::ValueList<String>>,
        /// Property `Comment`.
        pub comment: ::Value<String>,
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
        /// Property `Logging`.
        pub logging: Option<::Value<Logging>>,
        /// Property `PriceClass`.
        pub price_class: Option<::Value<String>>,
        /// Property `S3Origin`.
        pub s3_origin: ::Value<S3Origin>,
        /// Property `TrustedSigners`.
        pub trusted_signers: ::Value<TrustedSigners>,
    }

    impl ::codec::SerializeValue for StreamingDistributionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            if let Some(ref price_class) = self.price_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PriceClass", price_class)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Origin", &self.s3_origin)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedSigners", &self.trusted_signers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamingDistributionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamingDistributionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamingDistributionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamingDistributionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aliases: Option<::ValueList<String>> = None;
                    let mut comment: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut logging: Option<::Value<Logging>> = None;
                    let mut price_class: Option<::Value<String>> = None;
                    let mut s3_origin: Option<::Value<S3Origin>> = None;
                    let mut trusted_signers: Option<::Value<TrustedSigners>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PriceClass" => {
                                price_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Origin" => {
                                s3_origin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrustedSigners" => {
                                trusted_signers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamingDistributionConfig {
                        aliases: aliases,
                        comment: comment.ok_or(::serde::de::Error::missing_field("Comment"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        logging: logging,
                        price_class: price_class,
                        s3_origin: s3_origin.ok_or(::serde::de::Error::missing_field("S3Origin"))?,
                        trusted_signers: trusted_signers.ok_or(::serde::de::Error::missing_field("TrustedSigners"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CloudFront::StreamingDistribution.TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html) property type.
    #[derive(Debug)]
    pub struct TrustedSigners {
        /// Property `AwsAccountNumbers`.
        pub aws_account_numbers: Option<::ValueList<String>>,
        /// Property `Enabled`.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for TrustedSigners {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_account_numbers) = self.aws_account_numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountNumbers", aws_account_numbers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrustedSigners {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrustedSigners, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrustedSigners;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrustedSigners")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_account_numbers: Option<::ValueList<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccountNumbers" => {
                                aws_account_numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrustedSigners {
                        aws_account_numbers: aws_account_numbers,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
