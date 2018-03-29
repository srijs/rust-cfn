/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource type.
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Serialize, Deserialize)]
pub struct CloudFrontOriginAccessIdentityProperties {
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
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Serialize, Deserialize)]
pub struct DistributionProperties {
    #[serde(rename="DistributionConfig")]
    pub distribution_config: self::distribution::DistributionConfig,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
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
pub struct StreamingDistribution {
    properties: StreamingDistributionProperties
}

/// Properties for the `StreamingDistribution` resource.
#[derive(Serialize, Deserialize)]
pub struct StreamingDistributionProperties {
    #[serde(rename="StreamingDistributionConfig")]
    pub streaming_distribution_config: self::streaming_distribution::StreamingDistributionConfig,
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
    /// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity.CloudFrontOriginAccessIdentityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct CloudFrontOriginAccessIdentityConfig {
        #[serde(rename="Comment")]
        pub comment: String,
    }

}

pub mod distribution {
    /// The [`AWS::CloudFront::Distribution.CacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct CacheBehavior {
        #[serde(rename="AllowedMethods")]
        pub allowed_methods: Vec<String>,
        #[serde(rename="CachedMethods")]
        pub cached_methods: Vec<String>,
        #[serde(rename="Compress")]
        pub compress: bool,
        #[serde(rename="DefaultTTL")]
        pub default_ttl: f64,
        #[serde(rename="ForwardedValues")]
        pub forwarded_values: ForwardedValues,
        #[serde(rename="LambdaFunctionAssociations")]
        pub lambda_function_associations: Vec<LambdaFunctionAssociation>,
        #[serde(rename="MaxTTL")]
        pub max_ttl: f64,
        #[serde(rename="MinTTL")]
        pub min_ttl: f64,
        #[serde(rename="PathPattern")]
        pub path_pattern: String,
        #[serde(rename="SmoothStreaming")]
        pub smooth_streaming: bool,
        #[serde(rename="TargetOriginId")]
        pub target_origin_id: String,
        #[serde(rename="TrustedSigners")]
        pub trusted_signers: Vec<String>,
        #[serde(rename="ViewerProtocolPolicy")]
        pub viewer_protocol_policy: String,
    }

    /// The [`AWS::CloudFront::Distribution.Cookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Cookies {
        #[serde(rename="Forward")]
        pub forward: String,
        #[serde(rename="WhitelistedNames")]
        pub whitelisted_names: Vec<String>,
    }

    /// The [`AWS::CloudFront::Distribution.CustomErrorResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct CustomErrorResponse {
        #[serde(rename="ErrorCachingMinTTL")]
        pub error_caching_min_ttl: f64,
        #[serde(rename="ErrorCode")]
        pub error_code: u32,
        #[serde(rename="ResponseCode")]
        pub response_code: u32,
        #[serde(rename="ResponsePagePath")]
        pub response_page_path: String,
    }

    /// The [`AWS::CloudFront::Distribution.CustomOriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct CustomOriginConfig {
        #[serde(rename="HTTPPort")]
        pub http_port: u32,
        #[serde(rename="HTTPSPort")]
        pub https_port: u32,
        #[serde(rename="OriginKeepaliveTimeout")]
        pub origin_keepalive_timeout: u32,
        #[serde(rename="OriginProtocolPolicy")]
        pub origin_protocol_policy: String,
        #[serde(rename="OriginReadTimeout")]
        pub origin_read_timeout: u32,
        #[serde(rename="OriginSSLProtocols")]
        pub origin_ssl_protocols: Vec<String>,
    }

    /// The [`AWS::CloudFront::Distribution.DefaultCacheBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DefaultCacheBehavior {
        #[serde(rename="AllowedMethods")]
        pub allowed_methods: Vec<String>,
        #[serde(rename="CachedMethods")]
        pub cached_methods: Vec<String>,
        #[serde(rename="Compress")]
        pub compress: bool,
        #[serde(rename="DefaultTTL")]
        pub default_ttl: f64,
        #[serde(rename="ForwardedValues")]
        pub forwarded_values: ForwardedValues,
        #[serde(rename="LambdaFunctionAssociations")]
        pub lambda_function_associations: Vec<LambdaFunctionAssociation>,
        #[serde(rename="MaxTTL")]
        pub max_ttl: f64,
        #[serde(rename="MinTTL")]
        pub min_ttl: f64,
        #[serde(rename="SmoothStreaming")]
        pub smooth_streaming: bool,
        #[serde(rename="TargetOriginId")]
        pub target_origin_id: String,
        #[serde(rename="TrustedSigners")]
        pub trusted_signers: Vec<String>,
        #[serde(rename="ViewerProtocolPolicy")]
        pub viewer_protocol_policy: String,
    }

    /// The [`AWS::CloudFront::Distribution.DistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DistributionConfig {
        #[serde(rename="Aliases")]
        pub aliases: Vec<String>,
        #[serde(rename="CacheBehaviors")]
        pub cache_behaviors: Vec<CacheBehavior>,
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="CustomErrorResponses")]
        pub custom_error_responses: Vec<CustomErrorResponse>,
        #[serde(rename="DefaultCacheBehavior")]
        pub default_cache_behavior: DefaultCacheBehavior,
        #[serde(rename="DefaultRootObject")]
        pub default_root_object: String,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="HttpVersion")]
        pub http_version: String,
        #[serde(rename="IPV6Enabled")]
        pub ipv6_enabled: bool,
        #[serde(rename="Logging")]
        pub logging: Logging,
        #[serde(rename="Origins")]
        pub origins: Vec<Origin>,
        #[serde(rename="PriceClass")]
        pub price_class: String,
        #[serde(rename="Restrictions")]
        pub restrictions: Restrictions,
        #[serde(rename="ViewerCertificate")]
        pub viewer_certificate: ViewerCertificate,
        #[serde(rename="WebACLId")]
        pub web_acl_id: String,
    }

    /// The [`AWS::CloudFront::Distribution.ForwardedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ForwardedValues {
        #[serde(rename="Cookies")]
        pub cookies: Cookies,
        #[serde(rename="Headers")]
        pub headers: Vec<String>,
        #[serde(rename="QueryString")]
        pub query_string: bool,
        #[serde(rename="QueryStringCacheKeys")]
        pub query_string_cache_keys: Vec<String>,
    }

    /// The [`AWS::CloudFront::Distribution.GeoRestriction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct GeoRestriction {
        #[serde(rename="Locations")]
        pub locations: Vec<String>,
        #[serde(rename="RestrictionType")]
        pub restriction_type: String,
    }

    /// The [`AWS::CloudFront::Distribution.LambdaFunctionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LambdaFunctionAssociation {
        #[serde(rename="EventType")]
        pub event_type: String,
        #[serde(rename="LambdaFunctionARN")]
        pub lambda_function_arn: String,
    }

    /// The [`AWS::CloudFront::Distribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Logging {
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="IncludeCookies")]
        pub include_cookies: bool,
        #[serde(rename="Prefix")]
        pub prefix: String,
    }

    /// The [`AWS::CloudFront::Distribution.Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Origin {
        #[serde(rename="CustomOriginConfig")]
        pub custom_origin_config: CustomOriginConfig,
        #[serde(rename="DomainName")]
        pub domain_name: String,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="OriginCustomHeaders")]
        pub origin_custom_headers: Vec<OriginCustomHeader>,
        #[serde(rename="OriginPath")]
        pub origin_path: String,
        #[serde(rename="S3OriginConfig")]
        pub s3_origin_config: S3OriginConfig,
    }

    /// The [`AWS::CloudFront::Distribution.OriginCustomHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct OriginCustomHeader {
        #[serde(rename="HeaderName")]
        pub header_name: String,
        #[serde(rename="HeaderValue")]
        pub header_value: String,
    }

    /// The [`AWS::CloudFront::Distribution.Restrictions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Restrictions {
        #[serde(rename="GeoRestriction")]
        pub geo_restriction: GeoRestriction,
    }

    /// The [`AWS::CloudFront::Distribution.S3OriginConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3OriginConfig {
        #[serde(rename="OriginAccessIdentity")]
        pub origin_access_identity: String,
    }

    /// The [`AWS::CloudFront::Distribution.ViewerCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ViewerCertificate {
        #[serde(rename="AcmCertificateArn")]
        pub acm_certificate_arn: String,
        #[serde(rename="CloudFrontDefaultCertificate")]
        pub cloud_front_default_certificate: bool,
        #[serde(rename="IamCertificateId")]
        pub iam_certificate_id: String,
        #[serde(rename="MinimumProtocolVersion")]
        pub minimum_protocol_version: String,
        #[serde(rename="SslSupportMethod")]
        pub ssl_support_method: String,
    }

}

pub mod streaming_distribution {
    /// The [`AWS::CloudFront::StreamingDistribution.Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Logging {
        #[serde(rename="Bucket")]
        pub bucket: String,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Prefix")]
        pub prefix: String,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.S3Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3Origin {
        #[serde(rename="DomainName")]
        pub domain_name: String,
        #[serde(rename="OriginAccessIdentity")]
        pub origin_access_identity: String,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StreamingDistributionConfig {
        #[serde(rename="Aliases")]
        pub aliases: Vec<String>,
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="Enabled")]
        pub enabled: bool,
        #[serde(rename="Logging")]
        pub logging: Logging,
        #[serde(rename="PriceClass")]
        pub price_class: String,
        #[serde(rename="S3Origin")]
        pub s3_origin: S3Origin,
        #[serde(rename="TrustedSigners")]
        pub trusted_signers: TrustedSigners,
    }

    /// The [`AWS::CloudFront::StreamingDistribution.TrustedSigners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TrustedSigners {
        #[serde(rename="AwsAccountNumbers")]
        pub aws_account_numbers: Vec<String>,
        #[serde(rename="Enabled")]
        pub enabled: bool,
    }

}

