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
    #[serde(rename="CloudFrontOriginAccessIdentityConfig")]
    pub cloud_front_origin_access_identity_config: self::cloud_front_origin_access_identity::CloudFrontOriginAccessIdentityConfig,
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
    #[serde(rename="DistributionConfig")]
    pub distribution_config: self::distribution::DistributionConfig,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
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
    #[serde(rename="StreamingDistributionConfig")]
    pub streaming_distribution_config: self::streaming_distribution::StreamingDistributionConfig,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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
        #[serde(rename="Comment")]
        pub comment: String,
    }
}

pub mod distribution {
    //! Property types for the `Distribution` resource.

    /// The [`AWS::CloudFront::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CacheBehavior {
        /// Property `AllowedMethods`.
        #[serde(rename="AllowedMethods")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<Vec<String>>,
        /// Property `CachedMethods`.
        #[serde(rename="CachedMethods")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cached_methods: Option<Vec<String>>,
        /// Property `Compress`.
        #[serde(rename="Compress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compress: Option<bool>,
        /// Property `DefaultTTL`.
        #[serde(rename="DefaultTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_ttl: Option<f64>,
        /// Property `ForwardedValues`.
        #[serde(rename="ForwardedValues")]
        pub forwarded_values: ForwardedValues,
        /// Property `LambdaFunctionAssociations`.
        #[serde(rename="LambdaFunctionAssociations")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        #[serde(rename="MaxTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_ttl: Option<f64>,
        /// Property `MinTTL`.
        #[serde(rename="MinTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_ttl: Option<f64>,
        /// Property `PathPattern`.
        #[serde(rename="PathPattern")]
        pub path_pattern: String,
        /// Property `SmoothStreaming`.
        #[serde(rename="SmoothStreaming")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub smooth_streaming: Option<bool>,
        /// Property `TargetOriginId`.
        #[serde(rename="TargetOriginId")]
        pub target_origin_id: String,
        /// Property `TrustedSigners`.
        #[serde(rename="TrustedSigners")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trusted_signers: Option<Vec<String>>,
        /// Property `ViewerProtocolPolicy`.
        #[serde(rename="ViewerProtocolPolicy")]
        pub viewer_protocol_policy: String,
    }

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Cookies {
        /// Property `Forward`.
        #[serde(rename="Forward")]
        pub forward: String,
        /// Property `WhitelistedNames`.
        #[serde(rename="WhitelistedNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub whitelisted_names: Option<Vec<String>>,
    }

    /// The [`AWS::CloudFront::Distribution.CustomErrorResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomErrorResponse {
        /// Property `ErrorCachingMinTTL`.
        #[serde(rename="ErrorCachingMinTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_caching_min_ttl: Option<f64>,
        /// Property `ErrorCode`.
        #[serde(rename="ErrorCode")]
        pub error_code: u32,
        /// Property `ResponseCode`.
        #[serde(rename="ResponseCode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub response_code: Option<u32>,
        /// Property `ResponsePagePath`.
        #[serde(rename="ResponsePagePath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub response_page_path: Option<String>,
    }

    /// The [`AWS::CloudFront::Distribution.CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CustomOriginConfig {
        /// Property `HTTPPort`.
        #[serde(rename="HTTPPort")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub http_port: Option<u32>,
        /// Property `HTTPSPort`.
        #[serde(rename="HTTPSPort")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub https_port: Option<u32>,
        /// Property `OriginKeepaliveTimeout`.
        #[serde(rename="OriginKeepaliveTimeout")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_keepalive_timeout: Option<u32>,
        /// Property `OriginProtocolPolicy`.
        #[serde(rename="OriginProtocolPolicy")]
        pub origin_protocol_policy: String,
        /// Property `OriginReadTimeout`.
        #[serde(rename="OriginReadTimeout")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_read_timeout: Option<u32>,
        /// Property `OriginSSLProtocols`.
        #[serde(rename="OriginSSLProtocols")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_ssl_protocols: Option<Vec<String>>,
    }

    /// The [`AWS::CloudFront::Distribution.DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DefaultCacheBehavior {
        /// Property `AllowedMethods`.
        #[serde(rename="AllowedMethods")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<Vec<String>>,
        /// Property `CachedMethods`.
        #[serde(rename="CachedMethods")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cached_methods: Option<Vec<String>>,
        /// Property `Compress`.
        #[serde(rename="Compress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compress: Option<bool>,
        /// Property `DefaultTTL`.
        #[serde(rename="DefaultTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_ttl: Option<f64>,
        /// Property `ForwardedValues`.
        #[serde(rename="ForwardedValues")]
        pub forwarded_values: ForwardedValues,
        /// Property `LambdaFunctionAssociations`.
        #[serde(rename="LambdaFunctionAssociations")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation>>,
        /// Property `MaxTTL`.
        #[serde(rename="MaxTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_ttl: Option<f64>,
        /// Property `MinTTL`.
        #[serde(rename="MinTTL")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_ttl: Option<f64>,
        /// Property `SmoothStreaming`.
        #[serde(rename="SmoothStreaming")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub smooth_streaming: Option<bool>,
        /// Property `TargetOriginId`.
        #[serde(rename="TargetOriginId")]
        pub target_origin_id: String,
        /// Property `TrustedSigners`.
        #[serde(rename="TrustedSigners")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trusted_signers: Option<Vec<String>>,
        /// Property `ViewerProtocolPolicy`.
        #[serde(rename="ViewerProtocolPolicy")]
        pub viewer_protocol_policy: String,
    }

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DistributionConfig {
        /// Property `Aliases`.
        #[serde(rename="Aliases")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub aliases: Option<Vec<String>>,
        /// Property `CacheBehaviors`.
        #[serde(rename="CacheBehaviors")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cache_behaviors: Option<Vec<CacheBehavior>>,
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        /// Property `CustomErrorResponses`.
        #[serde(rename="CustomErrorResponses")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub custom_error_responses: Option<Vec<CustomErrorResponse>>,
        /// Property `DefaultCacheBehavior`.
        #[serde(rename="DefaultCacheBehavior")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_cache_behavior: Option<DefaultCacheBehavior>,
        /// Property `DefaultRootObject`.
        #[serde(rename="DefaultRootObject")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_root_object: Option<String>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `HttpVersion`.
        #[serde(rename="HttpVersion")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub http_version: Option<String>,
        /// Property `IPV6Enabled`.
        #[serde(rename="IPV6Enabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ipv6_enabled: Option<bool>,
        /// Property `Logging`.
        #[serde(rename="Logging")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logging: Option<Logging>,
        /// Property `Origins`.
        #[serde(rename="Origins")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origins: Option<Vec<Origin>>,
        /// Property `PriceClass`.
        #[serde(rename="PriceClass")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price_class: Option<String>,
        /// Property `Restrictions`.
        #[serde(rename="Restrictions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub restrictions: Option<Restrictions>,
        /// Property `ViewerCertificate`.
        #[serde(rename="ViewerCertificate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub viewer_certificate: Option<ViewerCertificate>,
        /// Property `WebACLId`.
        #[serde(rename="WebACLId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub web_acl_id: Option<String>,
    }

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ForwardedValues {
        /// Property `Cookies`.
        #[serde(rename="Cookies")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cookies: Option<Cookies>,
        /// Property `Headers`.
        #[serde(rename="Headers")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<Vec<String>>,
        /// Property `QueryString`.
        #[serde(rename="QueryString")]
        pub query_string: bool,
        /// Property `QueryStringCacheKeys`.
        #[serde(rename="QueryStringCacheKeys")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query_string_cache_keys: Option<Vec<String>>,
    }

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeoRestriction {
        /// Property `Locations`.
        #[serde(rename="Locations")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locations: Option<Vec<String>>,
        /// Property `RestrictionType`.
        #[serde(rename="RestrictionType")]
        pub restriction_type: String,
    }

    /// The [`AWS::CloudFront::Distribution.LambdaFunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaFunctionAssociation {
        /// Property `EventType`.
        #[serde(rename="EventType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub event_type: Option<String>,
        /// Property `LambdaFunctionARN`.
        #[serde(rename="LambdaFunctionARN")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lambda_function_arn: Option<String>,
    }

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Logging {
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `IncludeCookies`.
        #[serde(rename="IncludeCookies")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include_cookies: Option<bool>,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
    }

    /// The [`AWS::CloudFront::Distribution.Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Origin {
        /// Property `CustomOriginConfig`.
        #[serde(rename="CustomOriginConfig")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub custom_origin_config: Option<CustomOriginConfig>,
        /// Property `DomainName`.
        #[serde(rename="DomainName")]
        pub domain_name: String,
        /// Property `Id`.
        #[serde(rename="Id")]
        pub id: String,
        /// Property `OriginCustomHeaders`.
        #[serde(rename="OriginCustomHeaders")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_custom_headers: Option<Vec<OriginCustomHeader>>,
        /// Property `OriginPath`.
        #[serde(rename="OriginPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_path: Option<String>,
        /// Property `S3OriginConfig`.
        #[serde(rename="S3OriginConfig")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_origin_config: Option<S3OriginConfig>,
    }

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OriginCustomHeader {
        /// Property `HeaderName`.
        #[serde(rename="HeaderName")]
        pub header_name: String,
        /// Property `HeaderValue`.
        #[serde(rename="HeaderValue")]
        pub header_value: String,
    }

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Restrictions {
        /// Property `GeoRestriction`.
        #[serde(rename="GeoRestriction")]
        pub geo_restriction: GeoRestriction,
    }

    /// The [`AWS::CloudFront::Distribution.S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3OriginConfig {
        /// Property `OriginAccessIdentity`.
        #[serde(rename="OriginAccessIdentity")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub origin_access_identity: Option<String>,
    }

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ViewerCertificate {
        /// Property `AcmCertificateArn`.
        #[serde(rename="AcmCertificateArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub acm_certificate_arn: Option<String>,
        /// Property `CloudFrontDefaultCertificate`.
        #[serde(rename="CloudFrontDefaultCertificate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cloud_front_default_certificate: Option<bool>,
        /// Property `IamCertificateId`.
        #[serde(rename="IamCertificateId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iam_certificate_id: Option<String>,
        /// Property `MinimumProtocolVersion`.
        #[serde(rename="MinimumProtocolVersion")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub minimum_protocol_version: Option<String>,
        /// Property `SslSupportMethod`.
        #[serde(rename="SslSupportMethod")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ssl_support_method: Option<String>,
    }
}

