/// The [`AWS::CloudFront::CloudFrontOriginAccessIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html) resource.
#[derive(Serialize, Deserialize)]
pub struct CloudFrontOriginAccessIdentity {
    properties: CloudFrontOriginAccessIdentityProperties
}

/// Properties for the `CloudFrontOriginAccessIdentity` resource.
#[derive(Serialize, Deserialize)]
pub struct CloudFrontOriginAccessIdentityProperties {
    #[serde(rename="CloudFrontOriginAccessIdentityConfig")]
    pub cloud_front_origin_access_identity_config: (),
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

impl From<CloudFrontOriginAccessIdentityProperties> for CloudFrontOriginAccessIdentity {
    fn from(properties: CloudFrontOriginAccessIdentityProperties) -> CloudFrontOriginAccessIdentity {
        CloudFrontOriginAccessIdentity { properties }
    }
}

/// The [`AWS::CloudFront::Distribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Distribution {
    properties: DistributionProperties
}

/// Properties for the `Distribution` resource.
#[derive(Serialize, Deserialize)]
pub struct DistributionProperties {
    #[serde(rename="DistributionConfig")]
    pub distribution_config: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

impl From<DistributionProperties> for Distribution {
    fn from(properties: DistributionProperties) -> Distribution {
        Distribution { properties }
    }
}

/// The [`AWS::CloudFront::StreamingDistribution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html) resource.
#[derive(Serialize, Deserialize)]
pub struct StreamingDistribution {
    properties: StreamingDistributionProperties
}

/// Properties for the `StreamingDistribution` resource.
#[derive(Serialize, Deserialize)]
pub struct StreamingDistributionProperties {
    #[serde(rename="StreamingDistributionConfig")]
    pub streaming_distribution_config: (),
    #[serde(rename="Tags")]
    pub tags: (),
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

impl From<StreamingDistributionProperties> for StreamingDistribution {
    fn from(properties: StreamingDistributionProperties) -> StreamingDistribution {
        StreamingDistribution { properties }
    }
}

