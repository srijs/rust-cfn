//! Types for the `CloudFront` service.

/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource type.
#[derive(Debug)]
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CloudFrontOriginAccessIdentityProperties {
    /// Property `CloudFrontOriginAccessIdentityConfig`.
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    pub cloud_front_origin_access_identity_config: ::Value<self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig>,
}

impl<'a> ::Resource<'a> for CloudFrontOriginAccessIdentity {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionProperties {
    /// Property `DistributionConfig`.
    #[serde(rename = "DistributionConfig")]
    pub distribution_config: ::Value<self::distribution::DistributionConfig>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
}

impl<'a> ::Resource<'a> for Distribution {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingDistributionProperties {
    /// Property `StreamingDistributionConfig`.
    #[serde(rename = "StreamingDistributionConfig")]
    pub streaming_distribution_config: ::Value<self::streaming_distribution::StreamingDistributionConfig>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    pub tags: ::ValueList<::Tag>,
}

impl<'a> ::Resource<'a> for StreamingDistribution {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloudFrontOriginAccessIdentityConfig {
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        pub comment: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CloudFrontOriginAccessIdentityConfig);
}

pub mod distribution {
    //! Property types for the `Distribution` resource.

    /// The [`AWS::CloudFront::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CacheBehavior {
        /// Property `AllowedMethods`.
        #[serde(rename = "AllowedMethods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property `CachedMethods`.
        #[serde(rename = "CachedMethods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cached_methods: Option<::ValueList<String>>,
        /// Property `Compress`.
        #[serde(rename = "Compress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compress: Option<::Value<bool>>,
        /// Property `DefaultTTL`.
        #[serde(rename = "DefaultTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_ttl: Option<::Value<f64>>,
        /// Property `ForwardedValues`.
        #[serde(rename = "ForwardedValues")]
        pub forwarded_values: ::Value<ForwardedValues>,
        /// Property `LambdaFunctionAssociations`.
        #[serde(rename = "LambdaFunctionAssociations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        #[serde(rename = "MaxTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_ttl: Option<::Value<f64>>,
        /// Property `MinTTL`.
        #[serde(rename = "MinTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_ttl: Option<::Value<f64>>,
        /// Property `PathPattern`.
        #[serde(rename = "PathPattern")]
        pub path_pattern: ::Value<String>,
        /// Property `SmoothStreaming`.
        #[serde(rename = "SmoothStreaming")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property `TargetOriginId`.
        #[serde(rename = "TargetOriginId")]
        pub target_origin_id: ::Value<String>,
        /// Property `TrustedSigners`.
        #[serde(rename = "TrustedSigners")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property `ViewerProtocolPolicy`.
        #[serde(rename = "ViewerProtocolPolicy")]
        pub viewer_protocol_policy: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CacheBehavior);

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Cookies {
        /// Property `Forward`.
        #[serde(rename = "Forward")]
        pub forward: ::Value<String>,
        /// Property `WhitelistedNames`.
        #[serde(rename = "WhitelistedNames")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub whitelisted_names: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(Cookies);

    /// The [`AWS::CloudFront::Distribution.CustomErrorResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomErrorResponse {
        /// Property `ErrorCachingMinTTL`.
        #[serde(rename = "ErrorCachingMinTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_caching_min_ttl: Option<::Value<f64>>,
        /// Property `ErrorCode`.
        #[serde(rename = "ErrorCode")]
        pub error_code: ::Value<u32>,
        /// Property `ResponseCode`.
        #[serde(rename = "ResponseCode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_code: Option<::Value<u32>>,
        /// Property `ResponsePagePath`.
        #[serde(rename = "ResponsePagePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub response_page_path: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CustomErrorResponse);

    /// The [`AWS::CloudFront::Distribution.CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomOriginConfig {
        /// Property `HTTPPort`.
        #[serde(rename = "HTTPPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_port: Option<::Value<u32>>,
        /// Property `HTTPSPort`.
        #[serde(rename = "HTTPSPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub https_port: Option<::Value<u32>>,
        /// Property `OriginKeepaliveTimeout`.
        #[serde(rename = "OriginKeepaliveTimeout")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_keepalive_timeout: Option<::Value<u32>>,
        /// Property `OriginProtocolPolicy`.
        #[serde(rename = "OriginProtocolPolicy")]
        pub origin_protocol_policy: ::Value<String>,
        /// Property `OriginReadTimeout`.
        #[serde(rename = "OriginReadTimeout")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_read_timeout: Option<::Value<u32>>,
        /// Property `OriginSSLProtocols`.
        #[serde(rename = "OriginSSLProtocols")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_ssl_protocols: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(CustomOriginConfig);

    /// The [`AWS::CloudFront::Distribution.DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DefaultCacheBehavior {
        /// Property `AllowedMethods`.
        #[serde(rename = "AllowedMethods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<::ValueList<String>>,
        /// Property `CachedMethods`.
        #[serde(rename = "CachedMethods")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cached_methods: Option<::ValueList<String>>,
        /// Property `Compress`.
        #[serde(rename = "Compress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compress: Option<::Value<bool>>,
        /// Property `DefaultTTL`.
        #[serde(rename = "DefaultTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_ttl: Option<::Value<f64>>,
        /// Property `ForwardedValues`.
        #[serde(rename = "ForwardedValues")]
        pub forwarded_values: ::Value<ForwardedValues>,
        /// Property `LambdaFunctionAssociations`.
        #[serde(rename = "LambdaFunctionAssociations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda_function_associations: Option<::ValueList<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        #[serde(rename = "MaxTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_ttl: Option<::Value<f64>>,
        /// Property `MinTTL`.
        #[serde(rename = "MinTTL")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_ttl: Option<::Value<f64>>,
        /// Property `SmoothStreaming`.
        #[serde(rename = "SmoothStreaming")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub smooth_streaming: Option<::Value<bool>>,
        /// Property `TargetOriginId`.
        #[serde(rename = "TargetOriginId")]
        pub target_origin_id: ::Value<String>,
        /// Property `TrustedSigners`.
        #[serde(rename = "TrustedSigners")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trusted_signers: Option<::ValueList<String>>,
        /// Property `ViewerProtocolPolicy`.
        #[serde(rename = "ViewerProtocolPolicy")]
        pub viewer_protocol_policy: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(DefaultCacheBehavior);

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DistributionConfig {
        /// Property `Aliases`.
        #[serde(rename = "Aliases")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aliases: Option<::ValueList<String>>,
        /// Property `CacheBehaviors`.
        #[serde(rename = "CacheBehaviors")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cache_behaviors: Option<::ValueList<CacheBehavior>>,
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<::Value<String>>,
        /// Property `CustomErrorResponses`.
        #[serde(rename = "CustomErrorResponses")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_error_responses: Option<::ValueList<CustomErrorResponse>>,
        /// Property `DefaultCacheBehavior`.
        #[serde(rename = "DefaultCacheBehavior")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_cache_behavior: Option<::Value<DefaultCacheBehavior>>,
        /// Property `DefaultRootObject`.
        #[serde(rename = "DefaultRootObject")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub default_root_object: Option<::Value<String>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `HttpVersion`.
        #[serde(rename = "HttpVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub http_version: Option<::Value<String>>,
        /// Property `IPV6Enabled`.
        #[serde(rename = "IPV6Enabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ipv6_enabled: Option<::Value<bool>>,
        /// Property `Logging`.
        #[serde(rename = "Logging")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logging: Option<::Value<Logging>>,
        /// Property `Origins`.
        #[serde(rename = "Origins")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origins: Option<::ValueList<Origin>>,
        /// Property `PriceClass`.
        #[serde(rename = "PriceClass")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub price_class: Option<::Value<String>>,
        /// Property `Restrictions`.
        #[serde(rename = "Restrictions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub restrictions: Option<::Value<Restrictions>>,
        /// Property `ViewerCertificate`.
        #[serde(rename = "ViewerCertificate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub viewer_certificate: Option<::Value<ViewerCertificate>>,
        /// Property `WebACLId`.
        #[serde(rename = "WebACLId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_acl_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DistributionConfig);

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ForwardedValues {
        /// Property `Cookies`.
        #[serde(rename = "Cookies")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cookies: Option<::Value<Cookies>>,
        /// Property `Headers`.
        #[serde(rename = "Headers")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub headers: Option<::ValueList<String>>,
        /// Property `QueryString`.
        #[serde(rename = "QueryString")]
        pub query_string: ::Value<bool>,
        /// Property `QueryStringCacheKeys`.
        #[serde(rename = "QueryStringCacheKeys")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query_string_cache_keys: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(ForwardedValues);

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoRestriction {
        /// Property `Locations`.
        #[serde(rename = "Locations")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locations: Option<::ValueList<String>>,
        /// Property `RestrictionType`.
        #[serde(rename = "RestrictionType")]
        pub restriction_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(GeoRestriction);

    /// The [`AWS::CloudFront::Distribution.LambdaFunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaFunctionAssociation {
        /// Property `EventType`.
        #[serde(rename = "EventType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub event_type: Option<::Value<String>>,
        /// Property `LambdaFunctionARN`.
        #[serde(rename = "LambdaFunctionARN")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda_function_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LambdaFunctionAssociation);

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Logging {
        /// Property `Bucket`.
        #[serde(rename = "Bucket")]
        pub bucket: ::Value<String>,
        /// Property `IncludeCookies`.
        #[serde(rename = "IncludeCookies")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub include_cookies: Option<::Value<bool>>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub prefix: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Logging);

    /// The [`AWS::CloudFront::Distribution.Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Origin {
        /// Property `CustomOriginConfig`.
        #[serde(rename = "CustomOriginConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_origin_config: Option<::Value<CustomOriginConfig>>,
        /// Property `DomainName`.
        #[serde(rename = "DomainName")]
        pub domain_name: ::Value<String>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `OriginCustomHeaders`.
        #[serde(rename = "OriginCustomHeaders")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_custom_headers: Option<::ValueList<OriginCustomHeader>>,
        /// Property `OriginPath`.
        #[serde(rename = "OriginPath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_path: Option<::Value<String>>,
        /// Property `S3OriginConfig`.
        #[serde(rename = "S3OriginConfig")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_origin_config: Option<::Value<S3OriginConfig>>,
    }

    cfn_internal__inherit_codec_impls!(Origin);

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OriginCustomHeader {
        /// Property `HeaderName`.
        #[serde(rename = "HeaderName")]
        pub header_name: ::Value<String>,
        /// Property `HeaderValue`.
        #[serde(rename = "HeaderValue")]
        pub header_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(OriginCustomHeader);

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Restrictions {
        /// Property `GeoRestriction`.
        #[serde(rename = "GeoRestriction")]
        pub geo_restriction: ::Value<GeoRestriction>,
    }

    cfn_internal__inherit_codec_impls!(Restrictions);

    /// The [`AWS::CloudFront::Distribution.S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3OriginConfig {
        /// Property `OriginAccessIdentity`.
        #[serde(rename = "OriginAccessIdentity")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin_access_identity: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(S3OriginConfig);

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ViewerCertificate {
        /// Property `AcmCertificateArn`.
        #[serde(rename = "AcmCertificateArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub acm_certificate_arn: Option<::Value<String>>,
        /// Property `CloudFrontDefaultCertificate`.
        #[serde(rename = "CloudFrontDefaultCertificate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cloud_front_default_certificate: Option<::Value<bool>>,
        /// Property `IamCertificateId`.
        #[serde(rename = "IamCertificateId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iam_certificate_id: Option<::Value<String>>,
        /// Property `MinimumProtocolVersion`.
        #[serde(rename = "MinimumProtocolVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub minimum_protocol_version: Option<::Value<String>>,
        /// Property `SslSupportMethod`.
        #[serde(rename = "SslSupportMethod")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssl_support_method: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ViewerCertificate);
}

pub mod streaming_distribution {
    //! Property types for the `StreamingDistribution` resource.

    /// The [`AWS::CloudFront::StreamingDistribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Logging {
        /// Property `Bucket`.
        #[serde(rename = "Bucket")]
        pub bucket: ::Value<String>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `Prefix`.
        #[serde(rename = "Prefix")]
        pub prefix: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Logging);

    /// The [`AWS::CloudFront::StreamingDistribution.S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Origin {
        /// Property `DomainName`.
        #[serde(rename = "DomainName")]
        pub domain_name: ::Value<String>,
        /// Property `OriginAccessIdentity`.
        #[serde(rename = "OriginAccessIdentity")]
        pub origin_access_identity: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(S3Origin);

    /// The [`AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamingDistributionConfig {
        /// Property `Aliases`.
        #[serde(rename = "Aliases")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aliases: Option<::ValueList<String>>,
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        pub comment: ::Value<String>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
        /// Property `Logging`.
        #[serde(rename = "Logging")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logging: Option<::Value<Logging>>,
        /// Property `PriceClass`.
        #[serde(rename = "PriceClass")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub price_class: Option<::Value<String>>,
        /// Property `S3Origin`.
        #[serde(rename = "S3Origin")]
        pub s3_origin: ::Value<S3Origin>,
        /// Property `TrustedSigners`.
        #[serde(rename = "TrustedSigners")]
        pub trusted_signers: ::Value<TrustedSigners>,
    }

    cfn_internal__inherit_codec_impls!(StreamingDistributionConfig);

    /// The [`AWS::CloudFront::StreamingDistribution.TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TrustedSigners {
        /// Property `AwsAccountNumbers`.
        #[serde(rename = "AwsAccountNumbers")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aws_account_numbers: Option<::ValueList<String>>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
    }

    cfn_internal__inherit_codec_impls!(TrustedSigners);
}