pub mod streaming_distribution {
    //! Property types for the `StreamingDistribution` resource.

    /// The [`AWS::CloudFront::StreamingDistribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Logging {
        /// Property `Bucket`.
        #[serde(rename="Bucket")]
        pub bucket: String,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `Prefix`.
        #[serde(rename="Prefix")]
        pub prefix: String,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Origin {
        /// Property `DomainName`.
        #[serde(rename="DomainName")]
        pub domain_name: String,
        /// Property `OriginAccessIdentity`.
        #[serde(rename="OriginAccessIdentity")]
        pub origin_access_identity: String,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamingDistributionConfig {
        /// Property `Aliases`.
        #[serde(rename="Aliases")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub aliases: Option<Vec<String>>,
        /// Property `Comment`.
        #[serde(rename="Comment")]
        pub comment: String,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
        /// Property `Logging`.
        #[serde(rename="Logging")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logging: Option<Logging>,
        /// Property `PriceClass`.
        #[serde(rename="PriceClass")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price_class: Option<String>,
        /// Property `S3Origin`.
        #[serde(rename="S3Origin")]
        pub s3_origin: S3Origin,
        /// Property `TrustedSigners`.
        #[serde(rename="TrustedSigners")]
        pub trusted_signers: TrustedSigners,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TrustedSigners {
        /// Property `AwsAccountNumbers`.
        #[serde(rename="AwsAccountNumbers")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub aws_account_numbers: Option<Vec<String>>,
        /// Property `Enabled`.
        #[serde(rename="Enabled")]
        pub enabled: bool,
    }
}
